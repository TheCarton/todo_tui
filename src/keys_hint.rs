use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Block, Row, Table, Widget},
};
use strum::{IntoEnumIterator, VariantArray};

use crate::{app::CurrentScreen, ActionKind};

pub struct KeysHint {
    pub screen: CurrentScreen,
}

impl Widget for KeysHint {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let b = Block::bordered().title("Help");
        let action_kinds = ActionKind::VARIANTS;
        let rows = [Row::new(action_kinds)];
        let widths = [Constraint::Length(25)];
        let table = Table::new(rows, widths);
        table.render(b.inner(area), buf);
        b.render(area, buf);
    }
}
