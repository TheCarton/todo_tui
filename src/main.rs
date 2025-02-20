mod app;
mod input_keys;
mod keys_hint;
mod project;
mod task;
mod ui;
use crate::app::App;
use crate::ui::ui;
use std::{
    fs::File,
    io::{self, stdout, Read, Write},
};

/*
Here's the workflow I want:
Mode 1:
I realize I want to add a task to my giant pile of tasks. I open ToDoTui, select a project or don't, type up a task, hit
'add' and close the app and forget about it.
Mode 2: I'm wondering what I should be working on, trying to select a task from my giant pile of tasks. I open up ToDoTui
and see a task. Most likely, I decide I don't want to work on that. I select a project I want to work on, or I don't, and
I hit the 'show a new task' button a bunch of times until I see a task that I want to work on. Then I close ToDoTui. The
next time I open it, that same task will still be displayed.
Mode 3: Cleanup mode. This is for going through tasks that I have skipped many times or have in some other way been marked
as tasks I'm very likely to abandon and abandoning them. It's also for the opposite: Reviving abandoned tasks with new
due dates.
*/
// TODO: next! Add projects which are collections of tasks.
// Add a project screen that shows which project you have selected.
// The project screen should also show what tasks have been marked done.
// TODO: Make a task choosing algorithm.
// TODO: Add a due date picker.
// TODO: Add a cleanup mode for abadoning tasks that are often skipped etc.
// TODO: The 'editing task' control flow is weird. You can edit a task, then add the edited task
// which will now exist alongside the original task as a new task. Also, the title bar
// says 'add new task' even when you selected edit an existing task.
// TODO: track how many times a task gets skipped.

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
                        app.edit_mode = EditMode::Title;
                        app.title_input = String::new();
                        app.description_input = String::new();
                        app.current_screen = CurrentScreen::Editing;
                        app.edit_mode = EditMode::Main;
                    }
                    Some(ActionKind::EditMode(_)) => {
                        app.current_screen = CurrentScreen::Editing;
                        app.edit_mode = EditMode::Main;
                        if let Some(task) = &app.current_task {
                            app.edit_mode = EditMode::Main;
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
                        app.mark_task_done();
                    }
                    Some(ActionKind::MarkTaskInProgress(_)) => {
                        app.undo_mark_task_done();
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
                        (EditMode::Main, Some(action)) => {
                            main_edit_mode_action_mapping(action, app);
                        }
                        (_, Some(ActionKind::ChangeMode(_))) => match app.popup {
                            Some(_) => app.popup = None,
                            None => app.edit_mode = EditMode::Main,
                        },
                        (EditMode::Title, _) => {
                            type_to_string(key.code, &mut app.title_input);
                        }
                        (EditMode::Description, _) => {
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
            app.add_task();
            app.current_screen = CurrentScreen::Main;
        }
        ActionKind::FocusTitle(_) => {
            app.edit_mode = EditMode::Title;
        }
        ActionKind::FocusDescription(_) => {
            app.edit_mode = EditMode::Description;
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
