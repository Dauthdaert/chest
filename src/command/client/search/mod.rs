use std::{borrow::Cow, error::Error, fmt::Display, io};

use ::tui::prelude::*;
use anyhow::Result;
use clap::Parser;
use promptly::prompt;
use regex::Regex;

use self::{
    app::{App, RunStatus},
    handler::handle_key_events,
    tui::Tui,
};

use super::{
    engine::{Database, Engine},
    event::{Event, EventHandler},
    shell_command::ShellCommand,
};

mod app;
mod handler;
mod tui;
mod ui;

#[derive(Parser)]
pub struct Cmd {
    /// Open interactive search UI
    #[arg(long, short)]
    interactive: bool,
    /// Search query
    query: Vec<String>,
}

impl Cmd {
    pub fn run(self) -> Result<()> {
        if self.interactive {
            match interactive(self.query) {
                Ok(command) => {
                    let mut command_text = Cow::from(command.command_text);

                    let expansion_regex = Regex::new("#")?;
                    while expansion_regex.is_match(&command_text) {
                        eprintln!("Current command: {}", &command_text);
                        let expansion: String = prompt("Expand next placeholder into")?;
                        if let Cow::Owned(new) = expansion_regex.replace(&command_text, &expansion)
                        {
                            command_text = Cow::Owned(new);
                        }
                    }
                    println!("{}", command_text);
                }
                Err(e) => match e.downcast_ref::<CommandFetchFailed>() {
                    // If we failed to fetch a command, we silently fail and return ok.
                    Some(_) => return Ok(()),
                    None => return Err(e),
                },
            }
        } else {
            let commands = non_interactive(self.query)?;
            if commands.is_empty() {
                println!("No matches");
            } else {
                println!("Closest matches:");
                for command in commands {
                    println!("{} : {}", command.command_text, command.description);
                }
            }
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct CommandFetchFailed;

impl Error for CommandFetchFailed {}
impl Display for CommandFetchFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No command was found")
    }
}

fn interactive(query: Vec<String>) -> Result<ShellCommand> {
    // Create an application.
    let mut app: App<Database> = App::new(query.join(" "));

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.status.running() {
        // Render the user interface.
        tui.draw(&app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    Tui::<CrosstermBackend<io::Stderr>>::reset()?;

    // Return the selected command if the selection was confirmed
    // Vec::get handles out of bounds access if the Vec is empty
    if app.status == RunStatus::Confirmed {
        if let Some(command) = app.current_commands.get(app.selected) {
            Ok(command.clone())
        } else {
            Err(CommandFetchFailed.into())
        }
    } else {
        Err(CommandFetchFailed.into())
    }
}

fn non_interactive(query: Vec<String>) -> Result<Vec<ShellCommand>> {
    let engine = Database::init()?;
    let mut commands = engine.search_commands(&query.join(" "));

    // Keep the 5 best matches
    commands.truncate(5);

    // Return the first matching command
    Ok(commands)
}
