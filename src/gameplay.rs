use avian3d::prelude::*;
pub use bevy::{color::palettes::css::*, prelude::*};

use crate::{
    asset_tracking::LoadResource,
    gameplay::{health::Death, tractor::Tractor},
    menus::Menu,
};
//all the gameplay stuff

/// Event that is triggered when the game is over!
#[derive(Event)]
pub struct GameOver;

pub mod apple;
pub mod bullet;
pub mod controls;
pub mod health;
pub mod score;
pub mod tractor;
pub mod tree;
pub mod turret;
pub mod turret_aiming;

/// contains the heads up display during game;
pub mod hud;
mod seed;

#[cfg(feature = "dev_native")]
use bevy_simple_subsecond_system::hot;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct WorldAssets {
    pub ground: Handle<Scene>,
}

impl FromWorld for WorldAssets {
    fn from_world(world: &mut World) -> Self {
        let assets: &AssetServer = world.resource::<AssetServer>();
        Self {
            ground: assets.load(GltfAssetLabel::Scene(0).from_asset("models/map/map.glb")),
        }
    }
}

pub mod level {
    #[derive(Component)]
    pub struct Ground;

    use super::*;
    pub fn level(assets: &WorldAssets) -> impl Bundle {
        (
            RigidBody::Static,
            Friction::new(1.0),
            // Transform::from_translation(Vec3::new(0.0, -1., 0.0)),
            ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
            Ground,
            SceneRoot(assets.ground.clone()),
        )
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
}
