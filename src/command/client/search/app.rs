use sqlx::SqlitePool;
use tui_input::Input;

use crate::command::client::{
    db::{self, search_commands},
    shell_command::ShellCommand,
};

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub search_box: Input,
    pub searching: bool,
    pub selected: usize,
    pub current_commands: Vec<ShellCommand>,
    pub db: SqlitePool,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let connection = db::init();
        let commands = search_commands(&connection, "");
        Self {
            running: true,
            search_box: Input::default(),
            searching: false,
            selected: 0,
            current_commands: commands,
            db: connection,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn enter_search(&mut self) {
        self.searching = true;
    }

    pub fn exit_search(&mut self) {
        self.searching = false;
        self.selected = 0;
    }
}
