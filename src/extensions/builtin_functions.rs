use crate::*;

pub fn nop(_: Result<Document>) -> Result<Document> {
    Ok(Document::String("FUNCTION!!!".into()))
}
