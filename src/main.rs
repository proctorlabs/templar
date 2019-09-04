pub use templar::error::*;
pub use templar::*;

mod cli;

fn main() {
    ::std::process::exit(match cli::run() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Failure!");
            eprintln!("➜ {}", e);
            2
        }
    })
}
