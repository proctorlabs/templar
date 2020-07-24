use super::*;
use std::fs::File;
use std::path::PathBuf;

pub fn read_stdin() -> Result<String> {
    let mut result = String::new();
    std::io::stdin().read_to_string(&mut result)?;
    Ok(result)
}

pub fn read_file(path: &PathBuf) -> Result<String> {
    let mut file = File::open(path)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    Ok(result)
}

pub fn write_stdout(contents: &str) -> Result<()> {
    print!("{}", contents);
    Ok(())
}

pub fn write_file(file: &PathBuf, contents: &str) -> Result<()> {
    let mut f = File::create(file)?;
    f.write_all(contents.as_bytes())?;
    Ok(())
}
