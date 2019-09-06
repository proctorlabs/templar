use crate::*;

pub fn then(condition: TemplarResult, contents: TemplarResult) -> TemplarResult {
    if condition? == Document::Bool(true) {
        contents
    } else {
        Ok("".into())
    }
}
