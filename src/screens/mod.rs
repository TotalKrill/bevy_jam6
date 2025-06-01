//! The game's main screen states and transitions between them.
#[cfg(feature = "dev_native")]
use bevy_simple_subsecond_system::hot;

#[derive(Component)]
pub struct ReplaceOnHotreload;

mod gameplay;
mod loading;
mod splash;
mod title;

#[cfg(feature = "dev_native")]
mod tractorbuild {

    use super::*;

    pub(super) fn plugin(app: &mut App) {
        // Toggle pause on key press.
        app.add_systems(OnEnter(Screen::TractorBuild), spawn_tractor);
    }

    #[derive(Component)]
    pub struct ReplaceOnHotreload;

    #[cfg_attr(feature = "dev_native", hot(rerun_on_hot_patch = true))]
    fn spawn_tractor(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        query: Query<Entity, With<ReplaceOnHotreload>>,
    ) {
        use crate::gameplay::{level, tractor};

        for e in query.iter() {
            commands.entity(e).despawn();
        }

        log::info!("spawning tractor");
        commands.spawn((
            ReplaceOnHotreload,
            Transform::from_xyz(0.0, tractor::TRACTOR_HEIGHT * 2., 0.0),
            tractor::spawn_tractor(&mut meshes, &mut materials),
        ));

        commands.spawn((
            ReplaceOnHotreload,
            level::level(&mut meshes, &mut materials),
        ));
    }
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
    ));
    #[cfg(feature = "dev_native")]
    app.add_plugins(tractorbuild::plugin);
}

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Screen {
    Splash,
    Title,
    Loading,
    Gameplay,
    #[default]
    TractorBuild,
}
