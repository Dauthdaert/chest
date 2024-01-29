use anyhow::Result;
use std::fs;

use crate::dirs::db_path;

pub fn run() -> Result<()> {
    // Remove db file if it exists
    let db_path = db_path();
    Ok(fs::remove_file(db_path)?)
}
