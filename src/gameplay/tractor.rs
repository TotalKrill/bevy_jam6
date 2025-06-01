use super::*;

pub fn spawn_tractor(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> impl Bundle {
    (
        Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(4.0, 2.0, 2.)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GREEN.into(),
            ..Default::default()
        })),
    )
}
