use super::*;

pub const TRACTOR_WIDTH: f32 = 4.0;
pub const TRACTOR_HEIGHT: f32 = 2.0;
pub const TRACTOR_LENGTH: f32 = 2.0;

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
        children![
            (
                Mesh3d(meshes.add(Cylinder::new(0.4, 0.2))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: BLACK.into(),
                    ..Default::default()
                })),
                Transform {
                    translation: Vec3::new(
                        TRACTOR_WIDTH / 2.,
                        -TRACTOR_HEIGHT / 2.0,
                        TRACTOR_LENGTH / 2.0
                    ),
                    ..Default::default()
                }
            ),
            (
                Mesh3d(meshes.add(Cylinder::new(0.4, 0.2))),
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
                    ..Default::default()
                }
            )
        ],
    )
}
