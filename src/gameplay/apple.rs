use avian3d::prelude::*;
use bevy::{color::palettes::css::RED, prelude::*};

use crate::screens::*;

#[derive(Component)]
pub struct Apple;

#[derive(Event)]
pub struct AppleSpawnEvent {
    pub position: Vec3,
    pub max_radius: f32,
}

const APPLE_RADIUS: f32 = 1.0;

fn spawn_apple(
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
        commands.spawn((
            Apple,
            Name::new("Apple"),
            ReplaceOnHotreload,
            Mesh3d(meshes.add(Sphere::new(APPLE_RADIUS))),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(RED))),
            RigidBody::Dynamic,
            Collider::sphere(APPLE_RADIUS),
            Transform::from_translation(position),
        ));
    }
}

pub(super) fn plugin(app: &mut App) {
    log::info!("Adding apple plugin");
    app.add_event::<AppleSpawnEvent>();
    app.add_systems(Update, (spawn_apple.run_if(in_state(Screen::Gameplay)),));
}
