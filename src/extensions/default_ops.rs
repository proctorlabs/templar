use crate::*;

pub fn add(left: TemplarResult, right: TemplarResult) -> TemplarResult {
    println!("{:?}:{:?}", left, right);
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
