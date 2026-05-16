use std::time::{Duration, Instant};

use crate::core::timing::word_time;
use crate::core::word::{Word, WordPosition};
use crate::core::action::Action;

pub struct ReadingEngine {
    words: Vec<Word>,
    cursor: usize,
    wpm: u16,
    paused: bool,
    next_word_at: Instant,
    words_read: u64,
    peak_wpm: u16,
    time_paused: Duration,
    pause_started: Option<Instant>,
}

impl ReadingEngine {
    pub fn new(words: Vec<Word>, wpm: u16) -> Self {
        Self {
            words,
            cursor: 0,
            wpm,
            paused: false,
            next_word_at: Instant::now(),
            words_read: 0,
            peak_wpm: wpm,
            time_paused: Duration::ZERO,
            pause_started: None,
        }
    }

    pub fn resume_from(words: Vec<Word>, wpm: u16, index: u64) -> Self {
        let cursor = (index as usize).min(words.len().saturating_sub(1));
        let mut engine = Self::new(words, wpm);
        engine.cursor = cursor;
        engine
    }

    pub fn tick(&mut self, now: Instant) -> Option<&Word> {
        if self.paused || self.cursor >= self.words.len() {
            return None;
        }
        if now < self.next_word_at {
            return None;
        }
        let word = &self.words[self.cursor];
        let is_first = self.cursor == 0;
        let delay = word_time(&word.text, word.pause, self.wpm, is_first);
        self.next_word_at = now + delay;
        self.words_read += 1;
        self.cursor += 1;
        self.words.get(self.cursor - 1)
    }

    pub fn apply(&mut self, action: Action, now: Instant) {
        match action {
            Action::TogglePause => self.toggle_pause(now),
            Action::ChangeSpeed(factor) => {
                self.wpm = ((self.wpm as f32 * factor) as u16).clamp(100, 1200);
                if self.wpm > self.peak_wpm {
                    self.peak_wpm = self.wpm;
                }
            }
            Action::SetWpm(wpm) => {
                self.wpm = wpm.clamp(100, 1200);
                if self.wpm > self.peak_wpm {
                    self.peak_wpm = self.wpm;
                }
            }
            Action::SeekWords(delta) => {
                let new = self.cursor as i64 + delta;
                self.cursor = new.clamp(0, self.words.len() as i64) as usize;
                self.next_word_at = now;
            }
            Action::GotoStart => {
                self.cursor = 0;
                self.next_word_at = now;
            }
            Action::GotoEnd => {
                self.cursor = self.words.len().saturating_sub(1);
                self.next_word_at = now;
            }
            Action::GotoPercent(pct) => {
                let idx = (self.words.len() as f32 * pct as f32 / 100.0) as usize;
                self.cursor = idx.min(self.words.len().saturating_sub(1));
                self.next_word_at = now;
            }
            _ => {}
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn is_finished(&self) -> bool {
        self.cursor >= self.words.len()
    }

    pub fn wpm(&self) -> u16 {
        self.wpm
    }

    pub fn progress_percent(&self) -> u8 {
        if self.words.is_empty() {
            return 100;
        }
        ((self.cursor as f32 / self.words.len() as f32) * 100.0) as u8
    }

    pub fn current_position(&self) -> WordPosition {
        self.words
            .get(self.cursor)
            .map(|w| w.position)
            .unwrap_or_default()
    }

    pub fn context_words(&self, n: usize) -> &[Word] {
        let end = self.cursor.saturating_sub(1);
        let start = end.saturating_sub(n);
        &self.words[start..end]
    }

    pub fn words_read(&self) -> u64 {
        self.words_read
    }

    pub fn peak_wpm(&self) -> u16 {
        self.peak_wpm
    }

    fn toggle_pause(&mut self, now: Instant) {
        if self.paused {
            if let Some(started) = self.pause_started.take() {
                self.time_paused += now.duration_since(started);
            }
            self.next_word_at = now;
        } else {
            self.pause_started = Some(now);
        }
        self.paused = !self.paused;
    }
}
