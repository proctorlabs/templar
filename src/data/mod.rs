use super::*;
use std::ops::{Deref, DerefMut};
use unstructured::Document;

#[derive(Debug, Clone)]
pub struct Data {
    doc: Document,
    error: Option<TemplarError>,
}

impl Data {
    pub fn result(self) -> Result<Data> {
        match self.error {
            Some(e) => Err(e),
            None => Ok(self),
        }
    }

    pub fn is_failed(&self) -> bool {
        self.error.is_some()
    }

    pub fn into_doc(self) -> Document {
        self.doc
    }

    pub fn from_vec(seq: Vec<Data>) -> Self {
        let mut result = vec![];
        for data in seq.into_iter() {
            if data.is_failed() {
                return Data {
                    doc: Document::Unit,
                    error: data.error,
                };
            }
            result.push(data.into_doc());
        }
        Data {
            doc: result.into(),
            error: None,
        }
    }
}

impl Deref for Data {
    type Target = Document;

    fn deref(&self) -> &Self::Target {
        &self.doc
    }
}

impl DerefMut for Data {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.doc
    }
}

impl<T: Into<Document>> From<T> for Data {
    fn from(doc: T) -> Self {
        Data {
            doc: doc.into(),
            error: None,
        }
    }
}

impl From<TemplarError> for Data {
    fn from(error: TemplarError) -> Self {
        Data {
            doc: Document::Unit,
            error: Some(error),
        }
    }
}
