use clap::Parser;
use promptly::prompt;

use crate::{
    command::client::engine::{Database, Engine},
    AppResult,
};

#[derive(Parser)]
pub struct Cmd;

impl Cmd {
    pub fn run(self) -> AppResult<()> {
        let engine = Database::init()?;

        let name = loop {
            let name: String = prompt("Enter a name for the command in Chest")?;
            if engine.get_command(&name).is_some() {
                break name;
            } else {
                println!("No command by this name.");
            }
        };

        engine.remove_command(&name)
    }
}
