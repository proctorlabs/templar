/*!
All of the error types returned by Templar.
 */

use std::error::Error;
use std::fmt;
use std::sync::Arc;

/// This is the primary error type for template
#[derive(Debug, Clone)]
pub enum TemplarError {
    /// Some error occurred while parsing a template
    ParseFailure(String),
    /// Some error occurred while rendering a template
    RenderFailure(String),
    /// Filter referred to by template is not available
    FilterNotFound(String),
    /// Function referred to by template is not available
    FunctionNotFound(String),
    /// An I/O error occurred
    IO(String),
    /// Some other error, check the inner value
    Other(Arc<Box<dyn Error>>),
}

impl fmt::Display for TemplarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplarError::ParseFailure(s) => write!(f, "Could not parse template. {}", s),
            TemplarError::RenderFailure(s) => write!(f, "Could not render template. {}", s),
            TemplarError::FilterNotFound(s) => write!(
                f,
                "Filter '{}' was not found while building this expression",
                s
            ),
            TemplarError::FunctionNotFound(s) => write!(
                f,
                "Function '{}' was not found while building this expression",
                s
            ),
            TemplarError::IO(s) => write!(f, "An IO Error occurred. {}", s),
            TemplarError::Other(e) => e.fmt(f),
        }
    }
}

impl Error for TemplarError {}

/// Result type for all Templar methods
pub type Result<T> = std::result::Result<T, TemplarError>;

impl From<Box<dyn Error>> for TemplarError {
    fn from(e: Box<dyn Error>) -> TemplarError {
        TemplarError::Other(Arc::new(e))
    }
}

/// Helper trait for wrapping other error types as a Templar error
pub trait ResultMap<U> {
    /// Wrap an external error as a Templar error
    fn wrap(self) -> Result<U>;
}

impl<U, T> ResultMap<U> for std::result::Result<U, T>
where
    T: std::error::Error + Into<Box<dyn Error>>,
{
    fn wrap(self) -> Result<U> {
        self.map_err(|e| TemplarError::Other(Arc::new(e.into())))
    }
}

impl From<std::io::Error> for TemplarError {
    fn from(e: std::io::Error) -> TemplarError {
        TemplarError::IO(e.to_string())
    }
}
