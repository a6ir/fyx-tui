use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

use crate::core::{app::App, context::AppContext, state};

const DOUBLE_CLICK_THRESHOLD: Duration = Duration::from_millis(350);

pub fn handle_mouse(
    app: &mut App,
    ctx: &AppContext,
    mouse: MouseEvent,
    current_pane: Rect,
) -> Result<()> {
    match mouse.kind {
        MouseEventKind::ScrollDown => {
            state::move_selection(app, 1);
            state::request_preview(app, ctx);
        }
        MouseEventKind::ScrollUp => {
            state::move_selection(app, -1);
            state::request_preview(app, ctx);
        }
        MouseEventKind::Down(MouseButton::Left) => {
            if !point_in_rect(mouse.column, mouse.row, current_pane) {
                return Ok(());
            }

            if current_pane.height <= 2 || mouse.row <= current_pane.y {
                return Ok(());
            }

            let row_in_list = mouse.row.saturating_sub(current_pane.y + 1) as usize;
            let target_index = app.scroll + row_in_list;

            if target_index < app.filtered.len() {
                app.selected = target_index;
                state::request_preview(app, ctx);

                let now = Instant::now();
                let is_double_click = app
                    .last_click
                    .as_ref()
                    .map(|prev| prev.row == mouse.row && now.duration_since(prev.at) <= DOUBLE_CLICK_THRESHOLD)
                    .unwrap_or(false);

                app.last_click = Some(crate::core::app::ClickState {
                    row: mouse.row,
                    at: now,
                });

                if is_double_click && state::enter_selected(app)? {
                    state::request_preview(app, ctx);
                }
            }
        }
        _ => {}
    }

    Ok(())
}

fn point_in_rect(x: u16, y: u16, rect: Rect) -> bool {
    x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height
}
