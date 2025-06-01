use crate::{gameplay::tractor::Tractor, screens::Screen};

use super::*;
use bevy::log;
use bevy_enhanced_input::prelude::*;

#[derive(Debug, InputAction)]
#[input_action(output = Vec3)]
struct Move;

#[derive(InputContext)]
struct InTractor;

fn tractor_move(mut tractor: Single<&mut Transform, With<Tractor>>, time: Res<Time>) {
    // Apply movement based on input

    // tractor.translation += Vec3::new(1.0, 0.0, 0.0) * time.elapsed_secs() * 0.1;
}

fn bind_actions(trigger: Trigger<Binding<InTractor>>, mut actions: Query<&mut Actions<InTractor>>) {
    debug!("Binding actions");
    let mut actions = actions.get_mut(trigger.target()).unwrap();
    actions
        .bind::<Move>()
        .to(Spatial::wasd_and(KeyCode::Space, KeyCode::ShiftLeft));
}

fn apply_movement(
    trigger: Trigger<Fired<Move>>,
    // mut tractor_velocity: Single<&mut LinearVelocity, With<Tractor>>,
    mut query: Query<(&Tractor, &mut LinearVelocity)>,
    time: Res<Time>,
) {
    debug!("Applying movement: {:?}", trigger.value);

    for (tractor, mut tractor_velocity) in &mut query {
        if tractor_velocity.length() > 10.0 {
            continue; // Limit speed
        }

        tractor_velocity.x += trigger.value.x * time.elapsed_secs() * 0.1;
        tractor_velocity.y += trigger.value.y * time.elapsed_secs() * 0.1;
        tractor_velocity.z += trigger.value.z * time.elapsed_secs() * 0.1;
    }

    // tractor_velocity. += trigger.value * time.elapsed_secs() * 0.1;
}

pub(super) fn plugin(app: &mut App) {
    debug!("Adding movement controls plugin");

    app.add_plugins(EnhancedInputPlugin);

    app.add_systems(Update, (tractor_move.run_if(in_state(Screen::Gameplay)),));
    app.add_systems(Startup, spawn);

    let mut actions = Actions::<InTractor>::default();

    // actions
    //     .bind::<Move>()
    //     .to((KeyCode::KeyW, GamepadButton::South));

    app.add_input_context::<InTractor>()
        .add_observer(bind_actions)
        .add_observer(apply_movement);
}

fn spawn(mut commands: Commands) {
    debug!("Spawning movement controls");
    commands.spawn(Actions::<InTractor>::default());
}
