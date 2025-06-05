use crate::{gameplay::score::ScoreCounter, screens::Screen, theme::widget};

use super::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::GameOver), spawn_gameover);
}

fn spawn_gameover(mut commands: Commands, score: Res<ScoreCounter>) {
    commands.spawn((
        widget::ui_root("Game Over"),
        GlobalZIndex(2),
        StateScoped(Menu::GameOver),
        children![
            widget::header("Game Over"),
            widget::label(format!("Score: {}", score.points)),
            widget::button("Quit to title", quit_to_title),
        ],
    ));
}

fn quit_to_title(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
