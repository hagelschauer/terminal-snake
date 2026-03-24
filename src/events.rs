use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::{Game, direction::Direction, game::GamePhase};

pub fn handle_event(game: &mut Game) -> Result<()> {
    match event::read()? {
        Event::Mouse(_) => {}
        Event::Key(k) => handle_key_event(k, game),
        _ => {}
    }

    Ok(())
}

fn handle_key_event(k: KeyEvent, game: &mut Game) {
    match game.game_phase {
        GamePhase::Running => match k.code {
            KeyCode::Char('q') => game.game_phase = GamePhase::Quitting,
            KeyCode::Esc => game.game_phase = GamePhase::Paused,
            KeyCode::Up => game.game_state.turn(Direction::Up),
            KeyCode::Down => game.game_state.turn(Direction::Down),
            KeyCode::Left => game.game_state.turn(Direction::Left),
            KeyCode::Right => game.game_state.turn(Direction::Right),
            _ => {}
        },
        GamePhase::Paused => match k.code {
            KeyCode::Char('q') => game.game_phase = GamePhase::Quitting,
            KeyCode::Char('r') => game.restart(),
            KeyCode::Char('c') => game.game_phase = GamePhase::Running,
            _ => {}
        },
        GamePhase::GameOver => match k.code {
            KeyCode::Char('q') => game.game_phase = GamePhase::Quitting,
            KeyCode::Char('r') => game.restart(),
            _ => {}
        },
        _ => {}
    }
}
