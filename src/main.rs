use std::{error::Error, fs::File, io::Read};

use crate::{
    actions::lex, types::source_code::SourceCode,
};

mod actions;
mod tests;
mod types;

fn main() -> Result<(), Box<dyn Error>> {
    let args =
        std::env::args().collect::<Vec<String>>();

    for arg in args {
        let mut buf: [u8; 10] = [0; 10];
        let mut file = File::open(arg)?;
        match file.read(&mut buf) {
            Ok(a) => println!("{a}"),
            Err(e) => return Err(Box::new(e)),
        };

        let _ = lex::lex(SourceCode::new(
            String::from_utf8(buf.to_vec())?,
        ));

    }

    Ok(())
}
