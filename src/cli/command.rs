use super::*;
use std::error::Error;
use std::path::PathBuf;
use structopt::clap::AppSettings::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "templar",
    rename_all = "kebab_case",
    author,
    about,
    settings = &[InferSubcommands, DisableHelpSubcommand, UnifiedHelpMessage, VersionlessSubcommands]
)]
pub enum Command {
    #[structopt(
        rename_all = "kebab_case", 
        settings = &[UnifiedHelpMessage]
    )]
    /// Execute an expression and render the output
    Expression {
        /// The expression to run
        text: String,
        #[structopt(flatten)]
        options: Options,
    },

    #[structopt(
        rename_all = "kebab_case", 
        settings = &[UnifiedHelpMessage]
    )]
    /// Execute a template and render the output
    Template {
        /// Template file(s) to open
        file: PathBuf,
        #[structopt(flatten)]
        options: Options,
    },
}

impl Command {
    pub fn parse() -> Result<Self> {
        Ok(Self::from_args())
    }
}

#[derive(StructOpt, Debug)]
pub struct Options {
    /// Directly set a variable on the context
    #[structopt(short, long, parse(try_from_str = parse_key_val), number_of_values = 1)]
    pub set: Vec<(String, String)>,

    /// File to parse and load into the templating context
    #[structopt(short, long, number_of_values = 1, multiple = true)]
    pub input: Vec<PathBuf>,

    /// File to parse and load into the templating context as a dynamic input
    #[structopt(
        short,
        long = "dynamic",
        name = "dynamic",
        number_of_values = 1,
        multiple = true
    )]
    pub dynamic_input: Vec<PathBuf>,

    /// Output to send the result to, defaults to stdout
    #[structopt(short, long, parse(from_os_str))]
    pub output: Option<PathBuf>,
}

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> std::result::Result<(T, U), Box<dyn Error>>
where
    T: std::str::FromStr,
    T::Err: Error + 'static,
    U: std::str::FromStr,
    U::Err: Error + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}
