use ratatui::layout::{Constraint, Layout, Rect};

use crate::ui::game_board;

const HEADER_HEIGHT: u16 = 2;

pub struct GameLayout {
    pub root_area: Rect,
    pub header: Rect,
    pub game_board: Rect,
}

impl GameLayout {
    pub fn build(area: Rect) -> Self {
        let root_area = area.centered(
            Constraint::Length(game_board::WIDTH_CHARS + 2),
            Constraint::Length(game_board::HEIGHT_CHARS + 2 + HEADER_HEIGHT),
        );

        let vertical = Layout::vertical([
            Constraint::Length(HEADER_HEIGHT),
            Constraint::Length(game_board::HEIGHT_CHARS),
            ])
            .margin(1)
            .split(root_area);
        let header = vertical[0];
        let game_board = vertical[1];

        Self {
            root_area,
            header,
            game_board,
        }
    }
}
