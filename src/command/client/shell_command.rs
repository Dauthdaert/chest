#[derive(sqlx::FromRow, Clone)]
pub struct ShellCommand {
    pub name: String,
    pub command_text: String,
    pub description: String,
}
