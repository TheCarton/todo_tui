mod app;
mod task;
mod ui;
use crate::app::App;
use crate::ui::ui;
use std::{
    fs::File,
    io::{self, stdout, Read, Write},
};

use app::{CurrentScreen, EditMode, TaskEditMode};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Terminal,
};
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
    IncrementDueDate(i64),
    DecrementDueDate(i64),
    DeleteChar,
}

impl ActionKind {
    const fn key_code(&self) -> KeyCode {
        match self {
            ActionKind::AddTask => ADD_MODE_KEY,
            ActionKind::EditMode => EDIT_MODE_KEY,
            ActionKind::ShuffleTasks => SHUFFLE_TASK_KEY,
            ActionKind::Quit => QUIT_KEY,
            ActionKind::MarkTaskDone => MARK_TASK_DONE_KEY,
            ActionKind::MarkTaskInProgress => MARK_TASK_IN_PROGRESS_KEY,
            ActionKind::KeysHint => KEYS_HINT_KEY,
            ActionKind::FocusTitle => FOCUS_TITLE_KEY,
            ActionKind::FocusDescription => FOCUS_DESCRIPTION_KEY,
            ActionKind::ChangeMode => CHANGE_MODE_KEY,
            ActionKind::IncrementDueDate(_) => INCREMENT_DUE_DATE_BY_1,
            ActionKind::DecrementDueDate(_) => DECREMENT_DUE_DATE_BY_1,
            ActionKind::DeleteChar => DELETE_CHAR_KEY,
        }
    }

    fn action_description(&self) -> String {
        String::from("need to make descriptions")
    }
}

const ADD_MODE_KEY: KeyCode = KeyCode::Char('a');
const EDIT_MODE_KEY: KeyCode = KeyCode::Char('e');
const SHUFFLE_TASK_KEY: KeyCode = KeyCode::Char('r');
const QUIT_KEY: KeyCode = KeyCode::Char('q');
const MARK_TASK_DONE_KEY: KeyCode = KeyCode::Char('d');
const MARK_TASK_IN_PROGRESS_KEY: KeyCode = KeyCode::Char('D');
const KEYS_HINT_KEY: KeyCode = KeyCode::Char('?');

const ADD_TASK_KEY: KeyCode = KeyCode::Enter;
const FOCUS_TITLE_KEY: KeyCode = KeyCode::Char('a');
const FOCUS_DESCRIPTION_KEY: KeyCode = KeyCode::Char('d');
const CHANGE_MODE_KEY: KeyCode = KeyCode::Esc;
const INCREMENT_DUE_DATE_BY_1: KeyCode = KeyCode::Char('y');
const DECREMENT_DUE_DATE_BY_1: KeyCode = KeyCode::Char('Y');
const DELETE_CHAR_KEY: KeyCode = KeyCode::Backspace;

fn main_screen_key_to_action(key: KeyCode) -> Option<ActionKind> {
    match key {
        ADD_MODE_KEY => Some(ActionKind::AddTask),
        EDIT_MODE_KEY => Some(ActionKind::EditMode),
        SHUFFLE_TASK_KEY => Some(ActionKind::ShuffleTasks),
        QUIT_KEY => Some(ActionKind::Quit),
        MARK_TASK_DONE_KEY => Some(ActionKind::MarkTaskDone),
        MARK_TASK_IN_PROGRESS_KEY => Some(ActionKind::MarkTaskInProgress),
        KEYS_HINT_KEY => Some(ActionKind::KeysHint),
        _ => None,
    }
}

fn edit_mode_key_to_action(key: KeyCode) -> Option<ActionKind> {
    match key {
        ADD_TASK_KEY => Some(ActionKind::AddTask),
        FOCUS_TITLE_KEY => Some(ActionKind::FocusTitle),
        FOCUS_DESCRIPTION_KEY => Some(ActionKind::FocusDescription),
        CHANGE_MODE_KEY => Some(ActionKind::ChangeMode),
        INCREMENT_DUE_DATE_BY_1 => Some(ActionKind::IncrementDueDate(1)),
        DECREMENT_DUE_DATE_BY_1 => Some(ActionKind::DecrementDueDate(-1)),
        DELETE_CHAR_KEY => Some(ActionKind::DeleteChar),
        KEYS_HINT_KEY => Some(ActionKind::KeysHint),
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
            match app.current_screen {
                CurrentScreen::Main => match main_screen_key_to_action(key.code) {
                    Some(ActionKind::AddTask) => {
                        app.edit_mode = TaskEditMode::CreateNew;
                        app.title_input = String::new();
                        app.description_input = String::new();
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(EditMode::Main);
                    }
                    Some(ActionKind::EditMode) => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(EditMode::Main);
                        if let Some(task) = &app.current_task {
                            app.edit_mode = TaskEditMode::Active;
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
                    _ => {}
                },
                CurrentScreen::Editing => match edit_mode_key_to_action(key.code) {
                    Some(ActionKind::AddTask) => {
                        app.save_task();
                        app.current_screen = CurrentScreen::Main;
                    }
                    Some(ActionKind::FocusTitle) => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                EditMode::Main => {
                                    app.currently_editing = Some(EditMode::Title);
                                }
                                _ => {}
                            }
                        }
                    }
                    Some(ActionKind::FocusDescription) => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                EditMode::Main => {
                                    app.currently_editing = Some(EditMode::Description);
                                }
                                _ => {}
                            }
                        }
                    }
                    Some(ActionKind::DeleteChar) => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                EditMode::Title => {
                                    app.title_input.pop();
                                }
                                EditMode::Description => {
                                    app.description_input.pop();
                                }
                                EditMode::Main => {}
                            }
                        }
                    }
                    Some(ActionKind::ChangeMode) => match app.current_screen {
                        CurrentScreen::Main => {}
                        CurrentScreen::Editing => {
                            if let Some(focused_field) = app.currently_editing {
                                match focused_field {
                                    app::EditMode::Main => todo!(),
                                    app::EditMode::Title => todo!(),
                                    app::EditMode::Description => todo!(),
                                }
                            }
                        }
                    },
                    Some(ActionKind::IncrementDueDate(i)) => {
                        app.change_active_task_due_date(i);
                    }
                    Some(ActionKind::DecrementDueDate(i)) => {
                        app.change_active_task_due_date(i);
                    }
                    None => {
                        if let KeyCode::Char(value) = key.code {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    EditMode::Title => {
                                        app.title_input.push(value);
                                    }
                                    EditMode::Description => {
                                        app.description_input.push(value);
                                    }
                                    EditMode::Main => {}
                                }
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
    }
}
