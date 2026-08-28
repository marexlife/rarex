use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::string::FromUtf8Error;
use std::{io, os, path};

use crate::actions::lexer::{self, LexerErr};
use crate::types::source_code::SourceCode;
use crate::types::token_stream::TokenStream;

#[derive(Debug)]
pub(crate) enum RarexErrKind {
    ReadError(io::Error),
    LexerErr(LexerErr),
}

pub(crate) fn run() -> Result<(), RarexErrKind> {
    eprintln!("run");

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Not enough arguments");

        exit(-1);
    }

    let mut source_codes = vec![];

    for arg in &args {
        println!("{arg}");
        let path = path::Path::new(arg);

        let mut contents = String::new();

        let mut file = match File::open(path) {
            Ok(v) => v,
            Err(e) => {
                return Err(RarexErrKind::ReadError(
                    e,
                ));
            }
        };

        match file.read_to_string(&mut contents) {
            Ok(_) => {}
            Err(e) => {
                return Err(RarexErrKind::ReadError(e));
            }
        };

        source_codes
            .push(SourceCode::new(&arg, contents));
    }

    compile_files(source_codes);

    Ok(())
}

fn compile_files(
    source_codes: Vec<SourceCode>,
) -> Vec<Result<(), RarexErrKind>> {
    eprintln!("starting compile");

    source_codes.into_iter().map(compile).collect()
}

fn compile(
    source_code: SourceCode,
) -> Result<(), RarexErrKind> {
    eprintln!("compiling {}", source_code.filepath());

    let _ = lex(source_code)?;

    Ok(())
}

fn lex(
    source_code: SourceCode,
) -> Result<TokenStream, RarexErrKind> {
    match lexer::Lexer::new().lex(source_code) {
        Ok(v) => Ok(v),
        Err(e) => {
            return Err(RarexErrKind::LexerErr(e));
        }
    }
}
