use crate::gameplay::Val::Px;
use crate::menus::Menu;
use crate::screens::Screen;
use crate::theme::widget;
use bevy::asset::uuid::Uuid;
use bevy::ecs::spawn::SpawnIter;
use bevy::ecs::system::command::trigger;
use bevy::prelude::*;
use bevy_jornet::{JornetEvent, JornetPlugin, Leaderboard, Player};
use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};

const GAME_NAME: &str = "newton-survivor";

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(JornetPlugin::with_leaderboard(
        "b6cd7cbc-d05f-4d51-a942-d5c735efe74a",
        "cc358714-99fb-46b0-80a5-9fd9e021d377",
    ))
    .add_event::<AddUserScore>()
    .add_systems(Startup, setup_local_storage)
    .add_systems(Startup, load_local_user.after(setup_local_storage))
    .add_systems(Update, (test_create_score, save_local_user));

    app.add_systems(OnEnter(Menu::Leaderboard), spawn_leaderboard);

    app.add_observer(add_score_to_leaderboard);
}

fn spawn_leaderboard(mut commands: Commands, leaderboard: Res<Leaderboard>) {
    let mut scores: Vec<_> = leaderboard.get_leaderboard().to_vec();

    scores.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut result: Vec<[String; 3]> = Vec::new();
    for score in scores.iter().take(10) {
        let score = score.clone();
        let timestamp = score.timestamp.clone();
        let p = &timestamp[0..10];
        let player = score.player.clone();
        let score_str = score.score.to_string();
        let data_array = [p.to_string(), player, score_str];
        result.push(data_array);
    }

    commands.spawn((
        widget::ui_root("Leaderboard"),
        GlobalZIndex(2),
        StateScoped(Menu::Leaderboard),
        children![
            widget::header("Top 10 Results"),
            grid(result),
            widget::button("Back", go_back_on_click)
        ],
    ));
}

fn grid(content: Vec<[String; 3]>) -> impl Bundle {
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(3, 400.0),
            ..default()
        },
        Children::spawn(SpawnIter(content.into_iter().flatten().enumerate().map(
            |(i, text)| {
                (
                    widget::label(text), // String implements Into<Text> so this should work
                    Node {
                        justify_self: if i % 3 == 0 {
                            JustifySelf::End
                        } else {
                            JustifySelf::Start
                        },
                        ..default()
                    },
                )
            },
        ))),
    )
}

fn go_back_on_click(
    _: Trigger<Pointer<Click>>,
    screen: Res<State<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}

#[derive(Event)]
pub struct AddUserScore {
    pub value: f32,
}

#[derive(Resource, Serialize, Deserialize, Debug, Default)]
pub struct User {
    pub name: String,
    pub high_score: f32,
    last_score: f32,
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
    let config_dir = if let Some(config_dir) = dirs::config_dir() {
        config_dir.join(GAME_NAME)
    } else {
        std::path::PathBuf::from("local").join(GAME_NAME)
    };

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
    mut event_writer: EventWriter<AddUserScore>,
) {
    if kb_input.just_pressed(KeyCode::KeyI) {
        event_writer.write(AddUserScore {
            // value: 1000.0,
            value: rand::random::<f32>() * 1000.0,
        });
    }
}

fn load_local_user(mut leaderboard: ResMut<Leaderboard>, user: Res<Persistent<User>>) {
    if user.name != User::default().name {
        leaderboard.as_player(user.as_player());
    } else {
        // No local user - create a new one
        leaderboard.create_player(None);
    }

    leaderboard.refresh_leaderboard();
}

fn save_local_user(
    mut event_reader: EventReader<JornetEvent>,
    leaderboard: Res<Leaderboard>,
    mut user: ResMut<Persistent<User>>,
) {
    for event in event_reader.read() {
        match event {
            JornetEvent::SendScoreSuccess => {
                leaderboard.refresh_leaderboard();
            }
            JornetEvent::SendScoreFailure => {}
            JornetEvent::CreatePlayerSuccess => {
                match leaderboard.get_player() {
                    Some(player) => {
                        user.update(|user| {
                            // TODO Make the name configurable
                            user.name = player.name.to_string();
                            user.id = player.id;
                            user.key = player.key;
                        })
                        .expect("failed to update user score");
                    }
                    // TODO did we fail to create the user?
                    _ => {}
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
    trigger: Trigger<AddUserScore>,
    leaderboard: Res<Leaderboard>,
    mut user: ResMut<Persistent<User>>,
) {
    let user_score = trigger.event();

    println!("user score: {:#?}", user_score.value);

    // Send score to the leaderboard
    leaderboard.send_score(user_score.value);

    // Update user high score
    if user_score.value > user.high_score {
        user.update(|user| {
            user.high_score = user_score.value;
        })
        .expect("failed to update user score");
    }

    // Update user last score
    user.update(|user| {
        user.last_score = user_score.value;
    })
    .expect("failed to update user score");
}
