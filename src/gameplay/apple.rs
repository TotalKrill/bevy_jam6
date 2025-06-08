use crate::PausableSystems;
use crate::asset_tracking::LoadResource;
use crate::audio::sound_effect;
use crate::gameplay::health::{Death, Health};
use crate::gameplay::healthbars::healthbar;
use crate::gameplay::level::TERRAIN_HEIGHT;
use crate::gameplay::saw::Sawable;
use crate::gameplay::seed::SeedSpawnEvent;
use crate::{ReplaceOnHotreload, gameplay::tractor::Tractor, screens::*};
use avian3d::prelude::*;
use bevy::prelude::*;

const APPLE_MASS: f32 = 1.0;
pub const APPLE_RADIUS: f32 = 1.0;
const APPLE_INITIAL_VELOCITY: f32 = 10.0;
const APPLE_INITIAL_ROTATION: f32 = 5.0;
use bevy_firework::core::ParticleSpawner;
use bevy_ui_anchor::AnchoredUiNodes;

#[derive(Component)]
pub struct Apple {
    pub radius: f32,
}

#[derive(Event)]
pub struct AppleSpawnEvent {
    pub at: Vec3,
    pub apple_strength: AppleStrength,
    pub radius: f32,
}

#[derive(Component, Clone, Debug)]
pub struct AppleStrength {
    pub health: u32,
    pub damage: u32,
    pub speed: u32,
}

impl AppleStrength {
    pub fn from_tree_level(level: u32) -> Self {
        AppleStrength {
            health: level,
            damage: level,
            speed: level,
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct AppleAssets {
    pub apple: Handle<Scene>,
    eaten_apple: Handle<Scene>,
    eaten_apple_2: Handle<Scene>,
    death_sound: Handle<AudioSource>,
}

impl FromWorld for AppleAssets {
    fn from_world(world: &mut World) -> Self {
        let assets: &AssetServer = world.resource::<AssetServer>();
        Self {
            apple: assets.load(GltfAssetLabel::Scene(0).from_asset("models/apple/apple.gltf")),
            eaten_apple: assets
                .load(GltfAssetLabel::Scene(1).from_asset("models/apple/apple.gltf")),
            eaten_apple_2: assets
                .load(GltfAssetLabel::Scene(2).from_asset("models/apple/apple.gltf")),
            death_sound: assets.load::<AudioSource>("audio/sound_effects/apple-death.wav"),
        }
    }
}

fn spawn_apple_event_handler(
    mut events: EventReader<AppleSpawnEvent>,
    mut commands: Commands,
    assets: Res<AppleAssets>,
    tractor: Single<&Transform, With<Tractor>>,
) {
    for event in events.read() {
        let position = event.at;
        let towards_player =
            ((tractor.translation - position).normalize() + Vec3::Y * 2.0).normalize();

        let rot = Quat::from_rotation_y(90_f32.to_radians());

        let apple_rotation =
            rot.mul_vec3((tractor.translation - position).normalize()) * APPLE_INITIAL_ROTATION;

        let apple_radius =
            event.radius + 0.1 * event.radius * (event.apple_strength.health - 1) as f32;

        commands
            .spawn((
                Apple {
                    radius: apple_radius,
                },
                Sawable::default(),
                Name::new("Apple"),
                Health::new(event.apple_strength.health),
                event.apple_strength.clone(),
                Mass(APPLE_MASS),
                ReplaceOnHotreload,
                AnchoredUiNodes::spawn_one(healthbar(100.)),
                RigidBody::Dynamic,
                Collider::sphere(apple_radius),
                Transform::from_translation(position).with_scale(Vec3::splat(apple_radius)),
                LinearVelocity(towards_player * APPLE_INITIAL_VELOCITY),
                AngularVelocity(apple_rotation),
                SceneRoot(assets.apple.clone()),
            ))
            .observe(
                |trigger: Trigger<Death>,
                 mut commands: Commands,
                 assets: Res<AppleAssets>,
                 mut eventwriter: EventWriter<SeedSpawnEvent>,
                 query: Query<(Entity, &Transform), With<Apple>>| {
                    if let Ok((_apple_e, apple_t)) = query.get(trigger.target()) {
                        eventwriter.write(SeedSpawnEvent::new(apple_t.translation));

                        commands.spawn((
                            Name::new("particles"),
                            ParticleSpawner {
                                one_shot: true,
                                ..Default::default()
                            },
                            Transform::from_translation(apple_t.translation),
                        ));
                    }
                    commands.spawn(sound_effect(assets.death_sound.clone()));

                    if let Ok(mut ec) = commands.get_entity(trigger.target()) {
                        ec.despawn();
                    }
                },
            );
    }
}

fn apply_apple_force(
    mut query: Query<(&mut ExternalForce, &Transform, &AppleStrength), With<Apple>>,
    tractor: Single<&Transform, With<Tractor>>,
) {
    for (mut apple_force, apple_transform, apple_strength) in query.iter_mut() {
        let force = (tractor.translation - apple_transform.translation).normalize()
            * (apple_strength.speed as f32 * 1.3 + 5.);

        apple_force.set_force(force);
    }
}

fn despawn_apples_below_map(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Apple>>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.y < -1. * TERRAIN_HEIGHT {
            commands.entity(entity).despawn();
        }
    }
}

fn update_apple_mesh(
    mut commands: Commands,
    query: Query<(Entity, &Health), (With<Apple>, Changed<Health>)>,
    assets: Res<AppleAssets>,
) {
    for (apple, health) in query {
        // commands.entity(apple).remove::<SceneRoot>();
        if let Ok(mut entity_commands) = commands.get_entity(apple) {
            let health_percentage = health.percentage();
            if health_percentage >= 90 {
                entity_commands.try_insert(SceneRoot(assets.apple.clone()));
            } else if health_percentage >= 40 {
                entity_commands.try_insert(SceneRoot(assets.eaten_apple.clone()));
            } else if health.current > 0 {
                entity_commands.try_insert(SceneRoot(assets.eaten_apple_2.clone()));
            }
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    log::info!("Adding apple plugin");
    app.load_resource::<AppleAssets>();

    app.add_event::<AppleSpawnEvent>();
    app.add_systems(
        Update,
        (
            spawn_apple_event_handler.run_if(in_state(Screen::InGame)),
            apply_apple_force.run_if(in_state(Screen::InGame)),
            despawn_apples_below_map.run_if(in_state(Screen::InGame)),
            update_apple_mesh.run_if(in_state(Screen::InGame)),
        )
            .in_set(PausableSystems),
    );
}
