use std::fs;
use std::path::Path;

use anyhow::Result;

use super::base::Reader;

pub struct TxtReader {
    content: String,
}

impl TxtReader {
    pub fn open(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(Self { content })
    }
}

impl Reader for TxtReader {
    fn paragraphs(&mut self) -> Box<dyn Iterator<Item = Result<String>> + '_> {
        let paras: Vec<Result<String>> = self
            .content
            .split("\n\n")
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .map(Ok)
            .collect();
        Box::new(paras.into_iter())
    }
}
