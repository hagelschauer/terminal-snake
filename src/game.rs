pub mod game_phase;
pub mod game_state;
pub mod snake;

use std::time::{Duration, Instant};

use color_eyre::eyre::Result;
use crossterm::event;
pub use game_phase::GamePhase;
pub use game_state::GameState;
use ratatui::{DefaultTerminal, Frame};
pub use snake::Snake;

pub struct Game {
    pub game_phase: GamePhase,
    pub game_state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            game_phase: GamePhase::Running,
            game_state: GameState::new(),
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        tick_rate: Duration,
        mut render: impl FnMut(&mut Self, &mut Frame),
        on_event: impl Fn(&mut Self) -> Result<()>,
    ) -> Result<()> {
        let mut last_tick = Instant::now();

        while self.game_phase != GamePhase::Quitting {
            terminal.draw(|frame| render(self, frame))?;

            if self.game_phase == GamePhase::Running {
                let time_until_next_tick = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(Duration::ZERO);

                if let Ok(true) = event::poll(time_until_next_tick) {
                    on_event(self)?;
                }

                if last_tick.elapsed() >= tick_rate {
                    if let Some(new_game_phase) = self.game_state.tick() {
                        self.game_phase = new_game_phase;
                    }
                    last_tick = Instant::now();
                }
            } else {
                on_event(self)?;
            }
        }
        Ok(())
    }

    pub fn restart(&mut self) {
        self.game_phase = GamePhase::Running;
        self.game_state = GameState::new();
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
