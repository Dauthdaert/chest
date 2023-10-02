use clap::Parser;
use promptly::{prompt, prompt_opt};

use crate::{
    command::client::{
        engine::{Database, Engine},
        shell_command::ShellCommand,
    },
    AppResult,
};

#[derive(Parser)]
pub struct Cmd;

impl Cmd {
    pub fn run(self) -> AppResult<()> {
        let engine = Database::init()?;

        let name = loop {
            let name: String = prompt("Enter a name for the command in Chest")?;
            if engine.get_command(&name).is_none() {
                break name;
            } else {
                println!("This command name is already taken.");
            }
        };

        let command_text: String = prompt("Enter the command text")?;
        let description: String =
            prompt_opt("Enter the description (optional)")?.unwrap_or("".to_string());

        engine.add_command(ShellCommand {
            name,
            command_text,
            description,
        })
    }
}
