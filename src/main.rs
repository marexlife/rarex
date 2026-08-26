#[forbid(unsafe_code, clippy::unwrap_used, clippy::expect_used)]
use crate::source_file::SourceFile;

mod lex;
mod parse;
mod source_file;
mod tok;

fn main() -> anyhow::Result<()> {
    SourceFile::new().fill();

    Ok(())
}
