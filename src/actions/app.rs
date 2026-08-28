use crate::actions::fs::{FileReaderErr, SourceFileReader};
use crate::actions::lexer::{self, LexerErr};
use crate::actions::parser::{self, ParserErr};
use crate::types::parsed_file::ParsedFile;
use crate::types::source_code::SourceCode;
use crate::types::token_stream::TokenStream;

#[derive(Debug)]
pub(crate) enum AppErrKind {
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

    fn run_file_mode(
        source_codes: Vec<Result<SourceCode, AppErrKind>>,
    ) -> Vec<Result<(), AppErrKind>> {
        source_codes
            .into_iter()
            .map(Self::lex)
            .map(Self::parse)
            .map(Self::execute)
            .collect()
    }

    fn execute(parsed_file: Result<ParsedFile, AppErrKind>) -> Result<(), AppErrKind> {
        let parsed_file = parsed_file?;

        Ok(())
    }

    fn read_file() -> Result<SourceCode, AppErrKind> {
        let source_file_reader = SourceFileReader::new();

        match source_file_reader.read() {
            Ok(v) => Ok(v),
            Err(e) => Err(AppErrKind::FileReaderErr(e)),
        }
    }

    fn lex(source_code: Result<SourceCode, AppErrKind>) -> Result<TokenStream, AppErrKind> {
        let source_code = source_code?;

        match lexer::Lexer::new().lex(source_code) {
            Ok(v) => Ok(v),
            Err(e) => return Err(AppErrKind::LexerErr(e)),
        }
    }

    fn parse(token_stream: Result<TokenStream, AppErrKind>) -> Result<ParsedFile, AppErrKind> {
        let token_stream = token_stream?;

        match parser::Parser::parse(token_stream) {
            Ok(v) => Ok(v),
            Err(e) => Err(AppErrKind::ParserErr(e)),
        }
    }
}
