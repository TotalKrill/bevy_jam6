use std::time::Duration;

use super::*;
use crate::gameplay::{
    level,
    tractor::{self, TractorAssets},
    turret::FireEvent,
    turret_aiming,
};

use bevy::time::common_conditions::on_timer;
use bevy_editor_cam::prelude::*;

pub(super) fn plugin(app: &mut App) {
    if !app.is_plugin_added::<MinimalEditorCamPlugin>() {
        app.add_plugins(DefaultEditorCamPlugins);
    }
    // Toggle pause on key press.
    app.add_systems(OnEnter(Screen::TractorBuild), spawn_tractor);
    // app.add_systems(OnEnter(Screen::TractorBuild), activate_debug_camera);
    // app.add_systems(OnEnter(Screen::TractorBuild), activate_gameplay_camera);

    app.add_systems(
        Update,
        fire_bullets
            .run_if(in_state(Screen::TractorBuild))
            .run_if(on_timer(Duration::from_secs(1))),
    );
}

fn fire_bullets(
    mut commands: Commands,
    turrets: Query<Entity, With<crate::gameplay::turret::Turret>>,
) {
    for turret in turrets {
        commands.trigger_targets(FireEvent, turret);
    }
}

#[cfg_attr(feature = "dev_native", hot(rerun_on_hot_patch = true))]
fn spawn_tractor(
    mut commands: Commands,
    assets: Res<TractorAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, With<ReplaceOnHotreload>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }

    commands.spawn(turret_aiming::sight());

    log::info!("spawning tractor");
    tractor::spawn_tractor(
        &mut commands,
        &mut meshes,
        &mut materials,
        &assets,
        (StateScoped(Screen::TractorBuild), ReplaceOnHotreload),
    );

    commands.spawn((
        ReplaceOnHotreload,
        StateScoped(Screen::TractorBuild),
        level::level(&mut meshes, &mut materials),
        Transform::from_translation(Vec3::Y * -4.),
    ));
}
