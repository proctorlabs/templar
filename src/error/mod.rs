#[derive(Debug)]
pub enum TemplarError {
    ParseFailure(String),
    RenderFailure(String),
    FilterNotFound(String),
    FunctionNotFound(String),
}

pub type Result<T> = std::result::Result<T, TemplarError>;
