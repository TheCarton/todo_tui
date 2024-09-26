use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Block, Row, Table, Widget},
};
use strum::IntoEnumIterator;

use crate::{app::CurrentScreen, ActionKind};

pub struct KeysHint {
    pub screen: CurrentScreen,
}

impl Widget for KeysHint {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let b = Block::bordered().title("Help");
        let rows = [Row::new(ActionKind::iter())];
        let widths = [Constraint::Length(50)];
        let table = Table::new(rows, widths);
        table.render(b.inner(area), buf);
        b.render(area, buf);
    }
}
