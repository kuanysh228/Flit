use anyhow::Result;

use crate::paths;
use crate::storage::db;
use crate::storage::library::list_files;

pub fn run() -> Result<()> {
    let conn = db::open(&paths::db_path())?;
    let files = list_files(&conn)?;

    if files.is_empty() {
        println!("No files in library.");
        return Ok(());
    }

    println!("{:<20}  {:>8}  {}", "Name", "Words", "Path");
    println!("{}", "-".repeat(60));
    for f in files {
        println!("{:<20}  {:>8}  {}", f.original_name, f.total_words, f.last_path);
    }
    Ok(())
}
