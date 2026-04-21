use ratatui::{layout::Rect, Frame};

use crate::{
    core::app::{App, PaneFocus},
    ui::components::list,
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    list::render(
        frame,
        area,
        "Current",
        &app.filtered,
        Some(app.selected),
        app.scroll,
        false,
        app.focus == PaneFocus::Current,
    );
}