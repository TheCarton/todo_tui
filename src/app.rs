use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::task::{Task, TaskStatus};

#[derive(Serialize, Deserialize)]
pub enum CurrentScreen {
    Main,
    Editing,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum EditMode {
    Main,
    Title,
    Description,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum TaskEditMode {
    Active,
    CreateNew,
}

#[derive(Serialize, Deserialize)]
pub struct App {
    pub title_input: String,
    pub description_input: String,
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub current_task: Option<Task>,
    pub currently_editing: Option<EditMode>,
    pub edit_mode: TaskEditMode,
    pub tasks: Vec<Task>,
}

impl App {
    pub(crate) fn new() -> App {
        App {
            title_input: String::new(),
            description_input: String::new(),
            current_screen: CurrentScreen::Main,
            current_task: None,
            currently_editing: None,
            edit_mode: TaskEditMode::CreateNew,
            tasks: Vec::new(),
        }
    }

    pub(crate) fn save_task(&mut self) {
        if self.title_input.is_empty() && self.description_input.is_empty() {
            return;
        }
        match self.edit_mode {
            TaskEditMode::Active => {
                let t = self
                    .current_task
                    .as_mut()
                    .expect("editing an active task that exists");
                t.title = self.title_input.clone();
                let description = if self.description_input.is_empty() {
                    None
                } else {
                    Some(self.description_input.clone())
                };
                t.description = description;
                t.time_edited = OffsetDateTime::now_local().unwrap();
            }
            TaskEditMode::CreateNew => {
                let new_task = if self.description_input.is_empty() {
                    Task::default(self.title_input.clone())
                } else {
                    Task::new(self.title_input.clone(), self.description_input.clone())
                };
                self.tasks.push(new_task);
            }
        }
        self.choose_shown_task();
    }

    pub(crate) fn change_active_task_due_date(&mut self, num_days: i64) {
        if let Some(ref mut active_task) = &mut self.current_task {
            active_task.change_due_date(num_days);
        }
    }

    pub fn change_task_status(&mut self, new_status: TaskStatus) {
        if let Some(ref mut active_task) = &mut self.current_task {
            active_task.task_status = new_status;
        }
    }

    pub fn choose_shown_task(&mut self) {
        if let Some(task) = &self.current_task {
            self.tasks.push(task.clone());
        }
        self.tasks.shuffle(&mut rand::thread_rng());
        self.current_task = self.tasks.pop();
    }
}
