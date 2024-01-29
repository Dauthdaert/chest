use anyhow::Result;
use clap::Parser;

use chest_rs::Chest;

fn main() -> Result<()> {
    Chest::parse().run()
}
