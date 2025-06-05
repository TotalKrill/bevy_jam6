use crate::gameplay::health::Health;
use crate::gameplay::{
    App, Assets, Commands, Event, Mesh, Mesh3d, MeshMaterial3d, Name,
    ResMut, Sphere, StandardMaterial, Transform, Vec3,
};
use crate::ReplaceOnHotreload;
use avian3d::prelude::{Collider, LinearDamping, LinearVelocity, Mass, RigidBody};
use bevy::color::palettes::basic::BLACK;
use bevy::prelude::{Component, Trigger};

const SEED_RADIUS: f32 = 0.1;

#[derive(Debug, Component)]
pub struct Seed;
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
    app.add_event::<SeedSpawnEvent>().add_observer(spawn_seeds);
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
