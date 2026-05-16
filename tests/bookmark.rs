use std::path::PathBuf;

use flit::storage::bookmark::{load, save};
use flit::storage::db;
use flit::storage::models::BookmarkRecord;
use tempfile::tempdir;

fn make_conn() -> (tempfile::TempDir, rusqlite::Connection) {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test.sqlite3");
    let conn = db::open(&path).unwrap();
    (dir, conn)
}

#[test]
fn save_and_load_bookmark() {
    let (_dir, conn) = make_conn();

    flit::storage::library::upsert_file(&conn, &flit::storage::models::FileRecord {
        file_id: "abc123".to_string(),
        original_name: "test.txt".to_string(),
        last_path: "/tmp/test.txt".to_string(),
        format: "txt".to_string(),
        total_words: 100,
        added_at: 0,
    }).unwrap();

    let bm = BookmarkRecord {
        file_id: "abc123".to_string(),
        word_index: 42,
        context_hash: "deadbeef".to_string(),
        next_word_text: "hello".to_string(),
        updated_at: 12345,
    };

    save(&conn, &bm).unwrap();
    let loaded = load(&conn, "abc123").unwrap().unwrap();

    assert_eq!(loaded.word_index, 42);
    assert_eq!(loaded.next_word_text, "hello");
}

#[test]
fn load_missing_returns_none() {
    let (_dir, conn) = make_conn();
    let result = load(&conn, "nonexistent").unwrap();
    assert!(result.is_none());
}

#[test]
fn save_overwrites_existing() {
    let (_dir, conn) = make_conn();

    flit::storage::library::upsert_file(&conn, &flit::storage::models::FileRecord {
        file_id: "abc123".to_string(),
        original_name: "test.txt".to_string(),
        last_path: "/tmp/test.txt".to_string(),
        format: "txt".to_string(),
        total_words: 100,
        added_at: 0,
    }).unwrap();

    let bm1 = BookmarkRecord {
        file_id: "abc123".to_string(),
        word_index: 10,
        context_hash: "aaa".to_string(),
        next_word_text: "first".to_string(),
        updated_at: 1,
    };
    save(&conn, &bm1).unwrap();

    let bm2 = BookmarkRecord {
        file_id: "abc123".to_string(),
        word_index: 99,
        context_hash: "bbb".to_string(),
        next_word_text: "second".to_string(),
        updated_at: 2,
    };
    save(&conn, &bm2).unwrap();

    let loaded = load(&conn, "abc123").unwrap().unwrap();
    assert_eq!(loaded.word_index, 99);
}
