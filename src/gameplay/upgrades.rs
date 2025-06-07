use crate::gameplay::{App, Component};
use bevy::prelude::{Event, ResMut, Resource, Trigger};

#[derive(Resource, Clone)]
pub struct BulletUpgrades {
    pub damage: u32,
    pub split_probability: f32,
    pub speed: f32,
}

#[derive(Event, Clone)]
pub enum UpgradeBulletEvent {
    Damage,
    SplitProbability,
    Speed,
}

impl BulletUpgrades {
    pub fn new() -> Self {
        Self {
            damage: 1,
            split_probability: 1.0,
            speed: 75.,
        }
    }

    pub fn upgrade(&mut self, event: &UpgradeBulletEvent) {
        match event {
            UpgradeBulletEvent::Damage => {
                self.damage += 1;
            }
            UpgradeBulletEvent::SplitProbability => {
                self.split_probability = 1.;
            }
            UpgradeBulletEvent::Speed => {
                self.speed = 75.;
            }
        }
    }
}

#[derive(Resource, Clone)]
pub struct TractorUpgrades {
    pub health: u32,
    pub acceleration: f32,
    pub turn_rate: f32,
    pub saw_damage: u32,
}

#[derive(Event, Clone)]
pub enum UpgradeTractorEvent {
    Health,
    Acceleration,
    TurnRate,
    SawDamage,
}

impl TractorUpgrades {
    pub fn new() -> Self {
        Self {
            health: 5,
            acceleration: 10000.0,
            turn_rate: 130.0,
            saw_damage: 1,
        }
    }

    pub fn upgrade(&mut self, event: &UpgradeTractorEvent) {
        match event {
            UpgradeTractorEvent::Health => self.health += 1,
            UpgradeTractorEvent::Acceleration => {
                self.acceleration += 10.;
            }
            UpgradeTractorEvent::TurnRate => {
                self.turn_rate += 1.;
            }
            UpgradeTractorEvent::SawDamage => {
                self.saw_damage += 1;
            }
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(BulletUpgrades::new());
    app.insert_resource(TractorUpgrades::new());

    app.add_observer(
        |trigger: Trigger<UpgradeTractorEvent>, mut upgrades: ResMut<TractorUpgrades>| {
            upgrades.upgrade(trigger.event());
        },
    );

    app.add_observer(
        |trigger: Trigger<UpgradeBulletEvent>, mut upgrades: ResMut<BulletUpgrades>| {
            upgrades.upgrade(trigger.event());
        },
    );
}
