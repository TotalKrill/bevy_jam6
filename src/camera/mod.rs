use crate::gameplay::tractor::{TRACTOR_MAX_SPEED, Tractor};
use avian3d::prelude::{LinearVelocity, RayCaster};
use bevy::core_pipeline::bloom::Bloom;
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use bevy_editor_cam::controller::projections;
use bevy_editor_cam::prelude::zoom::ZoomLimits;
use bevy_editor_cam::prelude::{EditorCam, OrbitConstraint};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera);
    app.add_systems(Update, (move_gameplay_camera, toggle_camera));
}

const CAMERA_DECAY_RATE: f32 = 15.0;
const CAMERA_HEIGHT_MIN: f32 = 30.0; // Height above the player
const CAMERA_HEIGHT_MAX: f32 = 60.0; // Height above the player
const CAMERA_OFFSET_MIN: f32 = 20.0; // How far back the camera sits from the player
const CAMERA_OFFSET_MAX: f32 = 30.0; // How far back the camera sits from the player
const CAMERA_ANGLE: f32 = -65.0; // Looking down at an angle (in degrees)
const VELOCITY_FILTER_WEIGHT: f32 = 0.30;
#[derive(Component)]
pub struct GameplayCamera;

#[derive(Default)]
struct PlayerVelocity {
    value: f32,
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        GameplayCamera,
        Camera3d::default(),
        Name::new("GameplayCamera"),
        AtmosphereCamera::default(),
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

fn move_gameplay_camera(
    mut camera: Single<&mut Transform, (With<GameplayCamera>, Without<Tractor>)>,
    player: Single<(&Transform, &LinearVelocity), (With<Tractor>, Without<GameplayCamera>)>,
    time: Res<Time>,
    mut player_velocity_old: Local<PlayerVelocity>,
) {
    
    // TODO Sample event N millisecond?
    // TODO Higher order low pass filtering?

    let (transform, velocity) = player.into_inner();

    let Vec3 { x, z, .. } = transform.translation; // Use x and z for horizontal movement

    let player_velocity_new = velocity.length() / TRACTOR_MAX_SPEED;

    let vel_ratio = VELOCITY_FILTER_WEIGHT
        * player_velocity_new
        + (1.0 - VELOCITY_FILTER_WEIGHT)
        * player_velocity_old.value;


    let camera_height = vel_ratio * (CAMERA_HEIGHT_MAX - CAMERA_HEIGHT_MIN) + CAMERA_HEIGHT_MIN;

    let camera_offset = vel_ratio * (CAMERA_OFFSET_MAX - CAMERA_OFFSET_MIN) + CAMERA_OFFSET_MIN;

    // let vel_old = player_velocity_old.value;

    // println!("player_velocity_new: {player_velocity_new}, player_velocity_old: {vel_old}, rat = {vel_ratio}, camera_height={camera_height}");

    player_velocity_old.value = player_velocity_new;

    let target_position = Vec3::new(x, camera_height, z + camera_offset);

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
                .remove::<GameplayCamera>();
            }
        }
    }
}
