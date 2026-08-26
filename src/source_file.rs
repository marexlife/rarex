use crate::{source_file, lex::SourceCode};
use std::file;

pub(crate) enum FileErr {}

pub(crate) struct SourceFile {
    source_code: String,
}

impl SourceFile {
    pub(crate) fn new() -> Self {
        Self {
            source_code: String::new(),
        }
    }

    pub(crate) fn fill(self) -> Result<SourceCode, FileErr> {

        Ok(SourceCode::new(self.source_code))
    }
}
