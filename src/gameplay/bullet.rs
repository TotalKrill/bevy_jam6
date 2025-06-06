use std::time::Duration;

use crate::{
    PausableSystems,
    audio::{SoundEffectType, SoundEffects},
    gameplay::apple::{APPLE_RADIUS, Apple},
};

use super::*;

#[derive(Event)]
pub struct BulletSpawnEvent {
    pub at: Vec3,
    pub dir: Dir3,
    pub speed: f32,
    pub bullet: Bullet,
}

#[derive(Event)]
pub struct BulletSplitEvent {
    // around where will the bullet be split
    pub center: Vec3,
    // which bullets
    pub bullet: Bullet,
}

#[derive(Resource)]
pub struct BulletAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}
const SIZE: f32 = 0.15;

impl FromWorld for BulletAssets {
    fn from_world(world: &mut World) -> Self {
        let mut material = StandardMaterial::from_color(RED);
        material.emissive = LinearRgba::rgb(100.0, 10.0, 10.0);
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        let material = materials.add(material);

        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh = meshes.add(Sphere::new(SIZE));
        Self { material, mesh }
    }
}

pub fn bullet_plugin(app: &mut App) {
    app.init_resource::<BulletAssets>();
    app.add_event::<BulletSpawnEvent>();
    app.add_event::<BulletSplitEvent>();

    app.add_systems(
        Update,
        (
            despawn_bullets,
            fire_bullet_event_handler,
            bullet_split_event_handler,
        )
            .in_set(PausableSystems),
    );
}

fn fire_bullet_event_handler(
    mut commands: Commands,
    assets: Res<BulletAssets>,
    mut spawnevent: EventReader<BulletSpawnEvent>,
    sound_effects: Res<SoundEffects>,
) {
    for evt in spawnevent.read() {
        commands.spawn(bullet(
            &assets,
            evt.bullet.clone(),
            evt.at,
            evt.dir,
            evt.speed,
        ));
        sound_effects.play_sound(&mut commands, SoundEffectType::Fire);
    }
}

#[cfg_attr(feature = "dev_native", hot)]
fn bullet_split_event_handler(
    apples: Query<&Transform, With<Apple>>,
    mut split_event: EventReader<BulletSplitEvent>,
    mut spawn_writer: EventWriter<BulletSpawnEvent>,
) {
    for evt in split_event.read() {
        for apple in apples
            .iter()
            .sort_by::<&Transform>(|t1, t2| {
                t1.translation
                    .distance_squared(evt.center)
                    .total_cmp(&t2.translation.distance_squared(evt.center))
            })
            .take(2)
        {
            if let Ok(dir) = (apple.translation - evt.center).normalize().try_into() {
                info!("Spawning new bullet!");
                spawn_writer.write(BulletSpawnEvent {
                    at: evt.center + dir * APPLE_RADIUS,
                    dir,
                    bullet: evt.bullet.clone(),
                    speed: 50.,
                });
            }
        }
    }
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

#[derive(Component, Clone)]
pub struct Bullet {
    pub timer: Timer,
    pub damage: u32,
    pub split_probability: f32,
}

impl Bullet {
    pub fn new(damage: u32, split_probability: f32) -> Self {
        Self {
            damage,
            split_probability,
            timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
        }
    }
    /// returns a bullet with half the values of the original
    pub fn half(&self) -> Self {
        Bullet {
            damage: self.damage / 2,
            split_probability: self.split_probability / 2.0,
            timer: self.timer.clone(),
        }
    }
}

#[cfg_attr(feature = "dev_native", hot)]
pub fn bullet(
    bulletasset: &BulletAssets,
    bullet: Bullet,
    at: Vec3,
    direction: Dir3,
    speed: f32,
) -> impl Bundle {
    (
        Name::new("Bullet"),
        Mesh3d(bulletasset.mesh.clone()),
        MeshMaterial3d(bulletasset.material.clone()),
        RigidBody::Dynamic,
        Mass(20.),
        Collider::sphere(SIZE),
        LinearVelocity(direction * speed),
        Transform::from_rotation(Quat::from_rotation_x(90f32.to_radians())).with_translation(at), // .rotate_local_x(90f32.to_radians()),
        bullet,
        PointLight {
            color: ORANGE_RED.into(),
            intensity: 50_000.,
            ..Default::default()
        },
        CollisionEventsEnabled,
        CollidingEntities::default(),
    )
}
