use crate::*;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;
use std::str;

#[cfg(feature = "json-extension")]
#[templar_function]
pub fn json(content: String) -> Result<InnerData> {
    serde_json::from_str(&content).wrap()
}

#[cfg(feature = "yaml-extension")]
#[templar_function]
pub fn yaml(content: String) -> Result<InnerData> {
    serde_yaml::from_str(&content).wrap()
}

#[templar_function]
pub fn file(path: String) -> Result<String> {
    let path: PathBuf = PathBuf::from(path);
    let mut file = File::open(path)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    Ok(result)
}

#[templar_function]
pub fn env(env_var: String) -> Result<String> {
    std::env::var(env_var).wrap()
}

pub fn script(args: Data) -> Data {
    let mut sh_args = vec![InnerData::String("sh".into()), "-c".into()];
    match args.into_result().map(|i| i.into_inner()) {
        Ok(InnerData::Seq(s)) => {
            for arg in s.iter() {
                sh_args.push(arg.clone())
            }
        }
        Err(e) => return e.into(),
        Ok(other) => sh_args.push(other),
    }
    command(sh_args.into())
}

pub fn command(args: Data) -> Data {
    let mut sh_args = vec![];
    match args.into_result().map(|i| i.into_inner()) {
        Ok(InnerData::Seq(s)) => {
            for arg in s.iter() {
                sh_args.push(arg.to_string())
            }
        }
        Err(e) => return e.into(),
        Ok(other) => sh_args.push(other.to_string()),
    }
    let result = Command::new("/usr/bin/env").args(sh_args).output();
    match result {
        Ok(result) => {
            let mut map = BTreeMap::<InnerData, InnerData>::new();
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
            InnerData::from(map).into()
        }
        Err(e) => TemplarError::from(e).into(),
    }
}
