#[forbid(unsafe_code, clippy::unwrap_used, clippy::expect_used)]
use crate::actions::app::App;

mod actions;
mod types;

fn main() {
    App::run()
}
