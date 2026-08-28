pub(crate) enum FileItem {}

#[must_use]
pub(crate) struct ParsedCode {
    file_items: Vec<FileItem>,
}

impl ParsedCode {
    pub fn new() -> Self {
        Self { file_items: vec![] }
    }
}
