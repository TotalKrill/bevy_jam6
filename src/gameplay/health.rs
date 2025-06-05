use crate::gameplay::apple::Apple;
use crate::gameplay::tractor::{LeftWheels, RightWheels, Tractor};
use crate::screens::Screen;
use avian3d::prelude::{CollidingEntities, CollisionStarted};
use bevy::prelude::*;
use std::collections::HashSet;

pub fn plugin(app: &mut App) {
    app.add_event::<Damage>().add_event::<Death>().add_systems(
        Update,
        (
            damage_health.run_if(in_state(Screen::Gameplay)),
            create_collision_damage.run_if(in_state(Screen::Gameplay)),
        ),
    );
}

#[derive(Component, Debug)]
pub struct Health {
    pub current: f32,
}

impl Health {
    pub fn new(health: f32) -> Self {
        Self { current: health }
    }
}

#[derive(Event, Debug)]
pub struct Damage {
    pub value: f32,
    pub entity: Entity,
}

#[derive(Event, Debug)]
pub struct Death;

fn damage_health(
    mut commands: Commands,
    mut event_reader: EventReader<Damage>,
    mut query: Query<(Entity, &mut Health), With<Health>>,
) {
    for event in event_reader.read() {
        for (entity, mut health) in query.iter_mut() {
            if event.entity == entity {
                println!("damage {entity}");

                health.current -= event.value;

                if health.current <= 0.0 {
                    commands.trigger_targets(Death, entity);
                }
            }
        }
    }
}

fn create_collision_damage(
    mut collision_event_reader: EventReader<CollisionStarted>,
    tractor: Single<(Entity, &LeftWheels, &RightWheels), With<Tractor>>,
    apples: Query<Entity, With<Apple>>,
    mut event_writer: EventWriter<Damage>,
) {
    let (tractor, left, right) = *tractor;

    let mut tractor_entities = HashSet::with_capacity(5);
    tractor_entities.insert(tractor);
    tractor_entities.extend(left.collection());
    tractor_entities.extend(right.collection());

    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        // println!(
        //     "{entity1} and {entity2} started colliding, hi1 = {}, hi2 = {}",
        //     tractor_entities.contains(entity1),
        //     tractor_entities.contains(entity2)
        // );

        let apple_entity = if tractor_entities.contains(entity1) {
            *entity2
        } else if tractor_entities.contains(entity2) {
            *entity1
        } else {
            continue;
        };

        if let Ok(apple) = apples.get(apple_entity) {
            println!("I am the apple {apple:?}");

            event_writer.write(Damage {
                value: 100.0,
                entity: apple,
            });

            event_writer.write(Damage {
                value: 1.0,
                entity: tractor,
            });
        }
    }
}
