use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

pub struct HelpWidget;

impl Widget for HelpWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let lines = vec![
            Line::from("  Flit — keybindings  ").centered(),
            Line::from(""),
            Line::from("  Space    pause / resume"),
            Line::from("  j / [   slow down  (×0.9)"),
            Line::from("  k / ]   speed up   (×1.1)"),
            Line::from("  h       back 1 word"),
            Line::from("  l       forward 1 word"),
            Line::from("  b       back 1 sentence"),
            Line::from("  w       forward 1 sentence"),
            Line::from("  { }     back / forward paragraph"),
            Line::from("  gg      go to start"),
            Line::from("  G       go to end"),
            Line::from("  /       search"),
            Line::from("  n / N   next / prev match"),
            Line::from("  :       command mode"),
            Line::from("  q       quit"),
            Line::from(""),
            Line::from("  any key to close").centered(),
        ];

        let block = Block::default()
            .title(" Help ")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center);

        let inner = block.inner(area);
        Clear.render(area, buf);
        block.render(area, buf);
        Paragraph::new(Text::from(lines))
            .alignment(Alignment::Left)
            .render(inner, buf);
    }
}
