use unicode_segmentation::UnicodeSegmentation;

use crate::core::orp::find_orp;
use crate::core::word::{Pause, Word, WordPosition};

pub fn tokenize(paragraphs: impl Iterator<Item = String>) -> Vec<Word> {
    let mut words = Vec::new();
    let mut index: u64 = 0;
    let mut byte_offset: u64 = 0;

    for para in paragraphs {
        for raw in para.split_whitespace() {
            let token = raw.trim_matches(|c: char| c == '"' || c == '\'' || c == '(' || c == ')' || c == '[' || c == ']');
            if token.is_empty() {
                continue;
            }
            let display = strip_leading_punct(token);
            if display.is_empty() {
                continue;
            }
            let orp_idx = find_orp(display);
            let pause = detect_pause(display);
            words.push(Word {
                text: display.to_string(),
                orp_idx,
                pause,
                position: WordPosition { index, byte_offset },
                is_multiword: false,
            });
            byte_offset += token.len() as u64 + 1;
            index += 1;
        }
    }

    words
}

fn strip_leading_punct(s: &str) -> &str {
    s.trim_start_matches(|c: char| !c.is_alphanumeric())
}

fn detect_pause(word: &str) -> Pause {
    match word.chars().last() {
        Some('.' | '?' | '!') => Pause::FullStop,
        Some(',' | ';' | ':') => Pause::Comma,
        _ => Pause::Normal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_sentence() {
        let paras = vec!["Hello world.".to_string()];
        let words = tokenize(paras.into_iter());
        assert_eq!(words.len(), 2);
        assert_eq!(words[0].text, "Hello");
        assert_eq!(words[1].pause, Pause::FullStop);
    }

    #[test]
    fn indices_increment() {
        let paras = vec!["one two three".to_string()];
        let words = tokenize(paras.into_iter());
        assert_eq!(words[0].position.index, 0);
        assert_eq!(words[1].position.index, 1);
        assert_eq!(words[2].position.index, 2);
    }
}
