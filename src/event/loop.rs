use std::{thread, time::Duration};

use anyhow::Result;
use crossbeam_channel::{unbounded, Receiver};
use crossterm::event;
use ratatui::{backend::Backend, Terminal};

use crate::{
    core::{app::App, context::AppContext, state},
    event::event::AppEvent,
    input, ui,
};

pub fn init_events(tick_rate: Duration) -> Receiver<AppEvent> {
    let (tx, rx) = unbounded();

    thread::spawn(move || {
        loop {
            match event::poll(tick_rate) {
                Ok(true) => {
                    if let Ok(ev) = event::read() {
                        let _ = tx.send(AppEvent::Input(ev));
                    }
                }
                Ok(false) => {}
                Err(_) => {}
            }

            let _ = tx.send(AppEvent::Tick);
        }
    });

    rx
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    ctx: &AppContext,
) -> Result<()> {
    state::refresh_directory(app)?;
    state::request_preview(app, ctx);

    let events = init_events(Duration::from_millis(16));

    while app.running {
        state::drain_preview(app, ctx);

        terminal.draw(|frame| {
            ui::draw(frame, app);
        })?;

        match events.recv() {
            Ok(AppEvent::Input(ev)) => {
                let area = terminal.size()?;
                input::handler::handle_input(app, ctx, ev, area)?;
            }
            Ok(AppEvent::Tick) => {
                app.is_scrolling = false;
            }
            Err(_) => {
                app.running = false;
            }
        }
    }

    Ok(())
}
