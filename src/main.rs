#[forbid(unsafe_code, clippy::unwrap_used, clippy::expect_used)]
use crate::actions::app;

mod actions;
mod types;

fn main() {
    app::run()
}
