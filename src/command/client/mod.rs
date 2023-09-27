mod add;
mod engine;
mod event;
mod search;
mod shell_command;

use clap::Subcommand;

use crate::AppResult;

#[derive(Subcommand)]
pub enum Cmd {
    /// Add a new command to your Chest
    Add(add::Cmd),
    /// Search the existing commands in your Chest
    Search(search::Cmd),
}

impl Cmd {
    pub fn run(self) -> AppResult<()> {
        match self {
            Cmd::Add(cmd) => cmd.run(),
            Cmd::Search(cmd) => cmd.run(),
        }
    }
}
