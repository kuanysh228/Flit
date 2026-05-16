use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::core::word::Word;
use crate::ui::theme::Theme;

pub struct WordWidget<'a> {
    pub word: &'a Word,
    pub pivot_col: u16,
    pub theme: &'a Theme,
}

impl Widget for WordWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 { return; }

        let graphemes: Vec<&str> = self.word.text.graphemes(true).collect();
        let pivot = self.word.orp_idx.min(graphemes.len().saturating_sub(1));

        let before: String = graphemes[..pivot].concat();
        let at: &str = graphemes[pivot];
        let after: String = graphemes[pivot + 1..].concat();

        let before_width = UnicodeWidthStr::width(before.as_str()) as u16;
        let x_start = (area.x + self.pivot_col).saturating_sub(before_width);

        let bold = Style::default().add_modifier(Modifier::BOLD);
        let pivot_style = Style::default().fg(self.theme.pivot).add_modifier(Modifier::BOLD);

        let line = Line::from(vec![
            Span::styled(before, bold.fg(self.theme.word)),
            Span::styled(at, pivot_style),
            Span::styled(after, bold.fg(self.theme.word)),
        ]);

        buf.set_line(x_start, area.y, &line, area.width);
    }
}
