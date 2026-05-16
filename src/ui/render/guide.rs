use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Span,
    widgets::Widget,
};

use crate::ui::theme::Theme;

pub struct GuideWidget<'a> {
    pub pivot_col: u16,
    pub theme: &'a Theme,
}

impl Widget for GuideWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 { return; }
        let col = self.pivot_col.min(area.width.saturating_sub(1));
        buf.set_span(
            area.x + col,
            area.y,
            &Span::styled("v", Style::default().fg(self.theme.guide)),
            1,
        );
    }
}
