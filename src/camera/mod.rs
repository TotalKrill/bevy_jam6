use crate::gameplay::tractor::{TRACTOR_MAX_SPEED, Tractor};
use avian3d::prelude::{LinearVelocity, RayCaster};
use bevy::core_pipeline::bloom::Bloom;
use bevy::prelude::*;
use bevy_editor_cam::controller::projections;
use bevy_editor_cam::prelude::zoom::ZoomLimits;
use bevy_editor_cam::prelude::{EditorCam, OrbitConstraint};
use bevy_rts_camera::RtsCamera;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera);
    app.add_systems(Update, (toggle_camera, move_rts_camera));
}

fn move_rts_camera(
    mut camera: Single<&mut RtsCamera>,
    player: Single<(&Transform, &LinearVelocity), With<Tractor>>,
) {
    camera.target_focus.translation = player.0.translation;
    camera.snap = true;
    camera.target_zoom = 1. - (player.1.length().abs() / TRACTOR_MAX_SPEED).clamp(0.01, 0.99);
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        RtsCamera {
            height_min: 30.,
            height_max: 90.,
            smoothness: 0.6,
            ..Default::default()
        },
        Camera3d::default(),
        #[cfg(target_family = "wasm")]
        Msaa::Off,
        Name::new("GameplayCamera"),
        Bloom::NATURAL,
        Camera {
            hdr: true,
            is_active: true,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 10.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        RayCaster::default(),
    ));
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
                ec.insert(RtsCamera {
                    height_min: 30.,
                    height_max: 90.,
                    smoothness: 0.6,
                    ..Default::default()
                })
                .remove::<EditorCam>();
            }
            CamState::Debug => {
                ec.insert(EditorCam {
                    zoom_limits: ZoomLimits {
                        min_size_per_pixel: 1e-6, // Any smaller and floating point rendering artifacts appear.
                        max_size_per_pixel: 1e27, // The diameter of the observable universe is probably a good upper limit.
                        zoom_through_objects: true,
                    },
                    orbit_constraint: OrbitConstraint::Fixed {
                        up: Vec3::Y,
                        can_pass_tdc: false,
                    },
                    last_anchor_depth: -cam_trans.translation.length() as f64,
                    orthographic: projections::OrthographicSettings {
                        scale_to_near_clip: 1_000_f32, // Needed for SSAO to work in ortho
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .remove::<RtsCamera>();
            }
        }
    }
}
