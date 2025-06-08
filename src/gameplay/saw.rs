use crate::{
    audio::sound_effect,
    gameplay::{health::DamageEvent, tractor::TractorSaw},
    screens::Screen,
};

use super::*;

#[derive(Component, Debug)]
pub struct Sawable {
    timer: Timer,
}

impl Default for Sawable {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.0, TimerMode::Once),
        }
    }
}

#[derive(Resource)]
struct SawAssets {
    damage_sound: Handle<AudioSource>,
}

impl FromWorld for SawAssets {
    fn from_world(world: &mut World) -> Self {
        let assets: &AssetServer = world.resource::<AssetServer>();
        Self {
            damage_sound: assets.load::<AudioSource>("audio/sound_effects/chainsaw.wav"),
        }
    }
}

fn check_saw_colitions(
    collisions: Collisions,
    sawables: Query<(Entity, &mut Sawable)>,
    saw: Single<(Entity, &TractorSaw, &GlobalTransform)>,
    mut commands: Commands,
    assets: Res<SawAssets>,
) {
    let (saw_entity, saw, saw_gt) = saw.into_inner();

    for (sawable_entity, mut sawable) in sawables {
        if collisions.contains(sawable_entity, saw_entity) && sawable.timer.finished() {
            // Object is currently beeing damaged by the saw
            commands.send_event(DamageEvent {
                value: saw.damage,
                entity: sawable_entity,
            });
            // Update rate of fire
            sawable.timer.set_duration(saw.rate_of_fire);
            sawable.timer.reset();

            commands.spawn(sound_effect(assets.damage_sound.clone()));
            commands.spawn((
                sawdust_particles(),
                Transform::from_translation(saw_gt.translation()),
            ));
        }
    }
}

pub fn sawdust_particles() -> impl Bundle {
    use bevy_firework::{
        bevy_utilitarian::prelude::{RandF32, RandValue, RandVec3},
        core::{BlendMode, ParticleSpawner},
        curve::{FireworkCurve, FireworkGradient},
        emission_shape::EmissionShape,
    };
    (
        Name::new("SawDustParticleEffect"),
        DespawnAfter::millis(750),
        ParticleSpawner {
            one_shot: true,
            rate: 500.0,
            emission_shape: EmissionShape::Circle {
                normal: Vec3::Y,
                radius: 0.85,
            },
            lifetime: RandF32::constant(0.75),
            inherit_parent_velocity: true,
            initial_velocity: RandVec3 {
                magnitude: RandF32 { min: 0., max: 3.6 },
                direction: Vec3::Y,
                spread: 180. / 180. * std::f32::consts::PI,
            },
            initial_scale: RandF32 {
                min: -0.5,
                max: 0.2,
            },
            scale_curve: FireworkCurve::constant(1.),
            color: FireworkGradient::uneven_samples(vec![
                (0., LinearRgba::new(1.0, 0.6, 0.2, 1.0)),
                (1., LinearRgba::new(0.1, 0.1, 0.1, 0.)),
            ]),
            acceleration: Vec3::new(0., -3., 0.),
            blend_mode: BlendMode::Blend,
            linear_drag: 5.0,
            pbr: false,
            spawn_transform_mode: bevy_firework::core::SpawnTransformMode::Local,
            ..default()
        },
    )
}

fn check_sawable_timers(mut sawables: Query<&mut Sawable>, time: Res<Time>) {
    for mut sawable in sawables.iter_mut() {
        sawable.timer.tick(time.delta());
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<SawAssets>();
    app.add_systems(
        Update,
        (
            check_saw_colitions.run_if(in_state(Screen::InGame)),
            check_sawable_timers.run_if(in_state(Screen::InGame)),
        ),
    );
}
