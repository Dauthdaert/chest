use crate::dirs::db_path;

use super::shell_command::ShellCommand;

mod db;

use async_std::task;
pub use db::Database;
use log::{debug, error};
use sqlx::{
    migrate,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};

pub trait Engine {
    fn init() -> Self;
    fn search_commands(&self, search_term: &str) -> Vec<ShellCommand>;
    fn add_command(&self, command: ShellCommand) -> Result<(), sqlx::Error>;
}

fn create_database_connection() -> SqlitePool {
    task::block_on(async {
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
    })
}

fn get_all_commands(db: &SqlitePool) -> Vec<ShellCommand> {
    task::block_on(async {
        sqlx::query_as::<_, ShellCommand>("SELECT rowid, * FROM Commands")
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
            "SELECT rowid, * FROM Commands WHERE Commands MATCH $1 ORDER BY rank",
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

#[allow(dead_code)]
fn add_command(db: &SqlitePool, command: ShellCommand) -> Result<(), sqlx::Error> {
    task::block_on(async {
        sqlx::query("insert into Commands (command_text, description) values ($1, $2);")
            .bind(command.command_text)
            .bind(command.description)
            .execute(db)
            .await
    })
    .map(|_res| ())
}
