use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{
    project::Project,
    task::{Task, TaskStatus},
};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum CurrentScreen {
    Main,
    Editing,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq)]
pub enum EditMode {
    Main,
    Title,
    Description,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum TaskCreationMode {
    Active,
    CreateNew,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Popup {
    Help,
    Error,
}

#[derive(Serialize, Deserialize)]
pub struct App {
    pub title_input: String,
    pub description_input: String,
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub current_project: Project,
    pub edit_mode: EditMode,
    pub popup: Option<Popup>,
    pub task_creation_mode: TaskCreationMode,
    pub projects: Vec<Project>,
}

impl App {
    pub(crate) fn new() -> App {
        App {
            title_input: String::new(),
            description_input: String::new(),
            current_screen: CurrentScreen::Main,
            current_project: Project::default(),
            edit_mode: EditMode::Main,
            popup: None,
            task_creation_mode: TaskCreationMode::CreateNew,
            projects: Vec::new(),
        }
    }

    pub(crate) fn add_task(&mut self) {
        if self.title_input.is_empty() {
            return;
        }
        let task = match self.task_creation_mode {
            TaskCreationMode::Active => {
                let edited_task = self
                    .current_project
                    .get_current_task()
                    .expect("editing an active task that exists");
                edited_task.title = self.title_input.clone();
                let description = if self.description_input.is_empty() {
                    None
                } else {
                    Some(self.description_input.clone())
                };
                edited_task.description = description;
                edited_task.time_edited = OffsetDateTime::now_local().unwrap();
                edited_task.to_owned()
            }
            TaskCreationMode::CreateNew => {
                if self.description_input.is_empty() {
                    Task::default(self.title_input.clone())
                } else {
                    Task::new(self.title_input.clone(), self.description_input.clone())
                }
            }
        };
        self.current_project.add_task(task.clone());
    }

    pub(crate) fn mark_task_done(&mut self) {
        self.current_project.mark_task_done();
        self.choose_shown_task();
    }

    pub(crate) fn undo_mark_task_done(&mut self) {
        // I need to make the undone done task the currently displayed task also.
        self.current_project.undo_mark_task_done();
    }

    pub(crate) fn change_active_task_due_date(&mut self, num_days: i64) {
        if let Some(ref mut active_task) = &mut self.current_task {
            active_task.change_due_date(num_days);
        }
    }

    /*
    This system chooses what task is shown. The basic idea of this app is that it isn't just a list, because I don't like lists
    and lists can be done about just as well with a pencil and paper. So instead, the app will choose a single task to show you.
    Obviously, how this task is shown to you is very important.
    */

    pub fn choose_shown_task(&mut self) {
        // I want to change the shown task to be a pointer instead of cloning
        if let Some(task) = &self.current_task {
            self.tasks.push(task.clone());
        }
        self.tasks.shuffle(&mut rand::thread_rng());
        self.current_task = self.tasks.pop();
    }
}
