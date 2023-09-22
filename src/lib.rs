use std::{path::PathBuf, sync::OnceLock};

use directories::ProjectDirs;

/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Event handler.
pub mod handler;

/// Command type
mod command;

/// Database queries
mod queries;

/// List mode specific behavior
mod list_mode;

static PROJECT_DIR: OnceLock<ProjectDirs> = OnceLock::new();
static DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

fn project_dir() -> ProjectDirs {
    PROJECT_DIR
        .get_or_init(|| {
            ProjectDirs::from("", "", "chest").expect("Unable to obtain project directory")
        })
        .clone()
}

pub fn data_dir() -> PathBuf {
    DATA_DIR
        .get_or_init(|| project_dir().data_dir().to_path_buf())
        .clone()
}

pub fn db_path() -> PathBuf {
    data_dir().join("commands.db")
}

pub fn log_path() -> PathBuf {
    data_dir().join("log.txt")
}
