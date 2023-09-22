use chest::app::{App, AppResult};
use chest::event::{Event, EventHandler};
use chest::handler::handle_key_events;
use chest::tui::Tui;
use chest::{data_dir, log_path};
use log::LevelFilter;
use simplelog::{Config, WriteLogger};
use std::fs::File;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> AppResult<()> {
    // Initialize data directory if it's missing
    std::fs::create_dir_all(data_dir()).expect("Unable to create data directory");

    let filter = if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Warn
    };

    WriteLogger::init(
        filter,
        Config::default(),
        File::options()
            .write(true)
            .create(true)
            .append(true)
            .open(log_path())
            .expect("Unable to open log file"),
    )
    .expect("Unable to start logger");

    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
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
    Tui::<CrosstermBackend<io::Stderr>>::reset()?;
    Ok(())
}
