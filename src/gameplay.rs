use std::time::Duration;

use crate::{asset_tracking::LoadResource, menus::Menu};
use avian3d::prelude::*;
use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
pub use bevy::{color::palettes::css::*, prelude::*};
//all the gameplay stuff

/// Event that is triggered when the game is over!
#[derive(Event)]
pub struct GameOver;

#[derive(Component)]
struct DespawnAfter(pub(super) Timer);

impl DespawnAfter {
    pub fn millis(millis: u64) -> Self {
        Self(Timer::new(Duration::from_millis(millis), TimerMode::Once))
    }
}

pub mod apple;
pub mod bullet;
pub mod controls;
pub mod health;
pub mod level;
pub mod saw;
pub mod score;
pub mod tractor;
pub mod tree;
pub mod turret;
pub mod turret_aiming;

pub mod healthbars;
/// contains the heads up display during game;
pub mod hud;
mod seed;
mod upgrades;

pub mod damage_indicator {
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
}

#[cfg(feature = "dev_native")]
use bevy_simple_subsecond_system::hot;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct WorldAssets {
    pub ground: Handle<Image>,
}

impl FromWorld for WorldAssets {
    fn from_world(world: &mut World) -> Self {
        let assets: &AssetServer = world.resource::<AssetServer>();
        let path = "models/map/Grass_04-512x512.png";
        Self {
            // ground: assets.load(path)
            ground: assets.load_with_settings(path, |s: &mut _| {
                *s = ImageLoaderSettings {
                    sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                        // rewriting mode to repeat image,
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        ..default()
                    }),
                    ..default()
                }
            }),
        }
    }
}

fn to_gameover(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::GameOver);
}

pub(super) fn plugin(app: &mut App) {
    log::info!("Adding gameplay plugins");

    app.load_resource::<WorldAssets>();

    app.add_event::<GameOver>();

    app.add_systems(Update, to_gameover.run_if(on_event::<GameOver>));

    app.add_systems(
        Update,
        |mut commands: Commands,
         time: Res<Time>,
         mut timers: Query<(Entity, &mut DespawnAfter)>| {
            for (e, mut timer) in timers.iter_mut() {
                timer.0.tick(time.delta());
                if timer.0.just_finished() {
                    commands.entity(e).despawn();
                }
            }
        },
    );

    app.add_plugins(controls::plugin);
    app.add_plugins(hud::hud_plugin);
    app.add_plugins(tractor::tractor_plugin);
    app.add_plugins(bullet::bullet_plugin);
    app.add_plugins(seed::plugin);
    app.add_plugins(turret_aiming::plugin);
    app.add_plugins(turret::turret_plugin);
    app.add_plugins(apple::plugin);
    app.add_plugins(health::plugin);
    app.add_plugins(tree::plugin);
    app.add_plugins(score::plugin);
    app.add_plugins(damage_indicator::plugin);
    app.add_plugins(saw::plugin);
    app.add_plugins(healthbars::plugin);
    app.add_plugins(upgrades::plugin);
}
