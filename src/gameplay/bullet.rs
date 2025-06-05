use std::time::Duration;

use crate::PausableSystems;

use super::*;

pub fn bullet_plugin(app: &mut App) {
    app.add_systems(Update, despawn_bullets.in_set(PausableSystems));
}

fn despawn_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Bullet)>,
) {
    for (e, mut bullet) in bullets.iter_mut() {
        bullet.timer.tick(time.delta());
        if bullet.timer.just_finished() {
            commands.entity(e).despawn();
        }
    }
}

#[derive(Component)]
pub struct Bullet {
    pub timer: Timer,
    pub damage: f32,
}

#[cfg_attr(feature = "dev_native", hot)]
pub fn bullet(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    direction: Dir3,
    speed: f32,
    at: Vec3,
) -> impl Bundle {
    let mut material = StandardMaterial::from_color(RED);
    material.emissive = LinearRgba::rgb(100.0, 10.0, 10.0);

    let size = 0.15;

    (
        Name::new("Bullet"),
        Mesh3d(meshes.add(Sphere::new(size))),
        MeshMaterial3d(materials.add(material)),
        RigidBody::Dynamic,
        Mass(20.),
        Collider::sphere(size),
        LinearVelocity(direction * speed),
        Transform::from_rotation(Quat::from_rotation_x(90f32.to_radians())).with_translation(at), // .rotate_local_x(90f32.to_radians()),
        Bullet {
            timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
            damage: 100.,
        },
        PointLight {
            color: ORANGE_RED.into(),
            intensity: 50_000.,
            ..Default::default()
        },
        CollisionEventsEnabled,
        CollidingEntities::default(),
    )
}
