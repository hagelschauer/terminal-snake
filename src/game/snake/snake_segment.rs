use crate::{direction::Direction, game::snake::SnakeSegmentType};

pub struct SnakeSegment {
    pub position: (u16, u16),
    pub direction: Direction,
    pub next: Option<Box<SnakeSegment>>,
    pub segment_type: SnakeSegmentType
}

impl SnakeSegment {
    pub fn advance(&mut self) {
        self.position = self.direction.apply(self.position);

        if let Some(next) = &mut self.next {
            next.advance();
            next.direction = self.direction;
        }
    }

    pub fn grow(&mut self) {
        let old_position = self.position;
        let old_next = std::mem::take(&mut self.next);

        self.position = self.direction.apply(self.position);
        self.next = Some(Box::new(Self {
            position: old_position,
            direction: self.direction,
            next: old_next,
            segment_type: SnakeSegmentType::Body
        }))
    }
}
