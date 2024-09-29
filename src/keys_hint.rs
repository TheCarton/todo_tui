use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Widget},
};

use crate::app::CurrentScreen;

pub struct KeysHint {
    pub screen: CurrentScreen,
}

impl Widget for KeysHint {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let b = Block::bordered().title("Help");
        b.render(area, buf);
    }
}
