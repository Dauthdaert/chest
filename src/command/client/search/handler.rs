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

    match key_event.code {
        // Exit search on `ESC`
        KeyCode::Esc => {
            app.cancel();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.cancel();
            } else {
                app.search_box.handle_event(&CrosstermEvent::Key(key_event));

                // Update commands based on current search
                app.update_commands();
            }
        }
        // Output command on 'Enter'
        KeyCode::Enter => {
            app.confirm();
        }
        KeyCode::Up => {
            app.selected = app
                .selected
                .checked_sub(1)
                .unwrap_or_else(|| app.current_commands.len().saturating_sub(1));
            debug!("selected idx : {}", app.selected);
        }
        KeyCode::Down => {
            app.selected = if app.selected < app.current_commands.len().saturating_sub(1) {
                app.selected.saturating_add(1)
            } else {
                0
            };
            debug!("selected idx : {}", app.selected);
        }
        // Handle all other events as input to the search box
        _ => {
            app.search_box.handle_event(&CrosstermEvent::Key(key_event));

            // Update commands based on current search
            app.update_commands();
        }
    }

    Ok(())
}
