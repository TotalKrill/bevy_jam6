use super::*;
use bevy_enhanced_input::prelude::*;

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
struct Move;

fn tractor_move(mut tractor: Single<&mut Transform, &Tractor>, time: Res<Time>) {}
