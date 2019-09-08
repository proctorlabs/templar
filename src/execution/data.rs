use super::*;
use unstructured::Document;

#[derive(Debug, Clone)]
pub struct Data {
    doc: Option<Document>,
    error: Option<TemplarError>,
}

impl Data {
    pub fn empty() -> Data {
        Data {
            doc: None,
            error: None,
        }
    }

    pub fn render(self) -> Result<String> {
        match (self.error, self.doc) {
            (Some(e), _) => Err(e),
            (_, Some(Document::Unit)) => Ok("null".into()),
            (_, Some(doc)) => Ok(doc.to_string()),
            _ => Ok("".into()),
        }
    }

    pub fn into_result(self) -> Result<Data> {
        match self.error {
            Some(e) => Err(e),
            None => Ok(self),
        }
    }

    pub fn is_failed(&self) -> bool {
        self.error.is_some()
    }

    pub fn into_document(self) -> Result<Document> {
        match (self.error, self.doc) {
            (Some(e), _) => Err(e),
            (_, Some(doc)) => Ok(doc),
            _ => Ok("".into()),
        }
    }

    pub fn from_vec(seq: Vec<Data>) -> Self {
        let result: Result<Vec<Document>> =
            seq.into_iter().map(|d| Ok(d.into_document()?)).collect();
        match result {
            Ok(docs) => Data {
                doc: Some(docs.into()),
                error: None,
            },
            Err(e) => Data {
                doc: None,
                error: Some(e),
            },
        }
    }
}

impl<T: Into<Document>> From<T> for Data {
    fn from(doc: T) -> Self {
        Data {
            doc: Some(doc.into()),
            error: None,
        }
    }
}

impl From<TemplarError> for Data {
    fn from(error: TemplarError) -> Self {
        Data {
            doc: None,
            error: Some(error),
        }
    }
}
