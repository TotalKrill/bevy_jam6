use avian3d::prelude::*;
use bevy::{color::palettes::css::BROWN, prelude::*};

use crate::gameplay::level::Ground;
use crate::screens::gameplay::setup_gamescreen;
use crate::{ReplaceOnHotreload, asset_tracking::LoadResource, screens::*};

const TREE_STARTING_RADIUS: f32 = 0.5;
const TREE_STARTING_HEIGHT: f32 = 3.0;
const DEFAULT_APPLE_SPAWN_TIME_SEC: f32 = 5.0; // Time between apple spawns

#[derive(Component)]
pub struct Tree {
    pub apple_spawn_time_sec: f32,
    pub last_apple_spawn: f32,
}

#[derive(Event)]
pub struct TreeSpawnEvent {
    position: Vec2,
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

fn spawn_tree(
    mut events: EventReader<TreeSpawnEvent>,
    mut commands: Commands,
    tree_assets: Res<TreeAssets>,
    mut raycast: MeshRayCast,
    ground: Query<&Ground>,
) {
    if ground.is_empty() {
        return;
    }

    for event in events.read() {
        log::info!("Spawning tree");

        // Calculate a ray pointing from the camera into the world based on the cursor's position.
        let ray_start = Vec3::new(event.position.x, 1000.0, event.position.y);

        let ray = Ray3d::new(ray_start, Dir3::NEG_Y);

        let hits = raycast.cast_ray(ray, &MeshRayCastSettings::default());

        let Some((_e, hit)) = hits.first() else {
            return;
        };

        let position = vec3(event.position.x, TREE_STARTING_HEIGHT, event.position.y);

        log::info!(
            "Spawning tree at position: {:?} (from {:?}, distance: {:?})",
            hit.point,
            event.position,
            hit.distance
        );
        commands.spawn((
            Tree {
                apple_spawn_time_sec: DEFAULT_APPLE_SPAWN_TIME_SEC,
                last_apple_spawn: 0.0,
            },
            StateScoped(Screen::Gameplay),
            Name::new("Apple"),
            ReplaceOnHotreload,
            SceneRoot(tree_assets.tree.clone()),
            ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
            RigidBody::Static,
            Collider::cylinder(TREE_STARTING_RADIUS, TREE_STARTING_HEIGHT),
            Transform::from_translation(position),
        ));
    }
}

fn startup_tree(mut commands: Commands) {
    commands.send_event(TreeSpawnEvent {
        position: vec2(22.0, 20.0),
    });
    commands.send_event(TreeSpawnEvent {
        position: vec2(-15.0, -10.0),
    });
    commands.send_event(TreeSpawnEvent {
        position: vec2(34.0, -20.0),
    });

    println!("Tree setup");
}

pub(super) fn plugin(app: &mut App) {
    log::info!("Adding tree plugin");
    app.load_resource::<TreeAssets>();

    app.add_event::<TreeSpawnEvent>();
    app.add_systems(
        Update,
        (spawn_tree
            .run_if(in_state(Screen::Gameplay))
            .after(setup_gamescreen),),
    );
    app.add_systems(OnEnter(Screen::Gameplay), startup_tree);
}
