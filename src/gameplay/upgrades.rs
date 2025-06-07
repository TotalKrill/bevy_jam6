use crate::gameplay::{App, Commands, IntoScheduleConfigs, Res, Time, Timer, TimerMode, in_state};
use crate::screens::Screen;
use bevy::app::FixedUpdate;
use bevy::prelude::{Event, ResMut, Resource, Trigger};
use std::time::Duration;

#[derive(Resource, Clone)]
pub struct BulletUpgrades {
    pub damage: u32,
    pub split_probability: f32,
    pub speed: f32,
}

#[derive(Event, Clone, Debug)]
pub enum UpgradeBulletEvent {
    Damage,
    SplitProbability,
    Speed, // TOOO not yet mapped correctly
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
        println!("upgrade: {:?}", event);
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

// TODO change to component?
#[derive(Event, Clone, Debug)]
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
        println!("upgrade: {:?}", event);
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

#[derive(Resource, Debug)]
pub struct AutoUpgrade {
    timer: Timer,
}
fn auto_upgrade_bullet_damage(
    mut commands: Commands,
    time: Res<Time>,
    mut auto_upgrade: ResMut<AutoUpgrade>,
) {
    auto_upgrade.timer.tick(time.delta());

    if auto_upgrade.timer.finished() {
        commands.trigger(UpgradeBulletEvent::Damage);
    }
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(AutoUpgrade {
        timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
    });
    app.insert_resource(BulletUpgrades::new());
    app.insert_resource(TractorUpgrades::new());

    app.add_event::<UpgradeBulletEvent>();
    app.add_event::<UpgradeTractorEvent>();

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

    app.add_systems(
        FixedUpdate,
        auto_upgrade_bullet_damage.run_if(in_state(Screen::InGame)),
    );
}
