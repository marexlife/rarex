use std::io;
use std::process::exit;
use std::string::FromUtf8Error;

use crate::actions::lexer::{self, LexerErr};
use crate::actions::parser::{self, ParserErr};
use crate::types::parsed_file::ParsedCode;
use crate::types::source_code::SourceCode;
use crate::types::token_stream::TokenStream;

#[derive(Debug)]
pub(crate) enum RarexErrKind {
    FileError(io::Error),
    Utf8Error(FromUtf8Error),
    LexerErr(LexerErr),
}

pub(crate) fn run() -> Result<(), RarexErrKind> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Not enough arguments");

        exit(-1);
    }

    let mut source_codes = vec![];

    for e in args {
        let contents = match std::fs::read(e) {
            Ok(v) => v,
            Err(e) => return Err(RarexErrKind::FileError(e)),
        };

        let contents = match String::from_utf8(contents) {
            Ok(v) => v,
            Err(e) => return Err(RarexErrKind::Utf8Error(e)),
        };

        source_codes.push(SourceCode::new(contents));
    }

    compile_files(source_codes);

    Ok(())
}

fn compile_files(source_codes: Vec<SourceCode>) -> Vec<Result<(), RarexErrKind>> {
    source_codes.into_iter().map(compile).collect()
}

fn compile(source_code: SourceCode) -> Result<(), RarexErrKind> {
    let _ = lex(source_code)?;

    Ok(())
}

fn lex(source_code: SourceCode) -> Result<TokenStream, RarexErrKind> {
    match lexer::Lexer::new().lex(source_code) {
        Ok(v) => Ok(v),
        Err(e) => return Err(RarexErrKind::LexerErr(e)),
    }
}
