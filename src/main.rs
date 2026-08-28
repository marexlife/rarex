use std::path::PathBuf;

use crate::actions::app::{self, RarexErrKind};

mod actions;
mod types;

fn main() {
    let result = app::run();

    if let Err(e) = result {
        eprintln!("{e:?}")
    }
}
