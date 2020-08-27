mod args;
mod model;
mod runner;
mod util;

pub use args::YaatArgs;
use model::*;
use std::path::PathBuf;
pub use templar::{error::*, *};

fn main() -> Result<()> {
    let varg = args::parse()?;
    ::std::process::exit(match runner::run(varg) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{}", e);
            2
        }
    })
}
