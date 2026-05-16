use std::fs;
use std::path::Path;

use anyhow::Result;

use super::base::Reader;

pub struct MdReader {
    content: String,
}

impl MdReader {
    pub fn open(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(Self { content })
    }
}

impl Reader for MdReader {
    fn paragraphs(&mut self) -> Box<dyn Iterator<Item = Result<String>> + '_> {
        let paras: Vec<Result<String>> = self
            .content
            .lines()
            .map(strip_markdown)
            .collect::<Vec<_>>()
            .join("\n")
            .split("\n\n")
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .map(Ok)
            .collect();
        Box::new(paras.into_iter())
    }
}

fn strip_markdown(line: &str) -> String {
    let line = line.trim_start_matches('#').trim();
    let line = line.trim_start_matches(|c| c == '*' || c == '-' || c == '>');
    line.trim().to_string()
}
