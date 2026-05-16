use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::Widget,
};

use crate::ui::theme::Theme;

pub struct StatusWidget<'a> {
    pub wpm: u16,
    pub progress: u8,
    pub elapsed_secs: u64,
    pub paused: bool,
    pub theme: &'a Theme,
}

impl Widget for StatusWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 { return; }

        let mins = self.elapsed_secs / 60;
        let secs = self.elapsed_secs % 60;
        let state = if self.paused { "⏸ paused" } else { "▶ playing" };
        let text = format!(
            " {} wpm  ·  {}%  ·  {:02}:{:02} elapsed  ·  {}",
            self.wpm, self.progress, mins, secs, state
        );

        let style = Style::default().fg(self.theme.status);
        buf.set_line(area.x, area.y, &Line::styled(text, style), area.width);
    }
}
