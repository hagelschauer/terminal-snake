use std::collections::VecDeque;

use rand::{RngExt, rng, rngs::ThreadRng};

use crate::{
    direction::Direction,
    game::{GamePhase, Snake},
};

pub const WIDTH: u16 = 15;
pub const HEIGHT: u16 = 15;

const DIRECTION_BUFFER_MAX: usize = 2;

pub struct GameState {
    pub apple_pos: (u16, u16),
    pub snake: Snake,
    pub score: u16,
    turns: VecDeque<Direction>,
    rng: ThreadRng,
}

impl GameState {
    pub fn new() -> Self {
        let apple_pos = (10, HEIGHT / 2);

        Self {
            apple_pos,
            snake: Snake::new(2, HEIGHT / 2),
            score: 0,
            turns: VecDeque::with_capacity(DIRECTION_BUFFER_MAX),
            rng: rng(),
        }
    }

    pub fn shuffle_apple(&mut self) {
        let avaliable_positions: Vec<(u16, u16)> = (0..WIDTH)
            .flat_map(|x| (0..HEIGHT).map(move |y| (x, y)))
            .filter(|pos| self.snake.occupies_position(*pos).is_none())
            .collect();

        self.apple_pos = avaliable_positions[self.rng.random_range(0..avaliable_positions.len())]
    }

    pub fn tick(&mut self) -> Option<GamePhase> {
        if let Some(direction) = self.turns.pop_front() {
            self.snake.turn(direction);
        }

        if self.snake.collision_ahead() {
            return Some(GamePhase::GameOver);
        } else if self.snake.head.position == self.apple_pos {
            self.score += 1;
            self.snake.grow();
            self.shuffle_apple();
        } else {
            self.snake.advance();
        }

        None
    }

    pub fn turn(&mut self, direction: Direction) {
        if self.turns.len() < DIRECTION_BUFFER_MAX {
            self.turns.push_back(direction);
        }
    }
}
