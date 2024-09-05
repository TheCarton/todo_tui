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
