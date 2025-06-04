use avian3d::prelude::*;
use bevy::{color::palettes::css::BROWN, prelude::*};

use crate::{
    gameplay::{LEVEL_WIDHT, tractor::Tractor},
    screens::*,
};

#[derive(Component)]
pub struct Tree {
    pub apple_spawn_time_sec: f32,
    pub last_apple_spawn: f32,
}

#[derive(Event)]
pub struct TreeSpawnEvent {
    position: Vec3,
}

const TREE_STARTING_RADIUS: f32 = 0.5;
const TREE_STARTING_HEIGHT: f32 = 3.0;
const MIN_DISTANCE_FROM_ANOTHER_OBJECT: f32 = 2.0;
const DEFAULT_APPLE_SPAWN_TIME_SEC: f32 = 5.0; // Time between apple spawns

fn spawn_tree(
    mut events: EventReader<TreeSpawnEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&Transform, (With<Tree>, With<Tractor>)>,
) {
    for event in events.read() {
        println!("Spawning tree");

        // let mut position;
        // let mut position_found = false;
        // while !position_found {
        //     let x = rand::random::<f32>() * LEVEL_WIDHT - LEVEL_WIDHT / 2.0;
        //     let z = rand::random::<f32>() * LEVEL_WIDHT - LEVEL_WIDHT / 2.0;

        //     position = Vec3::new(x, TREE_STARTING_HEIGHT, z);

        //     position_found = true;
        //     for transform in query.iter() {
        //         if position.distance(transform.translation) <= MIN_DISTANCE_FROM_ANOTHER_OBJECT {
        //             position_found = false;
        //             break;
        //         }
        //     }
        commands.spawn((
            Tree {
                apple_spawn_time_sec: DEFAULT_APPLE_SPAWN_TIME_SEC,
                last_apple_spawn: 0.0,
            },
            StateScoped(Screen::Gameplay),
            Name::new("Apple"),
            ReplaceOnHotreload,
            Mesh3d(meshes.add(Cylinder::new(TREE_STARTING_RADIUS, TREE_STARTING_HEIGHT))),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(BROWN))),
            RigidBody::Static,
            Collider::cylinder(TREE_STARTING_RADIUS, TREE_STARTING_HEIGHT),
            Transform::from_translation(event.position),
        ));
    }
}

fn startup_tree(mut commands: Commands) {
    commands.send_event(TreeSpawnEvent {
        position: Vec3::new(22.0, TREE_STARTING_HEIGHT, 20.0),
    });
    commands.send_event(TreeSpawnEvent {
        position: Vec3::new(-15.0, TREE_STARTING_HEIGHT, -10.0),
    });
    commands.send_event(TreeSpawnEvent {
        position: Vec3::new(34.0, TREE_STARTING_HEIGHT, -20.0),
    });

    println!("Tree setup");
}

pub(super) fn plugin(app: &mut App) {
    log::info!("Adding tree plugin");
    app.add_event::<TreeSpawnEvent>();
    app.add_systems(Update, (spawn_tree.run_if(in_state(Screen::Gameplay)),));
    app.add_systems(OnEnter(Screen::Gameplay), startup_tree);
}
