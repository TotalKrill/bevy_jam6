#[derive(Component)]
pub struct Ground;

use super::*;
pub fn level(assets: &WorldAssets) -> impl Bundle {
    (
        RigidBody::Static,
        Friction::new(1.0),
        // Transform::from_translation(Vec3::new(0.0, -1., 0.0)),
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        Ground,
        SceneRoot(assets.ground.clone()),
    )
}
