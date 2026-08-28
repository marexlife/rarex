use std::path::PathBuf;

use crate::actions::app;

mod actions;
mod types;

use clap::Parser;

fn main() {
    app::run();
}
