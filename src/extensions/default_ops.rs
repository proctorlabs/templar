use crate::*;

pub fn add(left: TemplarResult, right: TemplarResult) -> TemplarResult {
    Ok((left?.as_i64().unwrap_or_default() + right?.as_i64().unwrap_or_default()).into())
}

pub fn subtract(left: TemplarResult, right: TemplarResult) -> TemplarResult {
    Ok((left?.as_i64().unwrap_or_default() - right?.as_i64().unwrap_or_default()).into())
}

pub fn divide(left: TemplarResult, right: TemplarResult) -> TemplarResult {
    Ok((left?.as_i64().unwrap_or_default() / right?.as_i64().unwrap_or_default()).into())
}

pub fn multiply(left: TemplarResult, right: TemplarResult) -> TemplarResult {
    Ok((left?.as_i64().unwrap_or_default() * right?.as_i64().unwrap_or_default()).into())
}

pub fn modulus(left: TemplarResult, right: TemplarResult) -> TemplarResult {
    Ok((left?.as_i64().unwrap_or_default() % right?.as_i64().unwrap_or_default()).into())
}

pub fn and(left: TemplarResult, right: TemplarResult) -> TemplarResult {
    Ok((left?.as_bool().ok_or_else(|| {
        TemplarError::RenderFailure("Attempted to compare non-boolean values".into())
    })? && right?.as_bool().ok_or_else(|| {
        TemplarError::RenderFailure("Attempted to compare non-boolean values".into())
    })?)
    .into())
}

pub fn or(left: TemplarResult, right: TemplarResult) -> TemplarResult {
    Ok((left?.as_bool().ok_or_else(|| {
        TemplarError::RenderFailure("Attempted to compare non-boolean values".into())
    })? || right?.as_bool().ok_or_else(|| {
        TemplarError::RenderFailure("Attempted to compare non-boolean values".into())
    })?)
    .into())
}

pub fn equals(left: TemplarResult, right: TemplarResult) -> TemplarResult {
    Ok((left? == right?).into())
}

pub fn not_equals(left: TemplarResult, right: TemplarResult) -> TemplarResult {
    Ok((left? != right?).into())
}

pub fn greater_than(left: TemplarResult, right: TemplarResult) -> TemplarResult {
    Ok((left? > right?).into())
}

pub fn greater_than_equals(left: TemplarResult, right: TemplarResult) -> TemplarResult {
    Ok((left? >= right?).into())
}

pub fn less_than(left: TemplarResult, right: TemplarResult) -> TemplarResult {
    Ok((left? < right?).into())
}

pub fn less_than_equals(left: TemplarResult, right: TemplarResult) -> TemplarResult {
    Ok((left? <= right?).into())
}

pub fn concat(left: TemplarResult, right: TemplarResult) -> TemplarResult {
    Ok(format!("{}{}", left?, right?).into())
}
