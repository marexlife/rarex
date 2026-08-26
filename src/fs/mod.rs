use crate::lexer::Lexer;

#[derive(Debug)]
pub(crate) enum SourceFileReaderErr {}

pub(crate) struct SourceFileReader {
    source_code: String,
}

impl SourceFileReader {
    pub(crate) fn new() -> Self {
        Self {
            source_code: String::new(),
        }
    }

    pub(crate) fn fill(self) -> Result<Lexer, SourceFileReaderErr> {
        Ok(Lexer::new(self.source_code))
    }
}
