use super::*;
use crate::leaderboard::AddUserScore;
use crate::{
    gameplay::{apple::Apple, health::Death, tractor::Tractor},
    screens::Screen,
};

#[derive(Resource, Default)]
pub struct ScoreCounter {
    pub points: usize,
}

pub fn plugin(app: &mut App) {
    app.init_resource::<ScoreCounter>();

    app.add_systems(
        OnEnter(Screen::Gameplay),
        |mut score: ResMut<ScoreCounter>| {
            score.points = 0;
        },
    );

    app.add_observer(
        |trigger: Trigger<Death>,
         mut commands: Commands,
         mut score: ResMut<ScoreCounter>,
         apples: Query<&Apple>,
         tractor: Query<&Tractor>| {
            if let Ok(_apple) = apples.get(trigger.target()) {
                score.points += 1;
            }
            if let Ok(_tractor) = tractor.get(trigger.target()) {
                commands.trigger(AddUserScore {
                    value: score.points as f32,
                });
            }
        },
    );
}
