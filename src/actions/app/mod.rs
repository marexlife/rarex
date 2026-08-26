use crate::actions::fs::{FileReaderErr, SourceFileReader};
use crate::actions::lexer::{self, LexerErr};
use crate::actions::parser::{self, ParserErr};
use crate::types::source_code::SourceCode;

#[derive(Debug)]
pub(crate) enum CompilerErr {
    FileReaderErr(FileReaderErr),
    LexerErr(LexerErr),
    ParserErr(ParserErr),
}

#[must_use]
enum AppModeKind {
    ShellMode,
    FileMode(Vec<SourceCode>),
}

pub(crate) struct App;

impl App {
    pub(crate) fn run() {}

    fn run_shell_mode() {}

    fn run_file_mode(source_codes: Vec<SourceCode>) {
        source_codes.into_iter().map(Self::compile)
    }

    fn read_file() -> Result<SourceCode, FileReaderErr> {
        let source_file_reader = SourceFileReader::new();

        source_file_reader.read()
    }

    fn compile(source_code: SourceCode) -> Result<AstNode, CompilerErr> {
        let token_stream = match lexer::Lexer::new(source_code).lex() {
            Ok(v) => v,
            Err(e) => return Err(CompilerErr::LexerErr(e)),
        };

        parser::Parser::parse();

        Ok(())
    }
}
