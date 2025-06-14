use crate::{
    PausableSystems,
    audio::sound_effect,
    gameplay::apple::{APPLE_RADIUS, Apple},
    screens::Screen,
};
use bevy_inspector_egui::egui::debug_text::print;
use std::time::Duration;

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
    // apple radius
    pub radius: f32,
}

#[derive(Resource, Asset, Clone, Reflect)]
pub struct BulletAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    sound: Handle<AudioSource>,
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

        let assets: &AssetServer = world.resource::<AssetServer>();
        let sound = assets.load::<AudioSource>("audio/sound_effects/gunfire.wav");

        Self {
            material,
            mesh,
            sound,
        }
    }
}

pub fn bullet_plugin(app: &mut App) {
    app.load_resource::<BulletAssets>();
    app.add_event::<BulletSpawnEvent>();
    app.add_event::<BulletSplitEvent>();

    app.add_systems(
        Update,
        (fire_bullet_event_handler, bullet_split_event_handler)
            .chain()
            .in_set(PausableSystems)
            .run_if(in_state(Screen::InGame)),
    );
}

#[cfg_attr(feature = "dev_native", hot)]
fn fire_bullet_event_handler(
    mut commands: Commands,
    assets: Res<BulletAssets>,
    mut spawnevent: EventReader<BulletSpawnEvent>,
) {
    for evt in spawnevent.read() {
        if evt.bullet.damage > 0 {
            commands.spawn(bullet(
                &assets,
                evt.bullet.clone(),
                evt.at,
                evt.dir,
                evt.speed,
            ));
            commands.spawn(sound_effect(assets.sound.clone()));
        }
    }
}

pub const BULLET_SPEED: f32 = 70.;

#[cfg_attr(feature = "dev_native", hot)]
fn bullet_split_event_handler(
    apples: Query<(&Transform, &LinearVelocity), With<Apple>>,
    mut split_event: EventReader<BulletSplitEvent>,
    mut spawn_writer: EventWriter<BulletSpawnEvent>,
) {
    for evt in split_event.read() {
        let apples = apples
            .iter()
            .sort_by::<&Transform>(|t1, t2| {
                t1.translation
                    .distance_squared(evt.center)
                    .total_cmp(&t2.translation.distance_squared(evt.center))
            })
            .take(3);

        let mut iter = apples.into_iter();
        iter.next();

        for (apple_t, apple_v) in iter {
            let apple_target = apple_t.translation
                + apple_v.0 * (apple_t.translation.distance(evt.center) / BULLET_SPEED);

            let distance = (apple_t.translation - evt.center).length_squared();

            if distance > evt.radius + 0.0005 {
                if let Ok(dir) = (apple_target - evt.center).normalize().try_into() {
                    spawn_writer.write(BulletSpawnEvent {
                        at: evt.center + dir * evt.radius,
                        dir,
                        bullet: evt.bullet.clone(),
                        speed: BULLET_SPEED,
                        //     TODO Add apple entity spawning the new bullet so we can later can check so it is not killed by the new bullet
                    });
                }
            }
        }
    }
}

#[derive(Component, Clone)]
pub struct Bullet {
    pub damage: u32,
    pub split_probability: f32,
}

impl Bullet {
    pub fn new(damage: u32, split_probability: f32) -> Self {
        Self {
            damage,
            split_probability,
        }
    }
    /// returns a bullet with half the values of the original
    pub fn split(&self) -> Self {
        Bullet {
            damage: self.damage / 2,
            split_probability: 1.,
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
        DespawnAfter::millis(1000),
        RigidBody::Dynamic,
        Mass(bullet.damage as f32),
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
