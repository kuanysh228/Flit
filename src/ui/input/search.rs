pub struct SearchState {
    pub query: String,
    pub matches: Vec<usize>,
    pub current: usize,
}

impl SearchState {
    pub fn new(query: String, word_texts: &[String]) -> Self {
        let q = query.to_lowercase();
        let matches = word_texts
            .iter()
            .enumerate()
            .filter(|(_, w)| w.to_lowercase().contains(&q))
            .map(|(i, _)| i)
            .collect();
        Self { query, matches, current: 0 }
    }

    pub fn next_from(&self, cursor: usize) -> Option<usize> {
        self.matches.iter().find(|&&i| i > cursor).copied()
    }

    pub fn prev_from(&self, cursor: usize) -> Option<usize> {
        self.matches.iter().rev().find(|&&i| i < cursor).copied()
    }
}
