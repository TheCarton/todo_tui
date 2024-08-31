pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}
pub enum Actions {
    Mark_Done,
    Mark_Reject,
    Editing,
    Deleting,
    Exiting,
}
pub enum CurrentlyEditing {
    Title,
    Description,
}

pub enum TaskStatus {
    InProgress,
    Finished,
}

pub struct Task {
    pub(crate) title: String,
    pub(crate) description: Option<String>,
    pub(crate) task_status: TaskStatus,
}

pub struct App {
    pub title_input: String,
    pub description_input: String,
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: Option<CurrentlyEditing>,
    pub tasks: Vec<Task>,
}

impl App {
    pub(crate) fn new() -> App {
        App {
            title_input: String::new(),
            description_input: String::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            tasks: Vec::new(),
        }
    }

    pub(crate) fn save_task(&mut self) {
        if self.title_input.is_empty() {
            return;
        }
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
