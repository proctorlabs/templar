use crate::*;

pub fn length(value: TemplarResult, _: TemplarResult) -> TemplarResult {
    match value? {
        Document::Seq(arr) => Ok((arr.len() as u64).into()),
        Document::String(s) => Ok((s.chars().count() as u64).into()),
        _ => Err(TemplarError::RenderFailure("".into())),
    }
}

pub fn upper(value: TemplarResult, _: TemplarResult) -> TemplarResult {
    Ok(Document::String(value?.to_string().to_uppercase()))
}

pub fn lower(value: TemplarResult, _: TemplarResult) -> TemplarResult {
    Ok(Document::String(value?.to_string().to_lowercase()))
}

pub fn trim(value: TemplarResult, _: TemplarResult) -> TemplarResult {
    Ok(Document::String(value?.to_string().trim().to_string()))
}

pub fn split(value: TemplarResult, args: TemplarResult) -> TemplarResult {
    let res;
    let delim = args?.as_string().unwrap_or_else(|| "\n".into());
    match value? {
        Document::String(s) => res = Document::Seq(s.split(&delim).map(|s| s.into()).collect()),
        _ => res = Document::Seq(vec![]),
    }
    Ok(res)
}

pub fn index(value: TemplarResult, args: TemplarResult) -> TemplarResult {
    let res;
    let i = args?
        .as_usize()
        .ok_or_else(|| TemplarError::RenderFailure("Cannot index with non real value".into()))?;
    match value? {
        Document::Seq(s) => res = s[i].clone(),
        _ => res = Document::Unit,
    }
    Ok(res)
}

pub fn json(value: TemplarResult, args: TemplarResult) -> TemplarResult {
    let pretty = match args {
        Ok(Document::String(s)) => s == "pretty",
        _ => false,
    };
    Ok(Document::String(if pretty {
        serde_json::to_string_pretty(&value?).unwrap_or_default()
    } else {
        serde_json::to_string(&value?).unwrap_or_default()
    }))
}

pub fn yaml(value: TemplarResult, _: TemplarResult) -> TemplarResult {
    Ok(Document::String(
        serde_yaml::to_string(&value?).unwrap_or_default(),
    ))
}
