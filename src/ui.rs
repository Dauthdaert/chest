use tui::{backend::Backend, Frame};

use crate::{app::App, handler::AppMode, list_mode};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    match app.app_mode {
        AppMode::List { .. } => list_mode::render(app, frame),
        AppMode::Add => todo!(),
    }
}
