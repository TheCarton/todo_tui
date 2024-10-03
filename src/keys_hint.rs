use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Block, Cell, Row, Table, Widget},
};

use crate::{
    app::CurrentScreen,
    input_keys::{ActionKind, ACTION_KINDS},
};

pub struct KeysHint {
    pub _screen: CurrentScreen,
}

const KEY_CODE_MAX_NAME_LEN: u16 = length_of_longest_input_key_name(&ACTION_KINDS);
const KEY_CODE_MAX_DESC_LEN: u16 = length_of_longest_input_key_description(&ACTION_KINDS);

const fn length_of_longest_input_key_name(actions: &[ActionKind]) -> u16 {
    let mut max_len = actions[0].input_key().name.len();
    let mut i = 0;
    while i < actions.len() {
        let new_len = actions[i].input_key().name.len();
        if new_len > max_len {
            max_len = new_len;
        }
        i += 1;
    }
    max_len as u16
}

const fn length_of_longest_input_key_description(actions: &[ActionKind]) -> u16 {
    let mut max_len = actions[0].input_key().description.len();
    let mut i = 0;
    while i < actions.len() {
        let new_len = actions[i].input_key().description.len();
        if new_len > max_len {
            max_len = new_len;
        }
        i += 1;
    }
    max_len as u16
}
impl Widget for KeysHint {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let b = Block::bordered().title("Help");
        let rows = ACTION_KINDS.map(|action| {
            let key = action.input_key();
            let cells = vec![Cell::from(key.name), Cell::from(key.description)];
            Row::new(cells)
        });
        let widths = [
            Constraint::Length(KEY_CODE_MAX_NAME_LEN),
            Constraint::Length(KEY_CODE_MAX_DESC_LEN),
        ];
        let table = Table::new(rows, widths);
        table.render(b.inner(area), buf);
        b.render(area, buf);
    }
}
