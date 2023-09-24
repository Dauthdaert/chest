use tui::{
    backend::Backend,
    prelude::*,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::command::client::search::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
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

    let (msg, style) = if app.searching {
        (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop searching"),
            ],
            Style::default(),
        )
    } else {
        (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("s", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start searching."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        )
    };
    let mut text = Text::from(Line::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    frame.render_widget(help_message, chunks[0]);

    let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor

    let scroll = app.search_box.visual_scroll(width as usize);
    let input = Paragraph::new(app.search_box.value())
        .style(if app.searching {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        })
        .scroll((0, scroll as u16))
        .block(Block::default().borders(Borders::ALL).title("Search"));
    frame.render_widget(input, chunks[1]);
    if app.searching {
        // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
        frame.set_cursor(
            // Put cursor past the end of the input text
            chunks[1].x + ((app.search_box.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            // Move one line down, from the border to the input line
            chunks[1].y + 1,
        )
    }

    let messages: Vec<ListItem> = app
        .current_commands
        .iter()
        .enumerate()
        .map(|(i, command)| {
            ListItem::new(format!(
                "{} : {}",
                command.command_text, command.description
            ))
            .style(if app.selected == i {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            })
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Commands"));
    frame.render_widget(messages, chunks[2]);
}
