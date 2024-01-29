mod command;
mod dirs;

use std::fs::File;

use anyhow::{Context, Result};
use clap::Parser;
use command::ChestCommand;
#[cfg(not(debug_assertions))]
use dirs::data_dir;
use dirs::log_path;
use log::LevelFilter;
use simplelog::{Config, WriteLogger};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(author = "Wayan-Gwie Lapointe", version = VERSION)]
pub struct Chest {
    #[command(subcommand)]
    chest: ChestCommand,
}

impl Chest {
    pub fn run(self) -> Result<()> {
        init()?;
        self.chest.run()
    }
}

fn init() -> Result<()> {
    // Initialize data directory if it's missing
    #[cfg(not(debug_assertions))]
    std::fs::create_dir_all(data_dir()).context("Unable to create data directory")?;

    let filter = if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Warn
    };

    WriteLogger::init(
        filter,
        Config::default(),
        File::options()
            .create(true)
            .append(true)
            .open(log_path())
            .context("Unable to open log file")?,
    )
    .context("Unable to start logger")?;

    Ok(())
}

#[test]
fn verify_chest_cli() {
    use clap::CommandFactory;
    Chest::command().debug_assert()
}
