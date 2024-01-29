use super::{
    add_command, create_database_connection, get_all_commands, get_command, get_filtered_commands,
    get_filtered_commands_name, remove_command, update_command, Engine,
};
use crate::command::client::shell_command::ShellCommand;
use anyhow::Result;
use sqlx::SqlitePool;

pub struct Database {
    connection: SqlitePool,
}

impl Engine for Database {
    fn init() -> Result<Self> {
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

    fn search_commands_strict(&self, name: &str) -> Option<ShellCommand> {
        if name.len() > 1 {
            get_filtered_commands_name(&self.connection, name)
        } else {
            get_all_commands(&self.connection).first().cloned()
        }
    }

    fn get_command(&self, name: &str) -> Option<ShellCommand> {
        get_command(&self.connection, name)
    }

    fn add_command(&self, command: ShellCommand) -> Result<()> {
        add_command(&self.connection, command)
    }

    fn update_command(&self, command: ShellCommand) -> Result<()> {
        update_command(&self.connection, command)
    }

    fn remove_command(&self, name: &str) -> Result<()> {
        remove_command(&self.connection, name)
    }
}
