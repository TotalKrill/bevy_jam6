use std::{f32::consts::PI, time::Duration};

use crate::{
    ReplaceOnHotreload, asset_tracking::LoadResource, gameplay::level::LevelAssets, menus::Menu,
};
use avian3d::prelude::*;
pub use bevy::{color::palettes::css::*, prelude::*};
use bevy::{
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    pbr::CascadeShadowConfigBuilder,
};
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

pub mod damage_indicator;

#[cfg(feature = "dev_native")]
use bevy_simple_subsecond_system::hot;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct WorldAssets {
    pub texture: Handle<Image>,
}

impl FromWorld for WorldAssets {
    fn from_world(world: &mut World) -> Self {
        let assets: &AssetServer = world.resource::<AssetServer>();
        let path = "models/map/Grass_04-512x512_gray.png";
        Self {
            // ground: assets.load(path)
            texture: assets.load_with_settings(path, |s: &mut _| {
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

pub fn sun() -> impl Bundle {
    (
        Name::new("sun"),
        ReplaceOnHotreload,
        DirectionalLight {
            illuminance: 7000.,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .build(),
    )
}

pub(super) fn plugin(app: &mut App) {
    log::info!("Adding gameplay plugins");

    app.register_type::<WorldAssets>();
    app.init_resource::<WorldAssets>();

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
    app.add_plugins(level::plugin);
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
}
