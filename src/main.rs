use std::path::PathBuf;

use crate::actions::app;

mod actions;
mod types;

fn main() {
    app::run();
}
