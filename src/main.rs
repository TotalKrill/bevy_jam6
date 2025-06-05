// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use avian3d::prelude::*;

mod asset_tracking;
mod audio;

//all the gameplay stuff

#[cfg(feature = "dev")]
mod dev_tools;
mod gameplay;
mod menus;
mod screens;
mod theme;

mod camera;
mod leaderboard;

use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Bevy Practice".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );

        #[cfg(feature = "dev_native")]
        app.add_plugins(SimpleSubsecondPlugin::default());

        #[cfg(not(target_family = "wasm"))]
        {
            use bevy_atmosphere::prelude::*;
            app.add_plugins(AtmospherePlugin);
        }

        app.add_plugins(PhysicsPlugins::default());
        app.add_plugins(PhysicsDebugPlugin::default());
        app.add_systems(
            OnEnter(Pause(true)),
            |mut physics: ResMut<Time<Physics>>| {
                info!("starting physics!");
                physics.pause();
            },
        );
        app.add_systems(
            OnEnter(Pause(false)),
            |mut physics: ResMut<Time<Physics>>| {
                info!("starting physics!");
                physics.unpause();
            },
        );

        app.add_plugins(bevy_mod_lookat::RotateTowardsPlugin::default());
        // Add other plugins.
        app.add_plugins((
            camera::plugin,
            leaderboard::plugin,
            asset_tracking::plugin,
            audio::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            #[cfg(feature = "dev")]
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            #[cfg(feature = "dev")]
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Digit0)),
            menus::plugin,
            screens::plugin,
            theme::plugin,
            gameplay::plugin,
        ));


        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        // Set up the `Pause` state.
        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));
    }
}

#[derive(Component, Clone)]
pub struct ReplaceOnHotreload;

#[cfg_attr(feature = "dev_native", hot(rerun_on_hot_patch))]
pub fn cleanup(mut commands: Commands, to_replace: Query<Entity, With<ReplaceOnHotreload>>) {
    for entity in to_replace.iter() {
        commands.entity(entity).despawn();
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;

#[cfg(feature = "dev_native")]
use bevy_simple_subsecond_system::prelude::*;
