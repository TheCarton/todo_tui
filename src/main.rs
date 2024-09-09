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
use crossterm::event::KeyEventKind;
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('a') => {
                        app.edit_mode = EditMode::CreateNew;
                        app.title_input = String::new();
                        app.description_input = String::new();
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::Title);
                    }
                    KeyCode::Char('e') => {
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
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('d') => {
                        app.change_task_status(TaskStatus::Finished);
                    }
                    KeyCode::Char('D') => {
                        app.change_task_status(TaskStatus::InProgress);
                    }
                    KeyCode::Char('r') => {
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
