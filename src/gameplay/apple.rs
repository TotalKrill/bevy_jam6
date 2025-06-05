use crate::gameplay::health::{Death, Health};
use crate::{
    ReplaceOnHotreload,
    gameplay::{tractor::Tractor, tree::Tree},
    screens::*,
};
use avian3d::prelude::*;
use bevy::{color::palettes::css::RED, prelude::*};

#[derive(Component)]
pub struct Apple;

#[derive(Event)]
pub struct AppleSpawnEvent {
    pub position: Vec3,
    pub max_radius: f32,
    pub radius: f32,
}

const APPLE_RADIUS: f32 = 1.0;
const APPLE_SPAWN_RADIUS: f32 = 10.0;
const APPLE_FORCE_SCALAR: f32 = 10.0; // Rate at which the apple want to get to the player

fn spawn_apple_event_handler(
    mut events: EventReader<AppleSpawnEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in events.read() {
        let radii = rand::random::<f32>() * event.max_radius;
        let angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;

        let position = event.position
            + Vec3::new(radii * angle.cos(), APPLE_RADIUS * 2.0, radii * angle.sin());
        commands
            .spawn((
                Apple,
                Name::new("Apple"),
                Health::new(1.0),
                ReplaceOnHotreload,
                Mesh3d(meshes.add(Sphere::new(event.radius))),
                MeshMaterial3d(materials.add(StandardMaterial::from_color(RED))),
                RigidBody::Dynamic,
                Collider::sphere(APPLE_RADIUS),
                Transform::from_translation(position),
            ))
            .observe(|trigger: Trigger<Death>, mut commands: Commands| {
                commands
                    .get_entity(trigger.target().entity())
                    .unwrap()
                    .despawn();
            });
    }
}

fn apply_apple_force(
    mut query: Query<(&mut ExternalForce, &Transform), With<Apple>>,
    tractor: Single<&Transform, With<Tractor>>,
) {
    for (mut apple_force, apple_transform) in query.iter_mut() {
        let force =
            (tractor.translation - apple_transform.translation).normalize() * APPLE_FORCE_SCALAR;

        apple_force.set_force(force);
    }
}
fn spawn_apples(
    mut commands: Commands,
    mut query: Query<(&Transform, &mut Tree)>,
    time: Res<Time>,
) {
    let elapsed_time = time.elapsed_secs();
    for (&transform, mut tree) in query.iter_mut() {
        if elapsed_time > (tree.last_apple_spawn + tree.apple_spawn_time_sec) {
            tree.last_apple_spawn = elapsed_time;
            commands.send_event(AppleSpawnEvent {
                position: transform.translation,
                max_radius: APPLE_SPAWN_RADIUS,
                radius: APPLE_RADIUS,
            });
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    log::info!("Adding apple plugin");
    app.add_event::<AppleSpawnEvent>();
    app.add_systems(
        Update,
        (
            spawn_apple_event_handler.run_if(in_state(Screen::Gameplay)),
            spawn_apples.run_if(in_state(Screen::Gameplay)),
            apply_apple_force.run_if(in_state(Screen::Gameplay)),
        ),
    );
}
