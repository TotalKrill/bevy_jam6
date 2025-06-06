use crate::PausableSystems;
use crate::gameplay::apple::{Apple, AppleStrength};
use crate::gameplay::bullet::{Bullet, BulletSplitEvent};
use crate::gameplay::tractor::{LeftWheels, RightWheels, Tractor};
use crate::screens::Screen;
use avian3d::prelude::CollisionStarted;
use bevy::prelude::*;
use std::collections::HashSet;

use super::*;

pub fn plugin(app: &mut App) {
    app.add_event::<DamageEvent>()
        .add_event::<Death>()
        .add_systems(
            Update,
            (
                damage_health.run_if(in_state(Screen::InGame)),
                damage_tractor.run_if(in_state(Screen::InGame)),
                bullet_apple_collision_damage.run_if(in_state(Screen::InGame)),
            )
                .in_set(PausableSystems),
        );
}

#[derive(Component, Debug)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

impl Health {
    pub fn percentage(&self) -> u32 {
        (self.current / self.max) * 100
    }
    pub fn new(health: u32) -> Self {
        Self {
            current: health,
            max: health,
        }
    }
}

#[derive(Event, Debug)]
pub struct DamageEvent {
    pub value: u32,
    pub entity: Entity,
}

#[derive(Event, Debug)]
pub struct Death;

fn damage_health(
    mut commands: Commands,
    mut event_reader: EventReader<DamageEvent>,
    mut query: Query<(Entity, &mut Health), With<Health>>,
) {
    for event in event_reader.read() {
        for (entity, mut health) in query.iter_mut() {
            if event.entity == entity {

                if health.current <= event.value {
                    commands.trigger_targets(Death, entity);
                } else {
                    health.current -= event.value;    
                }
                
            }
        }
    }
}

fn damage_tractor(
    mut collision_event_reader: EventReader<CollisionStarted>,
    tractor: Single<(Entity, &LeftWheels, &RightWheels), With<Tractor>>,
    apples: Query<(Entity, &AppleStrength), With<Apple>>,
    mut event_writer: EventWriter<DamageEvent>,
) {
    let (tractor, left, right) = *tractor;

    let mut tractor_entities = HashSet::with_capacity(5);
    tractor_entities.insert(tractor);
    tractor_entities.extend(left.collection());
    tractor_entities.extend(right.collection());

    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        for (apple_candidate, tractor_candidate) in [(*entity1, *entity2), (*entity2, *entity1)] {
            if tractor_entities.contains(&tractor_candidate) {
                if let Ok((apple, apple_strength)) = apples.get(apple_candidate) {
                    event_writer.write(DamageEvent {
                        value: 100,
                        entity: apple,
                    });

                    event_writer.write(DamageEvent {
                        value: apple_strength.damage,
                        entity: tractor,
                    });

                    break;
                }
            }
        }
    }
}

#[cfg_attr(feature = "dev_native", hot)]
fn bullet_apple_collision_damage(
    mut collision_event_reader: EventReader<CollisionStarted>,
    bullets: Query<(Entity, &Bullet)>,
    apples: Query<(Entity, &Transform), With<Apple>>,
    mut event_writer: EventWriter<DamageEvent>,
    mut bullet_split: EventWriter<BulletSplitEvent>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        for (apple_candidate, bullet_candidate) in [(*entity1, *entity2), (*entity2, *entity1)] {
            if let (Ok((apple, apple_t)), Ok((_bullet_e, bullet))) =
                (apples.get(apple_candidate), bullets.get(bullet_candidate))
            {
                event_writer.write(DamageEvent {
                    value: bullet.damage,
                    entity: apple,
                });

                let percent: f32 = rand::random();
                if percent < bullet.split_probability {
                    info!("splitting bullet!");
                    bullet_split.write(BulletSplitEvent {
                        center: apple_t.translation,
                        bullet: bullet.half(),
                    });
                }
                break; // Only need to damage once per collision
            }
        }
    }
}
