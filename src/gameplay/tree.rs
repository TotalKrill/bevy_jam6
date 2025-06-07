use crate::PausableSystems;
use crate::gameplay::WorldAssets;
use crate::gameplay::apple::{APPLE_RADIUS, AppleSpawnEvent, AppleStrength};
use crate::gameplay::health::*;
use crate::gameplay::healthbars::healthbar;
use crate::gameplay::level::Ground;
use crate::gameplay::saw::Sawable;
use crate::screens::ingame::setup_gamescreen;
use crate::{ReplaceOnHotreload, asset_tracking::LoadResource, screens::*};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tweening::lens::TransformScaleLens;
use bevy_tweening::{AnimationSystem, Animator, Tween, component_animator_system};
use bevy_ui_anchor::AnchoredUiNodes;
use std::time::Duration;

const TREE_STARTING_RADIUS: f32 = 0.5;
pub const TREE_STARTING_HEIGHT: f32 = 3.0;
const DEFAULT_APPLE_SPAWN_TIME_SEC: f32 = 5.0; // Time between apple spawns

const RANDOM_SPAWN_X_MIN: f32 = -150.0;
const RANDOM_SPAWN_X_MAX: f32 = 150.0;
const RANDOM_SPAWN_Z_MIN: f32 = -150.0;
const RANDOM_SPAWN_Z_MAX: f32 = 150.0;
const RANDOM_SPAWN_REPEAT_TIME_SEC: u64 = 10;
const TREE_GROWTH_DURATION_SEC: u64 = 180;
const TREE_HEALTH_INIT: u32 = 1;
const TREE_HEALTH_INCREASE_TICK: u32 = 1;
const TREE_HEALTH_INCREASE_TICK_INTERVAL_SEC: u64 = 10;
const TREE_ACTIVE_THRESHOLD_SEC: f32 = 5.; // sec until tree starts spawning apples

const DEFAULT_TREE_LOCATIONS: [Vec2; 3] = [vec2(22.0, 20.0), vec2(-15.0, -10.0), vec2(34.0, -20.0)];

#[derive(Component)]
pub struct Tree {
    pub apple_spawn_time_sec: f32,
    pub last_apple_spawn: f32,
    pub timer: Timer,
    // level progression
    pub level: u32,
}

#[derive(Event)]
pub struct TreeSpawnEvent {
    pub(crate) position: Vec2,
    pub(crate) startlevel: u32,
    pub(crate) scale: f32,
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
    ground: Single<Entity, With<Ground>>,
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

        if let Some((_, hit)) = hits.into_iter().find(|(entity, _)| *entity == *ground) {
            println!("Spawngin tree gound found {:?}", hit.point);

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
                Duration::from_secs(TREE_GROWTH_DURATION_SEC),
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
                        timer: Timer::new(
                            Duration::from_secs(TREE_HEALTH_INCREASE_TICK_INTERVAL_SEC),
                            TimerMode::Repeating,
                        ),
                        level: 0,
                    },
                    Sawable::default(),
                    AnchoredUiNodes::spawn_one(healthbar(100.)),
                    Health::new(TREE_HEALTH_INIT),
                    AppleStrength::new(),
                    StateScoped(Screen::InGame),
                    ReplaceOnHotreload,
                    SceneRoot(tree_assets.tree.clone()),
                    RigidBody::Static,
                    Collider::cylinder(TREE_STARTING_RADIUS, TREE_STARTING_HEIGHT),
                    Transform {
                        translation: position,
                        scale: vec3(event.scale, event.scale, event.scale),
                        ..Default::default()
                    },
                    Animator::new(tween),
                ))
                .observe(|trigger: Trigger<Death>, mut commands: Commands| {
                    if let Ok(mut ec) = commands.get_entity(trigger.target().entity()) {
                        ec.despawn();
                    }
                });
        } else {
            log::error!("Ground not found when spawning tree");
        }
    }
}

fn spawn_tree_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut config: ResMut<TreeSpawnConfig>,
    trees: Query<&Tree>,
) {
    config.timer.tick(time.delta());

    let num_trees = trees.iter().count();
    if num_trees < DEFAULT_TREE_LOCATIONS.len() {
        commands.send_event(TreeSpawnEvent {
            position: DEFAULT_TREE_LOCATIONS[num_trees],
            startlevel: 0,
            scale: 1.0,
        });
    }

    if config.timer.finished() {
        let x =
            rand::random::<f32>() * (RANDOM_SPAWN_X_MAX - RANDOM_SPAWN_X_MIN) + RANDOM_SPAWN_X_MIN;
        let z =
            rand::random::<f32>() * (RANDOM_SPAWN_Z_MAX - RANDOM_SPAWN_Z_MIN) + RANDOM_SPAWN_Z_MIN;

        commands.send_event(TreeSpawnEvent {
            position: Vec2::new(x, z),
            scale: 0.0,
            startlevel: 0,
        });
    }
}

fn level_up_trees(time: Res<Time>, mut trees: Query<(&mut Health, &mut AppleStrength, &mut Tree)>) {
    for (mut health, mut apple_strength, mut tree) in trees.iter_mut() {
        tree.timer.tick(time.delta());

        if tree.timer.finished() {
            if health.current == health.max {
                // println!("increased tree strength: {:?}", tree.timer.elapsed_secs());

                health.current += TREE_HEALTH_INCREASE_TICK;
                health.max = TREE_HEALTH_INCREASE_TICK;
                tree.level += 1;

                apple_strength.increase();
            }
        }
    }
}

fn spawn_apples(
    mut commands: Commands,
    mut query: Query<(&AppleStrength, &mut Tree, &Transform)>,
    time: Res<Time>,
) {
    let elapsed_time = time.elapsed_secs();
    for (apple_strength, mut tree, tree_t) in query.iter_mut() {
        if tree.level > 0 && elapsed_time > (tree.last_apple_spawn + tree.apple_spawn_time_sec) {
            tree.last_apple_spawn = elapsed_time;
            let spawn_pos =
                tree_t.translation + TREE_STARTING_HEIGHT * tree_t.scale.y + APPLE_RADIUS * 2.0;

            commands.send_event(AppleSpawnEvent {
                at: spawn_pos,
                apple_strength: apple_strength.clone(),
                radius: 1.0, // TODO make const?
            });
        }
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
            spawn_tree.after(setup_gamescreen),
            spawn_tree_timer,
            component_animator_system::<Tree>.in_set(AnimationSystem::AnimationUpdate),
            spawn_apples,
        )
            .run_if(in_state(Screen::InGame))
            .in_set(PausableSystems),
    );

    app.add_systems(FixedUpdate, level_up_trees.run_if(in_state(Screen::InGame)));
}
