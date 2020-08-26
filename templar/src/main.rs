/*!
Templar CLI

# Usage

```bash
templar 0.1.0
Phil Proctor <philliptproctor@gmail.com>
Lightweight yet powerful templating

USAGE:
    templar [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>...    File to parse and load into the templating context
    -o, --output <output>     Output to send the result to, defaults to stdout
    -s, --set <set>...        Directly set a variable on the context

SUBCOMMANDS:
    expression    Run an expression directly
    template      Run a template
```
*/

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
