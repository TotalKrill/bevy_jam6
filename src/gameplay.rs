use avian3d::prelude::*;
pub use bevy::{color::palettes::css::*, prelude::*};

use crate::asset_tracking::LoadResource;
//all the gameplay stuff

pub mod apple;
pub mod bullet;
pub mod controls;
pub mod health;
pub mod tractor;
pub mod tree;
pub mod turret;
pub mod turret_aiming;

#[cfg(feature = "dev_native")]
use bevy_simple_subsecond_system::hot;

pub const LEVEL_WIDHT: f32 = 200.0;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct WorldAssets {
    pub ground: Handle<Scene>,
}

impl FromWorld for WorldAssets {
    fn from_world(world: &mut World) -> Self {
        let assets: &AssetServer = world.resource::<AssetServer>();
        Self {
            ground: assets.load(GltfAssetLabel::Scene(0).from_asset("models/map/map.glb")),
        }
    }
}

pub mod level {
    use bevy::color::palettes::tailwind::GRAY_100;

    #[derive(Component)]
    pub struct Ground;

    use super::*;
    pub fn level(assets: &WorldAssets) -> impl Bundle {
        (
            // Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(LEVEL_WIDHT, 0.1, LEVEL_WIDHT)))),
            // MeshMaterial3d(materials.add(StandardMaterial {
            //     base_color: GRAY_100.into(),
            //     ..Default::default()
            // })),
            RigidBody::Static,
            Friction::new(1.0),
            Transform::from_translation(Vec3::new(0.0, -1., 0.0)),
            ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
            Ground,
            SceneRoot(assets.ground.clone()),
        )
    }
}

pub(super) fn plugin(app: &mut App) {
    log::info!("Adding gameplay plugins");

    app.load_resource::<WorldAssets>();

    app.add_plugins(controls::plugin);
    app.add_plugins(tractor::tractor_plugin);
    app.add_plugins(bullet::bullet_plugin);
    app.add_plugins(turret_aiming::plugin);
    app.add_plugins(turret::turret_plugin);
    app.add_plugins(apple::plugin);
    app.add_plugins(health::plugin);
    app.add_plugins(tree::plugin);
}
