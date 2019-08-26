use crate::*;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn json(args: Result<Document>) -> Result<Document> {
    Ok(Document::String(
        serde_json::to_string(&args?).unwrap_or_default(),
    ))
}

pub fn yaml(args: Result<Document>) -> Result<Document> {
    Ok(Document::String(
        serde_yaml::to_string(&args?).unwrap_or_default(),
    ))
}

pub fn file(args: Result<Document>) -> Result<Document> {
    let path: PathBuf = args?.to_string().into();
    let mut f = File::open(path).unwrap();
    let mut result = String::new();
    f.read_to_string(&mut result).unwrap();
    Ok(Document::String(result))
}
