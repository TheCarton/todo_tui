mod app;
mod task;
mod ui;
use crate::app::App;
use crate::ui::ui;
use std::{
    fs::File,
    io::{self, stdout, Read, Write},
};

use app::{CurrentScreen, CurrentlyEditing, EditMode};
use crossterm::event::{KeyEvent, KeyEventKind};
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
    EditTask,
    ShuffleTasks,
    Quit,
    MarkTaskDone,
    MarkTaskInProgress,
    KeysHint,
}

impl ActionKind {
    const fn key_code(&self) -> KeyCode {
        match self {
            ActionKind::AddTask => KeyCode::Char('a'),
            ActionKind::EditTask => KeyCode::Char('e'),
            ActionKind::ShuffleTasks => KeyCode::Char('r'),
            ActionKind::Quit => KeyCode::Char('q'),
            ActionKind::MarkTaskDone => KeyCode::Char('d'),
            ActionKind::MarkTaskInProgress => KeyCode::Char('D'),
            ActionKind::KeysHint => KeyCode::Char('?'),
        }
    }
}

const ADD_TASK_KEY: KeyCode = ActionKind::AddTask.key_code();
const EDIT_TASK_KEY: KeyCode = ActionKind::EditTask.key_code();
const SHUFFLE_TASK_KEY: KeyCode = ActionKind::ShuffleTasks.key_code();
const QUIT_KEY: KeyCode = ActionKind::Quit.key_code();
const MARK_TASK_DONE_KEY: KeyCode = ActionKind::MarkTaskDone.key_code();
const MARK_TASK_IN_PROGRESS_KEY: KeyCode = ActionKind::MarkTaskInProgress.key_code();
const KEYS_HINT_KEY: KeyCode = ActionKind::KeysHint.key_code();

fn main_screen_key_to_action(key: KeyEvent) -> Option<ActionKind> {
    match key.code {
        ADD_TASK_KEY => Some(ActionKind::AddTask),
        EDIT_TASK_KEY => Some(ActionKind::EditTask),
        SHUFFLE_TASK_KEY => Some(ActionKind::ShuffleTasks),
        QUIT_KEY => Some(ActionKind::Quit),
        MARK_TASK_DONE_KEY => Some(ActionKind::MarkTaskDone),
        MARK_TASK_IN_PROGRESS_KEY => Some(ActionKind::MarkTaskInProgress),
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
                CurrentScreen::Main => match main_screen_key_to_action(key) {
                    Some(ActionKind::AddTask) => {
                        app.edit_mode = EditMode::CreateNew;
                        app.title_input = String::new();
                        app.description_input = String::new();
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::Title);
                    }
                    Some(ActionKind::EditTask) => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::Title);
                        if let Some(task) = &app.current_task {
                            app.edit_mode = EditMode::Active;
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
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Title => {
                                    app.currently_editing = Some(CurrentlyEditing::Description);
                                }
                                CurrentlyEditing::Description => {
                                    app.save_task();
                                    app.current_screen = CurrentScreen::Main;
                                }
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Title => {
                                    app.title_input.pop();
                                }
                                CurrentlyEditing::Description => {
                                    app.description_input.pop();
                                }
                            }
                        }
                    }
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.currently_editing = None;
                    }
                    KeyCode::Char('y') => {
                        app.change_active_task_due_date(1);
                    }
                    KeyCode::Char('Y') => {
                        app.change_active_task_due_date(5);
                    }
                    KeyCode::Char(value) => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Title => {
                                    app.title_input.push(value);
                                }
                                CurrentlyEditing::Description => {
                                    app.description_input.push(value);
                                }
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
