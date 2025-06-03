use super::*;
use crate::gameplay::tractor::Tractor;
use bevy::prelude::*;
use bevy_editor_cam::controller::projections;
use bevy_editor_cam::prelude::{EditorCam, OrbitConstraint};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Startup,
        (spawn_menu_camera, spawn_gameplay_camera, spawn_debug_camera),
    );
    app.add_systems(Update, (move_gameplay_camera, toggle_camera));
}

fn spawn_menu_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("MenuCamera"),
        Camera3d::default(),
        Camera {
            hdr: true,
            is_active: true,
            ..Default::default()
        },
        Transform::from_translation(Vec3::splat(20.)).looking_at(Vec3::splat(0.), Vec3::Y),
    ));
}

#[derive(Component)]
pub struct GameplayCamera;

pub fn spawn_gameplay_camera(mut commands: Commands) {
    commands.spawn((
        GameplayCamera,
        Camera3d::default(),
        Name::new("GameplayCamera"),
        Camera {
            hdr: true,
            is_active: false,
            ..Default::default()
        },
        // Camera::default(),
        Transform::from_xyz(0.0, 10.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn move_gameplay_camera(
    mut camera: Single<&mut Transform, (With<GameplayCamera>, Without<Tractor>)>,
    player: Single<&Transform, (With<Tractor>, Without<GameplayCamera>)>,
    time: Res<Time>,
) {
    const CAMERA_DECAY_RATE: f32 = 10.0;
    const CAMERA_HEIGHT: f32 = 29.0; // Height above the player
    const CAMERA_OFFSET: f32 = 20.0; // How far back the camera sits from the player
    const CAMERA_ANGLE: f32 = -60.0; // Looking down at an angle (in degrees)

    let Vec3 { x, z, .. } = player.translation; // Use x and z for horizontal movement
    let target_position = Vec3::new(x, CAMERA_HEIGHT, z + CAMERA_OFFSET);

    camera
        .translation
        .smooth_nudge(&target_position, CAMERA_DECAY_RATE, time.delta_secs());

    camera.rotation = Quat::from_rotation_x(CAMERA_ANGLE.to_radians());
}

fn spawn_debug_camera(mut commands: Commands, query: Query<Entity, With<Camera3d>>) {
    for camera in query.iter() {
        commands.entity(camera).despawn();
    }

    let cam_trans = Transform::from_xyz(2.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((
        Name::new("DebugCamera"),
        Camera3d::default(),
        Camera {
            hdr: true,
            is_active: false,
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

fn activate_camera(camera_name: &str, mut cameras: Query<(&Name, &mut Camera), With<Camera>>) {
    for (name, mut camera) in &mut cameras {
        camera.is_active = false;
    }
    for (name, mut camera) in &mut cameras {
        if name.as_str() == camera_name {
            camera.is_active = true;
        }
    }
}
pub fn activate_gameplay_camera(cameras: Query<(&Name, &mut Camera), With<Camera>>) {
    activate_camera("GameplayCamera", cameras);
}

pub fn activate_debug_camera(cameras: Query<(&Name, &mut Camera), With<Camera>>) {
    activate_camera("DebugCamera", cameras);
}

pub fn activate_menu_camera(cameras: Query<(&Name, &mut Camera), With<Camera>>) {
    activate_camera("MenuCamera", cameras);
}

fn toggle_camera(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut cameras: Query<(&Name, &mut Camera), With<Camera>>,
) {
    if kb_input.just_pressed(KeyCode::KeyC) {
        let mut current_active = "";
        for (name, mut camera) in &mut cameras {
            if camera.is_active {
                current_active = name;
            }
            camera.is_active = false;
        }

        let next_camera = match current_active {
            "MenuCamera" => "GameplayCamera",
            "GameplayCamera" => "DebugCamera",
            "DebugCamera" => "MenuCamera",
            &_ => "MenuCamera",
        };

        for (name, mut camera) in &mut cameras {
            if name.as_str() == next_camera {
                camera.is_active = true;
            }
        }
    }
}
