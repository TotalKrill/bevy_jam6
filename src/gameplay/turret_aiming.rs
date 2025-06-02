use bevy_mod_lookat::RotateTo;

use crate::gameplay::turret::Turret;

use super::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (move_sight, aim_all_turrets_to_sight));
}

#[derive(Component, Debug)]
pub struct Sight;

pub fn sight() -> impl Bundle {
    (Sight, Transform::from_translation(Vec3::X * 1000.))
}

fn aim_all_turrets_to_sight(
    mut commands: Commands,
    sight: Single<Entity, With<Sight>>,
    turrets: Query<Entity, (With<Turret>, Without<RotateTo>)>,
) {
    for turret in turrets.iter() {
        commands.entity(turret).insert(RotateTo {
            entity: *sight,
            updir: bevy_mod_lookat::UpDirection::Parent,
        });
    }
}

fn move_sight(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    ground: Single<&GlobalTransform, With<level::Ground>>,
    mut sight: Single<&mut Transform, With<Sight>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let Ok(windows) = windows.single() else {
        return;
    };

    let (camera, camera_transform) = *camera_query;

    let Some(cursor_position) = windows.cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) =
        ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    else {
        return;
    };
    let point = ray.get_point(distance);
    let above_ground = point + ground.up() * 0.4;

    sight.translation = above_ground;

    // Draw a circle just above the ground plane at that position.
    gizmos.circle(
        Isometry3d::new(
            above_ground,
            Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
        ),
        0.3,
        Color::WHITE,
    );
}
