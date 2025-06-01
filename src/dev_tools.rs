//! Development tools for the game. This plugin is only enabled in dev builds.

use avian3d::prelude::{Collider, DebugRender, PhysicsDebugPlugin};
use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
    ui::UiDebugOptions,
};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );
    app.add_plugins(PhysicsDebugPlugin::default());
}

const TOGGLE_KEY: KeyCode = KeyCode::KeyT;

fn toggle_debug_ui(
    mut options: ResMut<UiDebugOptions>,
    mut commands: Commands,
    // mut graphics_debug: ResMut<DebugRender>,
    colliders: Query<Entity, With<Collider>>,
    mut on: Local<bool>,
) {
    *on = !*on;

    for collider in colliders {
        if *on {
            commands.entity(collider).insert(DebugRender::all());
        } else {
            commands.entity(collider).insert(DebugRender::none());
        }
    }
    options.enabled = *on;
}
