use super::*;
use crate::audio::sound_effect;
use crate::leaderboard::AddUserScore;
use crate::{
    gameplay::{apple::Apple, health::Death, tractor::Tractor},
    screens::Screen,
};

#[derive(Resource, Default)]
pub struct ScoreCounter {
    pub points: usize,
}

#[derive(Resource)]
struct ScoreAssets {
    sound: Handle<AudioSource>,
}

impl FromWorld for ScoreAssets {
    fn from_world(world: &mut World) -> Self {
        let assets: &AssetServer = world.resource::<AssetServer>();
        Self {
            sound: assets.load::<AudioSource>("audio/sound_effects/point.wav"),
        }
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<ScoreCounter>();
    app.init_resource::<ScoreAssets>();

    app.add_systems(
        OnEnter(Screen::InGame),
        |mut score: ResMut<ScoreCounter>| {
            score.points = 0;
        },
    );

    app.add_observer(
        |trigger: Trigger<Death>,
         mut commands: Commands,
         mut score: ResMut<ScoreCounter>,
         apples: Query<&Apple>,
         tractor: Query<&Tractor>,
         assets: Res<ScoreAssets>| {
            if let Ok(_apple) = apples.get(trigger.target()) {
                score.points += 1;
                commands.spawn(sound_effect(assets.sound.clone()));
            }
            if let Ok(_tractor) = tractor.get(trigger.target()) {
                commands.trigger(AddUserScore {
                    value: score.points as f32,
                });
            }
        },
    );
}
