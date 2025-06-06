use bevy::prelude::*;

use crate::asset_tracking::LoadResource;

#[derive(Resource)]
pub struct SoundEffects {
    fire: Handle<AudioSource>,
    hurt: Handle<AudioSource>,
}

impl SoundEffects {
    pub fn play_sound(&self, commands: &mut Commands, sound: SoundEffectType) {
        match sound {
            SoundEffectType::Fire => {
                commands.spawn(sound_effect(self.fire.clone()));
            }
            SoundEffectType::Hurt => {
                commands.spawn(sound_effect(self.hurt.clone()));
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum SoundEffectType {
    Fire,
    Hurt,
}

impl FromWorld for SoundEffects {
    fn from_world(world: &mut World) -> Self {
        let assets: &AssetServer = world.resource::<AssetServer>();
        Self {
            fire: assets.load::<AudioSource>("audio/sound_effects/gunfire.wav"),
            hurt: assets.load::<AudioSource>("audio/sound_effects/tractor-damage.wav"),
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Music>();
    app.register_type::<SoundEffect>();
    app.init_resource::<SoundEffects>();

    app.add_systems(
        Update,
        apply_global_volume.run_if(resource_changed::<GlobalVolume>),
    );

    app.add_systems(Startup, setup_background_music);
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "music" category (e.g. global background music, soundtrack).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Music;

/// A music audio instance.
pub fn music(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::LOOP, Music)
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

fn setup_background_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_music = asset_server.load::<AudioSource>("audio/music/Swing-Machine-chosic.com_.ogg");
    commands.spawn(music(game_music));
}
