use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{bail, Result};

use crate::config::Config;
use crate::core::tokenizer::tokenize;
use crate::paths;
use crate::readers::factory::open_file;
use crate::readers::stdin::StdinReader;
use crate::readers::base::Reader;
use crate::storage::bookmark::{load as load_bookmark, save as save_bookmark};
use crate::storage::db;
use crate::storage::fingerprint::compute_file_id;
use crate::storage::library::upsert_file;
use crate::storage::models::{BookmarkRecord, FileRecord};
use crate::ui::app::App;
use crate::ui::theme::Theme;
use crate::ui::tui;

use super::args::ReadArgs;

pub fn run(args: ReadArgs) -> Result<()> {
    let config = Config::load();
    let wpm = args.wpm.unwrap_or(config.reading.default_wpm);

    let theme = match config.ui.theme.as_str() {
        "light" => Theme::light(),
        _ => Theme::dark(),
    };

    let conn = db::open(&paths::db_path())?;

    let (words, file_id) = if args.stdin {
        let mut reader = StdinReader::read()?;
        let paras = reader.paragraphs().collect::<Result<Vec<_>>>()?;
        let words = tokenize(paras.into_iter());
        (words, None)
    } else {
        let path = args.file.as_ref().ok_or_else(|| anyhow::anyhow!("file or --stdin required"))?;
        if !path.exists() {
            bail!("File not found: {}", path.display());
        }

        let fid = compute_file_id(path)?;
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

        let mut reader = open_file(path)?;
        let paras = reader.paragraphs().collect::<Result<Vec<_>>>()?;
        let words = tokenize(paras.into_iter());

        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
        upsert_file(&conn, &FileRecord {
            file_id: fid.clone(),
            original_name: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
            last_path: path.to_string_lossy().to_string(),
            format: ext,
            total_words: words.len() as i64,
            added_at: now,
        })?;

        (words, Some(fid))
    };

    let start_index = if args.restart {
        0
    } else if let Some(idx) = args.word_index {
        idx
    } else if let Some(ref fid) = file_id {
        load_bookmark(&conn, fid)?.map(|b| b.word_index as u64).unwrap_or(0)
    } else {
        0
    };

    let app = App::new(words, wpm, start_index, theme);
    let final_pos = tui::run(app)?;

    if let (Some(pos), Some(fid)) = (final_pos, file_id) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
        save_bookmark(&conn, &BookmarkRecord {
            file_id: fid,
            word_index: pos as i64,
            context_hash: String::new(),
            next_word_text: String::new(),
            updated_at: now,
        })?;
    }

    Ok(())
}
