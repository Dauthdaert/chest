mod command;
mod dirs;

use clap::Parser;
use command::ChestCommand;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Parser)]
#[command(author = "Wayan-Gwie Lapointe", version = VERSION)]
pub struct Chest {
    #[command(subcommand)]
    chest: ChestCommand,
}

impl Chest {
    pub fn run(self) -> AppResult<()> {
        self.chest.run()
    }
}
