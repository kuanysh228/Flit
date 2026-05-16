use anyhow::Result;
use rusqlite::Connection;

use super::models::FileRecord;

pub fn upsert_file(conn: &Connection, rec: &FileRecord) -> Result<()> {
    conn.execute(
        "INSERT INTO files (file_id, original_name, last_path, format, total_words, added_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(file_id) DO UPDATE SET
           last_path = excluded.last_path,
           total_words = excluded.total_words",
        rusqlite::params![
            rec.file_id,
            rec.original_name,
            rec.last_path,
            rec.format,
            rec.total_words,
            rec.added_at,
        ],
    )?;
    Ok(())
}

pub fn list_files(conn: &Connection) -> Result<Vec<FileRecord>> {
    let mut stmt = conn.prepare(
        "SELECT file_id, original_name, last_path, format, total_words, added_at
         FROM files ORDER BY added_at DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(FileRecord {
            file_id: row.get(0)?,
            original_name: row.get(1)?,
            last_path: row.get(2)?,
            format: row.get(3)?,
            total_words: row.get(4)?,
            added_at: row.get(5)?,
        })
    })?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

pub fn delete_file(conn: &Connection, file_id: &str) -> Result<()> {
    conn.execute("DELETE FROM files WHERE file_id = ?1", [file_id])?;
    Ok(())
}
