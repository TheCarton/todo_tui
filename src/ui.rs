use std::iter::zip;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentlyEditing, Task};

pub fn ui(frame: &mut Frame, app: &App) {
    match app.current_screen {
        crate::app::CurrentScreen::Main => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ])
                .split(frame.area());

            let (middle_text, bottom_text) = if let Some(t) = &app.current_task {
                let bottom = if let Some(d) = &t.description {
                    d.clone()
                } else {
                    "no description".to_string()
                };

                (t.title.clone(), bottom)
            } else {
                ("no tasks!".to_string(), "no tasks 2!".to_string())
            };
            let title = Paragraph::new("To-Do Tui");
            let middle = Paragraph::new(middle_text);
            let bottom = Paragraph::new(bottom_text);

            for (w, a) in zip([title, middle, bottom], chunks.iter()) {
                frame.render_widget(w, *a);
            }
        }
        crate::app::CurrentScreen::Editing => {
            if let Some(editing) = &app.currently_editing {
                let popup_block = Block::default()
                    .title("Enter a new task")
                    .borders(Borders::NONE)
                    .style(Style::default().bg(Color::DarkGray));

                let area = centered_rect(60, 25, frame.area());
                frame.render_widget(popup_block, area);

                let popup_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                    .split(area);
                let mut key_block = Block::default().title("Title").borders(Borders::ALL);
                let mut value_block = Block::default().title("Description").borders(Borders::ALL);

                let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

                match editing {
                    CurrentlyEditing::Title => key_block = key_block.style(active_style),
                    CurrentlyEditing::Description => value_block = value_block.style(active_style),
                };

                let key_text = Paragraph::new(app.title_input.clone()).block(key_block);
                frame.render_widget(key_text, popup_chunks[0]);

                let value_text = Paragraph::new(app.description_input.clone()).block(value_block);
                frame.render_widget(value_text, popup_chunks[1]);
            }
        }
        crate::app::CurrentScreen::Exiting => {
            frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
            let popup_block = Block::default()
                .title("Quit")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            let exit_text = Text::styled(
                "Are you sure you want to quit? (y/n)",
                Style::default().fg(Color::Red),
            );
            // the `trim: false` will stop the text from being cut off when over the edge of the block
            let exit_paragraph = Paragraph::new(exit_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });

            let area = centered_rect(60, 25, frame.area());
            frame.render_widget(exit_paragraph, area);
        }
    };
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
