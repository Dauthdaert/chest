use std::io;

use ::tui::prelude::*;

use crate::AppResult;

use self::{app::App, handler::handle_key_events, tui::Tui};

use super::event::{Event, EventHandler};

mod app;
mod handler;
mod tui;
mod ui;

pub fn run() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

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

    // TODO: Output the chosen command
    if !app.current_commands.is_empty() {
        eprintln!(
            "{}",
            app.current_commands
                .get(app.selected)
                .expect("Failed to get selected command")
                .command_text
        );
    }

    Ok(())
}
