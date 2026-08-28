use std::io;

use crate::actions::fs::{FileReaderErr, SourceFileReader};
use crate::actions::lexer::{self, LexerErr};
use crate::actions::parser::{self, ParserErr};
use crate::types::parsed_file::ParsedCode;
use crate::types::source_code::SourceCode;
use crate::types::token_stream::TokenStream;

#[derive(Debug)]
pub(crate) enum AppErrKind {
    FileReaderErr(FileReaderErr),
    UserInputFailed(io::Error),
    LexerErr(LexerErr),
    ParserErr(ParserErr),
    WriteErr(io::Error),
}

#[must_use]
enum AppModeKind {
    ShellMode,
    FileMode(Vec<SourceCode>),
}

pub(crate) fn run() {}

fn run_shell_mode() {
    loop {
        run_shell_mode_iter();
    }
}

fn take_user_input() -> Result<String, AppErrKind> {
    let input_stream = std::io::stdin();
    let mut input = String::new();

    match input_stream.read_line(&mut input) {
        Ok(_) => Ok(input),
        Err(e) => Err(AppErrKind::UserInputFailed(e)),
    }
}
fn take_user_code() -> Result<SourceCode, AppErrKind> {
    let user_input = take_user_input()?;

    Ok(SourceCode::new(user_input))
}

fn run_shell_mode_iter() -> Result<(), AppErrKind> {
    let user_code = take_user_code()?;

    interpret(user_code)
}

fn compile_files(source_codes: Vec<SourceCode>) -> Vec<Result<(), AppErrKind>> {
    source_codes.into_iter().map(compile).collect()
}

fn interpret(source_code: SourceCode) -> Result<(), AppErrKind> {
    let parsed_code = source_to_parsed(source_code)?;

    execute(parsed_code)
}

fn compile(source_code: SourceCode) -> Result<(), AppErrKind> {
    let parsed_code = source_to_parsed(source_code)?;

    write(parsed_code)
}

fn source_to_parsed(source_code: SourceCode) -> Result<ParsedCode, AppErrKind> {
    let token_stream = lex(source_code)?;

    parse(token_stream)
}

fn write(parsed_code: ParsedCode) -> Result<(), AppErrKind> {
    Ok(())
}

fn execute(parsed_code: ParsedCode) -> Result<(), AppErrKind> {
    Ok(())
}

fn read_file() -> Result<SourceCode, AppErrKind> {
    let source_code = SourceFileReader::new().read();

    match source_code {
        Ok(v) => Ok(v),
        Err(e) => Err(AppErrKind::FileReaderErr(e)),
    }
}

fn lex(source_code: SourceCode) -> Result<TokenStream, AppErrKind> {
    match lexer::Lexer::new().lex(source_code) {
        Ok(v) => Ok(v),
        Err(e) => return Err(AppErrKind::LexerErr(e)),
    }
}

fn parse(token_stream: TokenStream) -> Result<ParsedCode, AppErrKind> {
    match parser::Parser::parse(token_stream) {
        Ok(v) => Ok(v),
        Err(e) => Err(AppErrKind::ParserErr(e)),
    }
}
