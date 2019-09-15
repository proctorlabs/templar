use crate::*;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;
use std::str;

#[cfg(feature = "json-extension")]
pub fn json(args: Data) -> Data {
    let data_str = data_unwrap_into!(String: args);
    Data::from_result(serde_json::from_str(&data_str).wrap())
}

#[cfg(feature = "yaml-extension")]
pub fn yaml(args: Data) -> Data {
    let data_str = data_unwrap_into!(String: args);
    Data::from_result(serde_yaml::from_str(&data_str).wrap())
}

pub fn file(args: Data) -> Data {
    let path: PathBuf = data_unwrap_into!(String: args).into();
    match File::open(path) {
        Ok(mut f) => {
            let mut result = String::new();
            match f.read_to_string(&mut result) {
                Ok(_) => result.into(),
                Err(e) => TemplarError::from(e).into(),
            }
        }
        Err(e) => TemplarError::from(e).into(),
    }
}

pub fn env(args: Data) -> Data {
    let env_name = data_unwrap_into!(String: args);
    std::env::var(env_name).unwrap_or_default().into()
}

pub fn script(args: Data) -> Data {
    let mut sh_args = vec![Document::String("sh".into()), "-c".into()];
    match args.into_result() {
        Ok(Document::Seq(s)) => {
            for arg in s.iter() {
                sh_args.push(arg.clone())
            }
        }
        Ok(other) => sh_args.push(other),
        Err(e) => return e.into(),
    }
    command(sh_args.into())
}

pub fn command(args: Data) -> Data {
    let mut sh_args = vec![];
    match args.into_result() {
        Ok(Document::Seq(s)) => {
            for arg in s.iter() {
                sh_args.push(arg.to_string())
            }
        }
        Ok(other) => sh_args.push(other.to_string()),
        Err(e) => return e.into(),
    }
    let result = Command::new("/usr/bin/env").args(sh_args).output();
    match result {
        Ok(result) => {
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
            Document::from(map).into()
        }
        Err(e) => TemplarError::from(e).into(),
    }
}
