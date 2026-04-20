use ratatui::{layout::Rect, Frame};

use crate::{
    core::app::{App, PaneFocus},
    ui::components::list,
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    list::render(
        frame,
        area,
        "Parent",
        &app.parent,
        None,
        0,
        true,
        app.focus == PaneFocus::Left,
    );
}
