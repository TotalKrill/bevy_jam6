use crate::{gameplay::tractor::Tractor, screens::Screen};

use super::*;
use bevy_enhanced_input::prelude::*;

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
struct Move;

fn tractor_move(mut tractor: Single<&mut Transform, With<Tractor>>, time: Res<Time>) {
    // Apply movement based on input

    tractor.translation += Vec3::new(1.0, 0.0, 0.0) * time.elapsed_secs() * 0.1;
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EnhancedInputPlugin);

    app.add_systems(Update, (tractor_move.run_if(in_state(Screen::Gameplay)),));
}
