use anyhow::Result;
use rusqlite::Connection;

use super::models::SessionRecord;

pub fn insert_session(conn: &Connection, s: &SessionRecord) -> Result<()> {
    conn.execute(
        "INSERT INTO sessions
         (file_id, started_at, ended_at, words_read, avg_wpm, peak_wpm, time_paused_sec)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![
            s.file_id,
            s.started_at,
            s.ended_at,
            s.words_read,
            s.avg_wpm,
            s.peak_wpm,
            s.time_paused_sec,
        ],
    )?;
    Ok(())
}

pub fn summary(conn: &Connection, since: i64) -> Result<(i64, f64, i64, i64)> {
    let row = conn.query_row(
        "SELECT COALESCE(SUM(words_read),0), COALESCE(AVG(avg_wpm),0),
                COALESCE(MAX(peak_wpm),0),  COALESCE(COUNT(*),0)
         FROM sessions WHERE started_at >= ?1",
        [since],
        |r| Ok((r.get::<_, i64>(0)?, r.get::<_, f64>(1)?, r.get::<_, i64>(2)?, r.get::<_, i64>(3)?)),
    )?;
    Ok(row)
}
