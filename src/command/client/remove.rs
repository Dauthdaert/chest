use anyhow::Result;
use clap::Parser;
use promptly::prompt;

use crate::command::client::engine::{Database, Engine};

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
            None => prompt("Enter the command's name")?,
        };
        if engine.get_command(&name).is_some() {
            engine.remove_command(&name)?;
            println!("Successfully removed command '{}'", name);
            Ok(())
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
