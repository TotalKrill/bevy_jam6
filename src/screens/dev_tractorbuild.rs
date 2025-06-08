use super::*;
use crate::gameplay::WorldAssets;
use crate::gameplay::tree::TreeSpawnEvent;
use crate::gameplay::turret_aiming::Sight;
use crate::gameplay::{
    tractor::{self, TractorAssets},
    turret_aiming,
};
use avian3d::math::PI;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy_editor_cam::prelude::*;

pub(super) fn plugin(app: &mut App) {
    if !app.is_plugin_added::<MinimalEditorCamPlugin>() {
        app.add_plugins(DefaultEditorCamPlugins);
    }

    // Toggle pause on key press.
    app.add_systems(OnEnter(Screen::TractorBuild), setup_devscreen);

    // app.add_systems(OnEnter(Screen::TractorBuild), activate_debug_camera);
    // app.add_systems(OnEnter(Screen::TractorBuild), activate_gameplay_camera);

    app.add_systems(
        Update,
        spawn_tree_on_click.run_if(in_state(Screen::TractorBuild)),
    );
}

#[cfg_attr(feature = "dev_native", hot)]
fn spawn_tree_on_click(
    sight: Query<&Transform, With<Sight>>,
    mut tree_w: EventWriter<TreeSpawnEvent>,
    input: Res<ButtonInput<MouseButton>>,
) {
    if input.just_pressed(MouseButton::Middle) {
        for sight in sight.iter() {
            tree_w.write(TreeSpawnEvent {
                position: sight.translation,
                startlevel: 3,
            });
        }
    }
}

use crate::{ReplaceOnHotreload, gameplay::controls::InTractor};
#[cfg_attr(feature = "dev_native", hot(rerun_on_hot_patch = true))]
fn setup_devscreen(
    mut commands: Commands,
    tractor_assets: Res<TractorAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, With<ReplaceOnHotreload>>,
) {
    use bevy_enhanced_input::prelude::Actions;

    for e in query.iter() {
        commands.entity(e).despawn();
    }

    commands.spawn((ReplaceOnHotreload, turret_aiming::sight()));

    hud::spawn_hud(&mut commands);

    log::info!("spawning tractor");
    tractor::spawn_tractor(
        &mut commands,
        &mut meshes,
        &mut materials,
        &tractor_assets,
        (
            StateScoped(Screen::TractorBuild),
            ReplaceOnHotreload,
            Actions::<InTractor>::default(),
        ),
    );

    commands.spawn((
        ReplaceOnHotreload,
        Transform::from_translation(Vec3::Y * 3.),
        sawdust_particles(),
    ));
    // Spawn the Sun
    commands.spawn((
        ReplaceOnHotreload,
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 10.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // CascadeShadowConfigBuilder {
        //     first_cascade_far_bound: 4.0,
        //     maximum_distance: 10.0,
        //     ..default()
        // }.build(),
    ));

    /// ui
    use crate::gameplay::{
        hud::{self, *},
        saw::sawdust_particles,
        sun,
    };
    spawn_hud(&mut commands);

    // commands.spawn((
    //     StateScoped(Screen::TractorBuild),
    //     level::level(world_assets, meshes, materials),
    // ));

    // Spawn the Sun
    commands.spawn(sun());

    // commands.spawn(PerfUiAllEntries::default());
}
