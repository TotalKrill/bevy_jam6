use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_event::<Damage>()
        .add_event::<Death>()
        .add_systems(Update, damage_health);
}

#[derive(Component, Debug)]
pub struct Health {
    pub current: u32,
}

#[derive(Event, Debug)]
pub struct Damage {
    pub value: u32,
    pub entity: Entity,
}

#[derive(Event, Debug)]
pub struct Death;

fn damage_health(
    mut commands: Commands,
    mut event_reader: EventReader<Damage>,
    mut query: Query<(Entity, &mut Health), With<Health>>,
) {
    for event in event_reader.read() {
        for (entity, mut health) in query.iter_mut() {
            if event.entity == entity {
                health.current -= event.value;

                if health.current == 0 {
                    commands.trigger_targets(Death, entity);
                }
            }
        }
    }
}
