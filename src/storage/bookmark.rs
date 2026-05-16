use anyhow::Result;
use rusqlite::Connection;

use super::models::BookmarkRecord;

pub fn save(conn: &Connection, bm: &BookmarkRecord) -> Result<()> {
    conn.execute(
        "INSERT INTO bookmarks (file_id, word_index, context_hash, next_word_text, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(file_id) DO UPDATE SET
           word_index = excluded.word_index,
           context_hash = excluded.context_hash,
           next_word_text = excluded.next_word_text,
           updated_at = excluded.updated_at",
        rusqlite::params![
            bm.file_id,
            bm.word_index,
            bm.context_hash,
            bm.next_word_text,
            bm.updated_at,
        ],
    )?;
    Ok(())
}

pub fn load(conn: &Connection, file_id: &str) -> Result<Option<BookmarkRecord>> {
    let mut stmt = conn.prepare(
        "SELECT file_id, word_index, context_hash, next_word_text, updated_at
         FROM bookmarks WHERE file_id = ?1",
    )?;
    let mut rows = stmt.query_map([file_id], |row| {
        Ok(BookmarkRecord {
            file_id: row.get(0)?,
            word_index: row.get(1)?,
            context_hash: row.get(2)?,
            next_word_text: row.get(3)?,
            updated_at: row.get(4)?,
        })
    })?;
    Ok(rows.next().transpose()?)
}

pub fn delete(conn: &Connection, file_id: &str) -> Result<()> {
    conn.execute("DELETE FROM bookmarks WHERE file_id = ?1", [file_id])?;
    Ok(())
}
