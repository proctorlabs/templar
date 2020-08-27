use super::*;
use std::error::Error;
use structopt::clap::AppSettings::*;
use structopt::StructOpt;

pub fn parse() -> Result<YaatArgs> {
    Ok(YaatArgs::from_args())
}

#[derive(StructOpt, Debug, Default, Clone)]
#[structopt(
    name = "yaat",
    rename_all = "kebab_case",
    author,
    about,
    settings = &[DeriveDisplayOrder, DisableHelpSubcommand, UnifiedHelpMessage, TrailingVarArg]
)]
pub struct YaatArgs {
    /// Override a `.yaatfile` variable
    #[structopt(short, long, parse(try_from_str = parse_key_val), number_of_values = 1)]
    pub set: Vec<(String, String)>,

    /// Override the default `.yaatfile`
    #[structopt(short = "f", long, parse(from_os_str))]
    pub yaatfile: Option<PathBuf>,

    /// Enable debug logging
    #[structopt(short, long)]
    pub debug: bool,

    /// List available tasks
    #[structopt(short, long)]
    pub list: bool,

    /// The task to run
    #[structopt(name = "TASK")]
    pub taskname: String,

    /// The task to run
    #[structopt(name = "TASKARGS", multiple=true)]
    pub taskargs: Vec<String>,
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
