use crate::{
    PausableSystems, ReplaceOnHotreload,
    gameplay::{
        apple::Apple,
        health::Health,
        score::{Currency, ScoreCounter},
        tractor::{Tractor, TractorSaw},
        tree::Tree,
        turret::TurretDamage,
    },
    theme::widget,
};
use bevy::{color::palettes::tailwind::*, ecs::system::IntoObserverSystem, input::keyboard};
use bevy_tweening::*;
use bevy_tweening::{Animator, Tween, lens::TransformPositionLens};
use core::fmt;
use bevy::window::Ime::Disabled;
use super::*;
use crate::theme::palette::{BUTTON_TEXT, LABEL_TEXT};
use Val::*;

const HUD_WIDTH_ELEMENT: f32 = 82.0;

#[derive(Component, Default)]
pub struct PointCounter;

#[derive(Component, Default)]
pub struct AppleCounter;

#[derive(Component, Default)]
pub struct TreeCounter;

#[derive(Component, Default)]
struct Healthbar;

#[derive(Event, Default)]
struct TurretUpdateEvent;
#[derive(Event, Default)]
struct SawUpdateEvent;

pub fn hud_plugin(app: &mut App) {
    app.add_event::<SawUpdateEvent>();
    app.add_event::<TurretUpdateEvent>();

    app.add_systems(
        Update,
        (
            update_points,
            update_healthbar,
            update_apple_counter,
            update_tree_counter,
            update_upgrade_counter,
        ),
    );
    app.add_systems(Update, (keybind_updates).in_set(PausableSystems));

    app.add_systems(
        Update,
        (
            upgrade_saw.run_if(on_event::<SawUpdateEvent>),
            upgrade_turret.run_if(on_event::<TurretUpdateEvent>),
            toggle_upgrade_indicators
        )
            .in_set(PausableSystems),
    );
}

fn keybind_updates(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut turrw: EventWriter<TurretUpdateEvent>,
    mut saww: EventWriter<SawUpdateEvent>,
) {
    if keyboard.just_pressed(KeyCode::Digit1) {
        turrw.write(TurretUpdateEvent);
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        saww.write(SawUpdateEvent);
    }
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
        healthbar.width = Val::Percent(tractor.percentage() as f32);
    } else {
        healthbar.width = Val::Percent(0.);
    }
}

pub fn spawn_hud(commands: &mut Commands) {
    commands.spawn((
        Node {
            left: Val::Percent(5.),
            top: Val::Percent(3.0),
            width: Px(HUD_WIDTH_ELEMENT * 2.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        children![
            stat_tracker(),
            (Node {
                height: Px(20.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },),
            upgrade_tracker()
        ]
    ));
    // commands.spawn(stat_tracker());
    // commands.spawn(upgrade_tracker());
    commands.spawn(healthbar());
    commands.spawn(update_hud());
}

#[derive(Component, Default)]
struct TurretUpdateCounter;
#[derive(Component, Default)]
struct TurretUpdateIndicator;

#[derive(Component, Default)]
struct SawUpdateCounter;

fn update_hud() -> impl Bundle {
    (
        ReplaceOnHotreload,
        Node {
            left: Val::Percent(15.),
            bottom: Val::Percent(6.5),
            padding: UiRect::all(Val::Px(4.)),
            column_gap: Val::Px(10.),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Row,
            ..Default::default()
        },
        children![
            // upgrades_counter(),
            update_button::<TurretUpdateEvent, TurretUpdateCounter>("Turret", "Press 1"),
            update_button::<SawUpdateEvent, SawUpdateCounter>("Saw", "Press 2"),
            // update_button::<TurretUpdateEvent, TurretUpdateCounter>("Turret lvl 1", "Press 1"),
            // update_button::<SawUpdateEvent, SawUpdateCounter>("Saw lvl 1", "Press 2"),
        ],
    )
}

#[derive(Component)]
struct UpgradeCounter;

fn update_upgrade_counter(
    currency: Res<Currency>,
    mut upg_counts: Query<(&mut Text, &mut TextColor), With<UpgradeCounter>>,
) {
    for (mut upg_count, mut upg_color) in upg_counts.iter_mut() {
        if currency.get() > 0 {
            *upg_count = Text::new(format!("Upgrades: {}", currency.get()));
            *upg_color = TextColor(LABEL_TEXT);
        } else {
            *upg_count = Text::new(format!("Upgrades: {}", currency.get()));
            *upg_color = TextColor(BUTTON_TEXT);
        }
    }
}

fn upgrade_turret(
    mut upd_counters: Query<&mut Text, With<TurretUpdateCounter>>,
    mut turrets: Query<&mut TurretDamage>,
    mut currency: ResMut<Currency>,
) {
    if currency.spend(1) {
        for mut turret in turrets.iter_mut() {
            turret.0 += 1;
            for mut upd_counter in upd_counters.iter_mut() {
                *upd_counter = Text::new(format!("{}", turret.0));
            }
        }
    }
}
fn toggle_upgrade_indicators(
    mut nodes: Query<&mut Visibility, With<TurretUpdateIndicator,>>,
    currency: Res<Currency>,
) {
    for mut node in nodes.iter_mut() {
        if currency.get() > 0 {
            *node = Visibility::Visible;
        } else {
            *node = Visibility::Hidden;
        }
    }

    // if currency.spend(1) {
    //     for mut turret in turrets.iter_mut() {
    //         turret.0 += 1;
    //         for mut upd_counter in upd_counters.iter_mut() {
    //             *upd_counter = Text::new(format!("{}", turret.0));
    //         }
    //     }
    // }
}

fn upgrade_saw(
    mut upd_counters: Query<&mut Text, With<SawUpdateCounter>>,
    mut saws: Query<&mut TractorSaw>,
    mut currency: ResMut<Currency>,
) {
    if currency.spend(1) {
        for mut saw in saws.iter_mut() {
            saw.damage += 1;
            for mut upd_counter in upd_counters.iter_mut() {
                *upd_counter = Text::new(format!("{}", saw.damage));
            }
        }
    }
}

fn stat_tracker() -> impl Bundle {
    (
        ReplaceOnHotreload,
        Node {
            width: Px(HUD_WIDTH_ELEMENT * 2.),
            padding: UiRect::all(Val::Px(4.)),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        BackgroundColor(WHITE_SMOKE.with_alpha(0.1).into()),
        BorderRadius::all(Val::Px(4.)),
        Outline::new(Val::Px(2.), Val::Px(0.), WHITE.into()),
        Children::spawn((
            Spawn(value_counter("Points", 30., PointCounter)),
            Spawn((
                Node {
                    width: Val::Auto,
                    ..Default::default()
                },
                Outline::new(Val::Px(1.), Val::Px(0.0), WHITE.into()),
            )),
            Spawn(value_counter("Apples Alive", 16., AppleCounter)),
            Spawn(value_counter("Trees Alive", 16., TreeCounter)),
        )),
    )
}

fn upgrade_tracker() -> impl Bundle {
    (
        ReplaceOnHotreload,
        Node {
            width: Px(HUD_WIDTH_ELEMENT * 2.),
            padding: UiRect::all(Val::Px(4.)),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        BackgroundColor(WHITE_SMOKE.with_alpha(0.1).into()),
        BorderRadius::all(Val::Px(4.)),
        Outline::new(Val::Px(2.), Val::Px(0.), WHITE.into()),
        children![
            (
                Text::new("Upgrades:"),
                UpgradeCounter,
                TextColor(LABEL_TEXT),
                TextFont::from_font_size(20.0),
            )
        ]
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


fn update_button<E, C>(name: impl Into<String>, upgrade_text: impl Into<String>) -> impl Bundle
where
    E: Event + Default,
    C: Component + Default,
{
    (
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        children![
            (
                TurretUpdateIndicator,
                Node {
                    width: Px(HUD_WIDTH_ELEMENT),
                    height: Px(25.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(
                    Text(upgrade_text.into()),
                    TextFont::from_font_size(18.0),
                    TextColor(BUTTON_TEXT),
                ),]
            ),
            (
                TurretUpdateIndicator,
                Node {
                    width: Px(HUD_WIDTH_ELEMENT),
                    height: Px(25.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(WHITE_SMOKE.with_alpha(0.1).into()),
                Outline::new(Val::Px(2.), Val::Px(0.), WHITE.into()),
                BorderRadius::all(Val::Px(4.)),
                children![(
                    Text("Upgrade".into()),
                    TextFont::from_font_size(18.0),
                    TextColor(LABEL_TEXT),
                )]
            ),
            (Node {
                width: Px(HUD_WIDTH_ELEMENT),
                height: Px(10.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },),
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                BackgroundColor(WHITE_SMOKE.with_alpha(0.1).into()),
                Outline::new(Val::Px(2.), Val::Px(0.), WHITE.into()),
                BorderRadius::all(Val::Px(4.)),
                children![
                    (
                        Text(name.into()),
                        TextFont::from_font_size(20.0),
                        TextColor(BUTTON_TEXT),
                    ),
                    (widget::button_base_marked(
                        "1",
                        C::default(),
                        trigg_event::<E>,
                        Node {
                            width: Px(80.0),
                            height: Px(40.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                    ),)
                ],
            )
        ],
    )
}

fn trigg_event<E>(_t: Trigger<Pointer<Click>>, mut evtw: EventWriter<E>)
where
    E: Event + Default,
{
    evtw.write(E::default());
}
