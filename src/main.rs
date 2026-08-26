use crate::actions::app::App;
#[forbid(unsafe_code, clippy::unwrap_used, clippy::expect_used)]
use crate::actions::app::CompilerErr;

mod actions;
mod types;

fn main() -> Result<(), CompilerErr> {
    App::compile()
}
