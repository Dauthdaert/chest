use clap::Parser;
use promptly::{prompt, prompt_opt};

use crate::{
    command::client::engine::{Database, Engine},
    AppResult,
};

#[derive(Parser)]
pub struct Cmd;

impl Cmd {
    pub fn run(self) -> AppResult<()> {
        let engine = Database::init()?;

        let name: String = prompt("Enter a name for the command in Chest")?;
        if let Some(mut command) = engine.get_command(&name) {
            if let Some(command_text) =
                prompt_opt("Enter the new command text (empty to keep current)")?
            {
                command.command_text = command_text;
            }

            if let Some(description) = prompt_opt("Enter the new description (empty to keep)")? {
                command.description = description;
            }

            engine.update_command(command)
        } else {
            if let Some(alt_command) = engine.search_commands_strict(&name) {
                println!(
                    "No command by this name. Did you mean '{}'?",
                    alt_command.name
                );
            } else {
                println!("No command by this name.");
            }
            Ok(())
        }
    }
}
