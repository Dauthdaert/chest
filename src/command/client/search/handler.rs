use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use log::debug;
use tui_input::backend::crossterm::EventHandler;

use crate::{command::client::engine::Engine, AppResult};

use super::app::App;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events<T: Engine>(key_event: KeyEvent, app: &mut App<T>) -> AppResult<()> {
    // Don't process key events that aren't presses
    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    if app.searching {
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
                    app.search_box.handle_event(&CrosstermEvent::Key(key_event));

                    // Update commands based on current search
                    app.update_commands();
                }
            }
            // Handle all other events as input to the search box
            _ => {
                app.search_box.handle_event(&CrosstermEvent::Key(key_event));

                // Update commands based on current search
                app.update_commands();
            }
        }
    } else {
        match key_event.code {
            // Exit application on `ESC`
            KeyCode::Esc => {
                app.quit();
            }
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
            // Output command on 'Enter'
            KeyCode::Enter => {
                app.quit();
            }
            KeyCode::Up => {
                app.selected = 0.max(app.selected.saturating_sub(1));
                debug!("selected idx : {}", app.selected);
            }
            KeyCode::Down => {
                app.selected = app
                    .current_commands
                    .len()
                    .saturating_sub(1)
                    .min(app.selected.saturating_add(1));
                debug!("selected idx : {}", app.selected);
            }
            // Other handlers you could add here.
            _ => {}
        }
    }

    Ok(())
}
