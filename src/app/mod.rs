use crate::fs::{SourceFileReader, SourceFileReaderErr};
use crate::lexer::LexerErr;

#[derive(Debug)]
pub(crate) enum CompilerErr {
    SourceFileReaderErr(SourceFileReaderErr),
    LexerErr(LexerErr),
    ParserErr,
}

#[must_use]
enum AppModeKind {
    ShellMode,
    FileMode,
}

pub(crate) struct App {
    app_mode_kind: AppModeKind,
}

impl App {
    pub(crate) fn new() -> Self {
        Self { app_mode_kind: }
    }

    pub(crate) fn run() {}

    fn compile() -> Result<(), CompilerErr> {
        let source_file = SourceFileReader::new();

        let source_code = match source_file.fill() {
            Ok(v) => v,
            Err(e) => return Err(CompilerErr::SourceFileReaderErr(e)),
        };

        let token_stream = match source_code.lex() {
            Ok(v) => v,
            Err(e) => return Err(CompilerErr::LexerErr(e)),
        };

        token_stream;

        Ok(())
    }
}
