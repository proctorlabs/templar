use super::*;
use unstructured::Document;

#[derive(Debug, Clone)]
pub struct Data {
    doc: Option<Document>,
    error: Option<TemplarError>,
}

lazy_static! {
    static ref EMPTY_DOC: Document = { Document::String("".into()) };
}

impl Data {
    #[inline]
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

    #[inline]
    pub fn is_failed(&self) -> bool {
        self.error.is_some()
    }

    pub fn result(self) -> Result<Document> {
        match (self.error, self.doc) {
            (Some(e), _) => Err(e),
            (_, Some(doc)) => Ok(doc),
            _ => Ok(EMPTY_DOC.clone()),
        }
    }

    pub fn new_result(&self) -> Result<Document> {
        match (&self.error, &self.doc) {
            (Some(e), _) => Err(e.clone()),
            (_, Some(doc)) => Ok(doc.clone()),
            _ => Ok(EMPTY_DOC.clone()),
        }
    }

    pub fn ref_result(&self) -> Result<&Document> {
        match (&self.error, &self.doc) {
            (Some(e), _) => Err(e.clone()),
            (_, Some(doc)) => Ok(doc),
            _ => Ok(&EMPTY_DOC),
        }
    }

    pub fn from_vec(seq: Vec<Data>) -> Self {
        let result: Result<Vec<Document>> = seq.into_iter().map(|d| Ok(d.result()?)).collect();
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
    #[inline]
    fn from(doc: T) -> Self {
        Data {
            doc: Some(doc.into()),
            error: None,
        }
    }
}

impl From<TemplarError> for Data {
    #[inline]
    fn from(error: TemplarError) -> Self {
        Data {
            doc: None,
            error: Some(error),
        }
    }
}
