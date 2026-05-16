#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Normal { count: Option<u64> },
    Command { buffer: String },
    Search { buffer: String, forward: bool },
    Pause,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Normal { count: None }
    }
}
