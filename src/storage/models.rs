#[derive(Debug, Clone)]
pub struct FileRecord {
    pub file_id: String,
    pub original_name: String,
    pub last_path: String,
    pub format: String,
    pub total_words: i64,
    pub added_at: i64,
}

#[derive(Debug, Clone)]
pub struct BookmarkRecord {
    pub file_id: String,
    pub word_index: i64,
    pub context_hash: String,
    pub next_word_text: String,
    pub updated_at: i64,
}

#[derive(Debug, Clone)]
pub struct SessionRecord {
    pub id: i64,
    pub file_id: String,
    pub started_at: i64,
    pub ended_at: i64,
    pub words_read: i64,
    pub avg_wpm: f64,
    pub peak_wpm: i64,
    pub time_paused_sec: f64,
}
