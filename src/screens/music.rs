use bevy::prelude::*;

use crate::audio::{ChangeMusicEvent, Music, music};

use super::*;

#[derive(Component)]
struct FadeOut;

#[derive(Component)]
struct FadeIn;

#[derive(Resource)]
struct MusicAssets {
    title: Handle<AudioSource>,
    in_game: Handle<AudioSource>,
}

impl FromWorld for MusicAssets {
    fn from_world(world: &mut World) -> Self {
        let assets: &AssetServer = world.resource::<AssetServer>();
        Self {
            title: assets.load::<AudioSource>("audio/music/wondrous-waters-119518.ogg"),
            in_game: assets.load::<AudioSource>("audio/music/Swing-Machine-chosic.com_.ogg"),
        }
    }
}

fn check_screen_state(
    mut commands: Commands,
    assets: Res<MusicAssets>,
    screen: Res<State<Screen>>,
) {
    if !screen.is_changed() {
        return;
    }
    match screen.get() {
        Screen::Title => {
            commands.send_event(ChangeMusicEvent {
                music: assets.title.clone(),
            });
        }
        Screen::InGame => {
            commands.send_event(ChangeMusicEvent {
                music: assets.in_game.clone(),
            });
        }
        _ => {}
    };

    // commands.send_event(ChangeMusicEvent { music: music_track });
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MusicAssets>();
    app.add_systems(Update, check_screen_state);
}
