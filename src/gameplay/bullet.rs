use std::time::Duration;

use super::*;
use crate::gameplay::apple::Apple;
use crate::gameplay::health::Damage;

pub fn bullet_plugin(app: &mut App) {
    app.add_systems(Update, (despawn_bullets, shoot_apples));
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
        Mass(200.),
        Collider::sphere(0.1),
        LinearVelocity(direction * speed),
        Transform::from_rotation(Quat::from_rotation_x(90f32.to_radians())).with_translation(at), // .rotate_local_x(90f32.to_radians()),
        Bullet {
            timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
            damage: 100.,
        },
        PointLight {
            color: ORANGE_RED.into(),
            intensity: 10_000.,
            ..Default::default()
        },
        CollisionEventsEnabled,
        CollidingEntities::default(),
    )
}

fn shoot_apples(
    mut collision_event_reader: EventReader<CollisionStarted>,
    bullets: Query<Entity, With<Bullet>>,
    apples: Query<Entity, With<Apple>>,
    mut event_writer: EventWriter<Damage>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        if let (Ok(apple), Ok(bullet)) = (apples.get_many(*entity1), apples.get_many(*entity2)) {
            event_writer.write(Damage {
                value: 100.0,
                entity: apple,
            });
        }
        if let (Ok(apple), Ok(bullet)) = (apples.get(*entity2), apples.get(*entity1)) {
            event_writer.write(Damage {
                value: 100.0,
                entity: apple,
            });
        }
    }
}
