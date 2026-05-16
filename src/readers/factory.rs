use std::path::Path;

use anyhow::{bail, Result};

use super::base::Reader;
use super::md::MdReader;
use super::txt::TxtReader;

pub fn open_file(path: &Path) -> Result<Box<dyn Reader>> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "txt" | "text" => Ok(Box::new(TxtReader::open(path)?)),
        "md" | "markdown" => Ok(Box::new(MdReader::open(path)?)),
        other => bail!("Unsupported format: .{other}"),
    }
}
