pub use bevy::{color::palettes::css::*, prelude::*};
//all the gameplay stuff

pub mod tractor;

pub mod level {
    use super::*;
    pub fn level() -> impl Bundle {}
}

pub mod turrent {
    use super::*;
}

pub mod movementcontrols;

pub(super) fn plugin(app: &mut App) {
    println!("Adding gameplay plugin");
    app.add_plugins(movementcontrols::plugin);
}
