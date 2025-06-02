use std::time::Duration;

use super::*;
use crate::gameplay::{
    bullet, level,
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
    app.add_systems(OnEnter(Screen::TractorBuild), spawn_debug_camera);

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
    use crate::gameplay::turret;

    for e in query.iter() {
        commands.entity(e).despawn();
    }

    commands.spawn(turret_aiming::sight());

    log::info!("spawning tractor");
    let id = tractor::spawn_tractor(
        &mut commands,
        &assets,
        (StateScoped(Screen::TractorBuild), ReplaceOnHotreload),
    );

    commands.entity(id).with_child(turret::turret(
        &mut meshes,
        &mut materials,
        Vec3::new(
            0.5,
            (tractor::TRACTOR_HEIGHT / 2.0 + turret::BODY_RADIE),
            0.0,
        ),
    ));

    commands.spawn((
        ReplaceOnHotreload,
        StateScoped(Screen::TractorBuild),
        level::level(&mut meshes, &mut materials),
        Transform::from_translation(Vec3::Y * -4.),
    ));
}
