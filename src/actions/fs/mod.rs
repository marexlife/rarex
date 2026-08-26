use crate::{lexer::Lexer, source_code::SourceCode};

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

    pub(crate) fn read(self) -> Result<SourceCode, SourceFileReaderErr> {
        Ok(SourceCode::new(self.source_code))
    }
}
