use crate::PausableSystems;
use crate::audio::sound_effect;
use crate::gameplay::apple::{Apple, AppleStrength};
use crate::gameplay::bullet::{Bullet, BulletSplitEvent};
use crate::gameplay::tractor::{LeftWheels, RightWheels, Tractor};
use crate::screens::Screen;
use avian3d::prelude::CollisionStarted;
use bevy::prelude::*;
use std::collections::HashSet;

use super::*;

#[derive(Resource)]
struct HealthAssets {
    tractor_damage_sound: Handle<AudioSource>,
}

impl FromWorld for HealthAssets {
    fn from_world(world: &mut World) -> Self {
        let assets: &AssetServer = world.resource::<AssetServer>();
        Self {
            tractor_damage_sound: assets
                .load::<AudioSource>("audio/sound_effects/tractor-damage.wav"),
        }
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<HealthAssets>();

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
    pub fn increase_max(&mut self, amount: u32) {
        self.current += amount;
        self.max += amount;
    }

    pub fn set_max_to(&mut self, new_max: u32) {
        let diff = new_max - self.max;
        self.increase_max(diff);
    }

    pub fn percentage(&self) -> u32 {
        ((self.current as f32 / self.max as f32) * 100.) as u32
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
    mut health_query: Query<&mut Health>,
    tractor: Single<Entity, With<Tractor>>,
    assets: Res<HealthAssets>,
) {
    for event in event_reader.read() {
        if let Ok(mut health) = health_query.get_mut(event.entity) {
            if health.current <= event.value {
                commands.trigger_targets(Death, event.entity);
            } else {
                health.current -= event.value;
            }

            if event.entity == *tractor {
                commands.spawn(sound_effect(assets.tractor_damage_sound.clone()));
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
