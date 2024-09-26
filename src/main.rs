mod app;
mod keys_hint;
mod task;
mod ui;
use crate::app::App;
use crate::ui::ui;
use std::{
    fs::File,
    io::{self, stdout, Read, Write},
};

use app::{CurrentScreen, EditMode, Popup};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    text::Text,
    Terminal,
};
use strum::{EnumDiscriminants, EnumIter, EnumMessage, EnumString, VariantArray};
use task::TaskStatus;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = if let Ok(app) = load_from_disk() {
        app
    } else {
        App::new()
    };

    let _res = run_app(&mut terminal, &mut app);

    save_to_disk(&app)?;

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn save_to_disk(app: &App) -> std::io::Result<()> {
    let mut f = File::create("task_data.json")?;
    let json_string = serde_json::to_string(app)?;
    f.write_all(json_string.as_bytes())?;
    Ok(())
}

fn load_from_disk() -> std::io::Result<App> {
    let mut file = File::open("task_data.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let app: App = serde_json::from_str(&contents)?;
    Ok(app)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, VariantArray)]
enum ActionKind {
    AddTask,
    EditMode,
    ShuffleTasks,
    Quit,
    MarkTaskDone,
    MarkTaskInProgress,
    KeysHint,
    FocusTitle,
    FocusDescription,
    ChangeMode,
    IncrementDueDate,
    DecrementDueDate,
    AppendChar,
    DeleteChar,
}

impl From<&ActionKind> for Text<'_> {
    fn from(value: &ActionKind) -> Self {
        let key_char = if let Some(c) = key_code(value) {
            c.to_string()
        } else {
            "n/a".to_string()
        };
        Text::raw(format!("{} {}", key_char, value.action_description()))
    }
}

const fn key_code(action_kind: &ActionKind) -> Option<KeyCode> {
    match action_kind {
        ActionKind::AddTask => Some(ADD_MODE_KEY),
        ActionKind::EditMode => Some(EDIT_MODE_KEY),
        ActionKind::ShuffleTasks => Some(SHUFFLE_TASK_KEY),
        ActionKind::Quit => Some(QUIT_KEY),
        ActionKind::MarkTaskDone => Some(MARK_TASK_DONE_KEY),
        ActionKind::MarkTaskInProgress => Some(MARK_TASK_IN_PROGRESS_KEY),
        ActionKind::KeysHint => Some(KEYS_HINT_KEY),
        ActionKind::FocusTitle => Some(FOCUS_TITLE_KEY),
        ActionKind::FocusDescription => Some(FOCUS_DESCRIPTION_KEY),
        ActionKind::ChangeMode => Some(CHANGE_MODE_KEY),
        ActionKind::IncrementDueDate => Some(INCREMENT_DUE_DATE_BY_1),
        ActionKind::DecrementDueDate => Some(DECREMENT_DUE_DATE_BY_1),
        ActionKind::DeleteChar => Some(DELETE_CHAR_KEY),
        ActionKind::AppendChar => None,
    }
}
impl ActionKind {
    fn action_description(&self) -> String {
        String::from("need to make descriptions")
    }

    fn ref_array(&self) -> [String; 2] {
        if let Some(c) = key_code(self) {
            [format!("{}", c), self.action_description()]
        } else {
            ["N/A".to_string(), "Not assigned.".to_string()]
        }
    }
}

const ADD_MODE_KEY: KeyCode = KeyCode::Char('a');
const ADD_MODE_STRING: &str = "a";
const EDIT_MODE_KEY: KeyCode = KeyCode::Char('e');
const EDIT_MODE_STRING: &str = "e";
const SHUFFLE_TASK_KEY: KeyCode = KeyCode::Char('r');
const SHUFFLE_TASK_STRING: &str = "r";
const QUIT_KEY: KeyCode = KeyCode::Char('q');
const QUIT_STRING: &str = "q";
const MARK_TASK_DONE_KEY: KeyCode = KeyCode::Char('d');
const MARK_TASK_DONE_STRING: &str = "d";
const MARK_TASK_IN_PROGRESS_KEY: KeyCode = KeyCode::Char('D');
const MARK_TASK_IN_PROGRESS_STRING: &str = "D";
const KEYS_HINT_KEY: KeyCode = KeyCode::Char('?');
const KEYS_HINT_STRING: &str = "?";
const ADD_TASK_KEY: KeyCode = KeyCode::Enter;
const ADD_TASK_STRING: &str = "Enter";
const FOCUS_TITLE_KEY: KeyCode = KeyCode::Char('a');
const FOCUS_TITLE_STRING: &str = "a";
const FOCUS_DESCRIPTION_KEY: KeyCode = KeyCode::Char('d');
const FOCUS_DESCRIPTION_STRING: &str = "d";
const CHANGE_MODE_KEY: KeyCode = KeyCode::Esc;
const CHANGE_MODE_STRING: &str = "Esc";
const INCREMENT_DUE_DATE_BY_1: KeyCode = KeyCode::Char('y');
const INCREMENT_STRING: &str = "y";
const DECREMENT_DUE_DATE_BY_1: KeyCode = KeyCode::Char('Y');
const DECREMENT_STRING: &str = "Y";
const DELETE_CHAR_KEY: KeyCode = KeyCode::Backspace;
const DELETE_CHAR_STRING: &str = "Backspace";

fn main_screen_key_to_action(key: KeyCode) -> Option<ActionKind> {
    match key {
        ADD_MODE_KEY => Some(ActionKind::AddTask),
        CHANGE_MODE_KEY => Some(ActionKind::ChangeMode),
        EDIT_MODE_KEY => Some(ActionKind::EditMode),
        SHUFFLE_TASK_KEY => Some(ActionKind::ShuffleTasks),
        QUIT_KEY => Some(ActionKind::Quit),
        MARK_TASK_DONE_KEY => Some(ActionKind::MarkTaskDone),
        MARK_TASK_IN_PROGRESS_KEY => Some(ActionKind::MarkTaskInProgress),
        KEYS_HINT_KEY => Some(ActionKind::KeysHint),
        _ => None,
    }
}

fn popup_key_to_action(key: KeyCode) -> Option<ActionKind> {
    match key {
        CHANGE_MODE_KEY => Some(ActionKind::ChangeMode),
        QUIT_KEY => Some(ActionKind::Quit),
        _ => None,
    }
}

fn edit_screen_key_to_action(key: KeyCode) -> Option<ActionKind> {
    match key {
        ADD_TASK_KEY => Some(ActionKind::AddTask),
        FOCUS_TITLE_KEY => Some(ActionKind::FocusTitle),
        FOCUS_DESCRIPTION_KEY => Some(ActionKind::FocusDescription),
        CHANGE_MODE_KEY => Some(ActionKind::ChangeMode),
        INCREMENT_DUE_DATE_BY_1 => Some(ActionKind::IncrementDueDate),
        DECREMENT_DUE_DATE_BY_1 => Some(ActionKind::DecrementDueDate),
        DELETE_CHAR_KEY => Some(ActionKind::DeleteChar),
        KEYS_HINT_KEY => Some(ActionKind::KeysHint),
        KeyCode::Char(_) => Some(ActionKind::AppendChar),
        _ => None,
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            if app.popup.is_some() {
                match popup_key_to_action(key.code) {
                    Some(ActionKind::ChangeMode) => {
                        app.popup = None;
                    }
                    Some(ActionKind::Quit) => {
                        return Ok(());
                    }
                    _ => {}
                }
            }
            match app.current_screen {
                CurrentScreen::Main => match main_screen_key_to_action(key.code) {
                    Some(ActionKind::AddTask) => {
                        app.edit_mode = Some(EditMode::Title);
                        app.title_input = String::new();
                        app.description_input = String::new();
                        app.current_screen = CurrentScreen::Editing;
                        app.edit_mode = Some(EditMode::Main);
                    }
                    Some(ActionKind::EditMode) => {
                        app.current_screen = CurrentScreen::Editing;
                        app.edit_mode = Some(EditMode::Main);
                        if let Some(task) = &app.current_task {
                            app.edit_mode = Some(EditMode::Main);
                            app.title_input = task.title.clone();
                            if let Some(description) = &task.description {
                                app.description_input = description.clone();
                            }
                        }
                    }
                    Some(ActionKind::Quit) => {
                        return Ok(());
                    }
                    Some(ActionKind::MarkTaskDone) => {
                        app.change_task_status(TaskStatus::Finished);
                    }
                    Some(ActionKind::MarkTaskInProgress) => {
                        app.change_task_status(TaskStatus::InProgress);
                    }
                    Some(ActionKind::ShuffleTasks) => {
                        app.choose_shown_task();
                    }
                    Some(ActionKind::KeysHint) => {
                        app.popup = Some(app::Popup::Help);
                    }
                    _ => {}
                },
                CurrentScreen::Editing => {
                    let maybe_action = edit_screen_key_to_action(key.code);
                    match (app.edit_mode, maybe_action) {
                        (Some(EditMode::Main), Some(action)) => {
                            main_edit_mode_action_mapping(action, app);
                        }
                        (Some(_), Some(ActionKind::ChangeMode)) => {
                            app.edit_mode = Some(EditMode::Main);
                        }
                        (Some(_), Some(ActionKind::KeysHint)) => {
                            app.popup = Some(Popup::Help);
                        }
                        (Some(EditMode::Title), Some(action)) => {
                            type_to_string(action, &mut app.title_input, key.code);
                            // yuck
                        }
                        (Some(EditMode::Description), Some(action)) => {
                            type_to_string(action, &mut app.description_input, key.code);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn type_to_string(action: ActionKind, field: &mut String, key: KeyCode) {
    match (action, key) {
        (ActionKind::DeleteChar, _) => {
            field.pop();
        }
        (ActionKind::AppendChar, KeyCode::Char(c)) => {
            field.push(c);
        }
        _ => {}
    }
}

fn main_edit_mode_action_mapping(action: ActionKind, app: &mut App) {
    match action {
        ActionKind::ChangeMode => {
            app.current_screen = CurrentScreen::Main;
        }
        ActionKind::AddTask => {
            app.save_task();
            app.current_screen = CurrentScreen::Main;
        }
        ActionKind::FocusTitle => {
            app.edit_mode = Some(EditMode::Title);
        }
        ActionKind::FocusDescription => {
            app.edit_mode = Some(EditMode::Description);
        }
        ActionKind::IncrementDueDate => {
            app.change_active_task_due_date(1);
        }
        ActionKind::DecrementDueDate => {
            app.change_active_task_due_date(1);
        }
        ActionKind::KeysHint => {
            app.popup = Some(Popup::Help);
        }
        _ => {}
    }
}
