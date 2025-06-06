use avian3d::prelude::*;
use bevy::prelude::*;
use std::time::Duration;
use bevy_tweening::{component_animator_system, AnimationSystem, Animator, Tween};
use bevy_tweening::lens::{TransformPositionLens, TransformScaleLens};
use crate::PausableSystems;
use crate::gameplay::WorldAssets;
use crate::gameplay::health::*;
use crate::gameplay::saw::Sawable;
use crate::screens::ingame::setup_gamescreen;
use crate::{ReplaceOnHotreload, asset_tracking::LoadResource, screens::*};

const TREE_STARTING_RADIUS: f32 = 0.5;
const TREE_STARTING_HEIGHT: f32 = 3.0;
const DEFAULT_APPLE_SPAWN_TIME_SEC: f32 = 5.0; // Time between apple spawns

const RANDOM_SPAWN_X_MIN: f32 = -150.0;
const RANDOM_SPAWN_X_MAX: f32 = 150.0;
const RANDOM_SPAWN_Z_MIN: f32 = -150.0;
const RANDOM_SPAWN_Z_MAX: f32 = 150.0;
const RANDOM_SPAWN_REPEAT_TIME_SEC: u64 = 5;

#[derive(Component)]
pub struct Tree {
    pub apple_spawn_time_sec: f32,
    pub last_apple_spawn: f32,
}

#[derive(Event)]
pub struct TreeSpawnEvent {
    pub(crate) position: Vec2,
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct TreeAssets {
    pub tree: Handle<Scene>,
}

impl FromWorld for TreeAssets {
    fn from_world(world: &mut World) -> Self {
        let assets: &AssetServer = world.resource::<AssetServer>();
        Self {
            tree: assets.load(GltfAssetLabel::Scene(0).from_asset("models/tree/tree.glb")),
        }
    }
}
#[derive(Resource, Debug)]
pub struct TreeSpawnConfig {
    pub timer: Timer,
}

fn spawn_tree(
    mut events: EventReader<TreeSpawnEvent>,
    mut commands: Commands,
    tree_assets: Res<TreeAssets>,
    mut raycast: MeshRayCast,
    world_assets: Res<WorldAssets>,
    asset_server: Res<AssetServer>,
) {
    if !asset_server.is_loaded(world_assets.ground.id()) {
        println!("World is not loaded yet, skipping tree spawn");
        return;
    }

    for event in events.read() {
        // Calculate a ray pointing from the camera into the world based on the cursor's position.
        let ray_start = Vec3::new(event.position.x, 1000.0, event.position.y);

        let ray = Ray3d::new(ray_start, Dir3::NEG_Y);

        let hits = raycast.cast_ray(ray, &MeshRayCastSettings::default());

        let Some((_e, hit)) = hits.first() else {
            return;
        };

        if hit.point.y < -500.0 {
            commands.send_event(TreeSpawnEvent {
                position: event.position,
            });
        } else {
            let position = vec3(
                event.position.x,
                hit.point.y + TREE_STARTING_HEIGHT / 2.0,
                event.position.y,
            );

            log::info!(
                "Spawning tree at position: {:?} (from {:?}, distance: {:?})",
                hit.point,
                event.position,
                hit.distance
            );

            let tween = Tween::new(
                // Use a quadratic easing on both endpoints.
                EaseFunction::Linear,
                // Animation time (one way only; for ping-pong it takes 2 seconds
                // to come back to start).
                Duration::from_secs(10),
                // The lens gives the Animator access to the Transform component,
                // to animate it. It also contains the start and end values associated
                // with the animation ratios 0. and 1.
                TransformScaleLens {
                    start: Vec3::new(0.01, 0.01, 0.01),
                    end: Vec3::new(3., 3., 3.),
                },
            );

            commands
                .spawn((
                    Name::new("Tree"),
                    Tree {
                        apple_spawn_time_sec: DEFAULT_APPLE_SPAWN_TIME_SEC,
                        last_apple_spawn: 0.0,
                    },
                    Sawable::default(),
                    Health::new(1.0),
                    StateScoped(Screen::InGame),
                    ReplaceOnHotreload,
                    SceneRoot(tree_assets.tree.clone()),
                    ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
                    RigidBody::Static,
                    Collider::cylinder(TREE_STARTING_RADIUS, TREE_STARTING_HEIGHT),
                    Transform::from_translation(position),
                    Animator::new(tween),
                ))
                .observe(|trigger: Trigger<Death>, mut commands: Commands| {
                    commands
                        .get_entity(trigger.target().entity())
                        .unwrap()
                        .despawn();
                });
        }
    }
}

fn spawn_tree_timer(mut commands: Commands, time: Res<Time>, mut config: ResMut<TreeSpawnConfig>) {
    config.timer.tick(time.delta());

    if config.timer.finished() {
        let x =
            rand::random::<f32>() * (RANDOM_SPAWN_X_MAX - RANDOM_SPAWN_X_MIN) + RANDOM_SPAWN_X_MIN;
        let z =
            rand::random::<f32>() * (RANDOM_SPAWN_Z_MAX - RANDOM_SPAWN_Z_MIN) + RANDOM_SPAWN_Z_MIN;

        commands.send_event(TreeSpawnEvent {
            position: Vec2::new(x, z),
        });
    }
}

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<TreeAssets>();

    app.add_event::<TreeSpawnEvent>();

    app.insert_resource(TreeSpawnConfig {
        timer: Timer::new(
            Duration::from_secs(RANDOM_SPAWN_REPEAT_TIME_SEC),
            TimerMode::Repeating,
        ),
    });

    app.add_systems(
        Update,
        (
            spawn_tree
                .run_if(in_state(Screen::InGame))
                .after(setup_gamescreen),
            spawn_tree_timer.run_if(in_state(Screen::InGame)),
            component_animator_system::<Tree>.in_set(AnimationSystem::AnimationUpdate).run_if(in_state(Screen::InGame))
        )
            .in_set(PausableSystems),
    );
}
