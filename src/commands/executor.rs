use std::process::{Command, Stdio};

use anyhow::Result;

use crate::{
    commands::parser::ParsedCommand,
    core::{app::App, context::AppContext, state},
};

pub fn execute(app: &mut App, ctx: &AppContext, command: ParsedCommand) -> Result<()> {
    match command {
        ParsedCommand::NoOp => {
            app.status = String::from("empty command");
        }
        ParsedCommand::Quit => {
            app.running = false;
        }
        ParsedCommand::Cd(path) => {
            let next = if path.is_absolute() {
                path
            } else {
                app.cwd.join(path)
            };

            if !next.exists() || !next.is_dir() {
                app.status = format!("invalid directory: {}", next.display());
                return Ok(());
            }

            app.cwd = next;
            state::refresh_directory(app)?;
            state::request_preview(app, ctx);
            app.status = format!("cd {}", app.cwd.display());
        }
        ParsedCommand::Open(path) => {
            let target = if path.is_absolute() {
                path
            } else {
                app.cwd.join(path)
            };

            if !target.exists() {
                app.status = format!("missing path: {}", target.display());
                return Ok(());
            }

            match Command::new("xdg-open")
                .arg(&target)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
            {
                Ok(_) => {
                    app.status = format!("opened {}", target.display());
                }
                Err(error) => {
                    app.status = format!("open failed: {}", error);
                }
            }
        }
        ParsedCommand::Invalid(message) => {
            app.status = message;
        }
    }

    Ok(())
}
