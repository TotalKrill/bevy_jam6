use super::*;
use crate::gameplay::{
    level,
    tractor::{self, TractorAssets},
};

use bevy_editor_cam::prelude::*;

pub(super) fn plugin(app: &mut App) {
    if !app.is_plugin_added::<MinimalEditorCamPlugin>() {
        app.add_plugins(DefaultEditorCamPlugins);
    }
    // Toggle pause on key press.
    app.add_systems(OnEnter(Screen::TractorBuild), spawn_tractor);
    app.add_systems(OnEnter(Screen::TractorBuild), spawn_debug_camera);
}

fn spawn_debug_camera(mut commands: Commands, query: Query<Entity, With<Camera3d>>) {
    for camera in query.iter() {
        commands.entity(camera).despawn();
    }

    let cam_trans = Transform::from_xyz(2.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((
        Camera3d::default(),
        Camera {
            hdr: true,
            ..Default::default()
        },
        cam_trans,
        // Tonemapping::AcesFitted,
        // Bloom::default(),
        EditorCam {
            orbit_constraint: OrbitConstraint::Free,
            last_anchor_depth: -cam_trans.translation.length() as f64,
            orthographic: projections::OrthographicSettings {
                scale_to_near_clip: 1_000_f32, // Needed for SSAO to work in ortho
                ..Default::default()
            },
            ..Default::default()
        },
        // ScreenSpaceAmbientOcclusion::default(),
        // Smaa::default(),
        Msaa::Off,
    ));
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

    log::info!("spawning tractor");
    tractor::spawn_tractor(
        &mut commands,
        &assets,
        (StateScoped(Screen::TractorBuild), ReplaceOnHotreload),
    );

    // commands.spawn((
    //     ReplaceOnHotreload,
    //     StateScoped(Screen::TractorBuild),
    //     level::level(&mut meshes, &mut materials),
    // ));
}
