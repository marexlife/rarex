use crate::types::source_code::SourceCode;

#[derive(Debug)]
pub(crate) enum FileReaderErr {}

pub(crate) struct SourceFileReader {
    source_code: String,
}

impl SourceFileReader {
    pub(crate) fn new() -> Self {
        Self {
            source_code: String::new(),
        }
    }

    pub(crate) fn read(self) -> Result<SourceCode, FileReaderErr> {
        Ok(SourceCode::new(self.source_code))
    }
}
