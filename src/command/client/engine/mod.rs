use crate::{dirs::db_path, AppResult};

use super::shell_command::ShellCommand;

mod db;

use async_std::task;
pub use db::Database;
use log::{debug, error};
use sqlx::{
    migrate,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Error, SqlitePool,
};

pub trait Engine: Sized {
    fn init() -> AppResult<Self>;
    fn search_commands(&self, search_term: &str) -> Vec<ShellCommand>;
    fn search_commands_strict(&self, name: &str) -> Option<ShellCommand>;
    fn get_command(&self, name: &str) -> Option<ShellCommand>;
    fn add_command(&self, command: ShellCommand) -> AppResult<()>;
    fn update_command(&self, command: ShellCommand) -> AppResult<()>;
    fn remove_command(&self, name: &str) -> AppResult<()>;
}

fn create_database_connection() -> Result<SqlitePool, Error> {
    task::block_on(async {
        let options = SqliteConnectOptions::new()
            .filename(db_path())
            .create_if_missing(true)
            .optimize_on_close(true, None);
        let connection = SqlitePoolOptions::new()
            .connect_with(options)
            .await
            .map_err(|error| {
                error!(
                    "Unable to open or create database at path : {}",
                    db_path().display()
                );
                debug!("{}", error);
                error
            })?;

        migrate!().run(&connection).await.map_err(|error| {
            error!("Unable to migrate database");
            debug!("{}", error);
            error
        })?;

        Ok(connection)
    })
}

fn get_all_commands(db: &SqlitePool) -> Vec<ShellCommand> {
    task::block_on(async {
        sqlx::query_as::<_, ShellCommand>("SELECT * FROM Commands")
            .fetch_all(db)
            .await
            .unwrap_or_else(|error| {
                error!("Error while searching for commands without search_term",);
                debug!("{}", error);
                Vec::new()
            })
    })
}

fn get_filtered_commands(db: &SqlitePool, search_term: &str) -> Vec<ShellCommand> {
    task::block_on(async {
        sqlx::query_as::<_, ShellCommand>(
            "SELECT * FROM Commands WHERE Commands MATCH $1 ORDER BY rank",
        )
        .bind(search_term)
        .fetch_all(db)
        .await
        .unwrap_or_else(|error| {
            error!(
                "Error while searching for commands with search_term {}",
                search_term
            );
            debug!("{}", error);
            Vec::new()
        })
    })
}

fn get_filtered_commands_name(db: &SqlitePool, name: &str) -> Option<ShellCommand> {
    task::block_on(async {
        sqlx::query_as::<_, ShellCommand>(
            "SELECT * FROM Commands WHERE name MATCH $1 ORDER BY rank",
        )
        .bind(name)
        .fetch_optional(db)
        .await
        .unwrap_or_else(|error| {
            error!("Error while searching for commands with name {}", name);
            debug!("{}", error);
            None
        })
    })
}

fn get_command(db: &SqlitePool, name: &str) -> Option<ShellCommand> {
    task::block_on(async {
        sqlx::query_as::<_, ShellCommand>("SELECT * FROM Commands WHERE name = $1")
            .bind(name)
            .fetch_optional(db)
            .await
            .unwrap_or_else(|error| {
                error!("Error while searching for a command with name {}", name);
                debug!("{}", error);
                None
            })
    })
}

fn add_command(db: &SqlitePool, command: ShellCommand) -> AppResult<()> {
    Ok(task::block_on(async {
        sqlx::query("INSERT INTO Commands (name, command_text, description) VALUES ($1, $2, $3);")
            .bind(command.name)
            .bind(command.command_text)
            .bind(command.description)
            .execute(db)
            .await
            .map(|_ok| ())
    })?)
}

fn update_command(db: &SqlitePool, command: ShellCommand) -> AppResult<()> {
    Ok(task::block_on(async {
        sqlx::query("UPDATE Commands SET command_text=$2, description=$3 WHERE name=$1;")
            .bind(command.name)
            .bind(command.command_text)
            .bind(command.description)
            .execute(db)
            .await
            .map(|_ok| ())
    })?)
}

fn remove_command(db: &SqlitePool, name: &str) -> AppResult<()> {
    Ok(task::block_on(async {
        sqlx::query("DELETE FROM Commands WHERE name = $1")
            .bind(name)
            .execute(db)
            .await
            .map(|_ok| ())
    })?)
}
