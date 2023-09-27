mod client;
mod info;
mod init;
mod reset;

use clap::Subcommand;

use crate::AppResult;

#[derive(Subcommand)]
pub enum ChestCommand {
    #[command(flatten)]
    Client(client::Cmd),
    /// Print shell scripts to install Chest
    Init(init::Cmd),
    /// Resets saved commands database
    Reset,
    /// Print data and config directory
    Info,
}

impl ChestCommand {
    pub fn run(self) -> AppResult<()> {
        match self {
            ChestCommand::Init(init) => {
                init.run();
                Ok(())
            }
            ChestCommand::Client(client) => client.run(),
            ChestCommand::Reset => reset::run(),
            ChestCommand::Info => {
                info::run();
                Ok(())
            }
        }
    }
}
