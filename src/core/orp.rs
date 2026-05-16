use unicode_segmentation::UnicodeSegmentation;

pub fn find_orp(word: &str) -> usize {
    let len = word.graphemes(true).count();
    match len {
        0..=1 => 0,
        2..=3 => 1,
        4..=7 => 2,
        8..=13 => 3,
        _ => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_char() { assert_eq!(find_orp("a"), 0); }

    #[test]
    fn two_chars() { assert_eq!(find_orp("be"), 1); }

    #[test]
    fn four_chars() { assert_eq!(find_orp("word"), 2); }

    #[test]
    fn eight_chars() { assert_eq!(find_orp("longword"), 3); }

    #[test]
    fn fourteen_chars() { assert_eq!(find_orp("verylongworddd"), 4); }

    #[test]
    fn cyrillic() { assert_eq!(find_orp("привет"), 2); }

    #[test]
    fn empty_string() { assert_eq!(find_orp(""), 0); }
}
