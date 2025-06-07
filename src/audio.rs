use bevy::{audio::*, prelude::*};

const FADE_TIME: f32 = 1.0;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Music>();
    app.register_type::<SoundEffect>();
    app.add_event::<ChangeMusicEvent>();

    app.add_systems(
        Update,
        (
            apply_global_volume.run_if(resource_changed::<GlobalVolume>),
            change_music,
            fade_in,
            fade_out,
        ),
    );
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "music" category (e.g. global background music, soundtrack).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Music;

#[derive(Component)]
struct FadeIn;

#[derive(Component)]
struct FadeOut;

#[derive(Event)]
pub struct ChangeMusicEvent {
    pub music: Handle<AudioSource>,
}

/// A music audio instance.
pub fn music(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::LOOP, Music, FadeIn)
}

pub fn change_music(
    mut events: EventReader<ChangeMusicEvent>,
    mut commands: Commands,
    musics: Query<Entity, With<AudioSink>>,
) {
    for event in events.read() {
        for track in musics.iter() {
            commands.entity(track).insert(FadeOut);
        }

        commands.spawn((
            AudioPlayer(event.music.clone()),
            Music,
            PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::SILENT,
                ..default()
            },
            FadeIn,
        ));
    }
}

// Fades in the audio of entities that has the FadeIn component. Removes the FadeIn component once
// full volume is reached.
fn fade_in(
    mut commands: Commands,
    mut audio_sink: Query<(&mut AudioSink, Entity), With<FadeIn>>,
    time: Res<Time>,
) {
    for (mut audio, entity) in audio_sink.iter_mut() {
        let current_volume = audio.volume();
        audio.set_volume(current_volume + Volume::Linear(time.delta_secs() / FADE_TIME));
        if audio.volume().to_linear() >= 1.0 {
            audio.set_volume(Volume::Linear(1.0));
            commands.entity(entity).remove::<FadeIn>();
        }
    }
}

// Fades out the audio of entities that has the FadeOut component. Despawns the entities once audio
// volume reaches zero.
fn fade_out(
    mut commands: Commands,
    mut audio_sink: Query<(&mut AudioSink, Entity), With<FadeOut>>,
    time: Res<Time>,
) {
    for (mut audio, entity) in audio_sink.iter_mut() {
        let current_volume = audio.volume();
        audio.set_volume(current_volume - Volume::Linear(time.delta_secs() / FADE_TIME));
        if audio.volume().to_linear() <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "sound effect" category (e.g. footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SoundEffect;

/// A sound effect audio instance.
pub fn sound_effect(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::DESPAWN, SoundEffect)
}

/// [`GlobalVolume`] doesn't apply to already-running audio entities, so this system will update them.
fn apply_global_volume(
    global_volume: Res<GlobalVolume>,
    mut audio_query: Query<(&PlaybackSettings, &mut AudioSink)>,
) {
    for (playback, mut sink) in &mut audio_query {
        sink.set_volume(global_volume.volume * playback.volume);
    }
}
