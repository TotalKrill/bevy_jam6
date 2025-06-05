use core::fmt;

use bevy::color::palettes::tailwind::*;

use crate::{
    ReplaceOnHotreload,
    gameplay::{apple::Apple, health::Health, score::ScoreCounter, tractor::Tractor, tree::Tree},
};

use super::*;

#[derive(Component)]
pub struct PointCounter;

#[derive(Component)]
pub struct AppleCounter;

#[derive(Component)]
pub struct TreeCounter;

#[derive(Component)]
struct Healthbar;

pub fn hud_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_points,
            update_healthbar,
            update_apple_counter,
            update_tree_counter,
        ),
    );
}

fn update_tree_counter(
    apples: Query<&Tree>,
    mut counter: Single<&mut TextSpan, With<TreeCounter>>,
) {
    counter.0 = format!("{}", apples.iter().count());
}

fn update_apple_counter(
    apples: Query<&Apple>,
    mut counter: Single<&mut TextSpan, With<AppleCounter>>,
) {
    counter.0 = format!("{}", apples.iter().count());
}

fn update_points(
    score: Res<ScoreCounter>,
    mut hud_score: Single<&mut TextSpan, With<PointCounter>>,
) {
    hud_score.0 = format!("{}", score.points);
}

fn update_healthbar(
    tractor: Query<&Health, With<Tractor>>,
    mut healthbar: Single<&mut Node, With<Healthbar>>,
) {
    if let Ok(tractor) = tractor.single() {
        healthbar.width = Val::Percent(tractor.percentage());
    } else {
        healthbar.width = Val::Percent(0.);
    }
}

pub fn spawn_hud(commands: &mut Commands) {
    commands.spawn(stat_tracker());
    commands.spawn(healthbar());
}

fn stat_tracker() -> impl Bundle {
    (
        ReplaceOnHotreload,
        Node {
            right: Val::Percent(5.),
            top: Val::Percent(3.0),
            border: UiRect::all(Val::Px(4.)),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        BorderRadius::all(Val::Px(4.)),
        Outline::new(Val::Px(2.), Val::Px(3.), WHITE.into()),
        Children::spawn((
            Spawn(value_counter("Points", PointCounter)),
            Spawn(value_counter("Apples Alive", AppleCounter)),
            Spawn(value_counter("Trees Alive", TreeCounter)),
        )),
    )
}

fn value_counter(key: impl fmt::Display, marker: impl Component) -> impl Bundle {
    (
        ReplaceOnHotreload,
        Name::new("points"),
        Text::new(format!("{key}: ")),
        Children::spawn((Spawn((TextSpan::new("--"), marker)),)),
    )
}

fn healthbar() -> impl Bundle {
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
