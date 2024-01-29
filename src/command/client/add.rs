use anyhow::Result;
use clap::Parser;
use promptly::{prompt, prompt_opt};

use crate::command::client::{
    engine::{Database, Engine},
    shell_command::ShellCommand,
};

#[derive(Parser)]
pub struct Cmd {
    /// Name of the command
    name: Option<String>,
}

impl Cmd {
    pub fn run(self) -> Result<()> {
        let engine = Database::init()?;

        let name = match self.name {
            Some(name) => name,
            None => prompt("Enter a name for the command in Chest")?,
        };
        if engine.get_command(&name).is_none() {
            let command_text = prompt("Enter the command text")?;
            let description =
                prompt_opt("Enter the description (optional)")?.unwrap_or("".to_string());

            engine.add_command(ShellCommand {
                name,
                command_text,
                description,
            })
        } else {
            println!("Name '{}' is already taken.", name);
            Ok(())
        }
    }
}
