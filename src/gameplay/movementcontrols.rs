use crate::gameplay::tractor::Tractor;

use super::*;
use bevy_enhanced_input::prelude::*;
use tractor::{LeftWheels, RightWheels};

const TRACTOR_ACCELERATION: f32 = 100.0;

#[derive(Debug, InputAction)]
#[input_action(output = Vec3)]
struct Move;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct Break;

#[derive(InputContext)]
struct InTractor;

fn bind_actions(trigger: Trigger<Binding<InTractor>>, mut actions: Query<&mut Actions<InTractor>>) {
    debug!("Binding actions");
    let mut actions = actions.get_mut(trigger.target()).unwrap();
    actions
        .bind::<Move>()
        .to(Spatial::wasd_and(KeyCode::ArrowUp, KeyCode::ArrowDown));
    actions.bind::<Break>().to(KeyCode::Space);
}

fn tractor_break(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut angular_velocity: Query<&mut AngularVelocity>,
    mut torque: Query<&mut ExternalTorque>,
    query: Query<(&Tractor, &LeftWheels, &RightWheels)>,
) {
    if keyboard.any_pressed([KeyCode::KeyW, KeyCode::KeyS]) {
        return;
    };

    let Ok((_, left_wheels, right_wheels)) = query.single() else {
        debug!("No tractor found, skipping break application");
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
    trigger: Trigger<Fired<Move>>,
    mut torque: Query<&mut ExternalTorque>,
    query: Query<(&Transform, &Tractor, &LeftWheels, &RightWheels)>,
    time: Res<Time>,
) {
    let Ok((transform, _, left_wheels, right_wheels)) = query.single() else {
        debug!("No tractor found, skipping break application");
        return;
    };

    let side = transform.local_z().normalize();

    let left_torque = side
        * (-trigger.value.z + trigger.value.x * 5.)
        * time.elapsed_secs()
        * TRACTOR_ACCELERATION;

    let right_torque = side
        * (-trigger.value.z - trigger.value.x * 5.)
        * time.elapsed_secs()
        * TRACTOR_ACCELERATION;

    debug!("Applying torque: (trigger={:?})", trigger.value);

    for wheel in right_wheels.iter() {
        torque.get_mut(wheel).unwrap().set_torque(right_torque);
    }

    for wheel in left_wheels.iter() {
        torque.get_mut(wheel).unwrap().set_torque(left_torque);
    }
}

pub(super) fn plugin(app: &mut App) {
    debug!("Adding movement controls plugin");

    app.add_plugins(EnhancedInputPlugin);
    app.add_systems(Startup, spawn);
    app.add_systems(Update, tractor_break);

    app.add_input_context::<InTractor>()
        .add_observer(bind_actions)
        .add_observer(tractor_move);
    // .add_observer(tractor_break);
}

fn spawn(mut commands: Commands) {
    debug!("Spawning movement controls");
    commands.spawn(Actions::<InTractor>::default());
}
