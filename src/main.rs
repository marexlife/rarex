#[forbid(unsafe_code, clippy::unwrap_used, clippy::expect_used)]
use app::{CompilerErr, compile};

mod app;
mod fs;
mod lexer;
mod source_code;
mod tok;
mod tok_stream;

fn main() -> Result<(), CompilerErr> {
    compile()
}
