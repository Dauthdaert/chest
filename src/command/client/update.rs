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

        let mut command = loop {
            let name: String = prompt("Enter a name for the command in Chest")?;
            if let Some(command) = engine.get_command(&name) {
                break command;
            } else {
                println!("No command by this name.");
            }
        };

        if let Some(command_text) =
            prompt_opt("Enter the new command text (empty to keep current)")?
        {
            command.command_text = command_text;
        }

        if let Some(description) = prompt_opt("Enter the new description (empty to keep)")? {
            command.description = description;
        }

        engine.update_command(command)
    }
}
