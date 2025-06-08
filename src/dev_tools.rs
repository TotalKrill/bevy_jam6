//! Development tools for the game. This plugin is only enabled in dev builds.

use avian3d::prelude::PhysicsGizmos;
use bevy::{
    dev_tools::states::log_transitions,
    input::common_conditions::{input_just_pressed, input_toggle_active},
    prelude::*,
    ui::UiDebugOptions,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );
    app.add_plugins(EguiPlugin {
        enable_multipass_for_primary_context: true,
    });
    app.add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, TOGGLE_KEY)));
}

const TOGGLE_KEY: KeyCode = KeyCode::KeyT;

fn toggle_debug_ui(
    mut options: ResMut<UiDebugOptions>,
    // mut graphics_debug: ResMut<DebugRender>,
    mut gizmoconf: ResMut<GizmoConfigStore>,
    mut on: Local<bool>,
) {
    *on = !*on;

    let (_, physgiz) = gizmoconf.config_mut::<PhysicsGizmos>();
    if *on {
        *physgiz = PhysicsGizmos::default();
    } else {
        *physgiz = PhysicsGizmos::none();
    }
    options.enabled = *on;
}
