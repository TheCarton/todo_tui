use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::task::{Task, TaskStatus};

#[derive(Clone, Serialize, Deserialize)]
pub struct Project {
    pub(crate) title: String,
    pub(crate) tasks: Vec<Task>,
    pub(crate) done_tasks: Vec<Task>,
    pub(crate) description: Option<String>,
    pub(crate) project_status: TaskStatus,
    pub(crate) time_added: OffsetDateTime,
    pub(crate) time_edited: OffsetDateTime,
    pub(crate) due_time: OffsetDateTime,
}

impl Project {
    pub(crate) fn new(title: String) -> Project {
        let now = OffsetDateTime::now_local().unwrap();
        Project {
            title,
            tasks: Vec::new(),
            done_tasks: Vec::new(),
            description: None,
            project_status: TaskStatus::InProgress,
            time_added: now,
            time_edited: now,
            due_time: now,
        }
    }
    pub(crate) fn default() -> Project {
        let now = OffsetDateTime::now_local().unwrap();
        Project {
            title: "misc".to_string(),
            tasks: Vec::new(),
            done_tasks: Vec::new(),
            description: Some("This is the default project. All unassigned tasks will be assigned to this project.".to_string()),
            project_status: TaskStatus::InProgress,
            time_added: now,
            time_edited: now,
            due_time: now,
        }
    }
    pub(crate) fn add_task(&mut self, task: Task) {
        let now = OffsetDateTime::now_local().unwrap();
        self.time_edited = now;
        self.tasks.push(task);
    }

    pub(crate) fn get_current_task(&mut self) -> Option<Task> {
        /* right now this is a pretty pointless wrapper function,
        but I want to be able to choose what tasks are displayed
        based on several factors (due date, times skipped maybe)
        so later this may be more complicated.
        */
        self.tasks.pop()
    }

    pub(crate) fn mark_task_done(&mut self, mut task: Task) {
        task.task_status = TaskStatus::Finished;
        self.done_tasks.push(task);
    }

    pub(crate) fn undo_mark_task_done(&mut self) -> Option<Task> {
        if let Some(mut task) = self.done_tasks.pop() {
            task.task_status = TaskStatus::InProgress;
            return Some(task);
        }
        None
    }
}
