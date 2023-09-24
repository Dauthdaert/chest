mod add;
mod db;
mod event;
mod search;
mod shell_command;

use std::fs::File;

use clap::Subcommand;
use log::LevelFilter;
use simplelog::{Config, WriteLogger};

use crate::{
    dirs::{data_dir, log_path},
    AppResult,
};

#[derive(Subcommand)]
pub enum Cmd {
    /// Add a new command to your Chest
    Add,
    /// Search the existing commands in your Chest
    Search,
}

impl Cmd {
    pub fn run(self) -> AppResult<()> {
        // Initialize data directory if it's missing
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

        match self {
            Cmd::Add => add::run(),
            Cmd::Search => search::run(),
        }
    }
}
