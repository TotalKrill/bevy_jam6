use crate::PausableSystems;
use crate::asset_tracking::LoadResource;
use crate::gameplay::health::{Death, Health};
use crate::gameplay::level::TERRAIN_HEIGHT;
use crate::gameplay::saw::Sawable;
use crate::gameplay::seed::SeedSpawnEvent;
use crate::{
    ReplaceOnHotreload,
    gameplay::{tractor::Tractor, tree::Tree},
    screens::*,
};
use avian3d::prelude::*;
use bevy::{color::palettes::css::RED, prelude::*};

#[derive(Component)]
pub struct Apple;

#[derive(Event)]
pub struct AppleSpawnEvent {
    pub position: Vec3,
    pub max_radius: f32,
    pub radius: f32,
    pub apple_strength: AppleStrength
}

#[derive(Component, Clone, Debug)]
pub struct AppleStrength {
    pub health: f32,
    pub damage: f32,
    pub speed: f32,
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct AppleAssets {
    apple: Handle<Scene>,
}

impl FromWorld for AppleAssets {
    fn from_world(world: &mut World) -> Self {
        let assets: &AssetServer = world.resource::<AssetServer>();
        Self {
            apple: assets.load(GltfAssetLabel::Scene(0).from_asset("models/apple/apple.glb")),
        }
    }
}

pub const APPLE_RADIUS: f32 = 1.0;
const APPLE_SPAWN_RADIUS: f32 = 10.0;
const APPLE_FORCE_SCALAR: f32 = 10.0; // Rate at which the apple want to get to the player

fn spawn_apple_event_handler(
    mut events: EventReader<AppleSpawnEvent>,
    mut commands: Commands,
    assets: Res<AppleAssets>,
) {
    for event in events.read() {
        let radii = rand::random::<f32>() * event.max_radius;
        let angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;

        let position = event.position
            + Vec3::new(radii * angle.cos(), APPLE_RADIUS * 2.0, radii * angle.sin());
        commands
            .spawn((
                Apple,
                Sawable::default(),
                Name::new("Apple"),
                Health::new(event.apple_strength.health),
                event.apple_strength.clone(),
                ReplaceOnHotreload,
                SceneRoot(assets.apple.clone()),
                RigidBody::Dynamic,
                Collider::sphere(APPLE_RADIUS),
                Transform::from_translation(position),
            ))
            .observe(
                |trigger: Trigger<Death>,
                 mut commands: Commands,
                 mut eventwriter: EventWriter<SeedSpawnEvent>,
                 query: Query<(Entity, &Transform), With<Apple>>| {
                    if let Ok(apple) = query.get(trigger.target()) {
                        eventwriter.write(SeedSpawnEvent::new(apple.1.translation));
                    }

                    commands
                        .get_entity(trigger.target().entity())
                        .unwrap()
                        .despawn();
                },
            );
    }
}

fn apply_apple_force(
    mut query: Query<(&mut ExternalForce, &Transform), With<Apple>>,
    tractor: Single<&Transform, With<Tractor>>,
) {
    for (mut apple_force, apple_transform) in query.iter_mut() {
        let force =
            (tractor.translation - apple_transform.translation).normalize() * APPLE_FORCE_SCALAR;

        apple_force.set_force(force);
    }
}
fn spawn_apples(
    mut commands: Commands,
    mut query: Query<((&Transform, &AppleStrength), &mut Tree)>,
    time: Res<Time>,
) {
    let elapsed_time = time.elapsed_secs();
    for (((&transform, apple_strength)), mut tree) in query.iter_mut() {
        if elapsed_time > (tree.last_apple_spawn + tree.apple_spawn_time_sec) {
            tree.last_apple_spawn = elapsed_time;
            commands.send_event(AppleSpawnEvent {
                position: transform.translation,
                max_radius: APPLE_SPAWN_RADIUS,
                radius: APPLE_RADIUS,
                apple_strength: apple_strength.clone(),
            });
        }
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

pub(super) fn plugin(app: &mut App) {
    log::info!("Adding apple plugin");
    app.load_resource::<AppleAssets>();

    app.add_event::<AppleSpawnEvent>();
    app.add_systems(
        Update,
        (
            spawn_apple_event_handler.run_if(in_state(Screen::InGame)),
            spawn_apples.run_if(in_state(Screen::InGame)),
            apply_apple_force.run_if(in_state(Screen::InGame)),
            despawn_apples_below_map.run_if(in_state(Screen::InGame)),
        )
            .in_set(PausableSystems),
    );
}
