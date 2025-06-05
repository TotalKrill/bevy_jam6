use crate::gameplay::health::Health;
use crate::gameplay::tree::TreeSpawnEvent;
use crate::gameplay::{
    App, Assets, Commands, Entity, Event, Mesh, Mesh3d, MeshMaterial3d, Name, Query, Res, ResMut,
    Sphere, StandardMaterial, Time, Timer, TimerMode, Transform, Vec3,
};
use crate::{PausableSystems, ReplaceOnHotreload};
use avian3d::prelude::{Collider, LinearDamping, LinearVelocity, Mass, RigidBody};
use bevy::color::palettes::basic::BLACK;
use bevy::prelude::*;
use std::time::Duration;

const SEED_RADIUS: f32 = 0.1;
const SEED_DESPAWN_TIME_SEC: u64 = 5;
const SEED_SPAWN_TREE_PROBABILITY: f32 = 0.1;

#[derive(Debug, Component)]
pub struct Seed {
    pub timer: Timer,
}

#[derive(Debug, Event)]
pub struct SeedSpawnEvent {
    position: Vec3,
}

impl SeedSpawnEvent {
    pub fn new(position: Vec3) -> Self {
        Self { position }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_event::<SeedSpawnEvent>();
    app.add_observer(spawn_seeds);
    app.add_systems(Update, despawn_seeds.in_set(PausableSystems));
}

fn spawn_seeds(
    trigger: Trigger<SeedSpawnEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let position = trigger.position + Vec3::new(0., 0.1, 0.);

    for vel in [
        Vec3::new(5., 5., 5.),
        Vec3::new(5., 5., -5.),
        Vec3::new(-5., 5., 5.),
        Vec3::new(-5., 5., -5.),
    ]
    .into_iter()
    {
        commands.spawn((
            Name::new("Seed"),
            Health::new(1.0),
            Mass(10.),
            Seed {
                timer: Timer::new(Duration::from_secs(SEED_DESPAWN_TIME_SEC), TimerMode::Once),
            },
            ReplaceOnHotreload,
            Mesh3d(meshes.add(Sphere::new(SEED_RADIUS))),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(BLACK))),
            RigidBody::Dynamic,
            Collider::sphere(SEED_RADIUS),
            Transform::from_translation(position),
            LinearVelocity(vel),
            LinearDamping(2.0),
        ));
    }
}

fn despawn_seeds(
    mut commands: Commands,
    time: Res<Time>,
    mut seeds: Query<(Entity, &Transform, &mut Seed)>,
) {
    for (e, transform, mut seed) in seeds.iter_mut() {
        seed.timer.tick(time.delta());
        if seed.timer.just_finished() {
            if rand::random::<f32>() < SEED_SPAWN_TREE_PROBABILITY {
                commands.send_event(TreeSpawnEvent {
                    position: Vec2::new(transform.translation.x, transform.translation.z),
                });
            }

            commands.entity(e).despawn();
        }
    }
}
