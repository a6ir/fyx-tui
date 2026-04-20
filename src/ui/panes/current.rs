use ratatui::{layout::Rect, Frame};

use crate::{core::app::App, ui::components::list};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    list::render(
        frame,
        area,
        "Current",
        &app.current,
        Some(app.selected),
        app.scroll,
        false,
    );
}
