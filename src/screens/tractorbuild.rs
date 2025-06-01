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
        // MovementController,
    ));

    commands.spawn((
        ReplaceOnHotreload,
        level::level(&mut meshes, &mut materials),
    ));
}
