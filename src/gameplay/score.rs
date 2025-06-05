use crate::{
    gameplay::{apple::Apple, health::Death, tractor::Tractor},
    screens::Screen,
};
use crate::leaderboard::AddUserScore;
use super::*;

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
         mut event_writer: EventWriter<AddUserScore>,
         mut score: ResMut<ScoreCounter>,
         apples: Query<&Apple>,
         tractor: Query<&Tractor>| {
            if let Ok(_apple) = apples.get(trigger.target()) {
                score.points += 1;
            }
            if let Ok(_tractor) = tractor.get(trigger.target()) {
                event_writer.write(AddUserScore {
                    value: score.points as f32,
                });
            }
        },
    );
}
