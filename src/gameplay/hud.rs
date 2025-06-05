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

fn update_tree_counter(tree: Query<&Tree>, mut counter: Single<&mut Text, With<TreeCounter>>) {
    counter.0 = format!("{}", tree.iter().count());
}

fn update_apple_counter(apples: Query<&Apple>, mut counter: Single<&mut Text, With<AppleCounter>>) {
    counter.0 = format!("{}", apples.iter().count());
}

fn update_points(score: Res<ScoreCounter>, mut hud_score: Single<&mut Text, With<PointCounter>>) {
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
            padding: UiRect::all(Val::Px(4.)),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        BackgroundColor(WHITE_SMOKE.with_alpha(0.1).into()),
        BorderRadius::all(Val::Px(4.)),
        Outline::new(Val::Px(2.), Val::Px(0.), WHITE.into()),
        Children::spawn((
            Spawn(value_counter("Points", 32., PointCounter)),
            Spawn((
                Node {
                    width: Val::Auto,
                    ..Default::default()
                },
                Outline::new(Val::Px(1.), Val::Px(0.0), WHITE.into()),
            )),
            Spawn(value_counter("Apples Alive", 18., AppleCounter)),
            Spawn(value_counter("Trees Alive", 18., TreeCounter)),
        )),
    )
}

fn value_counter(key: impl fmt::Display, size: f32, marker: impl Component) -> impl Bundle {
    (
        ReplaceOnHotreload,
        Name::new("value counter"),
        Node {
            // display: Display::Grid,
            row_gap: Val::Px(10.0),
            flex_direction: FlexDirection::Row,
            // justify_content: JustifyContent::Stretch,
            align_content: AlignContent::Stretch,
            ..Default::default()
        },
        children![
            (
                Name::new("points"),
                TextFont::from_font_size(size),
                Node {
                    justify_self: JustifySelf::Start,
                    ..default()
                },
                Text::new(format!("{key}: "))
            ),
            (
                Name::new("value"),
                TextFont::from_font_size(size),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                },
                Text::new(format!("--")),
                marker
            )
        ],
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
