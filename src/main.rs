mod events;
mod game;
mod render;
mod ui;

mod direction;

use game::Game;
use std::{io, time::Duration};

use color_eyre::eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    execute!(io::stdout(), EnableMouseCapture)?;

    let mut terminal = ratatui::init();

    let mut game = Game::new();

    let result = game.run(
        &mut terminal,
        Duration::from_millis(200),
        render::render,
        events::handle_event,
    );

    ratatui::restore();
    execute!(io::stdout(), DisableMouseCapture)?;

    result
}
