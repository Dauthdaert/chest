use async_std::task;
use log::{debug, error};
use sqlx::{Result, SqlitePool};

use crate::command::Command;

pub fn search_commands(db: &SqlitePool, search_term: &str) -> Vec<Command> {
    task::block_on(async {
        if search_term.len() > 1 {
            sqlx::query_as::<_, Command>(
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
        } else {
            sqlx::query_as::<_, Command>("SELECT rowid, * FROM Commands")
                .bind(search_term)
                .fetch_all(db)
                .await
                .unwrap_or_else(|error| {
                    error!("Error while searching for commands without search_term",);
                    debug!("{}", error);
                    Vec::new()
                })
        }
    })
}

pub fn add_command(db: &SqlitePool, command: Command) -> Result<(), sqlx::Error> {
    task::block_on(async {
        sqlx::query("insert into Commands (command_text, description) values ($1, $2);")
            .bind(command.command_text)
            .bind(command.description)
            .execute(db)
            .await
    })
    .map(|_res| ())
}
