#[derive(Component)]
pub struct Ground;

use super::*;
pub fn level(assets: &WorldAssets) -> impl Bundle {
    (
        RigidBody::Static,
        Friction::new(1.0),
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        Ground,
        Name::new("Ground"),
        SceneRoot(assets.ground.clone()),
    )
}
