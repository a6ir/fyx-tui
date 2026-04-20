pub mod components;
pub mod layout;
pub mod panes;
pub mod theme;

use ratatui::Frame;

use crate::core::{app::App, mode::Mode};

pub fn draw(frame: &mut Frame, app: &App) {
    let show_command = matches!(app.mode, Mode::Command | Mode::Search);
    let ui_layout = layout::split(frame.size(), show_command);

    panes::current::render(frame, ui_layout.left, app);
    panes::preview::render(frame, ui_layout.right, app);

    components::status::render(frame, ui_layout.status, app);

    if let Some(command_rect) = ui_layout.command {
        components::command_bar::render(frame, command_rect, app);
    }
}
