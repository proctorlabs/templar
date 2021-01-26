mod command;
mod parser;
mod prelude;

pub use prelude::*;

fn main() -> Result<()> {
    parser::parse()?;
    Ok(())
}
