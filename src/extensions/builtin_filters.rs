use crate::*;

#[cfg(feature = "base64-extension")]
use std::str;

pub fn length(value: TemplarResult, _: TemplarResult) -> TemplarResult {
    match value? {
        Document::Seq(arr) => Ok((arr.len() as u64).into()),
        Document::String(s) => Ok((s.chars().count() as u64).into()),
        _ => Err(TemplarError::RenderFailure("".into())),
    }
}

pub fn upper(value: TemplarResult, _: TemplarResult) -> TemplarResult {
    Ok(value?.to_string().to_uppercase().into())
}

pub fn lower(value: TemplarResult, _: TemplarResult) -> TemplarResult {
    Ok(value?.to_string().to_lowercase().into())
}

pub fn trim(value: TemplarResult, _: TemplarResult) -> TemplarResult {
    Ok(value?.to_string().trim().into())
}

#[cfg(feature = "base64-extension")]
pub fn base64(value: TemplarResult, args: TemplarResult) -> TemplarResult {
    let switch = args?.to_string().to_lowercase();
    match switch.as_ref() {
        "decode" => Ok(str::from_utf8(
            &base64::decode(&value?.to_string()).unwrap_or_else(|_| "".into()),
        )
        .unwrap_or_default()
        .into()),
        _ => Ok(base64::encode(&value?.to_string()).into()),
    }
}

pub fn split(value: TemplarResult, args: TemplarResult) -> TemplarResult {
    let delim = args?.as_string().unwrap_or_else(|| "\n".into());
    Ok(match value? {
        Document::String(s) => Document::Seq(s.split(&delim).map(|s| s.into()).collect()),
        _ => Document::Seq(vec![]),
    })
}

pub fn join(value: TemplarResult, args: TemplarResult) -> TemplarResult {
    let res;
    let delim = args?.as_string().unwrap_or_else(|| "\n".into());
    match value? {
        Document::Seq(s) => {
            res = s
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(&delim)
                .into()
        }
        v => res = v,
    }
    Ok(res)
}

pub fn index(value: TemplarResult, args: TemplarResult) -> TemplarResult {
    let res;
    let i = args?
        .as_usize()
        .ok_or_else(|| TemplarError::RenderFailure("Cannot index with non real value".into()))?;
    match value? {
        Document::Seq(s) => res = s.get(i).cloned().unwrap_or_default(),
        _ => res = Document::Unit,
    }
    Ok(res)
}

#[cfg(feature = "json-extension")]
pub fn json(value: TemplarResult, args: TemplarResult) -> TemplarResult {
    let arg_string = args?.to_string();
    Ok(if arg_string == "pretty" {
        serde_json::to_string_pretty(&value?)
            .unwrap_or_default()
            .into()
    } else {
        serde_json::to_string(&value?).unwrap_or_default().into()
    })
}

#[cfg(feature = "yaml-extension")]
pub fn yaml(value: TemplarResult, _: TemplarResult) -> TemplarResult {
    Ok(serde_yaml::to_string(&value?).unwrap_or_default().into())
}

pub fn string(value: TemplarResult, _: TemplarResult) -> TemplarResult {
    Ok(value?.to_string().into())
}

pub fn key(value: TemplarResult, args: TemplarResult) -> TemplarResult {
    Ok(match value? {
        Document::Map(map) => map[&args?].clone(),
        _ => {
            return Err(TemplarError::RenderFailure(
                "Attempted to retrieve a key on a value that is not a map".into(),
            ))
        }
    })
}

pub fn default(value: TemplarResult, args: TemplarResult) -> TemplarResult {
    match value {
        Ok(Document::Unit) | Err(_) => args,
        other => other,
    }
}

pub fn require(value: TemplarResult, _: TemplarResult) -> TemplarResult {
    match value {
        Ok(Document::Unit) => Err(TemplarError::RenderFailure(
            "Required value is missing.".into(),
        )),
        other => other,
    }
}
