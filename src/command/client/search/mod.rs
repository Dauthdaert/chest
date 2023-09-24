use std::io;

use ::tui::prelude::*;
use clap::Parser;

use crate::AppResult;

use self::{app::App, handler::handle_key_events, tui::Tui};

use super::{
    db,
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
    query: Vec<String>,
}

impl Cmd {
    pub fn run(self) -> AppResult<()> {
        if self.interactive {
            let command = interactive(self.query)?;
            if let Some(command) = command {
                eprintln!("{}", command.command_text);
            }
        } else {
            let command = non_interactive(self.query)?;
            if let Some(command) = command {
                println!("{} : {}", command.command_text, command.description);
            }
        };

        Ok(())
    }
}

fn interactive(query: Vec<String>) -> AppResult<Option<ShellCommand>> {
    // Create an application.
    let mut app = App::new(query.join(" "));

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    Tui::<CrosstermBackend<io::Stdout>>::reset()?;

    // Return the selected command
    // Vec::get handles out of bounds access if the Vec is empty
    Ok(app.current_commands.get(app.selected).cloned())
}

fn non_interactive(query: Vec<String>) -> AppResult<Option<ShellCommand>> {
    let db = db::init();
    let commands = db::search_commands(&db, &query.join(" "));

    // Return the first matching command
    // Vec::first returns None if the Vec is empty
    Ok(commands.first().cloned())
}
