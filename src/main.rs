mod app;
mod input_keys;
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
use crossterm::event::KeyCode;
use input_keys::{keycode_to_actionkind, ActionKind, DELETE_CHAR_KEYCODE};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, Event},
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match keycode_to_actionkind(key.code) {
                    Some(ActionKind::AddTask(_)) => {
                        app.edit_mode = Some(EditMode::Title);
                        app.title_input = String::new();
                        app.description_input = String::new();
                        app.current_screen = CurrentScreen::Editing;
                        app.edit_mode = Some(EditMode::Main);
                    }
                    Some(ActionKind::EditMode(_)) => {
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
                    Some(ActionKind::Quit(_)) => {
                        return Ok(());
                    }
                    Some(ActionKind::MarkTaskDone(_)) => {
                        app.change_task_status(TaskStatus::Finished);
                    }
                    Some(ActionKind::MarkTaskInProgress(_)) => {
                        app.change_task_status(TaskStatus::InProgress);
                    }
                    Some(ActionKind::ShuffleTasks(_)) => {
                        app.choose_shown_task();
                    }
                    Some(ActionKind::KeysHint(_)) => {
                        app.popup = Some(app::Popup::Help);
                    }
                    Some(ActionKind::ChangeMode(_)) => {
                        app.popup = None;
                    }
                    _ => {}
                },
                CurrentScreen::Editing => {
                    let maybe_action = keycode_to_actionkind(key.code);
                    match (app.edit_mode, maybe_action) {
                        (Some(EditMode::Main), Some(action)) => {
                            main_edit_mode_action_mapping(action, app);
                        }
                        (Some(_), Some(ActionKind::ChangeMode(_))) => match app.popup {
                            Some(_) => app.popup = None,
                            None => app.edit_mode = Some(EditMode::Main),
                        },
                        (Some(EditMode::Title), _) => {
                            type_to_string(key.code, &mut app.title_input);
                        }
                        (Some(EditMode::Description), _) => {
                            type_to_string(key.code, &mut app.description_input);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn type_to_string(key: KeyCode, field: &mut String) {
    match key {
        DELETE_CHAR_KEYCODE => {
            field.pop();
        }
        KeyCode::Char(ch) => {
            field.push(ch);
        }
        _ => {}
    }
}

fn main_edit_mode_action_mapping(action: ActionKind, app: &mut App) {
    match action {
        ActionKind::ChangeMode(_) => match app.popup {
            Some(_) => {
                app.popup = None;
            }
            None => {
                app.current_screen = CurrentScreen::Main;
            }
        },
        ActionKind::AddTask(_) => {
            app.save_task();
            app.current_screen = CurrentScreen::Main;
        }
        ActionKind::FocusTitle(_) => {
            app.edit_mode = Some(EditMode::Title);
        }
        ActionKind::FocusDescription(_) => {
            app.edit_mode = Some(EditMode::Description);
        }
        ActionKind::IncrementDueDate(_) => {
            app.change_active_task_due_date(1);
        }
        ActionKind::DecrementDueDate(_) => {
            app.change_active_task_due_date(1);
        }
        ActionKind::KeysHint(_) => {
            app.popup = Some(Popup::Help);
        }
        _ => {}
    }
}
