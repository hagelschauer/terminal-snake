mod snake_segment;
mod snake_segment_type;

pub use snake_segment::SnakeSegment;
pub use snake_segment_type::SnakeSegmentType;

use crate::{
    direction::Direction,
    game::game_state::{HEIGHT, WIDTH},
};

pub const INITIAL_LENGTH: u16 = 2;

pub struct Snake {
    pub head: SnakeSegment,
}

impl Snake {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            head: SnakeSegment {
                position: (x, y),
                direction: Direction::Right,
                next: Some(Box::new(SnakeSegment {
                    position: (x - 1, y),
                    direction: Direction::Right,
                    next: None,
                    segment_type: SnakeSegmentType::Tail,
                })),
                segment_type: SnakeSegmentType::Head,
            },
        }
    }

    pub fn advance(&mut self) {
        self.head.advance();
    }

    pub fn grow(&mut self) {
        self.head.grow();
    }

    pub fn collision_ahead(&self) -> bool {
        let (x, y) = self.head.position;
        if (x == 0 && self.head.direction == Direction::Left)
            || (y == 0 && self.head.direction == Direction::Up)
            || (x >= WIDTH - 1 && self.head.direction == Direction::Right)
            || (y >= HEIGHT - 1 && self.head.direction == Direction::Down)
        {
            return true;
        }

        let in_front = self.head.direction.apply(self.head.position);
        match self.occupies_position(in_front) {
            Some(segment) => segment.next.is_some(),
            _ => false,
        }
    }

    pub fn turn(&mut self, direction: Direction) {
        if direction == -self.head.direction {
            return;
        }
        self.head.direction = direction;
    }

    pub fn occupies_position(&self, pos: (u16, u16)) -> Option<&SnakeSegment> {
        let mut snake_segment = &self.head;
        while let Some(next) = &snake_segment.next {
            if next.position == pos {
                return Some(next);
            }
            snake_segment = next;
        }

        None
    }
}
