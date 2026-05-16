use thiserror::Error;

#[derive(Debug, Error)]
pub enum FlitError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Db(#[from] rusqlite::Error),

    #[error("Format not supported: {0}")]
    UnsupportedFormat(String),

    #[error("Terminal too narrow (minimum 40 columns)")]
    TerminalTooNarrow,

    #[error("Bookmark not found")]
    BookmarkNotFound,

    #[error("File not found: {0}")]
    FileNotFound(String),
}
