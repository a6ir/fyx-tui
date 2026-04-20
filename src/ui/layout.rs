use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct UiLayout {
    pub left: Rect,
    pub right: Rect,
    pub status: Rect,
    pub command: Option<Rect>,
}

pub fn split(area: Rect, show_command: bool) -> UiLayout {
    let vertical = if show_command {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1), Constraint::Length(1)])
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(area)
    };

    let main = vertical[0];
    let status = vertical[1];
    let command = if show_command { Some(vertical[2]) } else { None };

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main);

    UiLayout {
        left: columns[0],
        right: columns[1],
        status,
        command,
    }
}
