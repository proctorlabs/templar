#[derive(Debug)]
pub enum TemplarError {
    ParseFailure(String),
    RenderFailure(String),
    FilterNotFound(String),
    FunctionNotFound(String),
    IO(String),
}

pub type Result<T> = std::result::Result<T, TemplarError>;

impl From<std::io::Error> for TemplarError {
    fn from(e: std::io::Error) -> TemplarError {
        TemplarError::IO(e.to_string())
    }
}

impl From<serde_json::Error> for TemplarError {
    fn from(e: serde_json::Error) -> TemplarError {
        TemplarError::IO(e.to_string())
    }
}

impl From<serde_yaml::Error> for TemplarError {
    fn from(e: serde_yaml::Error) -> TemplarError {
        TemplarError::IO(e.to_string())
    }
}
