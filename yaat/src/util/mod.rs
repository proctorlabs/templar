use super::*;
use std::fs::File;
use std::io::prelude::*;

pub fn read_file(path: &PathBuf) -> Result<String> {
    let mut file = File::open(path)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    Ok(result)
}
