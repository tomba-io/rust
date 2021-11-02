use std::fmt;


#[derive(Debug, Clone)]
/// A general tomba error for requests.
pub enum TombaError {
    HttpError(String),
    ParseError(String),
}

impl std::error::Error for TombaError {}

impl fmt::Display for TombaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TombaError::HttpError(error) => {
                write!(f, "HTTP Request Error: {}", error.to_string())
            }
            TombaError::ParseError(error) => {
                write!(f, "JSON Parsing Error: {}", error.to_string())
            }
        }
    }
}