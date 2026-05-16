CREATE TABLE IF NOT EXISTS files (
    file_id       TEXT PRIMARY KEY,
    original_name TEXT NOT NULL,
    last_path     TEXT NOT NULL,
    format        TEXT NOT NULL,
    total_words   INTEGER NOT NULL DEFAULT 0,
    added_at      INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS bookmarks (
    file_id       TEXT PRIMARY KEY REFERENCES files(file_id) ON DELETE CASCADE,
    word_index    INTEGER NOT NULL,
    context_hash  TEXT NOT NULL,
    next_word_text TEXT NOT NULL,
    updated_at    INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
    id              INTEGER PRIMARY KEY,
    file_id         TEXT NOT NULL REFERENCES files(file_id) ON DELETE CASCADE,
    started_at      INTEGER NOT NULL,
    ended_at        INTEGER NOT NULL,
    words_read      INTEGER NOT NULL DEFAULT 0,
    avg_wpm         REAL NOT NULL DEFAULT 0,
    peak_wpm        INTEGER NOT NULL DEFAULT 0,
    time_paused_sec REAL NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS marks (
    file_id    TEXT NOT NULL REFERENCES files(file_id) ON DELETE CASCADE,
    mark_char  TEXT NOT NULL,
    word_index INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    PRIMARY KEY (file_id, mark_char)
);
