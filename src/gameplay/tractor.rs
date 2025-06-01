use avian3d::prelude::*;

use crate::asset_tracking::LoadResource;

use super::*;

pub const TRACTOR_WIDTH: f32 = 2.0;
pub const TRACTOR_HEIGHT: f32 = 2.0;
pub const TRACTOR_LENGTH: f32 = 4.0;

pub const FRONT_WHEEL_DIAMETER: f32 = 0.5;
pub const BACK_WHEEL_DIAMETER: f32 = 1.2;
pub const WHEEL_WIDTH: f32 = 0.25;

#[derive(Component)]
pub struct Tractor;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct TractorAssets {
    tractor: Handle<Scene>,
}

impl FromWorld for TractorAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            tractor: assets
                .load(GltfAssetLabel::Scene(0).from_asset("models/tractor/tractor_scaled.glb")),
        }
    }
}

pub fn tractor_plugin(app: &mut App) {
    app.load_resource::<TractorAssets>();
}

pub fn spawn_tractor(assets: &TractorAssets) -> impl Bundle {
    (
        SceneRoot(assets.tractor.clone()),
        RigidBody::Dynamic,
        Collider::cuboid(TRACTOR_WIDTH, TRACTOR_HEIGHT, TRACTOR_LENGTH),
        Tractor,
    )
}
