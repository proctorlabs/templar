use crate::*;

#[cfg(feature = "base64-extension")]
use std::str;

pub fn length(value: Data, _: Data) -> Data {
    match value.into_inner() {
        InnerData::Err(e) => e.into(),
        InnerData::Seq(arr) => (arr.len() as u64).into(),
        InnerData::String(s) => (s.chars().count() as u64).into(),
        _ => 1u64.into(),
    }
}

#[templar_filter]
pub fn upper(filter_in: String) -> String {
    filter_in.to_uppercase()
}

#[templar_filter]
pub fn lower(filter_in: String) -> String {
    filter_in.to_lowercase()
}

#[templar_filter]
pub fn trim(filter_in: String) -> String {
    filter_in.trim().into()
}

pub fn exists(value: Data, _: Data) -> Data {
    (!value.is_empty()).into()
}

#[cfg(feature = "base64-extension")]
#[templar_filter]
pub fn base64(
    filter_in: String,
    method: String,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let method = method.to_lowercase();
    let res: String = match method.as_ref() {
        "decode" => str::from_utf8(&base64::decode(&filter_in)?)?.to_string(),
        _ => base64::encode(&filter_in),
    };
    Ok(res)
}

#[cfg(feature = "base64-extension")]
#[templar_filter]
pub fn b64encode(in_string: String) -> std::result::Result<String, Box<dyn std::error::Error>> {
    Ok(str::from_utf8(&base64::decode(&in_string)?)?.to_string())
}

#[cfg(feature = "base64-extension")]
#[templar_filter]
pub fn b64decode(in_string: String) -> std::result::Result<String, Box<dyn std::error::Error>> {
    Ok(base64::encode(&in_string))
}

pub fn split(value: Data, args: Data) -> Data {
    let delim = match args.render() {
        Ok(s) => {
            if s.is_empty() {
                "\n".to_string()
            } else {
                s
            }
        }
        _ => "\n".to_string(),
    };
    match value.into_result() {
        Ok(d) => InnerData::Seq(
            d.render()
                .unwrap_or_default()
                .split(&delim)
                .map(|s| s.into())
                .collect(),
        ),
        _ => InnerData::Seq(vec![]),
    }
    .into()
}

pub fn join(value: Data, args: Data) -> Data {
    let delim = match args.render() {
        Ok(s) => {
            if s.is_empty() {
                "\n".to_string()
            } else {
                s
            }
        }
        _ => "\n".to_string(),
    };
    match value.into_inner() {
        InnerData::Seq(s) => s
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(&delim)
            .into(),
        InnerData::Err(e) => e.into(),
        v => v.into(),
    }
}

pub fn index(value: Data, args: Data) -> Data {
    let arg = match args.into_result() {
        Ok(i) => i.as_usize(),
        Err(e) => return e.into(),
    };
    if let Some(i) = arg {
        match value.into_inner() {
            InnerData::Seq(s) => s.get(i).cloned().unwrap_or_default().into(),
            _ => InnerData::Null.into(),
        }
    } else {
        TemplarError::RenderFailure("Cannot index with non real value".into()).into()
    }
}

#[cfg(feature = "json-extension")]
pub fn json(value: Data, args: Data) -> Data {
    match value.into_inner() {
        InnerData::Err(e) => e.into(),
        val => {
            let arg = args.render().unwrap_or_default();
            match arg.as_str() {
                "pretty" => serde_json::to_string_pretty(&val)
                    .unwrap_or_default()
                    .into(),
                _ => serde_json::to_string(&val).unwrap_or_default().into(),
            }
        }
    }
}

#[cfg(feature = "yaml-extension")]
pub fn yaml(value: Data, _: Data) -> Data {
    match value.into_inner() {
        InnerData::Err(e) => e.into(),
        val => serde_yaml::to_string(&val).unwrap_or_default().into(),
    }
}

#[templar_filter]
pub fn string(in_string: String) -> String { in_string }

pub fn key(value: Data, args: Data) -> Data {
    if args.is_empty() || args.is_failed() {
        return TemplarError::RenderFailure(
            "Attempted to retrieve a key on a value that is not a map".into(),
        )
        .into();
    }
    match value.into_inner() {
        InnerData::Map(map) => map[&args.into_result().unwrap()].clone().into(),
        _ => TemplarError::RenderFailure(
            "Attempted to retrieve a key on a value that is not a map".into(),
        )
        .into(),
    }
}

pub fn default(value: Data, args: Data) -> Data {
    if value.is_empty() || value.is_failed() {
        args
    } else {
        value
    }
}

#[templar_filter]
pub fn escape_html(input: String) -> String {
    let len = input.len();
    let mut out = String::with_capacity(len + (len / 4));
    for c in input.chars() {
        match c {
            '"' => out.push_str("&quot;"), //" VSCode thinks this match is invalid, but this comment fixes that annoyance
            '/' => out.push_str("&#x2F;"),
            '\'' => out.push_str("&#x27;"),
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(c),
        }
    }
    out
}

pub fn require(value: Data, _: Data) -> Data {
    if value.is_empty() || matches!(value.to_result(), Ok(d) if d == &InnerData::Null) {
        TemplarError::RenderFailure("Required value is missing.".into()).into()
    } else {
        value
    }
}

#[templar_filter]
pub fn replace(filter_in: String, old: String, new: String) -> String {
    filter_in.replace(&old, &new)
}

#[templar_filter]
pub fn truncate(filter_in: String, size: u64) -> String {
    let mut res = filter_in;
    res.truncate(size as usize);
    res
}
