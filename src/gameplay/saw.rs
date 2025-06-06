use crate::{
    gameplay::{health::DamageEvent, tractor::TractorSaw},
    screens::Screen,
};

use super::*;

#[derive(Component, Debug)]
pub struct Sawable {
    timer: Timer,
}

impl Default for Sawable {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.0, TimerMode::Once),
        }
    }
}

fn check_saw_colitions(
    collisions: Collisions,
    sawables: Query<(Entity, &mut Sawable)>,
    saw: Single<(Entity, &TractorSaw)>,
    mut commands: Commands,
) {
    let (saw_entity, saw) = saw.into_inner();

    for (sawable_entity, mut sawable) in sawables {
        if collisions.contains(sawable_entity, saw_entity) && sawable.timer.finished() {
            // Object is currently beeing damaged by the saw
            commands.send_event(DamageEvent {
                value: saw.damage,
                entity: sawable_entity,
            });
            // Update rate of fire
            sawable.timer.set_duration(saw.rate_of_fire);
            sawable.timer.reset();
        }
    }
}

fn check_sawable_timers(
    mut sawables: Query<&mut Sawable>,
    time: Res<Time>,
    saw: Single<&TractorSaw>,
) {
    for mut sawable in sawables.iter_mut() {
        sawable.timer.tick(time.delta());
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            check_saw_colitions.run_if(in_state(Screen::InGame)),
            check_sawable_timers.run_if(in_state(Screen::InGame)),
        ),
    );
}
