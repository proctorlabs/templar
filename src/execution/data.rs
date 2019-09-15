use super::*;
use unstructured::Document;

/// The `Data` struct is used to represent the raw execution result of a node in a template.
/// Data can currently be in one of three states:
///
/// - Empty: The execution was successful but there was no result associated. For example, value assignments (x = y)
/// - Success: The document data can be safely retrieved
/// - Failure: An error occurred executing this node
#[derive(Debug, Clone)]
pub struct Data {
    doc: Option<Document>,
    error: Option<TemplarError>,
}

lazy_static! {
    static ref EMPTY_DOC: Document = { Document::String("".into()) };
}

impl<'a> Data {
    /// Create a new empty result
    #[inline]
    pub fn empty() -> Data {
        Data {
            doc: None,
            error: None,
        }
    }

    /// Render this result
    ///
    /// If the data is empty, an empty string is returned.
    /// If this data is in an error state, the error is returned
    /// Otherwise, the rendered string is returned
    pub fn render(self) -> Result<String> {
        match (self.error, self.doc) {
            (Some(e), _) => Err(e),
            (_, Some(Document::Unit)) => Ok("null".into()),
            (_, Some(doc)) => Ok(doc.to_string()),
            _ => Ok("".into()),
        }
    }

    /// Check if this data struct has a failure
    #[inline]
    pub fn is_failed(&self) -> bool {
        self.error.is_some()
    }

    /// Check if this data struct is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.doc.is_none() && self.error.is_none()
    }

    /// Unwrap the data contents
    ///
    /// # PANIC!
    ///
    /// This will panic if this data struct is empty or contains an error
    #[inline]
    pub fn unwrap(self) -> Document {
        self.doc.unwrap()
    }

    /// Unwrap the data error
    ///
    /// # PANIC!
    ///
    /// This will panic if this data struct is empty or does not contain an error
    #[inline]
    pub fn unwrap_err(self) -> TemplarError {
        self.error.unwrap()
    }

    /// Convert the data into a Result<Document>.
    /// In the case of empty data, an empty string is returned.
    pub fn into_result(self) -> Result<Document> {
        match (self.error, self.doc) {
            (Some(e), _) => Err(e),
            (_, Some(doc)) => Ok(doc),
            _ => Ok(EMPTY_DOC.clone()),
        }
    }

    /// Clone this data into a new Result<Document>
    pub fn clone_result(&self) -> Result<Document> {
        match (&self.error, &self.doc) {
            (Some(e), _) => Err(e.clone()),
            (_, Some(doc)) => Ok(doc.clone()),
            _ => Ok(EMPTY_DOC.clone()),
        }
    }

    /// Retrieve a result with a reference to the underlying document
    pub fn to_result(&'a self) -> Result<&'a Document> {
        match (&self.error, &self.doc) {
            (Some(e), _) => Err(e.clone()),
            (_, Some(doc)) => Ok(doc),
            _ => Ok(&EMPTY_DOC),
        }
    }

    /// Create Data from a result
    pub fn from_result(result: Result<Document>) -> Data {
        match result {
            Ok(result) => Data {
                doc: Some(result),
                error: None,
            },
            Err(e) => Data {
                doc: None,
                error: Some(e),
            },
        }
    }

    pub(crate) fn check<T: std::fmt::Debug>(to_check: Result<T>) -> Data {
        match to_check {
            Err(e) => Data {
                doc: None,
                error: Some(e),
            },
            _ => Data::empty(),
        }
    }

    pub(crate) fn from_vec(seq: Vec<Data>) -> Self {
        let result: Result<Vec<Document>> = seq.into_iter().map(|d| Ok(d.into_result()?)).collect();
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
