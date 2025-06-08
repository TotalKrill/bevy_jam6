use crate::{
    PausableSystems, ReplaceOnHotreload,
    gameplay::{health::Health, level::Ground, tree::TreeSpawnEvent},
};

use super::*;
use avian3d::prelude::{Collider, LinearDamping, LinearVelocity, Mass, RigidBody};
use bevy::color::palettes::basic::BLACK;
use std::time::Duration;

const SEED_RADIUS: f32 = 0.1;
const SEED_DESPAWN_TIME_SEC: u64 = 5;
const SEED_SPAWN_TREE_PROBABILITY: f32 = 0.1;

#[derive(Resource)]
pub struct SeedAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl FromWorld for SeedAssets {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh = meshes.add(Sphere::new(SEED_RADIUS));

        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        let mut material = StandardMaterial::from_color(BLACK);
        material.emissive = LinearRgba::rgb(0.1, 0.1, 100.);

        let material = materials.add(material);

        Self { mesh, material }
    }
}

#[derive(Debug, Component)]
pub struct Seed;

#[derive(Debug, Event)]
pub struct SeedSpawnEvent {
    pub position: Vec3,
    pub velocity: Vec3,
}

pub(super) fn plugin(app: &mut App) {
    app.add_event::<SeedSpawnEvent>();
    app.init_resource::<SeedAssets>();

    app.add_systems(Update, (spawn_seed, plant_seed).in_set(PausableSystems));
}

fn spawn_seed(
    mut events: EventReader<SeedSpawnEvent>,
    mut commands: Commands,
    seedasset: Res<SeedAssets>,
) {
    for event in events.read() {
        let position = event.position + Vec3::new(0., 0.1, 0.);

        let up = Vec3::Y * event.velocity.length();
        let angle = rand::random::<f32>() * 90.0 - 45.0;
        let velocity =
            (Quat::from_rotation_y(angle.to_radians()).mul_vec3(event.velocity) + up) / 2.0;

        commands.spawn((
            Name::new("Seed"),
            Health::new(1),
            Mass(0.1),
            CollisionEventsEnabled,
            Seed,
            ReplaceOnHotreload,
            Mesh3d(seedasset.mesh.clone()),
            MeshMaterial3d(seedasset.material.clone()),
            RigidBody::Dynamic,
            Collider::sphere(SEED_RADIUS),
            Transform::from_translation(position),
            LinearVelocity(velocity),
            LinearDamping(2.0),
        ));
    }
}

fn plant_seed(
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionStarted>,
    ground: Single<Entity, With<Ground>>,
    seeds: Query<(Entity, &Transform), With<Seed>>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        for (ground_candidate, seed_candidate) in [(*entity1, *entity2), (*entity2, *entity1)] {
            if ground_candidate == *ground {
                if let Ok((seed, transform)) = seeds.get(seed_candidate) {
                    commands.send_event(TreeSpawnEvent {
                        position: Vec3::new(
                            transform.translation.x,
                            1000.,
                            transform.translation.z,
                        ),
                        startlevel: 0,
                        static_tree: false,
                    });

                    if let Ok(mut ec) = commands.get_entity(seed) {
                        ec.despawn();
                    }
                }
            }
        }
    }
}
