use tui_input::Input;

use crate::command::client::{engine::Engine, shell_command::ShellCommand};

/// Application.
pub struct App<T: Engine> {
    /// Is the application running?
    pub running: bool,
    pub search_box: Input,
    pub searching: bool,
    pub selected: usize,
    pub current_commands: Vec<ShellCommand>,
    pub db: T,
}

impl<T: Engine> App<T> {
    /// Constructs a new instance of [`App`].
    pub fn new(initial_search: String) -> Self {
        let connection = T::init();
        let commands = connection.search_commands(&initial_search);
        Self {
            running: true,
            search_box: Input::new(initial_search),
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

    pub fn update_commands(&mut self) {
        self.current_commands = self.db.search_commands(self.search_box.value());
    }
}
