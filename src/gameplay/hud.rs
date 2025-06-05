use bevy::color::palettes::tailwind::*;

use crate::{
    ReplaceOnHotreload,
    gameplay::{health::Health, tractor::Tractor},
};

use super::*;

#[derive(Component)]
pub struct PointCounter;

#[derive(Component)]
pub struct Healthbar;

pub fn hud_plugin(app: &mut App) {
    app.add_systems(Update, (update_points, update_healthbar));
}

pub fn update_points() {}
pub fn update_healthbar(
    tractor: Single<&Health, With<Tractor>>,
    mut healthbar: Single<&mut Node, With<Healthbar>>,
) {
    healthbar.width = Val::Percent(tractor.percentage());
}

pub fn points_node() -> impl Bundle {
    (
        ReplaceOnHotreload,
        PointCounter,
        Name::new("points"),
        Text::new(""),
        Node {
            left: Val::Px(5.),
            top: Val::Px(5.0),
            width: Val::Px(200.),
            height: Val::Px(40.),
            border: UiRect::all(Val::Px(4.)),
            ..Default::default()
        },
        BorderRadius::all(Val::Px(4.)),
        BorderColor(WHITE_SMOKE.into()),
        Children::spawn((
            Spawn(TextSpan::new("Points: ")),
            Spawn((TextSpan::new("100"), PointCounter)),
        )),
    )
}

pub fn healthbar() -> impl Bundle {
    (
        ReplaceOnHotreload,
        Name::new("healthbar"),
        Node {
            bottom: Val::Percent(3.0),
            justify_self: JustifySelf::Center,
            width: Val::Percent(70.),
            height: Val::Percent(2.),
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        BoxShadow::new(
            BLACK.into(),
            Val::Px(4.),
            Val::Px(4.0),
            Val::Px(4.0),
            Val::Px(4.),
        ),
        BorderRadius::all(Val::Px(4.)),
        Outline::new(Val::Px(3.), Val::Px(0.), WHITE_SMOKE.into()),
        BorderColor(WHITE_SMOKE.into()),
        BackgroundColor(RED.into()),
        Children::spawn((Spawn((
            Healthbar,
            BackgroundColor(GREEN_600.into()),
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..Default::default()
            },
        )),)),
    )
}
