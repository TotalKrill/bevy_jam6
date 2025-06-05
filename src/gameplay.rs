use avian3d::prelude::*;
pub use bevy::{color::palettes::css::*, prelude::*};
//all the gameplay stuff

pub mod apple;
pub mod bullet;
pub mod controls;
pub mod health;
pub mod tractor;
pub mod tree;
pub mod turret;
pub mod turret_aiming;

/// contains the heads up display during game;
pub mod hud;

#[cfg(feature = "dev_native")]
use bevy_simple_subsecond_system::hot;

pub const LEVEL_WIDHT: f32 = 200.0;

pub mod level {
    use bevy::color::palettes::tailwind::GRAY_100;

    #[derive(Component)]
    pub struct Ground;

    use super::*;
    pub fn level(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> impl Bundle {
        (
            Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(LEVEL_WIDHT, 0.1, LEVEL_WIDHT)))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: GRAY_100.into(),
                ..Default::default()
            })),
            RigidBody::Static,
            Friction::new(1.0),
            Collider::cuboid(LEVEL_WIDHT, 0.1, LEVEL_WIDHT),
            Ground,
        )
    }
}

pub(super) fn plugin(app: &mut App) {
    log::info!("Adding gameplay plugins");
    app.add_plugins(controls::plugin);
    app.add_plugins(hud::hud_plugin);
    app.add_plugins(tractor::tractor_plugin);
    app.add_plugins(bullet::bullet_plugin);
    app.add_plugins(turret_aiming::plugin);
    app.add_plugins(turret::turret_plugin);
    app.add_plugins(apple::plugin);
    app.add_plugins(health::plugin);
    app.add_plugins(tree::plugin);
}
