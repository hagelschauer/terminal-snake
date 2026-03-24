use ratatui::{
    Frame,
    layout::{Constraint, Direction, HorizontalAlignment, Layout},
    text::ToSpan,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

use crate::{
    Game,
    game::{GamePhase, GameState, game_state, snake},
    ui::{GameBoard, GameLayout},
};

pub fn render(game: &mut Game, frame: &mut Frame) {
    render_game(game, frame);

    match game.game_phase {
        GamePhase::Paused => render_pause_popup(frame),
        GamePhase::GameOver => render_gameover_popup(frame, &game.game_state),
        _ => {}
    }
}

fn render_game(game: &mut Game, frame: &mut Frame) {
    let layout = GameLayout::build(frame.area());

    let outer_border = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let header = Paragraph::new(game.game_state.score.to_span())
        .centered()
        .block(Block::default().borders(Borders::BOTTOM));

    frame.render_widget(outer_border, layout.root_area);
    frame.render_widget(header, layout.header);
    frame.render_stateful_widget(GameBoard, layout.game_board, &mut game.game_state);
}

fn render_gameover_popup(frame: &mut Frame, game_state: &GameState) {
    let popup_area = frame
        .area()
        .centered(Constraint::Length(30), Constraint::Length(7));

    let title =
        if game_state.score >= game_state::WIDTH * game_state::HEIGHT - snake::INITIAL_LENGTH {
            " YOU WON! "
        } else {
            " Game Over "
        };

    let block = Block::default()
        .title(title)
        .title_alignment(HorizontalAlignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Double);

    let content_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Fill(1), Constraint::Min(2)])
        .split(block.inner(popup_area));

    let scoreboard = Paragraph::new(format!("Score: {}", game_state.score)).centered();
    let text = Paragraph::new("Press q to quit\nPress r to restart").left_aligned();

    frame.render_widget(Clear, popup_area);
    frame.render_widget(block, popup_area);
    frame.render_widget(scoreboard, content_areas[0]);
    frame.render_widget(text, content_areas[1]);
}

fn render_pause_popup(frame: &mut Frame) {
    let popup_area = frame
        .area()
        .centered(Constraint::Length(30), Constraint::Length(5));

    let block = Block::default()
        .title(" Paused ")
        .title_alignment(HorizontalAlignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Double);

    let text = Paragraph::new("Press c to continue\nPress q to quit\nPress r to restart")
        .left_aligned()
        .block(block);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(text, popup_area);
}
