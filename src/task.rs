use std::iter::zip;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Paragraph, Widget},
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
        let date_added = Paragraph::new(format!(
            "{}:{}:{} {}",
            self.time_added.hour(),
            self.time_added.minute(),
            self.time_added.second(),
            self.time_added.date()
        ))
        .block(Block::bordered().title("Added"));
        let date_edited = Paragraph::new(format!(
            "{}:{}:{} {}",
            self.time_edited.hour(),
            self.time_edited.minute(),
            self.time_edited.second(),
            self.time_edited.date()
        ))
        .block(Block::bordered().title("Edited"));
        let chunks = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(80), Constraint::Percentage(20)],
        )
        .split(area);
        let time_chunks = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .split(chunks[1]);
        for (date_widget, time_chunk) in zip([date_added, date_edited], time_chunks.iter()) {
            date_widget.render(*time_chunk, buf);
        }
        let title = Paragraph::new(self.title.clone()).block(Block::bordered().title("Task"));
        if let Some(desc_text) = self.description {
            let task_chunks =
                Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)])
                    .split(chunks[0]);
            let description =
                Paragraph::new(desc_text.clone()).block(Block::bordered().title("Description"));
            for (task_widget, task_chunk) in zip([title, description], task_chunks.iter()) {
                task_widget.render(*task_chunk, buf);
            }
        } else {
            title.render(chunks[0], buf);
        }
    }
}
