//! The main menu (seen on the title screen).

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{
    asset_tracking::ResourceHandles,
    gameplay::{
        WorldAssets,
        level::{self, Ground, LevelAssets},
        sun,
    },
    menus::Menu,
    screens::Screen,
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
    app.add_systems(OnEnter(Menu::Main), setup_level);
}

fn banner(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Splash image"),
        Node {
            width: Val::Percent(50.0),
            ..default()
        },
        BackgroundColor::DEFAULT,
        ImageNode::new(asset_server.load("images/banner.png")),
    )
}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((sun(), StateScoped(Menu::Main)));

    #[cfg(feature = "dev")]
    commands.spawn((
        widget::ui_root("Main Menu"),
        BackgroundColor::DEFAULT,
        GlobalZIndex(2),
        StateScoped(Menu::Main),
        #[cfg(not(target_family = "wasm"))]
        children![
            banner(&asset_server),
            widget::button("Play", enter_loading_or_gameplay_screen),
            widget::button("Dev", enter_loading_or_dev_screen),
            widget::button("Leaderboard", open_leaderboard_screen),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
            widget::button("Exit", exit_app),
        ],
        #[cfg(target_family = "wasm")]
        children![
            banner(&asset_server),
            widget::button("Play", enter_loading_or_gameplay_screen),
            widget::button("Dev", enter_loading_or_dev_screen),
            widget::button("Leaderboard", open_leaderboard_screen),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
        ],
    ));
    #[cfg(not(feature = "dev"))]
    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Main),
        #[cfg(not(target_family = "wasm"))]
        children![
            banner(&asset_server),
            widget::button("Play", enter_loading_or_gameplay_screen),
            widget::button("Leaderboard", open_leaderboard_screen),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
            widget::button("Exit", exit_app),
        ],
        #[cfg(target_family = "wasm")]
        children![
            banner(&asset_server),
            widget::button("Play", enter_loading_or_gameplay_screen),
            widget::button("Leaderboard", open_leaderboard_screen),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
        ],
    ));
}

#[cfg(feature = "dev_native")]
use bevy_simple_subsecond_system::hot;

#[cfg_attr(feature = "dev_native", hot(rerun_on_hot_patch))]
fn setup_level(
    mut commands: Commands,
    world_assets: Res<WorldAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    level_assets: Res<LevelAssets>,
    query: Query<Entity, With<Ground>>, // use this to make sure there isnt already a ground
) {
    if query.is_empty() {
        level::level(
            &mut commands,
            world_assets,
            &mut meshes,
            &mut materials,
            &level_assets,
        );
    }
}

fn enter_loading_or_gameplay_screen(
    _: Trigger<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(Screen::InGame);
    } else {
        next_screen.set(Screen::Loading);
    }
}

#[cfg(feature = "dev")]
fn enter_loading_or_dev_screen(
    _: Trigger<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(Screen::TractorBuild);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn open_settings_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}
fn open_leaderboard_screen(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Leaderboard);
}

fn open_credits_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
