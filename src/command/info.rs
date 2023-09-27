use crate::dirs;

pub fn run() {
    let db_path = dirs::db_path();
    let log_path = dirs::log_path();
    println!("Paths to important files:");
    println!("Database containing commands: {}", db_path.display());
    println!("Log file: {}", log_path.display());
}
