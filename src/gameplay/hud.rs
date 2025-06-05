use crate::screens::ReplaceOnHotreload;

use super::*;

#[derive(Component)]
pub struct PointCounter;

pub fn hud_plugin(app: &mut App) {
    app.add_systems(Update, (update_points, update_healthbar));
}

pub fn update_points() {}
pub fn update_healthbar() {}

#[cfg_attr(feature = "dev_native", hot(rerun_on_hot_patch))]
pub fn setup_hud(mut commands: Commands) {
    commands.spawn((
        ReplaceOnHotreload,
        PointCounter,
        TextLayout::default(),
        Node {
            right: Val::Px(5.),
            top: Val::Px(5.0),
            width: Val::Px(200.),
            height: Val::Px(40.),
            border: UiRect::all(Val::Px(4.)),
            ..Default::default()
        },
        BorderRadius::all(Val::Px(4.)),
        Children::spawn((
            Spawn(TextSpan::new("Points: ")),
            Spawn(TextSpan::new("100")),
        )),
    ));
}
