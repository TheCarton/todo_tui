use rand::seq::SliceRandom;

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}
#[derive(Clone, Copy)]
pub enum Actions {
    Mark_Done,
    Mark_Reject,
    Editing,
    Deleting,
    Exiting,
}
#[derive(Clone, Copy)]
pub enum CurrentlyEditing {
    Title,
    Description,
}

#[derive(Clone, Copy)]
pub enum EditMode {
    Active,
    CreateNew,
}

#[derive(Clone, Copy)]
pub enum TaskStatus {
    InProgress,
    Finished,
}

#[derive(Clone)]
pub struct Task {
    pub(crate) title: String,
    pub(crate) description: Option<String>,
    pub(crate) task_status: TaskStatus,
}

pub struct App {
    pub title_input: String,
    pub description_input: String,
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub current_task: Option<Task>,
    pub currently_editing: Option<CurrentlyEditing>,
    pub edit_mode: EditMode,
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
            edit_mode: EditMode::CreateNew,
            tasks: Vec::new(),
        }
    }

    pub(crate) fn save_task(&mut self) {
        if self.title_input.is_empty() && self.description_input.is_empty() {
            return;
        }
        match self.edit_mode {
            EditMode::Active => {
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
            }
            EditMode::CreateNew => {
                let description = if self.description_input.is_empty() {
                    None
                } else {
                    Some(self.description_input.clone())
                };
                let new_task = Task {
                    title: self.title_input.clone(),
                    description,
                    task_status: TaskStatus::InProgress,
                };
                self.tasks.push(new_task);
            }
        }
        self.choose_shown_task();
    }

    pub fn choose_shown_task(&mut self) {
        if let Some(task) = &self.current_task {
            self.tasks.push(task.clone());
        }
        self.tasks.shuffle(&mut rand::thread_rng());
        self.current_task = self.tasks.pop();
    }
}
