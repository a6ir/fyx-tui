use crate::{commands::parser::ParsedCommand, core::app::App};

pub fn execute(app: &mut App, command: ParsedCommand) {
    match command {
        ParsedCommand::Quit => {
            app.running = false;
        }
        ParsedCommand::Unknown(cmd) => {
            app.status = format!("unknown command: {}", cmd);
        }
    }
}
