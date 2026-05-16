use std::time::Instant;

use crate::core::engine::ReadingEngine;
use crate::core::word::Word;
use crate::ui::input::mode::Mode;
use crate::ui::input::search::SearchState;
use crate::ui::theme::Theme;

pub struct App {
    pub engine: ReadingEngine,
    pub mode: Mode,
    pub theme: Theme,
    pub show_help: bool,
    pub show_stats: bool,
    pub search: Option<SearchState>,
    pub started_at: Instant,
    pub word_texts: Vec<String>,
}

impl App {
    pub fn new(words: Vec<Word>, wpm: u16, start_index: u64, theme: Theme) -> Self {
        let word_texts: Vec<String> = words.iter().map(|w| w.text.clone()).collect();
        let engine = if start_index > 0 {
            ReadingEngine::resume_from(words, wpm, start_index)
        } else {
            ReadingEngine::new(words, wpm)
        };
        Self {
            engine,
            mode: Mode::default(),
            theme,
            show_help: false,
            show_stats: false,
            search: None,
            started_at: Instant::now(),
            word_texts,
        }
    }

    pub fn elapsed_secs(&self) -> u64 {
        self.started_at.elapsed().as_secs()
    }
}
