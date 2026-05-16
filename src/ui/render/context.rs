use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::Widget,
};

use crate::core::word::Word;
use crate::ui::theme::Theme;

pub struct ContextWidget<'a> {
    pub words: &'a [Word],
    pub theme: &'a Theme,
}

impl Widget for ContextWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 || self.words.is_empty() { return; }

        let text: String = self.words.iter().map(|w| w.text.as_str()).collect::<Vec<_>>().join(" ");
        let style = Style::default().fg(self.theme.context);
        let line = Line::styled(text, style);
        buf.set_line(area.x, area.y, &line, area.width);
    }
}
