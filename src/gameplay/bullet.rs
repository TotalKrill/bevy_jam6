use std::time::Duration;

use bevy::color::palettes::css::*;

use super::*;

pub fn bullet_plugin(app: &mut App) {
    app.add_systems(Update, despawn_bullets);
}

fn despawn_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Bullet)>,
) {
    for (e, mut bullet) in bullets.iter_mut() {
        bullet.0.tick(time.delta());
        if bullet.0.just_finished() {
            commands.entity(e).despawn();
        }
    }
}

#[derive(Component)]
pub struct Bullet(pub Timer);

pub fn bullet(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    direction: Dir3,
    speed: f32,
    at: Vec3,
) -> impl Bundle {
    (
        Name::new("Bullet"),
        Mesh3d(meshes.add(Sphere::new(0.1))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(RED))),
        RigidBody::Dynamic,
        Collider::sphere(0.1),
        LinearVelocity(direction * speed),
        Transform::from_rotation(Quat::from_rotation_x(90f32.to_radians())).with_translation(at), // .rotate_local_x(90f32.to_radians()),
        Bullet(Timer::new(Duration::from_secs(3), TimerMode::Once)),
        PointLight {
            color: ORANGE_RED.into(),
            intensity: 10_000.,
            ..Default::default()
        },
    )
}
