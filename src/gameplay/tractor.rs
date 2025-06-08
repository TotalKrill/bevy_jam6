use std::time::Duration;

use super::*;
use crate::gameplay::apple::Apple;
use crate::gameplay::health::{Death, Health};
use crate::gameplay::level::TERRAIN_HEIGHT;
use crate::{ReplaceOnHotreload, asset_tracking::LoadResource};
use avian3d::prelude::*;
use bevy_tweening::lens::{TransformPositionLens, TransformRotateXLens};
use bevy_tweening::{Animator, RepeatCount, RepeatStrategy, Sequence, Tween};

pub const TRACTOR_WIDTH: f32 = 1.0;
pub const TRACTOR_HEIGHT: f32 = 2.0;
pub const TRACTOR_LENGTH: f32 = 4.0;

pub const TRACTOR_MAX_SPEED: f32 = 15.0;

pub const WHEEL_RADIE: f32 = 0.4;
pub const SAW_DEFAULT_RRATE_OF_FIRE: Duration = Duration::from_millis(500);
pub const SAW_DEFAULT_DAMAGE: u32 = 1;

pub fn tractor_plugin(app: &mut App) {
    app.load_resource::<TractorAssets>();

    app.add_systems(Update, kill_tractor_below_map);

    // add meshes to wheels
    app.add_observer(
        |trigger: Trigger<OnAdd, Wheel>, mut commands: Commands, assets: Res<TractorAssets>| {
            if let Ok(mut ec) = commands.get_entity(trigger.target()) {
                ec.insert(SceneRoot(assets.wheelball.clone()));
            }
        },
    );
}

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
    wheelball: Handle<Scene>,
    saw: Handle<Scene>,
    sound_hurt: Handle<AudioSource>,
}

impl FromWorld for TractorAssets {
    fn from_world(world: &mut World) -> Self {
        let assets: &AssetServer = world.resource::<AssetServer>();
        Self {
            tractor: assets
                .load(GltfAssetLabel::Scene(0).from_asset("models/tractor/tractor_scaled.glb")),
            wheelball: assets
                .load(GltfAssetLabel::Scene(0).from_asset("models/wheelball/wheelball.glb")),
            saw: assets.load(GltfAssetLabel::Scene(0).from_asset("models/saw/saw.glb")),
            sound_hurt: assets.load::<AudioSource>("audio/sound_effects/tractor-damage.wav"),
        }
    }
}

#[derive(Component)]
pub struct Wheel;

#[derive(Component)]
pub struct Tractor;

#[derive(Component)]
pub struct TractorSaw {
    pub rate_of_fire: Duration,
    pub damage: u32,
}

pub fn spawn_tractor<T: Bundle>(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    assets: &TractorAssets,
    extra_components: T,
) -> Entity {
    let tractor_id = commands
        .spawn((tractor_body(assets), extra_components))
        .observe(
            |trigger: Trigger<Death>, mut commands: Commands, mut writer: EventWriter<GameOver>| {
                // gameover when tractor dies
                writer.write(GameOver);

                commands
                    .get_entity(trigger.target().entity())
                    .unwrap()
                    .despawn();
            },
        )
        .with_child(turret::turret(
            meshes,
            materials,
            Vec3::new(
                0.0,
                tractor::TRACTOR_HEIGHT / 2.0 + turret::BODY_RADIE + WHEEL_RADIE,
                0.75,
            ),
        ))
        .id();

    spawn_tractor_saw(assets, tractor_id, commands);

    let wheel_offset_x = TRACTOR_WIDTH / 2.0 + 0.2 + WHEEL_RADIE;
    let wheel_offset_z = TRACTOR_LENGTH / 2.0 - WHEEL_RADIE - 0.2;
    let wheel_offset_y = -TRACTOR_HEIGHT / 2.0 + WHEEL_RADIE / 2. + 0.1;

    let wheel_pos = Vec3::new(-wheel_offset_x, wheel_offset_y, wheel_offset_z);
    left_wheel_with_joint(commands, ReplaceOnHotreload, tractor_id, wheel_pos, 0.8);

    let wheel_pos = Vec3::new(wheel_offset_x, wheel_offset_y, wheel_offset_z);
    left_wheel_with_joint(commands, ReplaceOnHotreload, tractor_id, wheel_pos, 0.8);

    let wheel_pos = Vec3::new(-wheel_offset_x, wheel_offset_y, -wheel_offset_z);
    right_wheel_with_joint(commands, ReplaceOnHotreload, tractor_id, wheel_pos, 0.8);

    let wheel_pos = Vec3::new(wheel_offset_x, wheel_offset_y, -wheel_offset_z);
    right_wheel_with_joint(commands, ReplaceOnHotreload, tractor_id, wheel_pos, 0.8);

    tractor_id
}

fn left_wheel_with_joint<T: Bundle + Clone>(
    commands: &mut Commands<'_, '_>,
    extra_components: T,
    tractor_id: Entity,
    wheel_pos: Vec3,
    friction: f32,
) {
    let front_left_wheel = commands
        .spawn((
            wheel(WHEEL_RADIE, wheel_pos),
            LeftWheel {
                vehicle: tractor_id,
            },
            Friction::new(friction),
            extra_components.clone(),
        ))
        .id();

    commands.spawn((
        RevoluteJoint::new(tractor_id, front_left_wheel)
            .with_local_anchor_1(wheel_pos)
            .with_local_anchor_2(Vec3::ZERO)
            .with_angular_velocity_damping(0.0)
            .with_aligned_axis(-Vec3::X),
        extra_components.clone(),
    ));
}
fn right_wheel_with_joint<T: Bundle + Clone>(
    commands: &mut Commands<'_, '_>,
    extra_components: T,
    tractor_id: Entity,
    wheel_pos: Vec3,
    friction: f32,
) {
    let front_left_wheel = commands
        .spawn((
            wheel(WHEEL_RADIE, wheel_pos),
            RightWheel {
                vehicle: tractor_id,
            },
            Friction::new(friction),
            extra_components.clone(),
        ))
        .id();

    commands.spawn((
        RevoluteJoint::new(tractor_id, front_left_wheel)
            .with_local_anchor_1(wheel_pos)
            .with_local_anchor_2(Vec3::ZERO)
            .with_angular_velocity_damping(0.0)
            .with_aligned_axis(Vec3::X),
        extra_components.clone(),
    ));
}

pub fn tractor_body(assets: &TractorAssets) -> impl Bundle {
    (
        Tractor,
        MaxLinearSpeed(TRACTOR_MAX_SPEED),
        Name::new("Tractor"),
        CollisionEventsEnabled,
        children![(
            Transform {
                translation: vec3(0.0, -TRACTOR_HEIGHT / 2. - 0.1, 0.2),
                rotation: Quat::from_rotation_y(-90_f32.to_radians()),
                ..default()
            },
            SceneRoot(assets.tractor.clone()),
        ),],
        RigidBody::Dynamic,
        Health::new(5),
        CenterOfMass::new(0.0, -TRACTOR_HEIGHT / 2.0, 0.0),
        Collider::cuboid(
            TRACTOR_WIDTH,
            TRACTOR_HEIGHT,
            TRACTOR_LENGTH - WHEEL_RADIE * 2.,
        ),
        CollidingEntities::default(),
    )
}

pub fn spawn_tractor_saw(assets: &TractorAssets, tractor_id: Entity, commands: &mut Commands) {
    let animation_length_x = 1.5;

    let saw_pos = Vec3::new(
        -(TRACTOR_WIDTH / 2.0 - WHEEL_RADIE),
        0.0,
        -(TRACTOR_LENGTH / 2.0),
    );

    let saw_pos_1 = Vec3::new(
        -(TRACTOR_WIDTH / 2.0 - WHEEL_RADIE) + animation_length_x / 2.,
        0.0,
        0.0, //-(TRACTOR_LENGTH / 2.0),
    );
    let saw_pos_2 = Vec3::new(
        -(TRACTOR_WIDTH / 2.0 - WHEEL_RADIE) - animation_length_x / 2.,
        0.0,
        0.0, //-(TRACTOR_LENGTH / 2.0),
    );

    let animation = Tween::new(
        EaseFunction::Linear,
        Duration::from_millis(200),
        TransformPositionLens {
            start: saw_pos_1,
            end: saw_pos_2,
        },
    )
    .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
    .with_repeat_count(RepeatCount::Infinite);

    let saw = commands
        .spawn((
            TractorSaw {
                rate_of_fire: SAW_DEFAULT_RRATE_OF_FIRE,
                damage: SAW_DEFAULT_DAMAGE,
            },
            ReplaceOnHotreload,
            CollisionEventsEnabled,
            Name::new("TractorSaw"),
            RigidBody::Dynamic,
            Transform::from_translation(saw_pos),
            Collider::cuboid((TRACTOR_WIDTH + WHEEL_RADIE * 2.0) * 2.0 - 1.0, 0.5, 0.5),
            children![(
                SceneRoot(assets.saw.clone()),
                Animator::new(animation),
                Transform::from_translation(saw_pos_1),
            )],
        ))
        .id();

    commands.spawn(
        FixedJoint::new(tractor_id, saw)
            .with_local_anchor_1(saw_pos)
            .with_local_anchor_2(Vec3::ZERO),
    );
}
pub fn wheel(radius: f32, pos: Vec3) -> impl Bundle {
    (
        Name::new("Wheel"),
        CollisionEventsEnabled,
        CollidingEntities::default(),
        RigidBody::Dynamic,
        Wheel,
        Collider::sphere(radius),
        Transform {
            translation: pos,
            // rotation: Quat::from_rotation_z(90_f32.to_radians()),
            ..default()
        },
    )
}

fn kill_tractor_below_map(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Tractor>>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.y < -1. * TERRAIN_HEIGHT {
            commands.trigger_targets(Death, entity);
        }
    }
}
