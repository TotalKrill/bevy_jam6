use avian3d::prelude::*;
pub use bevy::{color::palettes::css::*, prelude::*};
//all the gameplay stuff

pub mod bullet;
pub mod tractor;
pub mod turret;

pub mod turret_aiming;

pub mod level {
    use bevy::color::palettes::tailwind::GRAY_100;

    const LEVEL_WIDHT: f32 = 200.0;

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
            RigidBody::Kinematic,
            Collider::cuboid(LEVEL_WIDHT, 0.1, LEVEL_WIDHT),
            Ground,
        )
    }
}

pub mod turrent {
    use super::*;
}

pub mod movementcontrols;

pub(super) fn plugin(app: &mut App) {
    log::info!("Adding gameplay plugins");
    // app.add_plugins(movementcontrols::plugin);
    app.add_plugins(tractor::tractor_plugin);
    app.add_plugins(bullet::bullet_plugin);
    app.add_plugins(turret_aiming::plugin);
}
