use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::StatefulWidget};

use crate::{
    game::{
        GameState, Snake,
        game_state::{HEIGHT, WIDTH},
        snake::SnakeSegment,
    },
    ui::drawings::{APPLE, APPLE_COLOR, SNAKE_COLOR},
};

pub const CELL_WIDTH: u16 = 4;
pub const CELL_HEIGHT: u16 = 2;

pub const WIDTH_CHARS: u16 = WIDTH * CELL_WIDTH;
pub const HEIGHT_CHARS: u16 = HEIGHT * CELL_HEIGHT;

pub struct GameBoard;

impl StatefulWidget for GameBoard {
    type State = GameState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let (x, y) = state.apple_pos;
        draw_apple(x, y, area, buf);
        draw_snake(&state.snake, area, buf);
    }
}

fn draw_apple(x: u16, y: u16, area: Rect, buf: &mut Buffer) {
    let (x, y) = (area.x + x * CELL_WIDTH, area.y + y * CELL_HEIGHT);
    let style = Style::default().fg(APPLE_COLOR);
    for (dy, line) in APPLE.iter().enumerate() {
        buf.set_string(x, y + dy as u16, line, style);
    }
}

fn draw_snake(snake: &Snake, area: Rect, buf: &mut Buffer) {
    draw_snake_segment(&snake.head, area, buf);
}

fn draw_snake_segment(segment: &SnakeSegment, area: Rect, buf: &mut Buffer) {
    let (x, y) = segment.position;

    let (x, y) = (area.x + x * CELL_WIDTH, area.y + y * CELL_HEIGHT);
    let style = Style::default().fg(SNAKE_COLOR);
    for (dy, line) in APPLE.iter().enumerate() {
        buf.set_string(x, y + dy as u16, line, style);
    }

    if let Some(next) = &segment.next {
        draw_snake_segment(next, area, buf);
    }
}
