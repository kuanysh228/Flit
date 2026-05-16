use ratatui::style::Color;

pub struct Theme {
    pub pivot: Color,
    pub word: Color,
    pub context: Color,
    pub status: Color,
    pub guide: Color,
    pub background: Color,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            pivot: Color::Red,
            word: Color::White,
            context: Color::DarkGray,
            status: Color::Gray,
            guide: Color::Yellow,
            background: Color::Reset,
        }
    }

    pub fn light() -> Self {
        Self {
            pivot: Color::Red,
            word: Color::Black,
            context: Color::Gray,
            status: Color::DarkGray,
            guide: Color::Yellow,
            background: Color::Reset,
        }
    }
}
