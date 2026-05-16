use std::io::{self, BufRead};

use anyhow::Result;

use super::base::Reader;

pub struct StdinReader {
    lines: Vec<String>,
}

impl StdinReader {
    pub fn read() -> Result<Self> {
        let stdin = io::stdin();
        let lines: Vec<String> = stdin.lock().lines().collect::<std::io::Result<_>>()?;
        Ok(Self { lines })
    }
}

impl Reader for StdinReader {
    fn paragraphs(&mut self) -> Box<dyn Iterator<Item = Result<String>> + '_> {
        let paras: Vec<Result<String>> = self
            .lines
            .join("\n")
            .split("\n\n")
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .map(Ok)
            .collect();
        Box::new(paras.into_iter())
    }
}
