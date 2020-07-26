use super::*;
use std::error::Error;
use std::path::PathBuf;
use structopt::clap::AppSettings::*;
use structopt::StructOpt;

impl Command {
    pub fn parse() -> Result<Self> {
        Ok(Self::from_args())
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "templar",
    rename_all = "kebab_case",
    author,
    about,
    settings = &[ArgRequiredElseHelp, DeriveDisplayOrder, DisableHelpSubcommand, UnifiedHelpMessage]
)]
pub struct Command {
    /// Output to send the result to, defaults to stdout.
    #[structopt(short = "o", long, parse(from_os_str))]
    pub destination: Option<PathBuf>,

    /// Template file or directory to process
    #[structopt(short, long, conflicts_with = "expr")]
    pub template: Option<PathBuf>,

    /// Evaluate a single expression instead of a full template
    #[structopt(
        short,
        long = "expression",
        name = "expression",
        conflicts_with = "template"
    )]
    pub expr: Option<String>,

    /// File to parse and load into the templating context as a dynamic input
    #[structopt(
        short,
        long = "dynamic",
        name = "dynamic",
        number_of_values = 1,
        multiple = true
    )]
    pub dynamic_input: Vec<PathBuf>,

    /// File to parse and load into the templating context
    #[structopt(short, long, number_of_values = 1, multiple = true)]
    pub input: Vec<PathBuf>,

    /// Directly set a variable on the context
    #[structopt(short, long, parse(try_from_str = parse_key_val), number_of_values = 1)]
    pub set: Vec<(String, String)>,

    /// Allow directories to be recursively processed
    #[structopt(short, long)]
    pub recursive: bool,

    /// Overwrite target if it already exists
    #[structopt(short, long)]
    pub force: bool,
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
