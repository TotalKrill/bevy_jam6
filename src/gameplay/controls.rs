use crate::gameplay::{
    tractor::Tractor,
    turret::{BARREL_LEN, Turret},
};

use super::*;
use bevy_enhanced_input::prelude::*;
use tractor::{LeftWheels, RightWheels};

const TRACTOR_ACCELERATION: f32 = 10000.0;

#[derive(Debug, InputAction)]
#[input_action(output = Vec3)]
struct MoveEvent;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct BreakEvent;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct FireEvent;

#[derive(InputContext)]
pub struct InTractor;

pub(super) fn plugin(app: &mut App) {
    debug!("Adding movement controls plugin");

    app.add_plugins(EnhancedInputPlugin);
    app.add_systems(Update, tractor_break);

    app.add_input_context::<InTractor>()
        .add_observer(bind_actions)
        .add_observer(tractor_move)
        .add_observer(fire_turret)
        .add_observer(stop_firing_turret);
}

fn bind_actions(trigger: Trigger<Binding<InTractor>>, mut actions: Query<&mut Actions<InTractor>>) {
    debug!("Binding actions");
    let mut actions = actions.get_mut(trigger.target()).unwrap();
    actions
        .bind::<MoveEvent>()
        .to(Spatial::wasd_and(KeyCode::ArrowUp, KeyCode::ArrowDown));
    actions.bind::<BreakEvent>().to(KeyCode::Space);
    actions.bind::<FireEvent>().to(MouseButton::Left);
}

fn fire_turret(
    trigger: Trigger<Started<FireEvent>>,
    tractors: Query<&Children, With<Tractor>>,
    mut attached_turrets: Query<(&ChildOf, &GlobalTransform, &mut Turret)>,
) {
    info!("start firing!");
    let action_target = trigger.target();

    let Ok(tractor_children) = tractors.get(action_target) else {
        debug!("Goodbye");
        return;
    };

    for child in tractor_children {
        if let Ok((_childof, _t, mut turret)) = attached_turrets.get_mut(*child) {
            turret.firing = true;
        }
    }
}
fn stop_firing_turret(
    trigger: Trigger<Completed<FireEvent>>,
    tractors: Query<&Children, With<Tractor>>,
    mut attached_turrets: Query<(&ChildOf, &GlobalTransform, &mut Turret)>,
) {
    info!("Stop firing!");
    let action_target = trigger.target();

    let Ok(tractor_children) = tractors.get(action_target) else {
        debug!("Goodbye");
        return;
    };

    for child in tractor_children {
        if let Ok((_childof, _t, mut turret)) = attached_turrets.get_mut(*child) {
            turret.firing = false;
        }
    }
}

fn tractor_break(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut angular_velocity: Query<&mut AngularVelocity>,
    mut torque: Query<&mut ExternalTorque>,
    query: Query<(&Tractor, &LeftWheels, &RightWheels)>,
) {
    if keyboard.any_pressed([KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyD, KeyCode::KeyA]) {
        return;
    };

    let Ok((_, left_wheels, right_wheels)) = query.single() else {
        debug!("No tractor found, skipping brake application");
        return;
    };

    for wheel in left_wheels.iter() {
        *angular_velocity.get_mut(wheel).unwrap() = AngularVelocity::ZERO;
        torque.get_mut(wheel).unwrap().clear();
    }

    for wheel in right_wheels.iter() {
        *angular_velocity.get_mut(wheel).unwrap() = AngularVelocity::ZERO;
        torque.get_mut(wheel).unwrap().clear();
    }
}

fn tractor_move(
    trigger: Trigger<Fired<MoveEvent>>,
    mut torque: Query<&mut ExternalTorque>,
    query: Query<(&Transform, &Tractor, &LeftWheels, &RightWheels)>,
    time: Res<Time>,
) {
    // let force = trigger.value.x * time.delta_secs();

    let Ok((transform, _, left_wheels, right_wheels)) = query.single() else {
        debug!("No tractor found, skipping brake application");
        return;
    };

    let side = transform.local_z().normalize();

    let left_torque =
        side * (-trigger.value.z + trigger.value.x * 5.) * time.delta_secs() * TRACTOR_ACCELERATION;

    let right_torque =
        side * (-trigger.value.z - trigger.value.x * 5.) * time.delta_secs() * TRACTOR_ACCELERATION;

    debug!("Applying torque: (trigger={:?})", trigger.value);

    for wheel in right_wheels.iter() {
        torque.get_mut(wheel).unwrap().set_torque(right_torque);
    }

    for wheel in left_wheels.iter() {
        torque.get_mut(wheel).unwrap().set_torque(left_torque);
    }
}
