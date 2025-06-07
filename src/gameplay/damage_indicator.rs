use std::time::Duration;

use crate::gameplay::health::DamageEvent;
use bevy::prelude::*;
use bevy_tweening::{Animator, Tween, lens::TransformPositionLens};
use bevy_ui_anchor::*;

use super::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, spawn_damage_indicators_on_event);
}

#[derive(Component)]
pub struct DamageIndicatorBase;

#[cfg_attr(feature = "dev_native", hot)]
fn spawn_damage_indicators_on_event(
    mut commands: Commands,
    transforms: Query<&GlobalTransform>,
    mut damage_reader: EventReader<DamageEvent>,
) {
    const DUR: u64 = 500;
    for damage in damage_reader.read() {
        if let Ok(position) = transforms.get(damage.entity) {
            let dir: Vec3 = rand::random();
            let len = 3.0;
            let dir = dir.normalize() * len;
            let tween = Tween::new(
                // Use a quadratic easing on both endpoints.
                EaseFunction::Linear,
                // Animation time (one way only; for ping-pong it takes 2 seconds
                // to come back to start).
                Duration::from_millis(DUR),
                // The lens gives the Animator access to the Transform component,
                // to animate it. It also contains the start and end values associated
                // with the animation ratios 0. and 1.
                TransformPositionLens {
                    start: position.translation(),
                    end: position.translation() + dir,
                },
            );

            commands.spawn((
                Name::new("DamageIndicator"),
                DamageIndicatorBase,
                Animator::new(tween),
                DespawnAfter::millis(DUR),
                //
                // Mesh3d(meshes.add(Sphere::new(1.0))),
                // MeshMaterial3d(materials.add(StandardMaterial::from_color(RED))),
                Transform::from_translation(position.translation()),
                Visibility::Visible,
                AnchoredUiNodes::spawn_one((
                    DespawnAfter::millis(DUR),
                    Name::new("DamageIndicatorUI"),
                    Text::new(format!("{}", damage.value)),
                    AnchorUiConfig::default(),
                )),
            ));
        }
    }
}
