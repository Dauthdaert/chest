use tui::{
    prelude::*,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::command::client::{engine::Engine, search::app::App};

/// Renders the user interface widgets.
pub fn render<T: Engine>(app: &App<T>, frame: &mut Frame<'_>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(frame.size());

    help_text(chunks[0], frame);
    search_box(chunks[1], app, frame);
    command_list(chunks[2], app, frame);
}

fn command_list<T: Engine>(chunk: Rect, app: &App<T>, frame: &mut Frame) {
    let [list_chunk, command_chunk] = *Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(35), Constraint::Min(1)].as_ref())
        .split(chunk)
    else {
        unreachable!("Failed to split command_list chunks")
    };
    let [text_chunk, description_chunk] = *Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(command_chunk)
    else {
        unreachable!()
    };

    let commands = List::new(
        app.current_commands
            .iter()
            .enumerate()
            .map(|(i, command)| {
                let style = if app.selected == i {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                };
                ListItem::new(command.name.to_string()).style(style)
            })
            .collect::<Vec<ListItem>>(),
    )
    .block(Block::default().borders(Borders::ALL).title("Commands"));
    frame.render_widget(commands, list_chunk);

    let selected_command = app.current_commands.get(app.selected);
    let command_text = Paragraph::new(selected_command.map_or("", |command| &command.command_text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Stored command"),
        );
    frame.render_widget(command_text, text_chunk);

    let description = Paragraph::new(selected_command.map_or("", |command| &command.description))
        .block(Block::default().borders(Borders::ALL).title("Description"));
    frame.render_widget(description, description_chunk);
}

fn help_text(chunk: Rect, frame: &mut Frame) {
    let (msg, style) = (
        vec![
            Span::raw("Press "),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit, "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to confirm selection."),
        ],
        Style::default().add_modifier(Modifier::RAPID_BLINK),
    );
    let mut text = Text::from(Line::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    frame.render_widget(help_message, chunk);
}

fn search_box<T: Engine>(chunk: Rect, app: &App<T>, frame: &mut Frame) {
    // keep 2 for borders and 1 for cursor
    let width = chunk.width.max(3) - 3;

    let scroll = app.search_box.visual_scroll(width as usize);
    let input = Paragraph::new(app.search_box.value())
        .style(Style::default())
        .scroll((0, scroll as u16))
        .block(Block::default().borders(Borders::ALL).title("Chest"));
    frame.render_widget(input, chunk);

    // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
    frame.set_cursor(
        // Put cursor past the end of the input text
        chunk.x + ((app.search_box.visual_cursor()).max(scroll) - scroll) as u16 + 1,
        // Move one line down, from the border to the input line
        chunk.y + 1,
    );
}
