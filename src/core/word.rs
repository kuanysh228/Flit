#[derive(Debug, Clone, PartialEq)]
pub struct Word {
    pub text: String,
    pub orp_idx: usize,
    pub pause: Pause,
    pub position: WordPosition,
    pub is_multiword: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pause {
    Normal,
    Comma,
    FullStop,
    Multiword,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WordPosition {
    pub index: u64,
    pub byte_offset: u64,
}
