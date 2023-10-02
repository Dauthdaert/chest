mod add;
mod engine;
mod event;
mod remove;
mod search;
mod shell_command;
mod update;

use clap::Subcommand;

use crate::AppResult;

#[derive(Subcommand)]
pub enum Cmd {
    /// Add a new command to your Chest
    Add(add::Cmd),
    /// Update an existing command in your Chest
    Update(update::Cmd),
    /// Remove an existing command from your Chest
    Remove(remove::Cmd),
    /// Search the existing commands in your Chest
    Search(search::Cmd),
}

impl Cmd {
    pub fn run(self) -> AppResult<()> {
        match self {
            Cmd::Add(cmd) => cmd.run(),
            Cmd::Update(cmd) => cmd.run(),
            Cmd::Remove(cmd) => cmd.run(),
            Cmd::Search(cmd) => cmd.run(),
        }
    }
}
