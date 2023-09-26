#[derive(sqlx::FromRow, Clone)]
pub struct ShellCommand {
    #[allow(dead_code)]
    pub rowid: Option<i64>,
    pub command_text: String,
    pub description: String,
}
