use std::time::Duration;

use bevy::state::commands;

use crate::{
    gameplay::{health::Damage, tractor::TractorSaw, tree::Tree},
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
    saw: Single<(Entity, &TractorSaw)>,
    mut sawable: Query<(Entity, &mut Sawable)>,
) {
    let (saw_entity, saw) = saw.into_inner();

    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        if (*entity1 == saw_entity) || (*entity2 == saw_entity) {
            if let Ok((sawable_entity, mut sawable)) = sawable.get_mut(*entity1) {
                sawable.timer.unpause();
            } else if let Ok((sawable_entity, mut sawable)) = sawable.get_mut(*entity2) {
                sawable.timer.unpause();
            }
        }
    }
}

fn saw_collition_ended(
    mut collision_event_reader: EventReader<CollisionEnded>,
    saw: Single<(Entity, &TractorSaw)>,
    mut sawable: Query<(Entity, &mut Sawable)>,
) {
    let (saw_entity, saw) = saw.into_inner();

    for CollisionEnded(entity1, entity2) in collision_event_reader.read() {
        if (*entity1 == saw_entity) || (*entity2 == saw_entity) {
            if let Ok((sawable_entity, mut sawable)) = sawable.get_mut(*entity1) {
                sawable.timer.pause();
            } else if let Ok((sawable_entity, mut sawable)) = sawable.get_mut(*entity2) {
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
        println!("Sawable timer tick for entity: {:?}", entity);

        sawable.timer.tick(time.delta());
        if sawable.timer.finished() {
            sawable.timer.set_duration(saw.rate_of_fire);
            sawable.timer.reset();

            commands.send_event(Damage {
                value: 1.0, // Example damage value
                entity,
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
