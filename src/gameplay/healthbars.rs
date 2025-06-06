use bevy::color::palettes::tailwind::GREEN_300;
use bevy_ui_anchor::{AnchorUiConfig, AnchorUiNode, AnchoredUiNodes};

use crate::{PausableSystems, gameplay::health::Health};

use super::*;

#[derive(Component)]
pub struct HealthBar;
#[derive(Component)]
pub struct HealthBarBar;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            despawn_on_missing,
            update_healthbars,
            visible_healthbars_when_changed,
        )
            .in_set(PausableSystems),
    );
}

pub fn healthbar(start_percent: f32) -> impl Bundle {
    (
        Name::new("UnitHealthBar"),
        Node {
            width: Val::Px(40.),
            height: Val::Px(4.),
            ..Default::default()
        },
        BackgroundColor(RED.into()),
        Outline::new(Val::Px(1.), Val::Px(0.), WHITE.into()),
        HealthBar,
        Visibility::Hidden,
        AnchorUiConfig {
            offset: Some(Vec3::Y * 2.),
            ..Default::default()
        },
        Children::spawn_one((
            Name::new("UnitHealthBarBar"),
            HealthBarBar,
            Node {
                width: Val::Percent(start_percent),
                height: Val::Percent(100.),
                ..Default::default()
            },
            BackgroundColor(GREEN_300.into()),
        )),
    )
}

fn despawn_on_missing(
    mut commands: Commands,
    healtbars: Query<Entity, (With<HealthBar>, Without<AnchorUiNode>)>,
) {
    for bar_e in healtbars.iter() {
        commands.entity(bar_e).despawn();
    }
}

fn visible_healthbars_when_changed(
    mut healthbars: Query<&mut Visibility, With<HealthBar>>,
    health_entities: Query<(&AnchoredUiNodes, &Health), Changed<Health>>,
) {
    for (ui_nodes, health) in health_entities.iter() {
        if health.percentage() < 99 {
            for uinode in ui_nodes.iter() {
                if let Ok(mut nodes) = healthbars.get_mut(uinode) {
                    *nodes = Visibility::Visible;
                }
            }
        }
    }
}

fn update_healthbars(
    healthbars: Query<&Children, With<HealthBar>>,
    health_entities: Query<(&AnchoredUiNodes, &Health), Changed<Health>>,
    mut healthbar_bar: Query<&mut Node, With<HealthBarBar>>,
) {
    for (ui_nodes, health) in health_entities.iter() {
        let percent = health.percentage();

        for children in healthbars.iter_many(ui_nodes.collection()) {
            for child in children.collection() {
                if let Ok(mut bar_node) = healthbar_bar.get_mut(*child) {
                    bar_node.width = Val::Percent(percent as f32);
                }
            }
        }
    }
}
