mod app;
mod ui;
use crate::app::App;
use std::io::{self, stdout, Error};

use app::{Actions, CurrentScreen, CurrentlyEditing};
use crossterm::event::KeyEventKind;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Layout},
    widgets::{Block, Paragraph},
    Frame, Terminal,
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = App::new();

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::Title);
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
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

fn ui(frame: &mut Frame, app: &App) {
    let [title_area, main_area, status_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Percentage(80),
        Constraint::Length(1),
    ])
    .areas(frame.size());
    let [left_area, right_area] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(main_area);

    frame.render_widget(Block::bordered().title("To-do TUI"), title_area);
    frame.render_widget(Block::bordered().title("my special task"), main_area);
    frame.render_widget(
        Block::bordered().title("(A)ccet (R)eject (E)dit (D)elete (Q)uit"),
        status_area,
    );
}
