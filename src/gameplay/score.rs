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
#[derive(Resource, Default)]
pub struct Currency(u32);
impl Currency {
    pub fn reset(&mut self) {
        self.0 = 0;
    }
    pub fn add(&mut self, val: u32) {
        self.0 += val;
    }

    /// returns true if the points were spent, false if they could not
    pub fn spend(&mut self, val: u32) -> bool {
        if self.0 >= val {
            self.0 -= val;
            true
        } else {
            false
        }
    }

    pub fn get(&self) -> u32 {
        self.0
    }
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

    app.init_resource::<Currency>();

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
         mut currency: ResMut<Currency>,
         apples: Query<&Apple>,
         tractor: Query<&Tractor>,
         assets: Res<ScoreAssets>| {
            if let Ok(_apple) = apples.get(trigger.target()) {
                score.points += 1;
                if score.points % 10 == 0 {
                    currency.add(1);
                }
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
