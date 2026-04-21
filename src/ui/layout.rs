use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct UiLayout {
    pub topbar: Rect,
    pub shortcuts: Rect,
    pub current: Rect,
    pub status: Rect,
}

pub fn split(area: Rect) -> UiLayout {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let topbar = vertical[0];
    let main = vertical[1];
    let status = vertical[2];

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(main);

    UiLayout {
        topbar,
        shortcuts: horizontal[0],
        current: horizontal[1],
        status,
    }
}