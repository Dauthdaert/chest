mod command;
mod dirs;

use std::fs::File;

use clap::Parser;
use command::ChestCommand;
#[cfg(not(debug_assertions))]
use dirs::data_dir;
use dirs::log_path;
use log::LevelFilter;
use simplelog::{Config, WriteLogger};

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
        init();
        self.chest.run()
    }
}

fn init() {
    // Initialize data directory if it's missing
    #[cfg(not(debug_assertions))]
    std::fs::create_dir_all(data_dir()).expect("Unable to create data directory");

    let filter = if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Warn
    };

    WriteLogger::init(
        filter,
        Config::default(),
        File::options()
            .write(true)
            .create(true)
            .append(true)
            .open(log_path())
            .expect("Unable to open log file"),
    )
    .expect("Unable to start logger");
}

#[test]
fn verify_chest_cli() {
    use clap::CommandFactory;
    Chest::command().debug_assert()
}
