pub(crate) enum FileItem {}

#[must_use]
pub(crate) struct ParsedFile {
    file_items: Vec<FileItem>,
}

impl ParsedFile {
    pub fn new() -> Self {
        Self { file_items: vec![] }
    }
}
