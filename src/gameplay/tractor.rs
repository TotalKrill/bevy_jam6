use avian3d::prelude::*;

use crate::asset_tracking::LoadResource;

use super::*;

pub const TRACTOR_WIDTH: f32 = 1.0;
pub const TRACTOR_HEIGHT: f32 = 2.0;
pub const TRACTOR_LENGTH: f32 = 4.0;

pub const TRACTOR_MAX_SPEED: f32 = 10.0;

pub const WHEEL_RADIE: f32 = 0.9;

#[derive(Component, Debug)]
#[relationship(relationship_target = LeftWheels)]
pub struct LeftWheel {
    #[relationship]
    pub vehicle: Entity,
}

#[derive(Component, Debug)]
#[relationship_target(relationship = LeftWheel)]
pub struct LeftWheels(Vec<Entity>);

#[derive(Component, Debug)]
#[relationship(relationship_target = RightWheels)]
pub struct RightWheel {
    #[relationship]
    pub vehicle: Entity,
}

#[derive(Component, Debug)]
#[relationship_target(relationship = RightWheel)]
pub struct RightWheels(Vec<Entity>);

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

#[derive(Component)]
pub struct Tractor;

pub fn spawn_tractor<T: Bundle + Clone>(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    assets: &TractorAssets,
    extra_components: T,
) -> Entity {
    let tractor_id = commands
        .spawn((tractor_body(assets), extra_components.clone()))
        .with_child(turret::turret(
            meshes,
            materials,
            Vec3::new(0.5, tractor::TRACTOR_HEIGHT / 2.0 + turret::BODY_RADIE, 0.0),
        ))
        .id();

    let wheel_pos = Vec3::new(
        -(TRACTOR_LENGTH / 2. - WHEEL_RADIE),
        -TRACTOR_HEIGHT / 2.0 + WHEEL_RADIE / 2.,
        TRACTOR_WIDTH / 2.0 + 0.1 + WHEEL_RADIE,
    );

    left_wheel_with_joint(
        commands,
        // (extra_components.clone(), Mass(2.)),
        extra_components.clone(),
        tractor_id,
        wheel_pos,
        0.0,
    );

    let wheel_pos = Vec3::new(
        TRACTOR_LENGTH / 2. - WHEEL_RADIE,
        -TRACTOR_HEIGHT / 2.0 + WHEEL_RADIE / 2.,
        TRACTOR_WIDTH / 2.0 + 0.1 + WHEEL_RADIE,
    );
    left_wheel_with_joint(
        commands,
        // (extra_components.clone(), Mass(2.)),
        extra_components.clone(),
        tractor_id,
        wheel_pos,
        1.0,
    );

    let wheel_pos = Vec3::new(
        TRACTOR_LENGTH / 2. - WHEEL_RADIE,
        -TRACTOR_HEIGHT / 2.0 + WHEEL_RADIE / 2.,
        -(TRACTOR_WIDTH / 2.0 + 0.1 + WHEEL_RADIE),
    );
    right_wheel_with_joint(
        commands,
        extra_components.clone(),
        // (extra_components.clone(), Mass(2.)),
        tractor_id,
        wheel_pos,
        1.0,
    );

    let wheel_pos = Vec3::new(
        -(TRACTOR_LENGTH / 2. - WHEEL_RADIE),
        -TRACTOR_HEIGHT / 2.0 + WHEEL_RADIE / 2.,
        -(TRACTOR_WIDTH / 2.0 + 0.1 + WHEEL_RADIE),
    );
    right_wheel_with_joint(
        commands,
        // (extra_components.clone(), Mass(2.)),
        extra_components.clone(),
        tractor_id,
        wheel_pos,
        0.0,
    );

    tractor_id
}

fn left_wheel_with_joint<T: Bundle + Clone>(
    commands: &mut Commands<'_, '_>,
    extra_components: T,
    tractor_id: Entity,
    mut wheel_pos: Vec3,
    friction: f32,
) {
    const OFFSET: f32 = 0.1;
    let front_left_wheel = commands
        .spawn((
            wheel(WHEEL_RADIE, wheel_pos.clone(), Wheel),
            LeftWheel {
                vehicle: tractor_id,
            },
            Friction::new(friction),
            extra_components.clone(),
        ))
        .id();

    wheel_pos.z = wheel_pos.z - (WHEEL_RADIE + OFFSET);
    commands.spawn((
        RevoluteJoint::new(tractor_id, front_left_wheel)
            .with_local_anchor_1(wheel_pos)
            .with_local_anchor_2(-Vec3::Z * (WHEEL_RADIE + OFFSET))
            .with_angular_velocity_damping(0.0)
            .with_aligned_axis(Vec3::Z),
        extra_components.clone(),
    ));
}
fn right_wheel_with_joint<T: Bundle + Clone>(
    commands: &mut Commands<'_, '_>,
    extra_components: T,
    tractor_id: Entity,
    mut wheel_pos: Vec3,
    friction: f32,
) {
    const OFFSET: f32 = 0.1;
    let front_left_wheel = commands
        .spawn((
            wheel(WHEEL_RADIE, wheel_pos.clone(), Wheel),
            RightWheel {
                vehicle: tractor_id,
            },
            Friction::new(friction),
            extra_components.clone(),
        ))
        .id();

    wheel_pos.z = wheel_pos.z - (WHEEL_RADIE + OFFSET);
    commands.spawn((
        RevoluteJoint::new(tractor_id, front_left_wheel)
            .with_local_anchor_1(wheel_pos)
            .with_local_anchor_2(-Vec3::Z * (WHEEL_RADIE + 0.1))
            .with_angular_velocity_damping(0.0)
            .with_aligned_axis(Vec3::Z),
        extra_components.clone(),
    ));
}

pub fn tractor_body(assets: &TractorAssets) -> impl Bundle {
    (
        Tractor,
        Name::new("Tractor"),
        children![(
            Transform::from_xyz(0., -TRACTOR_HEIGHT / 2. - 0.4, 0.),
            SceneRoot(assets.tractor.clone()),
        ),],
        RigidBody::Dynamic,
        Mass(200.),
        // AngularInertia {
        //     principal: Vec3::splat(0.1),
        //     local_frame: Quat::IDENTITY,
        // },
        // CenterOfMass::new(TRACTOR_LENGTH / 2.0, -TRACTOR_HEIGHT / 2., 0.),
        Collider::cuboid(TRACTOR_LENGTH, TRACTOR_HEIGHT, TRACTOR_WIDTH),
    )
}

pub fn wheel<T: Component>(radius: f32, pos: Vec3, marker: T) -> impl Bundle {
    (
        Name::new("LeftWheel"),
        RigidBody::Dynamic,
        Mass(20.),
        Collider::sphere(radius),
        MaxAngularSpeed(TRACTOR_MAX_SPEED),
        // Mass(1.),
        // Collider::cylinder(radius, radius), //TODO: create a new collider with the axises correctly initiated
        Transform {
            translation: pos,
            rotation: Quat::from_rotation_z(90_f32.to_radians()),
            ..default()
        },
        marker,
    )
}
