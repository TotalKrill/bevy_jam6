use crate::PausableSystems;
use crate::gameplay::DespawnAfter;
use crate::gameplay::apple::{APPLE_RADIUS, AppleAssets, AppleSpawnEvent, AppleStrength};
use crate::gameplay::health::*;
use crate::gameplay::healthbars::healthbar;
use crate::gameplay::level::{Ground, TERRAIN_HEIGHT};
use crate::gameplay::saw::Sawable;
use crate::screens::ingame::setup_gamescreen;
use crate::{ReplaceOnHotreload, asset_tracking::LoadResource, screens::*};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tweening::lens::{TransformRotateXLens, TransformRotateZLens, TransformScaleLens};
use bevy_tweening::{Animator, Sequence, Tween};
use bevy_ui_anchor::AnchoredUiNodes;
use std::f32::consts::PI;
use std::time::Duration;

const TREE_STARTING_RADIUS: f32 = 0.5;
pub const TREE_STARTING_HEIGHT: f32 = 3.0;
const DEFAULT_APPLE_SPAWN_TIME_SEC: f32 = 5.0; // Time between apple spawns

const RANDOM_SPAWN_X_MIN: f32 = -135.0;
const RANDOM_SPAWN_X_MAX: f32 = 135.0;
const RANDOM_SPAWN_Z_MIN: f32 = -135.0;
const RANDOM_SPAWN_Z_MAX: f32 = 135.0;
const RANDOM_SPAWN_REPEAT_TIME_SEC: u64 = 10;
const TREE_HEALTH_INIT: u32 = 1;
const TREE_HEALTH_INCREASE_TICK: f32 = 1.5;
const MAXIMUM_TREES: usize = 50;

const DEFAULT_TREE_LOCATIONS: [Vec3; 3] = [
    vec3(22.0, 1000., 20.0),
    vec3(-15.0, 1000., -10.0),
    vec3(34.0, 1000., -20.0),
];

#[derive(Component, Reflect)]
pub struct Tree {
    pub apple_spawn_time_sec: f32,
    pub last_apple_spawn: f32,
    pub timer: Timer,
    // level progression
    pub level: u32,
}

impl Tree {
    const SCALE_PER_LEVEL: f32 = 0.5;
    const SCALE_DURATION_MS: u64 = 1500;
    const LEVEL_UP_TIME: u64 = 10;
    const SCALE_SHAKE_DURATION_MS: u64 = 50;
    const SCALE_SHAKE_ANGLE_RADIAN: f32 = PI / 9.0;
    const SCALE_SHAKE_COUNT: u32 = 10;
}

fn calculate_max_health(tree_level: u32) -> u32 {
    return 1 + (TREE_HEALTH_INCREASE_TICK * tree_level as f32) as u32;
}

#[derive(Event)]
pub struct TreeSpawnEvent {
    pub(crate) position: Vec3,
    pub(crate) startlevel: u32,
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct TreeAssets {
    pub tree: Handle<Scene>,
    pub crown: Handle<Scene>,
    pub trunks: [Handle<Scene>; 3],
}

#[derive(Component)]
pub struct TreeApple;

impl FromWorld for TreeAssets {
    fn from_world(world: &mut World) -> Self {
        let assets: &AssetServer = world.resource::<AssetServer>();
        Self {
            tree: assets.load("models/tree/tree.gltf#Scene1"),
            crown: assets.load("models/tree/tree.gltf#Scene0"),
            trunks: [
                assets.load("models/tree/tree.gltf#Scene2"),
                assets.load("models/tree/tree.gltf#Scene3"),
                assets.load("models/tree/tree.gltf#Scene4"),
            ],
        }
    }
}
#[derive(Resource, Debug)]
pub struct TreeSpawnConfig {
    pub timer: Timer,
}

pub fn shake_tree() -> Sequence<Transform> {
    let shakes = Tree::SCALE_SHAKE_COUNT as usize;
    let mut rotations = 0;
    let mut rotation = Tree::SCALE_SHAKE_ANGLE_RADIAN;
    let mut duration = Tree::SCALE_SHAKE_DURATION_MS as f32;

    let mut sequence = Sequence::with_capacity(shakes + 1).then(Tween::new(
        EaseFunction::Linear,
        Duration::from_millis(duration as u64),
        TransformRotateXLens {
            start: 0.0,
            end: rotation,
        },
    ));

    while rotations < shakes - 1 {
        let new_rotation = -1. * rotation / 1.618033988;
        let new_duration = duration / 1.618033988;

        if rotations % 2 == 0 {
            sequence = sequence.then(Tween::new(
                EaseFunction::Linear,
                Duration::from_millis(new_duration as u64),
                TransformRotateXLens {
                    start: rotation,
                    end: new_rotation,
                },
            ));
        } else {
            sequence = sequence.then(Tween::new(
                EaseFunction::Linear,
                Duration::from_millis(new_duration as u64),
                TransformRotateZLens {
                    start: rotation,
                    end: new_rotation,
                },
            ));
        };

        rotation = new_rotation;
        duration = new_duration;

        rotations += 1;
    }

    sequence
}

fn level_up_animation(start: Vec3, end: Vec3) -> Sequence<Transform> {
    let mut sequence = Sequence::with_capacity(7);

    sequence = sequence.then(Tween::new(
        EaseFunction::Linear,
        Duration::from_millis(Tree::SCALE_DURATION_MS),
        TransformScaleLens { start, end },
    ));

    sequence.then(shake_tree())
}

fn spawn_tree(
    mut events: EventReader<TreeSpawnEvent>,
    mut commands: Commands,
    tree_assets: Res<TreeAssets>,
    mut raycast: MeshRayCast,
    ground: Query<Entity, With<Ground>>,
    trees: Query<&Tree>,
) {
    for event in events.read() {
        let num_trees = trees.iter().len();
        if num_trees >= MAXIMUM_TREES {
            continue;
        }

        // Calculate a ray pointing from the camera into the world based on the cursor's position.
        let ray_start = event.position.with_y(event.position.y + TERRAIN_HEIGHT);
        // let ray_start = Vec3::new(event.position.x, 1000.0, event.position.y);

        let ray = Ray3d::new(ray_start, Dir3::NEG_Y);

        let hits = raycast.cast_ray(
            ray,
            &MeshRayCastSettings::default().with_filter(&|e| ground.contains(e)),
        );

        if let Some((_, hit)) = hits.first() {
            commands
                .spawn((
                    Name::new("Tree"),
                    Tree {
                        apple_spawn_time_sec: DEFAULT_APPLE_SPAWN_TIME_SEC,
                        last_apple_spawn: 0.0,
                        timer: Timer::new(
                            Duration::from_secs(Tree::LEVEL_UP_TIME),
                            TimerMode::Repeating,
                        ),
                        level: event.startlevel,
                    },
                    Health::new(calculate_max_health(event.startlevel)),
                    Sawable::default(),
                    AnchoredUiNodes::spawn_one(healthbar(100.)),
                    StateScoped(Screen::InGame),
                    ReplaceOnHotreload,
                    SceneRoot(tree_assets.tree.clone()),
                    RigidBody::Static,
                    Collider::cylinder(TREE_STARTING_RADIUS, TREE_STARTING_HEIGHT * 2.0),
                    Transform {
                        translation: hit.point,
                        scale: Vec3::splat(0.01),
                        ..Default::default()
                    },
                    Animator::new(level_up_animation(
                        Vec3::splat(0.01),
                        Vec3::splat(
                            Tree::SCALE_PER_LEVEL + event.startlevel as f32 * Tree::SCALE_PER_LEVEL,
                        ),
                    )),
                    // children![
                    //     TreeApple,
                    //     Transform {
                    //         translation: Vec3::Y * 500000.0,
                    //         scale: Vec3::splat(1.0),
                    //         ..default()
                    //     },
                    //     SceneRoot(apple_assets.apple.clone()),
                    // ],
                ))
                .observe(
                    |trigger: Trigger<Death>,
                     mut commands: Commands,
                     trees: Query<&Transform, With<Tree>>,
                     tree_assets: Res<TreeAssets>| {
                        let entity = trigger.target().entity();

                        if let Ok(pos) = trees.get(entity) {
                            for (i, trunk) in tree_assets.trunks.iter().enumerate() {
                                commands.spawn((
                                    DespawnAfter::millis(3000),
                                    RigidBody::Dynamic,
                                    SceneRoot(trunk.clone()),
                                    pos.clone(),
                                    LinearVelocity(Vec3::splat(2.0)),
                                    children![(
                                        Collider::cylinder(
                                            (TREE_STARTING_RADIUS) * 0.9,
                                            (TREE_STARTING_HEIGHT / 3.0) * 0.9
                                        ),
                                        Transform {
                                            translation: vec3(
                                                0.,
                                                TREE_STARTING_HEIGHT / 3.0 * i as f32 + 0.1,
                                                0.
                                            ),
                                            ..default()
                                        },
                                    )],
                                ));
                            }
                        }

                        if let Ok(mut ec) = commands.get_entity(entity) {
                            ec.despawn();
                        };
                    },
                );
        } else {
            log::error!("Ground not found when spawning tree at {:}", ray_start);
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
            position: Vec3::new(x, 1000., z),
            startlevel: 0,
        });
    }
}

fn spawn_initial_trees(mut commands: Commands) {
    for pos in DEFAULT_TREE_LOCATIONS {
        commands.send_event(TreeSpawnEvent {
            position: pos,
            startlevel: 1,
        });
    }
}

fn level_up_trees(
    mut commands: Commands,
    time: Res<Time>,
    mut trees: Query<(Entity, &mut Health, &mut Tree, &Transform)>,
) {
    for (ent, mut tree_health, mut tree, tree_t) in trees.iter_mut() {
        tree.timer.tick(time.delta());

        if tree.timer.just_finished() {
            if tree_health.current == tree_health.max {
                tree.level += 1;
                tree_health.set_max_to(calculate_max_health(tree.level));
            }

            commands
                .entity(ent)
                .insert(Animator::new(level_up_animation(
                    tree_t.scale,
                    Vec3::splat(Tree::SCALE_PER_LEVEL + tree.level as f32 * Tree::SCALE_PER_LEVEL),
                )));
        }
    }
}

fn shake_sawning_trees(
    mut commands: Commands,
    mut event_reader: EventReader<DamageEvent>,
    query: Query<Entity, With<Tree>>,
) {
    for event in event_reader.read() {
        if let Ok(entity) = query.get(event.entity) {
            if let Ok(mut entity) = commands.get_entity(entity) {
                entity.insert(Animator::new(shake_tree()));
            }

            break;
        }
    }
}

fn trees_spawn_apples(
    mut commands: Commands,
    mut query: Query<(&mut Tree, &Transform)>,
    time: Res<Time>,
) {
    let elapsed_time = time.elapsed_secs();
    for (mut tree, tree_t) in query.iter_mut() {
        if tree.level > 0 && elapsed_time > (tree.last_apple_spawn + tree.apple_spawn_time_sec) {
            tree.last_apple_spawn = elapsed_time;
            let spawn_pos = tree_t.translation
                + (Vec3::Y * TREE_STARTING_HEIGHT * tree_t.scale.y + APPLE_RADIUS * 2.0);

            commands.send_event(AppleSpawnEvent {
                at: spawn_pos,
                apple_strength: AppleStrength::from_tree_level(tree.level),
            });
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<TreeAssets>();

    app.register_type::<Tree>();

    app.add_event::<TreeSpawnEvent>();

    app.insert_resource(TreeSpawnConfig {
        timer: Timer::new(
            Duration::from_secs(RANDOM_SPAWN_REPEAT_TIME_SEC),
            TimerMode::Repeating,
        ),
    });

    app.add_systems(
        PostUpdate,
        (shake_sawning_trees
            .run_if(in_state(Screen::InGame))
            .in_set(PausableSystems),),
    );

    app.add_systems(OnEnter(Screen::InGame), spawn_initial_trees);

    app.add_systems(
        Update,
        spawn_tree
            .after(setup_gamescreen)
            .run_if(in_state(Screen::InGame)),
    );
    #[cfg(feature = "dev")]
    app.add_systems(
        Update,
        spawn_tree
            .after(setup_gamescreen)
            .run_if(in_state(Screen::TractorBuild)),
    );

    app.add_systems(
        Update,
        (trees_spawn_apples, spawn_tree_timer, level_up_trees)
            .run_if(in_state(Screen::InGame))
            .in_set(PausableSystems),
    );
}
