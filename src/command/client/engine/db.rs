use super::{
    add_command, create_database_connection, get_all_commands, get_command, get_filtered_commands,
    remove_command, update_command, Engine,
};
use crate::{command::client::shell_command::ShellCommand, AppResult};
use sqlx::SqlitePool;

pub struct Database {
    connection: SqlitePool,
}

impl Engine for Database {
    fn init() -> AppResult<Self> {
        Ok(Self {
            connection: create_database_connection()?,
        })
    }

    fn search_commands(&self, search_term: &str) -> Vec<ShellCommand> {
        if search_term.len() > 1 {
            get_filtered_commands(&self.connection, search_term)
        } else {
            get_all_commands(&self.connection)
        }
    }

    fn get_command(&self, name: &str) -> Option<ShellCommand> {
        get_command(&self.connection, name)
    }

    fn add_command(&self, command: ShellCommand) -> AppResult<()> {
        add_command(&self.connection, command)
    }

    fn update_command(&self, command: ShellCommand) -> AppResult<()> {
        update_command(&self.connection, command)
    }

    fn remove_command(&self, name: &str) -> AppResult<()> {
        remove_command(&self.connection, name)
    }
}
