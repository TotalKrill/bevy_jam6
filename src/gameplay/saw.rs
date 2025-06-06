use crate::{
    gameplay::{health::Damage, tractor::TractorSaw},
    screens::Screen,
};

use super::*;

#[derive(Component, Debug)]
pub struct Sawable {
    timer: Timer,
}

impl Default for Sawable {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(0.0, TimerMode::Once);
        timer.pause();
        Self { timer }
    }
}

fn saw_collition_started(
    mut collision_event_reader: EventReader<CollisionStarted>,
    saw_entity: Single<Entity, With<TractorSaw>>,
    mut sawable: Query<&mut Sawable>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        if (*entity1 == *saw_entity) || (*entity2 == *saw_entity) {
            if let Ok(mut sawable) = sawable.get_mut(*entity1) {
                sawable.timer.unpause();
            } else if let Ok(mut sawable) = sawable.get_mut(*entity2) {
                sawable.timer.unpause();
            }
        }
    }
}

fn saw_collition_ended(
    mut collision_event_reader: EventReader<CollisionEnded>,
    saw_entity: Single<Entity, With<TractorSaw>>,
    mut sawable: Query<&mut Sawable>,
) {
    for CollisionEnded(entity1, entity2) in collision_event_reader.read() {
        if (*entity1 == *saw_entity) || (*entity2 == *saw_entity) {
            if let Ok(mut sawable) = sawable.get_mut(*entity1) {
                sawable.timer.pause();
            } else if let Ok(mut sawable) = sawable.get_mut(*entity2) {
                sawable.timer.pause();
            }
        }
    }
}

fn check_sawable_timers(
    mut commands: Commands,
    mut sawables: Query<(Entity, &mut Sawable)>,
    time: Res<Time>,
    saw: Single<&TractorSaw>,
) {
    for (entity, mut sawable) in sawables.iter_mut() {
        if sawable.timer.paused() {
            continue; // Skip if the timer is paused
        }

        sawable.timer.tick(time.delta());
        if sawable.timer.finished() {
            sawable.timer.set_duration(saw.rate_of_fire);
            sawable.timer.reset();

            commands.send_event(Damage {
                value: saw.damage,
                entity: entity,
            });
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            saw_collition_started.run_if(in_state(Screen::InGame)),
            saw_collition_ended.run_if(in_state(Screen::InGame)),
            check_sawable_timers.run_if(in_state(Screen::InGame)),
        ),
    );
}
