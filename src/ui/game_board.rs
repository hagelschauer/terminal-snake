use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::StatefulWidget,
};

use crate::{
    direction::Direction,
    game::{
        GameState, Snake,
        game_state::{HEIGHT, WIDTH},
        snake::SnakeSegment,
    },
    ui::drawings::{
        APPLE, APPLE_COLOR, SNAKE_BASE, SNAKE_COLOR, SNAKE_DOWN, SNAKE_LEFT, SNAKE_RIGHT, SNAKE_UP,
    },
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

fn draw_drawing(
    x: u16,
    y: u16,
    area: Rect,
    buf: &mut Buffer,
    drawing: [[char; 4]; 2],
    color: Color,
) {
    let (x, y) = (area.x + x * CELL_WIDTH, area.y + y * CELL_HEIGHT);
    let style = Style::default().fg(color);
    for (dy, line) in drawing.iter().enumerate() {
        for (dx, char) in line.iter().enumerate() {
            buf.cell_mut((x + dx as u16, y + dy as u16))
                .map(|cell| cell.set_char(*char).set_style(style));
        }
    }
}

fn draw_over(base: [[char; 4]; 2], on_top: [[char; 4]; 2]) -> [[char; 4]; 2] {
    let mut base = base;
    for (y, line) in on_top.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if !char.is_whitespace() {
                base[y][x] = *char;
            }
        }
    }
    base
}

fn draw_apple(x: u16, y: u16, area: Rect, buf: &mut Buffer) {
    draw_drawing(x, y, area, buf, APPLE, APPLE_COLOR);
}

fn draw_snake(snake: &Snake, area: Rect, buf: &mut Buffer) {
    draw_snake_segment(&snake.head, area, buf);
}

fn draw_snake_segment(segment: &SnakeSegment, area: Rect, buf: &mut Buffer) {
    let (x, y) = segment.position;

    let mut drawing = SNAKE_BASE;
    drawing = draw_over(
        drawing,
        match segment.direction {
            Direction::Up => SNAKE_UP,
            Direction::Down => SNAKE_DOWN,
            Direction::Left => SNAKE_LEFT,
            Direction::Right => SNAKE_RIGHT,
        },
    );

    if let Some(next) = &segment.next {
        drawing = draw_over(
            drawing,
            match next.direction {
                Direction::Up => SNAKE_DOWN,
                Direction::Down => SNAKE_UP,
                Direction::Left => SNAKE_RIGHT,
                Direction::Right => SNAKE_LEFT,
            },
        );
    }

    draw_drawing(x, y, area, buf, drawing, SNAKE_COLOR);

    if let Some(next) = &segment.next {
        draw_snake_segment(next, area, buf);
    }
}
