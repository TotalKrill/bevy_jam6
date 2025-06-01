//! The screen state for the main gameplay.

use bevy::{input::common_conditions::input_just_pressed, prelude::*, ui::Val::*};

use crate::{Pause, gameplay::tractor, menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    // Toggle pause on key press.
    app.add_systems(
        Update,
        (
            (pause, spawn_pause_overlay, open_pause_menu).run_if(
                in_state(Screen::Gameplay)
                    .and(in_state(Menu::None))
                    .and(input_just_pressed(KeyCode::KeyP).or(input_just_pressed(KeyCode::Escape))),
            ),
            close_menu.run_if(
                in_state(Screen::Gameplay)
                    .and(not(in_state(Menu::None)))
                    .and(input_just_pressed(KeyCode::KeyP)),
            ),
        ),
    );
    app.add_systems(OnExit(Screen::Gameplay), (close_menu, unpause));
    app.add_systems(
        OnEnter(Menu::None),
        unpause.run_if(in_state(Screen::Gameplay)),
    );

    app.add_systems(OnEnter(Screen::Gameplay), spawn_tractor);
}

#[cfg(feature = "dev_native")]
use bevy_simple_subsecond_system::hot;

#[derive(Component)]
pub struct ReplaceOnHotreload;

#[cfg_attr(feature = "dev_native", hot(rerun_on_hot_patch = true))]
fn spawn_tractor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, With<ReplaceOnHotreload>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }

    log::info!("spawning tractor");
    commands.spawn((
        ReplaceOnHotreload,
        tractor::spawn_tractor(&mut meshes, &mut materials),
    ));
}

fn unpause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(false));
}

fn pause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(true));
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Name::new("Pause Overlay"),
        Node {
            width: Percent(100.0),
            height: Percent(100.0),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        StateScoped(Pause(true)),
    ));
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
