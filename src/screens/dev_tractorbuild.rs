use super::*;
use crate::gameplay::{
    hud::spawn_hud,
    level,
    tractor::{self, TractorAssets},
    turret_aiming,
};
use avian3d::math::PI;
use bevy::pbr::CascadeShadowConfigBuilder;
use crate::gameplay::WorldAssets;
use bevy_editor_cam::prelude::*;

pub(super) fn plugin(app: &mut App) {
    if !app.is_plugin_added::<MinimalEditorCamPlugin>() {
        app.add_plugins(DefaultEditorCamPlugins);
    }

    // Toggle pause on key press.
    app.add_systems(OnEnter(Screen::TractorBuild), setup_devscreen);

    // app.add_systems(OnEnter(Screen::TractorBuild), activate_debug_camera);
    // app.add_systems(OnEnter(Screen::TractorBuild), activate_gameplay_camera);

    // app.add_systems(
    //     Update,
    //     fire_bullets
    //         .run_if(in_state(Screen::TractorBuild))
    //         .run_if(on_timer(Duration::from_secs(1))),
    // );
}

use crate::{ReplaceOnHotreload, gameplay::controls::InTractor};
#[cfg_attr(feature = "dev_native", hot(rerun_on_hot_patch = true))]
fn setup_devscreen(
    mut commands: Commands,
    tractor_assets: Res<TractorAssets>,
    world_assets: Res<WorldAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, With<ReplaceOnHotreload>>,
) {
    use bevy_enhanced_input::prelude::Actions;

    for e in query.iter() {
        commands.entity(e).despawn();
    }

    commands.spawn((ReplaceOnHotreload, turret_aiming::sight()));

    hud::spawn_hud(&mut commands);

    log::info!("spawning tractor");
    tractor::spawn_tractor(
        &mut commands,
        &mut meshes,
        &mut materials,
        &tractor_assets,
        (
            StateScoped(Screen::TractorBuild),
            ReplaceOnHotreload,
            Actions::<InTractor>::default(),
        ),
    );

    // Spawn the Sun
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 10.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // CascadeShadowConfigBuilder {
        //     first_cascade_far_bound: 4.0,
        //     maximum_distance: 10.0,
        //     ..default()
        // }.build(),
    ));

    /// ui
    use crate::gameplay::hud::{self, *};
    spawn_hud(&mut commands);

    commands.spawn((
        StateScoped(Screen::TractorBuild),
        level::level(world_assets, meshes, materials),
    ));

    // Spawn the Sun
    commands.spawn((
        ReplaceOnHotreload,
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT / 2.0,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
            .build(),
    ));

    // commands.spawn(PerfUiAllEntries::default());
}
