use crate::gameplay::tractor::Tractor;
use avian3d::prelude::RayCaster;
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use bevy_editor_cam::controller::projections;
use bevy_editor_cam::prelude::zoom::ZoomLimits;
use bevy_editor_cam::prelude::{EditorCam, OrbitConstraint};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera);
    app.add_systems(Update, (move_gameplay_camera, toggle_camera));
}

#[derive(Component)]
pub struct GameplayCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        GameplayCamera,
        Camera3d::default(),
        Name::new("GameplayCamera"),
        AtmosphereCamera::default(),
        Camera {
            hdr: true,
            is_active: true,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 10.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        RayCaster::default(),
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

#[derive(Default)]
enum CamState {
    #[default]
    Gameplay,
    Debug,
}

impl CamState {
    pub fn toggle(&mut self) {
        match self {
            CamState::Gameplay => *self = CamState::Debug,
            CamState::Debug => *self = CamState::Gameplay,
        }
    }
}

fn toggle_camera(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    camera: Single<(Entity, &mut Camera)>,
    mut local: Local<CamState>,
) {
    if kb_input.just_pressed(KeyCode::KeyC) {
        local.toggle();

        let (cam_e, _cam) = &*camera;
        let mut ec = commands.entity(*cam_e);

        let cam_trans = Transform::from_xyz(2.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y);

        match *local {
            CamState::Gameplay => {
                ec.insert(GameplayCamera).remove::<EditorCam>();
            }
            CamState::Debug => {
                ec.insert(EditorCam {
                    zoom_limits: ZoomLimits {
                        min_size_per_pixel: 1e-6, // Any smaller and floating point rendering artifacts appear.
                        max_size_per_pixel: 1e27, // The diameter of the observable universe is probably a good upper limit.
                        zoom_through_objects: true,
                    },
                    orbit_constraint: OrbitConstraint::Free,
                    last_anchor_depth: -cam_trans.translation.length() as f64,
                    orthographic: projections::OrthographicSettings {
                        scale_to_near_clip: 1_000_f32, // Needed for SSAO to work in ortho
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .remove::<GameplayCamera>();
            }
        }
    }
}
