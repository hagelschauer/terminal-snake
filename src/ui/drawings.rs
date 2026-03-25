use ratatui::style::Color;

pub const APPLE: [[char; 4]; 2] = [[' ', '▄', '▄', ' '], [' ', '▀', '▀', ' ']];
pub const APPLE_COLOR: Color = Color::Red;

pub const SNAKE_BASE: [[char; 4]; 2] = [[' ', '▄', '▄', ' '], [' ', '▀', '▀', ' ']];
pub const SNAKE_UP: [[char; 4]; 2] = [[' ', '█', '█', ' '], [' ', ' ', ' ', ' ']];
pub const SNAKE_DOWN: [[char; 4]; 2] = [[' ', ' ', ' ', ' '], [' ', '█', '█', ' ']];
pub const SNAKE_LEFT: [[char; 4]; 2] = [['▄', ' ', ' ', ' '], ['▀', ' ', ' ', ' ']];
pub const SNAKE_RIGHT: [[char; 4]; 2] = [[' ', ' ', ' ', '▄'], [' ', ' ', ' ', '▀']];
pub const SNAKE_COLOR: Color = Color::Green;
