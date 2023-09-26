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
    command_text: String,
    description: String,
}

impl Cmd {
    pub fn run(self) -> AppResult<()> {
        let engine = Database::init();
        Ok(engine.add_command(ShellCommand {
            rowid: None,
            command_text: self.command_text,
            description: self.description,
        })?)
    }
}
