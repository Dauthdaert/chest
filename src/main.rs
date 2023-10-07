use chest_rs::{AppResult, Chest};
use clap::Parser;

fn main() -> AppResult<()> {
    Chest::parse().run()
}
