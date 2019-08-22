use crate::*;

pub fn length(value: TemplarResult, _: TemplarResult) -> TemplarResult {
    match value? {
        Document::Seq(arr) => Ok((arr.len() as u64).into()),
        Document::String(s) => Ok((s.chars().count() as u64).into()),
        _ => Err(TemplarError::RenderFailure("".into())),
    }
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

pub fn upper(value: TemplarResult, _: TemplarResult) -> TemplarResult {
    Ok(Document::String(value?.to_string().to_uppercase()))
}

pub fn lower(value: TemplarResult, _: TemplarResult) -> TemplarResult {
    Ok(Document::String(value?.to_string().to_lowercase()))
}

pub fn trim(value: TemplarResult, _: TemplarResult) -> TemplarResult {
    Ok(Document::String(value?.to_string().trim().to_string()))
}
