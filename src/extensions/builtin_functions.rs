use crate::*;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;
use std::str;

pub fn json(args: TemplarResult) -> TemplarResult {
    Ok(serde_json::from_str(&args?.to_string()).wrap()?)
}

pub fn yaml(args: TemplarResult) -> TemplarResult {
    Ok(serde_yaml::from_str(&args?.to_string()).wrap()?)
}

pub fn file(args: TemplarResult) -> TemplarResult {
    let path: PathBuf = args?.to_string().into();
    let mut f = File::open(path)?;
    let mut result = String::new();
    f.read_to_string(&mut result)?;
    Ok(result.into())
}

pub fn env(args: TemplarResult) -> TemplarResult {
    let env_name = args?.to_string();
    Ok(std::env::var(env_name).unwrap_or_default().into())
}

pub fn script(args: TemplarResult) -> TemplarResult {
    let mut sh_args = vec![Document::String("sh".into()), "-c".into()];
    match args? {
        Document::Seq(s) => {
            for arg in s.iter() {
                sh_args.push(arg.clone())
            }
        }
        other => sh_args.push(other),
    }
    command(Ok(sh_args.into()))
}

pub fn command(args: TemplarResult) -> TemplarResult {
    let mut sh_args = vec![];
    match args? {
        Document::Seq(s) => {
            for arg in s.iter() {
                sh_args.push(arg.to_string())
            }
        }
        other => sh_args.push(other.to_string()),
    }
    let result = Command::new("/usr/bin/env").args(sh_args).output()?;
    let mut map = BTreeMap::<Document, Document>::new();
    map.insert(
        "stdout".into(),
        str::from_utf8(&result.stdout).unwrap_or_default().into(),
    );
    map.insert(
        "stderr".into(),
        str::from_utf8(&result.stderr).unwrap_or_default().into(),
    );
    map.insert(
        "status".into(),
        result.status.code().unwrap_or_default().into(),
    );
    Ok(map.into())
}
