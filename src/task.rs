use std::iter::zip;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Paragraph, Widget},
};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub(crate) title: String,
    pub(crate) description: Option<String>,
    pub(crate) task_status: TaskStatus,
    pub(crate) time_added: OffsetDateTime,
    pub(crate) time_edited: OffsetDateTime,
    pub(crate) due_time: OffsetDateTime,
}

impl Task {
    pub(crate) fn default(title: String) -> Task {
        let now = OffsetDateTime::now_local().unwrap();
        Task {
            title,
            description: None,
            task_status: TaskStatus::InProgress,
            time_added: now,
            time_edited: now,
            due_time: now + Duration::DAY,
        }
    }
    pub(crate) fn new(title: String, description: String) -> Task {
        let now = OffsetDateTime::now_local().unwrap();
        Task {
            title,
            description: Some(description),
            task_status: TaskStatus::InProgress,
            time_added: now,
            time_edited: now,
            due_time: now + Duration::DAY,
        }
    }

    pub(crate) fn change_due_date(&mut self, num_days: i64) {
        self.due_time += Duration::days(num_days);
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum TaskStatus {
    InProgress,
    Finished,
}

impl TaskStatus {
    fn to_text(&self) -> String {
        match self {
            TaskStatus::InProgress => "In Progress".to_string(),
            TaskStatus::Finished => "Finished".to_string(),
        }
    }
}

impl Widget for Task {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let b = Block::bordered().title("Task");
        let chunks = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(80), Constraint::Percentage(20)],
        )
        .split(b.inner(area));

        b.render(area, buf);
        let task_completion =
            Paragraph::new(self.task_status.to_text()).block(Block::bordered().title("Status"));

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

        let due_date = Paragraph::new(format!(
            "{}:{}:{} {}",
            self.due_time.hour(),
            self.due_time.minute(),
            self.due_time.second(),
            self.due_time.date()
        ))
        .block(Block::bordered().title("Due"));

        let status_chunks = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ],
        )
        .split(chunks[1]);

        for (date_widget, status_chunk) in zip(
            [task_completion, date_added, date_edited, due_date],
            status_chunks.iter(),
        ) {
            date_widget.render(*status_chunk, buf);
        }
        let title = Paragraph::new(self.title.clone()).block(Block::bordered().title("Title"));
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
