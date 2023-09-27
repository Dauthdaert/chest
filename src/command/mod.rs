mod client;
mod info;
mod init;

use clap::Subcommand;

use crate::AppResult;

#[derive(Subcommand)]
pub enum ChestCommand {
    #[command(flatten)]
    Client(client::Cmd),
    /// Print shell scripts to install Chest
    Init(init::Cmd),
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
            ChestCommand::Info => {
                info::run();
                Ok(())
            }
        }
    }
}
