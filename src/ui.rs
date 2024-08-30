use std::iter::zip;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::Paragraph,
    Frame,
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title = Paragraph::new("To-Do Tui");
    let middle = Paragraph::new("middle text");
    let bottom = Paragraph::new("bottom text");

    for (w, a) in zip([title, middle, bottom], chunks.iter()) {
        frame.render_widget(w, *a);
    }
}
