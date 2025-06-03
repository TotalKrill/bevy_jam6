use super::*;
use bevy::asset::uuid::Uuid;
use bevy::prelude::*;
use bevy_jornet::{JornetEvent, JornetPlugin, Leaderboard, Player};
use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};


const GAME_NAME: &str = "newton-survivor";

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(JornetPlugin::with_leaderboard(
        "0ebdf1b6-433d-4efa-b453-ac5e040b3bf1",
        "8b5fdba6-540f-41fe-8d38-70e0f6428e1c",
    ))
    .add_event::<UserScore>()
    .add_systems(Startup, setup_local_storage)
    .add_systems(Startup, load_local_user.after(setup_local_storage))
    .add_systems(
        Update,
        (
            test_create_score,
            display_leaderboard,
            save_local_user,
            add_score_to_leaderboard,
        ),
    );
}

#[derive(Event)]
struct UserScore {
    value: f32,
}

#[derive(Resource, Serialize, Deserialize, Debug, Default)]
struct User {
    name: String,
    high_score: f32,
    id: Uuid,
    key: Uuid,
}

impl User {
    fn as_player(&self) -> Player {
        Player {
            id: self.id,
            key: self.key,
            name: self.name.to_string(),
        }
    }
}

fn setup_local_storage(mut commands: Commands) {
    let config_dir = dirs::config_dir().unwrap().join(GAME_NAME);

    commands.insert_resource(
        Persistent::<User>::builder()
            .name("User")
            .format(StorageFormat::Toml)
            .path(config_dir.join("user.toml"))
            .default(User::default())
            .build()
            .expect("failed to initialize user"),
    )
}

fn test_create_score(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<UserScore>,
) {
    if kb_input.just_pressed(KeyCode::KeyI) {
        event_writer.write(UserScore { value: 44.1 });
    }
}

fn display_leaderboard(leaderboard: Res<Leaderboard>) {
    if leaderboard.is_changed() {
        for score in leaderboard.get_leaderboard() {
            println!(
                "{:#?}\t{:#?}\t{:#?}",
                score.timestamp, score.player, score.score
            );
        }
    }
}

fn load_local_user(mut leaderboard: ResMut<Leaderboard>, user: Res<Persistent<User>>) {
    if user.name != User::default().name {
        println!("loading user: {:#?}", user);
        leaderboard.as_player(user.as_player());
    } else {
        // No local user - create a new one
        leaderboard.create_player(None);
    }
}

fn save_local_user(
    mut event_reader: EventReader<JornetEvent>,
    leaderboard: Res<Leaderboard>,
    mut user: ResMut<Persistent<User>>,
) {
    for event in event_reader.read() {
        println!("Jornet event: {:?}", event);
        match event {
            JornetEvent::SendScoreSuccess => {
                leaderboard.refresh_leaderboard();
            }
            JornetEvent::SendScoreFailure => {}
            JornetEvent::CreatePlayerSuccess => {
                match leaderboard.get_player() {
                    Some(player) => {
                        println!("user created with name: {:#?}", player.name);

                        user.update(|user| {
                            // TODO Make the name configurable
                            user.name = player.name.to_string();
                            user.id = player.id;
                            user.key = player.key;
                        })
                        .expect("failed to update user score");
                    }
                    // TODO did we fail to create the user?
                    _ => {
                        println!("No user found");
                    }
                }

                leaderboard.refresh_leaderboard();
            }
            JornetEvent::CreatePlayerFailure => {}
            JornetEvent::RefreshLeaderboardSuccess => {}
            JornetEvent::RefreshLeaderboardFailure => {}
        }
    }
}

fn add_score_to_leaderboard(
    mut event_reader: EventReader<UserScore>,
    leaderboard: Res<Leaderboard>,
) {
    for user_score in event_reader.read() {
        leaderboard.send_score(user_score.value);
    }
}
