use super::*;

use avian3d::prelude::*;

pub const TRACTOR_WIDTH: f32 = 2.0;
pub const TRACTOR_HEIGHT: f32 = 2.0;
pub const TRACTOR_LENGTH: f32 = 4.0;

pub const FRONT_WHEEL_DIAMETER: f32 = 0.5;
pub const BACK_WHEEL_DIAMETER: f32 = 1.2;
pub const WHEEL_WIDTH: f32 = 0.25;

#[derive(Component)]
pub struct Tractor;

pub fn spawn_tractor(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> impl Bundle {
    (
        Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(
            TRACTOR_WIDTH,
            TRACTOR_HEIGHT,
            TRACTOR_LENGTH,
        )))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GREEN.into(),
            ..Default::default()
        })),
        RigidBody::Dynamic,
        Collider::cuboid(TRACTOR_WIDTH, TRACTOR_HEIGHT, TRACTOR_LENGTH),
        children![
            (
                Mesh3d(meshes.add(Cylinder::new(FRONT_WHEEL_DIAMETER, WHEEL_WIDTH))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: BLACK.into(),
                    ..Default::default()
                })),
                Transform {
                    translation: Vec3::new(
                        TRACTOR_WIDTH / 2.,
                        -TRACTOR_HEIGHT / 2.0,
                        TRACTOR_LENGTH / 2.0,
                    ),
                    rotation: Quat::from_rotation_z(90_f32.to_radians()),
                    ..Default::default()
                },
            ),
            (
                Mesh3d(meshes.add(Cylinder::new(FRONT_WHEEL_DIAMETER, WHEEL_WIDTH))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: BLACK.into(),
                    ..Default::default()
                })),
                Transform {
                    translation: Vec3::new(
                        -TRACTOR_WIDTH / 2.,
                        -TRACTOR_HEIGHT / 2.0,
                        TRACTOR_LENGTH / 2.0
                    ),
                    rotation: Quat::from_rotation_z(90_f32.to_radians()),
                    ..Default::default()
                }
            ),
            (
                Mesh3d(meshes.add(Cylinder::new(BACK_WHEEL_DIAMETER, WHEEL_WIDTH))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: BLACK.into(),
                    ..Default::default()
                })),
                Transform {
                    translation: Vec3::new(
                        TRACTOR_WIDTH / 2.,
                        -TRACTOR_HEIGHT / 2.0,
                        -TRACTOR_LENGTH / 2.0
                    ),
                    rotation: Quat::from_rotation_z(90_f32.to_radians()),
                    ..Default::default()
                }
            ),
            (
                Mesh3d(meshes.add(Cylinder::new(BACK_WHEEL_DIAMETER, WHEEL_WIDTH))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: BLACK.into(),
                    ..Default::default()
                })),
                Transform {
                    translation: Vec3::new(
                        -TRACTOR_WIDTH / 2.,
                        -TRACTOR_HEIGHT / 2.0,
                        -TRACTOR_LENGTH / 2.0
                    ),
                    rotation: Quat::from_rotation_z(90_f32.to_radians()),
                    ..Default::default()
                }
            )
        ],
    )
}
