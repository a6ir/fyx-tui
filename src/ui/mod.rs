pub mod components;
pub mod layout;
pub mod panes;
pub mod theme;

use ratatui::Frame;
use ratatui::widgets::Clear;

use crate::core::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    frame.render_widget(Clear, frame.size());

    let ui_layout = layout::split(frame.size());
    components::topbar::render(frame, ui_layout.topbar, app);
    panes::shortcuts::render(frame, ui_layout.shortcuts, app);
    panes::current::render(frame, ui_layout.current, app);
    components::status::render(frame, ui_layout.status, app);
}