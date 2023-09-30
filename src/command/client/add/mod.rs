use clap::Parser;

use crate::{
    command::client::{
        engine::{Database, Engine},
        shell_command::ShellCommand,
    },
    AppResult,
};

#[derive(Parser)]
pub struct Cmd {
    /// Name of the command in Chest
    name: String,
    /// Full command
    command_text: String,
    /// Description of the command
    description: String,
}

impl Cmd {
    pub fn run(self) -> AppResult<()> {
        let engine = Database::init();
        Ok(engine.add_command(ShellCommand {
            name: self.name,
            command_text: self.command_text,
            description: self.description,
        })?)
    }
}
