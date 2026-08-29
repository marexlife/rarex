use std::{
    error::Error, fs::File, io::Read, process::exit,
};

use crate::{
    actions::lex, types::source_code::SourceCode,
};

mod actions;
mod tests;
mod types;

fn main() -> Result<(), Box<dyn Error>> {
    let args =
        std::env::args().collect::<Vec<String>>();

    if args.len() == 0 {
        eprintln!("no file provided");

        exit(-1)
    }

    for arg in args {
        let mut buf: Vec<u8> = vec![];
        let mut file = File::open(arg)?;

        match file.read(&mut buf) {
            Ok(a) => eprintln!("{a} characters read"),
            Err(e) => return Err(Box::new(e)),
        };

        let code = String::from_utf8(buf)?;
        let _ = lex::lex(SourceCode::new(code));
    }

    Ok(())
}
