use super::*;
use crate::gameplay::{
    level,
    tractor::{self, TractorAssets},
    turret_aiming,
};

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
    assets: Res<TractorAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, With<ReplaceOnHotreload>>,
) {
    use bevy_enhanced_input::prelude::Actions;

    for e in query.iter() {
        commands.entity(e).despawn();
    }

    commands.spawn((ReplaceOnHotreload, turret_aiming::sight()));

    log::info!("spawning tractor");
    tractor::spawn_tractor(
        &mut commands,
        &mut meshes,
        &mut materials,
        &assets,
        (
            StateScoped(Screen::TractorBuild),
            ReplaceOnHotreload,
            Actions::<InTractor>::default(),
        ),
    );

    /// ui
    use crate::gameplay::hud::*;
    commands.spawn(points_node());
    commands.spawn(healthbar());

    commands.spawn((
        ReplaceOnHotreload,
        StateScoped(Screen::TractorBuild),
        level::level(&mut meshes, &mut materials),
        Transform::from_translation(Vec3::Y * -4.),
    ));
}
