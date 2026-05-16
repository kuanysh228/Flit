use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::Result;
use sha2::{Digest, Sha256};
use unicode_segmentation::UnicodeSegmentation;

pub fn compute_file_id(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let size = file.metadata()?.len();
    let mut buf = vec![0u8; 65536];
    let n = file.read(&mut buf)?;
    buf.truncate(n);

    let mut hasher = Sha256::new();
    hasher.update(&buf);
    hasher.update(b"\x00");
    hasher.update(size.to_le_bytes());
    Ok(hex::encode(&hasher.finalize()[..8]))
}

pub fn compute_context_hash(words: &[&str]) -> String {
    let normalized: String = words
        .iter()
        .map(|w| {
            w.graphemes(true)
                .filter(|g| g.chars().all(|c| c.is_alphabetic()))
                .collect::<String>()
                .to_lowercase()
        })
        .collect::<Vec<_>>()
        .join(" ");

    let mut hasher = Sha256::new();
    hasher.update(normalized.as_bytes());
    hex::encode(&hasher.finalize()[..8])
}
