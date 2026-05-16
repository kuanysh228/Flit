use anyhow::Result;

pub struct DocumentMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
}

pub trait Reader {
    fn paragraphs(&mut self) -> Box<dyn Iterator<Item = Result<String>> + '_>;
    fn metadata(&self) -> Option<DocumentMetadata> { None }
}
