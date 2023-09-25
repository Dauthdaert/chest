use super::{
    add_command, create_database_connection, get_all_commands, get_filtered_commands, Engine,
};
use crate::command::client::shell_command::ShellCommand;
use sqlx::SqlitePool;

pub struct Database {
    connection: SqlitePool,
}

impl Engine for Database {
    fn init() -> Self {
        Self {
            connection: create_database_connection(),
        }
    }

    fn search_commands(&self, search_term: &str) -> Vec<ShellCommand> {
        if search_term.len() > 1 {
            get_filtered_commands(&self.connection, search_term)
        } else {
            get_all_commands(&self.connection)
        }
    }

    #[allow(dead_code)]
    fn add_command(&self, command: ShellCommand) -> Result<(), sqlx::Error> {
        add_command(&self.connection, command)
    }
}
