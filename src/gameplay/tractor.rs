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
pub struct Tractor {
    pub front_left_wheel: Entity,
}

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

#[derive(Component)]
pub struct Wheel;

pub fn spawn_tractor<T: Bundle + Clone>(
    commands: &mut Commands,
    assets: &TractorAssets,
    extra_components: T,
) -> Entity {
    let tractor_id = commands
        .spawn((tractor_body(assets), extra_components.clone()))
        .id();

    let front_left_wheel = commands
        .spawn((
            wheel(
                FRONT_WHEEL_DIAMETER,
                // Vec3::new(
                //     TRACTOR_LENGTH / 2. - FRONT_WHEEL_DIAMETER,
                //     -TRACTOR_HEIGHT / 2.0,
                //     TRACTOR_WIDTH / 2.0 + WHEEL_WIDTH + 0.1,
                // ),
                Vec3::ZERO,
                Wheel,
            ),
            extra_components.clone(),
        ))
        .id();

    commands.spawn((
        RevoluteJoint::new(tractor_id, front_left_wheel)
            // .with_local_anchor_2(Vec3::new(
            //     TRACTOR_LENGTH / 2. - FRONT_WHEEL_DIAMETER,
            //     -TRACTOR_HEIGHT / 2.0,
            //     TRACTOR_WIDTH / 2.0,
            // ))
            .with_local_anchor_1(Vec3::new(
                TRACTOR_LENGTH / 2. - FRONT_WHEEL_DIAMETER,
                -TRACTOR_HEIGHT / 2.0,
                TRACTOR_WIDTH / 2.0 + 0.1,
            ))
            // .with_local_anchor_2(Vec3::Y * WHEEL_WIDTH / 2.0)
            .with_aligned_axis(Vec3::Y),
        extra_components.clone(),
    ));

    commands
        .entity(tractor_id)
        .insert(Tractor { front_left_wheel });

    tractor_id
}

pub fn tractor_body(assets: &TractorAssets) -> impl Bundle {
    (
        Name::new("Tractor"),
        children![(
            Transform::from_xyz(0., -TRACTOR_HEIGHT / 2., 0.),
            // SceneRoot(assets.tractor.clone()),
        ),],
        RigidBody::Static,
        Collider::cuboid(TRACTOR_LENGTH, TRACTOR_HEIGHT, TRACTOR_WIDTH),
    )
}

pub fn wheel<T: Component>(radius: f32, pos: Vec3, marker: T) -> impl Bundle {
    (
        Name::new("Wheel"),
        RigidBody::Dynamic,
        Collider::cylinder(radius, WHEEL_WIDTH),
        Transform {
            translation: pos,
            rotation: Quat::from_rotation_x(90_f32.to_radians()),
            ..default()
        },
        marker,
    )
}
