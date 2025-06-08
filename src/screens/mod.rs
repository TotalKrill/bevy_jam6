//! The game's main screen states and transitions between them.
#[cfg(feature = "dev_native")]
use bevy_simple_subsecond_system::hot;

pub(crate) mod ingame;
mod loading;
mod music;
mod splash;
mod title;

// #[cfg(feature = "dev_native")]
mod dev_tractorbuild;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((
        ingame::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
        music::plugin,
    ));
    #[cfg(feature = "dev")]
    app.add_plugins(dev_tractorbuild::plugin);
}

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Loading,
    InGame,
    TractorBuild,
}
