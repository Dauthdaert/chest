use crate::{
    app::{App, AppResult},
    command::Command,
    list_mode,
};
use crossterm::event::KeyEvent;
use tui_input::Input;

pub enum AppMode {
    List {
        input: Input,
        searching: bool,
        current_commands: Vec<Command>,
    },
    Add,
}

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match &app.app_mode {
        AppMode::List { .. } => list_mode::handle_key_events(key_event, app),
        AppMode::Add => todo!(),
    }?;
    Ok(())
}
