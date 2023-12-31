use std::{path::PathBuf, sync::OnceLock};

use directories::ProjectDirs;

static PROJECT_DIR: OnceLock<ProjectDirs> = OnceLock::new();
static DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

fn project_dir() -> ProjectDirs {
    PROJECT_DIR
        .get_or_init(|| {
            ProjectDirs::from("", "", "chest").expect("Unable to obtain project directory")
        })
        .clone()
}

#[allow(dead_code)]
pub fn data_dir() -> PathBuf {
    DATA_DIR
        .get_or_init(|| project_dir().data_dir().to_path_buf())
        .clone()
}

pub fn db_path() -> PathBuf {
    #[cfg(debug_assertions)]
    return PathBuf::new().join("commands.db");

    #[cfg(not(debug_assertions))]
    return data_dir().join("commands.db");
}

pub fn log_path() -> PathBuf {
    #[cfg(debug_assertions)]
    return PathBuf::new().join("log.txt");

    #[cfg(not(debug_assertions))]
    return data_dir().join("log.txt");
}
