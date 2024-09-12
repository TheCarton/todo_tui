use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{
        calendar::{CalendarEventStore, Monthly},
        Block, Borders, Padding, Paragraph,
    },
    Frame,
};
use time::OffsetDateTime;

use crate::app::{App, CurrentlyEditing};

pub fn ui(frame: &mut Frame, app: &App) {
    let app_chunks = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());
    let main_screen_chunk = app_chunks[0];
    let edit_screen_chunk = app_chunks[1];
    if let Some(active_task) = &app.current_task {
        frame.render_widget(active_task.clone(), main_screen_chunk);
    }
    match app.current_screen {
        crate::app::CurrentScreen::Main => {}
        crate::app::CurrentScreen::Editing => {
            if let Some(editing) = &app.currently_editing {
                let title_text = match &app.edit_mode {
                    crate::app::EditMode::Active => "edit current task",
                    crate::app::EditMode::CreateNew => "enter a new task",
                };
                let edit_block = Block::default()
                    .title(title_text)
                    .borders(Borders::NONE)
                    .style(Style::default().bg(Color::DarkGray));

                frame.render_widget(edit_block, edit_screen_chunk);

                let edit_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([
                        Constraint::Percentage(40),
                        Constraint::Percentage(30),
                        Constraint::Percentage(30),
                    ])
                    .split(edit_screen_chunk);
                let mut title_block = Block::default().title("Title").borders(Borders::ALL);
                let mut description_block =
                    Block::default().title("Description").borders(Borders::ALL);

                let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

                match editing {
                    CurrentlyEditing::Title => title_block = title_block.style(active_style),
                    CurrentlyEditing::Description => {
                        description_block = description_block.style(active_style)
                    }
                };

                let task_text = Paragraph::new(app.title_input.clone()).block(title_block);
                frame.render_widget(task_text, edit_chunks[0]);

                let description_text =
                    Paragraph::new(app.description_input.clone()).block(description_block);
                frame.render_widget(description_text, edit_chunks[1]);

                let mut calendar_event_store = CalendarEventStore::today(Style::new().red().bold());
                if let Some(active_task) = &app.current_task {
                    calendar_event_store
                        .add(active_task.due_time.date(), Style::new().blue().bold());
                }

                let todays_date = OffsetDateTime::now_utc().date();
                let calendar = Monthly::new(todays_date, calendar_event_store)
                    .block(Block::new().padding(Padding::new(0, 0, 2, 0)))
                    .show_month_header(Style::new().bold())
                    .show_weekdays_header(Style::new().italic());
                frame.render_widget(calendar, edit_chunks[2]);
            }
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
