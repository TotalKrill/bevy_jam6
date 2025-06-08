use bevy_mod_lookat::RotateTo;

use crate::{
    PausableSystems,
    gameplay::{apple::Apple, bullet::BULLET_SPEED, tractor::Tractor, turret::Turret},
};

use super::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (move_sight, aim_all_turrets_to_sight).in_set(PausableSystems),
    );
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
    mut raycast: MeshRayCast,
    ground: Single<&GlobalTransform, With<level::Ground>>,
    mut sight: Single<&mut Transform, (With<Sight>, Without<Apple>)>,
    windows: Query<&Window>,
    apples: Query<(&Transform, &LinearVelocity), (With<Apple>, Without<Sight>)>,
    tractor: Single<&Transform, (With<Tractor>, Without<Apple>, Without<Sight>)>,
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
    let hits = raycast.cast_ray(ray, &MeshRayCastSettings::default().always_early_exit());

    let Some((_e, hit)) = hits.first() else {
        return;
    };

    let target = if let Some((apple_t, apple_v)) = apples
        .iter()
        .filter(|(t, _v)| t.translation.distance_squared(hit.point) < 100.)
        .min_by(|(t1, _v1), (t2, _v2)| {
            t1.translation
                .distance_squared(hit.point)
                .total_cmp(&t2.translation.distance_squared(hit.point))
        }) {
        apple_t.translation
            + apple_v.0 * (apple_t.translation.distance(tractor.translation) / BULLET_SPEED)
    } else {
        hit.point + ground.up() * 0.4
    };

    sight.translation = target;

    // Draw a circle just above the ground plane at that position.
    gizmos.circle(
        Isometry3d::new(
            target,
            Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
        ),
        0.3,
        Color::WHITE,
    );
}
