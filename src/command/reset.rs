use std::fs;

use crate::{dirs::db_path, AppResult};

pub fn run() -> AppResult<()> {
    // Remove db file if it exists
    let db_path = db_path();
    Ok(fs::remove_file(db_path)?)
}
