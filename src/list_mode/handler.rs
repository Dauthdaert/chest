use crate::{
    app::{App, AppResult},
    handler::AppMode,
    queries,
};
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    let AppMode::List {
        input,
        searching,
        current_commands,
    } = &mut app.app_mode else {unreachable!()};

    if *searching {
        match key_event.code {
            // Exit search on `ESC`
            KeyCode::Esc => {
                app.exit_search();
            }
            // Exit application on `Ctrl-C`
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                } else {
                    input.handle_event(&CrosstermEvent::Key(key_event));

                    // Update commands based on current search
                    let search_term = input.value().to_string();
                    *current_commands = queries::search_commands(&app.db, &search_term);
                }
            }
            // Handle all other events as input to the search box
            _ => {
                input.handle_event(&CrosstermEvent::Key(key_event));

                // Update commands based on current search
                let search_term = input.value().to_string();
                *current_commands = queries::search_commands(&app.db, &search_term);
            }
        }
    } else {
        match key_event.code {
            // Exit application on `Ctrl-C`
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }
            }
            // Enter search on 's'
            KeyCode::Char('s') => {
                app.enter_search();
            }
            // Enter add mode on 'a'
            KeyCode::Char('a') => {
                app.enter_add_mode();
            }
            // Exit app on 'q'
            KeyCode::Char('q') => {
                app.quit();
            }
            // Other handlers you could add here.
            _ => {}
        }
    }

    Ok(())
}
