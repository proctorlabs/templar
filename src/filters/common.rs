use crate::*;

#[cfg(feature = "base64-extension")]
use std::str;

pub fn length(value: Data, _: Data) -> Data {
    match data_unwrap!(value) {
        Document::Seq(arr) => (arr.len() as u64).into(),
        Document::String(s) => (s.chars().count() as u64).into(),
        _ => 1u64.into(),
    }
}

pub fn upper(value: Data, _: Data) -> Data {
    render_unwrap!(value).to_uppercase().into()
}

pub fn lower(value: Data, _: Data) -> Data {
    render_unwrap!(value).to_lowercase().into()
}

pub fn trim(value: Data, _: Data) -> Data {
    render_unwrap!(value).trim().into()
}

pub fn exists(value: Data, _: Data) -> Data {
    (!value.is_empty()).into()
}

#[cfg(feature = "base64-extension")]
pub fn base64(value: Data, args: Data) -> Data {
    let switch = match args.render() {
        Ok(s) => s.to_lowercase(),
        Err(e) => return e.into(),
    };
    let val = match value.render() {
        Ok(s) => s,
        Err(e) => return e.into(),
    };
    let res: String = match switch.as_ref() {
        "decode" => str::from_utf8(&base64::decode(&val).unwrap_or_default())
            .unwrap_or_default()
            .into(),
        _ => base64::encode(&val),
    };
    res.into()
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
        Ok(Document::String(s)) => Document::Seq(s.split(&delim).map(|s| s.into()).collect()),
        _ => Document::Seq(vec![]),
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
    match value.into_result() {
        Ok(Document::Seq(s)) => s
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(&delim)
            .into(),
        Ok(v) => v.into(),
        Err(e) => e.into(),
    }
}

pub fn index(value: Data, args: Data) -> Data {
    let arg = match args.into_result() {
        Ok(i) => i.as_usize(),
        Err(e) => return e.into(),
    };
    if let Some(i) = arg {
        match value.into_result() {
            Ok(Document::Seq(s)) => s.get(i).cloned().unwrap_or_default().into(),
            _ => Document::Unit.into(),
        }
    } else {
        TemplarError::RenderFailure("Cannot index with non real value".into()).into()
    }
}

#[cfg(feature = "json-extension")]
pub fn json(value: Data, args: Data) -> Data {
    match value.into_result() {
        Ok(val) => {
            let arg = args.render().unwrap_or_default();
            match arg.as_str() {
                "pretty" => serde_json::to_string_pretty(&val)
                    .unwrap_or_default()
                    .into(),
                _ => serde_json::to_string(&val).unwrap_or_default().into(),
            }
        }
        Err(e) => e.into(),
    }
}

#[cfg(feature = "yaml-extension")]
pub fn yaml(value: Data, _: Data) -> Data {
    match value.into_result() {
        Ok(val) => serde_yaml::to_string(&val).unwrap_or_default().into(),
        Err(e) => e.into(),
    }
}

pub fn string(value: Data, _: Data) -> Data {
    match value.render() {
        Ok(val) => val.into(),
        Err(e) => e.into(),
    }
}

pub fn key(value: Data, args: Data) -> Data {
    if args.is_empty() || args.is_failed() {
        return TemplarError::RenderFailure(
            "Attempted to retrieve a key on a value that is not a map".into(),
        )
        .into();
    }
    match value.into_result() {
        Ok(Document::Map(map)) => map[&args.into_result().unwrap()].clone().into(),
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

pub fn escape_html(value: Data, _: Data) -> Data {
    let input = render_unwrap!(value);
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
    out.into()
}

pub fn require(value: Data, _: Data) -> Data {
    if value.is_empty() || matches!(value.to_result(), Ok(d) if d == &Document::Unit) {
        TemplarError::RenderFailure("Required value is missing.".into()).into()
    } else {
        value
    }
}
