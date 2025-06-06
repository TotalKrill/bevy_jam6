use crate::PausableSystems;
use crate::asset_tracking::LoadResource;
use crate::gameplay::health::{Death, Health};
use crate::gameplay::healthbars::healthbar;
use crate::gameplay::level::TERRAIN_HEIGHT;
use crate::gameplay::saw::Sawable;
use crate::gameplay::seed::SeedSpawnEvent;
use crate::gameplay::tree::{TREE_GROWTH_DURATION_SEC, TREE_STARTING_HEIGHT};
use crate::{
    ReplaceOnHotreload,
    gameplay::{tractor::Tractor, tree::Tree},
    screens::*,
};
use avian3d::prelude::*;
use bevy::prelude::*;

const APPLE_MASS: f32 = 1.0;
pub const APPLE_RADIUS: f32 = 1.0;
const APPLE_HEALTH_MIN: f32 = 1.0;
const APPLE_HEALTH_MAX: f32 = 10.0;
const APPLE_DAMAGE_MIN: f32 = 1.0;
const APPLE_DAMAGE_MAX: f32 = 10.0;
const APPLE_SPEED_MIN: f32 = 2.0;
const APPLE_SPEED_MAX: f32 = 40.0;
const APPLE_INITIAL_VELOCITY: f32 = 10.0;
use bevy_ui_anchor::AnchoredUiNodes;

#[derive(Component)]
pub struct Apple;

#[derive(Event)]
pub struct AppleSpawnEvent {
    pub tree: Entity,
    pub apple_strength: AppleStrength,
    pub radius: f32,
}

#[derive(Component, Clone, Debug)]
pub struct AppleStrength {
    pub health: f32,
    pub damage: f32,
    pub speed: f32,
}

impl AppleStrength {
    pub fn new() -> Self {
        Self {
            health: APPLE_HEALTH_MIN,
            damage: APPLE_DAMAGE_MIN,
            speed: APPLE_SPEED_MIN,
        }
    }
    pub fn increase(&mut self, tree_growth: f32) {
        self.health = tree_growth * (APPLE_HEALTH_MAX - APPLE_HEALTH_MIN) + APPLE_HEALTH_MIN;
        self.damage = tree_growth * (APPLE_DAMAGE_MAX - APPLE_DAMAGE_MIN) + APPLE_DAMAGE_MIN;
        self.speed = tree_growth * (APPLE_SPEED_MAX - APPLE_SPEED_MIN) + APPLE_SPEED_MIN;
    }
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

fn spawn_apple_event_handler(
    mut events: EventReader<AppleSpawnEvent>,
    mut commands: Commands,
    assets: Res<AppleAssets>,
    trees: Query<&Transform, With<Tree>>,
    tractor: Single<&Transform, With<Tractor>>,
) {
    for event in events.read() {
        // let radii = rand::random::<f32>() * event.max_radius;
        // let angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;

        let tree_transform = trees.get(event.tree).unwrap();
        let spawn_height = TREE_STARTING_HEIGHT * tree_transform.scale.y + APPLE_RADIUS * 2.0;
        let position = tree_transform.translation + Vec3::Y * spawn_height;
        let towards_player =
            ((tractor.translation - position).normalize() + Vec3::Y * 2.0).normalize();

        println!(" * event.radius + 0.1: {:?}", event.radius + 0.1);

        commands
            .spawn((
                Apple,
                Sawable::default(),
                Name::new("Apple"),
                Health::new(event.apple_strength.health),
                event.apple_strength.clone(),
                Mass(APPLE_MASS),
                ReplaceOnHotreload,
                AnchoredUiNodes::spawn_one(healthbar(100.)),
                SceneRoot(assets.apple.clone()),
                RigidBody::Dynamic,
                Collider::sphere(APPLE_RADIUS),
                Transform::from_translation(position).with_scale(Vec3::splat(event.radius + 0.2)),
                LinearVelocity(towards_player * APPLE_INITIAL_VELOCITY),
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
    mut query: Query<(&mut ExternalForce, &Transform, &AppleStrength), With<Apple>>,
    tractor: Single<&Transform, With<Tractor>>,
) {
    for (mut apple_force, apple_transform, apple_strength) in query.iter_mut() {
        let force =
            (tractor.translation - apple_transform.translation).normalize() * apple_strength.speed;

        apple_force.set_force(force);
    }
}
fn spawn_apples(
    mut commands: Commands,
    mut query: Query<((Entity, &AppleStrength), &mut Tree)>,
    time: Res<Time>,
) {
    let elapsed_time = time.elapsed_secs();
    for ((entity, apple_strength), mut tree) in query.iter_mut() {
        if elapsed_time > (tree.last_apple_spawn + tree.apple_spawn_time_sec) {
            tree.last_apple_spawn = elapsed_time;
            commands.send_event(AppleSpawnEvent {
                tree: entity,
                apple_strength: apple_strength.clone(),
                radius: tree.timer.elapsed_secs() / TREE_GROWTH_DURATION_SEC as f32,
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
