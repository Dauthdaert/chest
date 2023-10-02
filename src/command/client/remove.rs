use clap::Parser;
use promptly::prompt;

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
        if engine.get_command(&name).is_some() {
            engine.remove_command(&name)
        } else {
            if let Some(alt_command) = engine.search_commands_strict(&name) {
                println!(
                    "No command by this name. Did you mean {}?",
                    alt_command.name
                );
            } else {
                println!("No command by this name.");
            }
            Ok(())
        }
    }
}
