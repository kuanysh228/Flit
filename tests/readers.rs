use flit::core::tokenizer::tokenize;
use flit::readers::base::Reader;
use flit::readers::md::MdReader;
use flit::readers::txt::TxtReader;

mod common;

#[test]
fn txt_reader_extracts_words() {
    let path = common::fixtures_dir().join("sample.txt");
    let mut reader = TxtReader::open(&path).unwrap();
    let paras: Vec<String> = reader.paragraphs().map(|p| p.unwrap()).collect();
    assert!(!paras.is_empty());
    let words = tokenize(paras.into_iter());
    assert!(words.len() > 10);
}

#[test]
fn txt_reader_preserves_punctuation_pause() {
    use flit::core::word::Pause;
    let path = common::fixtures_dir().join("sample.txt");
    let mut reader = TxtReader::open(&path).unwrap();
    let paras: Vec<String> = reader.paragraphs().map(|p| p.unwrap()).collect();
    let words = tokenize(paras.into_iter());
    let has_fullstop = words.iter().any(|w| w.pause == Pause::FullStop);
    assert!(has_fullstop, "should detect sentence-ending punctuation");
}

#[test]
fn md_reader_strips_markup() {
    let path = common::fixtures_dir().join("sample.md");
    let mut reader = MdReader::open(&path).unwrap();
    let paras: Vec<String> = reader.paragraphs().map(|p| p.unwrap()).collect();
    let all_text = paras.join(" ");
    assert!(!all_text.contains("**"), "bold markers should be stripped");
    assert!(!all_text.contains("##"), "headings should be stripped");
}

#[test]
fn md_reader_produces_words() {
    let path = common::fixtures_dir().join("sample.md");
    let mut reader = MdReader::open(&path).unwrap();
    let paras: Vec<String> = reader.paragraphs().map(|p| p.unwrap()).collect();
    let words = tokenize(paras.into_iter());
    assert!(words.len() > 5);
}
