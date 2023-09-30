use tui_input::Input;

use crate::command::client::{engine::Engine, shell_command::ShellCommand};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RunStatus {
    Running,
    Confirmed,
    Canceled,
}

impl RunStatus {
    pub fn running(&self) -> bool {
        match self {
            RunStatus::Running => true,
            RunStatus::Confirmed | RunStatus::Canceled => false,
        }
    }
}

/// Application.
pub struct App<T: Engine> {
    /// Is the application running?
    pub status: RunStatus,
    pub search_box: Input,
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
            status: RunStatus::Running,
            search_box: Input::new(initial_search),
            selected: 0,
            current_commands: commands,
            db: connection,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set status to `RunState::Confirmed` to quit the application and output the selected command.
    pub fn confirm(&mut self) {
        self.status = RunStatus::Confirmed;
    }

    /// Set status to `RunState::Canceled` to quit the application without outputting the selected command.
    pub fn cancel(&mut self) {
        self.status = RunStatus::Canceled;
    }

    /// Update current commands based on search query.
    pub fn update_commands(&mut self) {
        self.current_commands = self.db.search_commands(self.search_box.value());
        self.selected = self
            .current_commands
            .len()
            .saturating_sub(1)
            .min(self.selected);
    }
}
