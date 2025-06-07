use std::time::Duration;
use bevy_tweening::{Animator, RepeatStrategy, Tween};
use bevy_tweening::lens::TransformPositionLens;
use super::*;

pub const BARREL_LEN: f32 = 2.0;
pub const BARREL_RADIE: f32 = 0.2;
pub const BODY_RADIE: f32 = 0.5;

pub fn turret_plugin(app: &mut App) {
    app.register_type::<Turret>();
    app.register_type::<TurretDamage>();
    app.add_systems(Update, tick_and_fire_turret);
}

#[derive(Component, Reflect)]
pub struct Turret {
    pub rate_of_fire: Timer,
    pub firing: bool,
}

#[derive(Component, Reflect)]
pub struct TurretDamage(pub u32);
impl Default for TurretDamage {
    fn default() -> Self {
        Self(1)
    }
}

use crate::gameplay::bullet::BulletSpawnEvent;
use crate::gameplay::tree::Tree;

#[cfg_attr(feature = "dev_native", hot)]
fn tick_and_fire_turret(
    mut commands: Commands,
    time: Res<Time>,
    mut turrets: Query<(Entity, &Transform, &mut Turret, &GlobalTransform, &TurretDamage)>,
    mut fire_bullet_evt: EventWriter<BulletSpawnEvent>,
) {
    use crate::gameplay::bullet::Bullet;

    for (entity, local_transform, mut turret, transform, turret_damage) in turrets.iter_mut() {
        turret.rate_of_fire.tick(time.delta());
        if turret.rate_of_fire.finished() && turret.firing {

            commands
                .entity(entity)
                .insert(Animator::new(Tween::new(
                    EaseFunction::Linear,
                    Duration::from_millis(90), // TODO Scale with damage/speed ?
                    TransformPositionLens {
                        start: local_transform.translation,
                        end: local_transform.translation - local_transform.forward().normalize() * 0.45, // TODO Scale with damage/speed ?
                    },
                ).with_repeat_count(2).with_repeat_strategy(RepeatStrategy::MirroredRepeat)));

            turret.rate_of_fire.reset();
            let forward = transform.forward();
            let bullet_spawnpoint = transform.translation() + (BARREL_LEN + 0.5) * forward;
            fire_bullet_evt.write(BulletSpawnEvent {
                at: bullet_spawnpoint,
                dir: forward,
                speed: 50.,
                bullet: Bullet::new(turret_damage.0, 1.0),
            });
        }
    }
}

pub fn turret(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
) -> impl Bundle {
    let timer = Timer::new(Duration::from_millis(1000), TimerMode::Once);

    (
        Name::new("Turret Body"),
        Turret {
            rate_of_fire: timer,
            firing: false,
        },
        Mesh3d(meshes.add(Sphere::new(BODY_RADIE))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(BLACK))),
        Transform::from_translation(pos),
        TurretDamage::default(),
        children![(
            Transform::from_rotation(Quat::from_rotation_x(-90f32.to_radians())),
            children![(
                Name::new("Turret Barrel"),
                Mesh3d(meshes.add(Cylinder::new(BARREL_RADIE, BARREL_LEN))),
                MeshMaterial3d(materials.add(StandardMaterial::from_color(GRAY))),
                // Transform::from_translation(Vec3::ZERO),
                Transform::from_translation(Vec3::Y * BARREL_LEN / 2.)
            )]
        )],
    )
}
