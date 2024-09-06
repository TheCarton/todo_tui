use std::iter::zip;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Paragraph, Widget},
};
use time::OffsetDateTime;

#[derive(Clone)]
pub struct Task {
    pub(crate) title: String,
    pub(crate) description: Option<String>,
    pub(crate) task_status: TaskStatus,
    pub(crate) time_added: OffsetDateTime,
    pub(crate) time_edited: OffsetDateTime,
}

#[derive(Clone, Copy)]
pub enum TaskStatus {
    InProgress,
    Finished,
}

impl Widget for Task {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let app_header = Paragraph::new("To-Do Tui");
        let title = Paragraph::new(self.title.clone());
        let date_added = Paragraph::new(format!(
            "added: {}:{}:{} {}",
            self.time_added.hour(),
            self.time_added.minute(),
            self.time_added.second(),
            self.time_added.date()
        ));
        let date_edited = Paragraph::new(format!(
            "edited: {}:{}:{} {}",
            self.time_edited.hour(),
            self.time_edited.minute(),
            self.time_edited.second(),
            self.time_edited.date()
        ));
        if let Some(description) = self.description {
            let description = Paragraph::new(description.clone());
            let chunks = Layout::new(
                Direction::Horizontal,
                [
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ],
            )
            .split(area);

            for (w, a) in zip(
                [app_header, title, description, date_added, date_edited],
                chunks.iter(),
            ) {
                w.render(area, buf);
            }
        } else {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ])
                .split(area);

            for (w, a) in zip([app_header, title, date_added, date_edited], chunks.iter()) {
                w.render(area, buf);
            }
        }
    }
}
