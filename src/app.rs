use async_std::task;
use log::{debug, error};
use sqlx::{
    migrate,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use tui_input::Input;

use crate::{db_path, handler::AppMode, queries};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub app_mode: AppMode,
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
        let connection = task::block_on(async {
            let options = SqliteConnectOptions::new()
                .filename(db_path())
                .create_if_missing(true)
                .optimize_on_close(true, None);
            let connection = SqlitePoolOptions::new().connect_with(options).await;
            let connection = match connection {
                Ok(db) => db,
                Err(error) => {
                    error!(
                        "Unable to open or create database at path : {}",
                        db_path().display()
                    );
                    debug!("{}", error);
                    panic!("Unable to open or create database");
                }
            };

            if let Err(error) = migrate!().run(&connection).await {
                error!("Unable to migrate database");
                debug!("{}", error);
                panic!("Unable to migrate database");
            }

            connection
        });

        let commands = queries::search_commands(&connection, "");
        Self {
            running: true,
            app_mode: AppMode::List {
                input: Input::default(),
                searching: false,
                current_commands: commands,
            },
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
        if let AppMode::List { searching, .. } = &mut self.app_mode {
            *searching = true;
        }
    }

    pub fn exit_search(&mut self) {
        if let AppMode::List { searching, .. } = &mut self.app_mode {
            *searching = false;
        }
    }

    pub fn enter_add_mode(&mut self) {
        self.app_mode = AppMode::Add;
    }

    pub fn enter_list_mode(&mut self) {
        let commands = queries::search_commands(&self.db, "");
        self.app_mode = AppMode::List {
            input: Input::default(),
            searching: false,
            current_commands: commands,
        };
    }
}
