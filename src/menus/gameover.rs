use bevy_persistent::Persistent;
use crate::{gameplay::score::ScoreCounter, screens::Screen, theme::widget};
use crate::leaderboard::User;
use super::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::GameOver), spawn_gameover);
}

fn spawn_gameover(mut commands: Commands, score: Res<ScoreCounter>, user: Res<Persistent<User>>) {

    let (header, ending) = if score.points > user.high_score as usize {
        ("New High Score!", format!("Maybe you should check the leaderboard {}!", user.name))
    } else {
        ("Game Over", format!("Better luck next time {}", user.name))
    };

    commands.spawn((
        widget::ui_root("Game Over"),
        GlobalZIndex(2),
        StateScoped(Menu::GameOver),
        children![
                widget::header(header),
                widget::label(format!("Score: {}", score.points)),
                widget::label(ending),
                widget::button("Quit to title", quit_to_title),
        ],
    ));
}

fn quit_to_title(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
