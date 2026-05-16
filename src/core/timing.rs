use std::time::Duration;
use unicode_segmentation::UnicodeSegmentation;

use crate::core::word::Pause;

pub fn word_time(text: &str, pause: Pause, wpm: u16, is_first: bool) -> Duration {
    let base_secs = 60.0 / wpm as f64;
    let graphemes = text.graphemes(true).count() as f64;

    let factor = if ends_with_fullstop(text) || pause == Pause::FullStop {
        3.0
    } else if ends_with_comma(text) || pause == Pause::Comma {
        2.0
    } else if pause == Pause::Multiword {
        1.2
    } else {
        0.9 + graphemes.sqrt() * 0.04
    };

    let secs = base_secs * factor;

    if is_first {
        Duration::from_secs_f64(secs.max(0.2))
    } else {
        Duration::from_secs_f64(secs)
    }
}

fn ends_with_comma(text: &str) -> bool {
    matches!(text.chars().last(), Some(',' | ';' | ':'))
}

fn ends_with_fullstop(text: &str) -> bool {
    matches!(text.chars().last(), Some('.' | '?' | '!'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_word_at_300wpm() {
        let d = word_time("hello", Pause::Normal, 300, false);
        assert!(d.as_secs_f64() > 0.15 && d.as_secs_f64() < 0.25);
    }

    #[test]
    fn fullstop_triples_time() {
        let normal = word_time("hi", Pause::Normal, 300, false);
        let stop = word_time("hi.", Pause::FullStop, 300, false);
        assert!(stop.as_secs_f64() > normal.as_secs_f64() * 2.5);
    }

    #[test]
    fn first_word_minimum() {
        let d = word_time("a", Pause::Normal, 1200, true);
        assert!(d.as_millis() >= 200);
    }
}
