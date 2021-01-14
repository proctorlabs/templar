use super::*;
use unstructured::{Unstructured, UnstructuredDataTrait};
use derive_more::{Deref, DerefMut};

pub type InnerData = Unstructured<Data>;

#[derive(Clone, Debug)]
pub enum OtherData {
    // Expr(Vec<NodeData>),
    // Scope(Box<NodeData>),
    // Operation(Arc<Operation>),
}

impl fmt::Display for OtherData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<OtherData>")
    }
}

/// The `Data` struct is used to represent the raw execution result of a node in a template.
/// Data can currently be in one of three states:
///
/// - Empty: The execution was successful but there was no result associated. For example, value assignments (x = y)
/// - Success: The document data can be safely retrieved
/// - Failure: An error occurred executing this node
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Data {
    #[deref]
    inner: InnerData
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Node>")
    }
}

impl UnstructuredDataTrait for Data {
    type ErrorType = TemplarError;
    type OtherType = OtherData;
}

impl<'a> Data {
    /// Create a new empty result
    #[inline]
    pub fn empty() -> Data {
        Data { inner: InnerData::Unassigned }
    }

    /// Create new data value
    pub fn new<T: Into<InnerData>>(inner: T) -> Self {
        Self { inner: inner.into() }
    }

    /// Get reference to the inner value
    pub fn inner_data(&self) -> &InnerData {
        &self.inner
    }
    
    /// Get reference to the inner value
    pub fn inner_data_mut(&mut self) -> &mut InnerData {
        &mut self.inner
    }

    /// Get reference to the inner value
    pub fn into_inner(self) -> InnerData {
        self.inner
    }

    /// Render this result
    ///
    /// If the data is empty, an empty string is returned.
    /// If this data is in an error state, the error is returned
    /// Otherwise, the rendered string is returned
    pub fn render(self) -> Result<String> {
        match self.inner {
            InnerData::Err(e) => Err(e),
            InnerData::Null => Ok("null".into()),
            InnerData::Unassigned => Ok("".into()),
            doc => Ok(doc.to_string()),
        }
    }

    /// Check if this data struct has a failure
    #[inline]
    pub fn is_failed(&self) -> bool {
        matches!(self.inner, InnerData::Err(_))
    }

    /// Check if this data struct is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        matches!(self.inner, InnerData::Unassigned)
    }

    /// Unwrap the data contents
    ///
    /// # PANIC!
    ///
    /// This will panic if this data struct is empty or contains an error
    #[inline]
    pub fn unwrap(self) -> Self {
        self
    }

    /// Unwrap the data error
    ///
    /// # PANIC!
    ///
    /// This will panic if this data struct is empty or does not contain an error
    #[inline]
    pub fn unwrap_err(self) -> TemplarError {
        match self.inner {
            InnerData::Err(e) => e,
            _ => panic!("Not an error!"),
        }
    }

    /// Convert the data into a Result<Document>.
    /// In the case of empty data, an empty string is returned.
    pub fn into_result(self) -> Result<Self> {
        match self.inner {
            InnerData::Err(e) => Err(e),
            dat => Ok(Self { inner: dat })
        }
    }

    /// Clone this data into a new Result<Document>
    pub fn clone_result(&self) -> Result<Self> {
        self.clone().into_result()
    }

    /// Retrieve a result with a reference to the underlying document
    pub fn to_result(&'a self) -> Result<&'a InnerData> {
        match &self.inner {
            InnerData::Err(e) => Err(e.clone()),
            ref dat => Ok(dat)
        }
    }

    // /// Create Data from a result
    pub fn from_result(result: Result<InnerData>) -> Data {
        match result {
            Ok( inner ) => Data { inner },
            Err(e) => Data { inner: InnerData::Err(e) },
        }
    }

    pub(crate) fn check<T: std::fmt::Debug>(to_check: Result<T>) -> Data {
        match to_check {
            Err(e) => Data::new(InnerData::Err(e)),
            _ => Data::empty(),
        }
    }

    pub(crate) fn from_vec(seq: Vec<Data>) -> Self {
        let result: Result<Vec<InnerData>> = seq.into_iter().map(|d| Ok(d.inner)).collect();
        match result {
            Ok(docs) => Data::new(docs),
            Err(e) => e.into(),
        }
    }
}

impl<T: Into<InnerData>> From<T> for Data {
    #[inline]
    fn from(doc: T) -> Self {
        Data { inner: doc.into() }
    }
}

impl From<TemplarError> for Data {
    #[inline]
    fn from(error: TemplarError) -> Self {
        Data { inner: InnerData::Err(error) }
    }
}
