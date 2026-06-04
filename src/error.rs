use std::{fmt, io};

#[derive(Debug)]
pub enum RustScopeError {
    Io(io::Error),
    InvalidPath(String),
    Usage(String),
}

impl fmt::Display for RustScopeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "I/O error: {error}"),
            Self::InvalidPath(message) | Self::Usage(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for RustScopeError {}

impl From<io::Error> for RustScopeError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
