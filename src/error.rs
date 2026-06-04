use std::{
    fmt, io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum RustScopeError {
    Io(io::Error),
    InvalidPath(String),
    Argument(String),
    Parse(String),
    ReportGeneration(String),
    OutputWrite { path: PathBuf, source: io::Error },
}

impl fmt::Display for RustScopeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "I/O error: {error}"),
            Self::InvalidPath(message)
            | Self::Argument(message)
            | Self::Parse(message)
            | Self::ReportGeneration(message) => formatter.write_str(message),
            Self::OutputWrite { path, source } => {
                write!(
                    formatter,
                    "failed to write report to {}: {source}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for RustScopeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::OutputWrite { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl From<io::Error> for RustScopeError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl RustScopeError {
    pub fn output_write(path: &Path, source: io::Error) -> Self {
        Self::OutputWrite {
            path: path.to_path_buf(),
            source,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_argument_error_message() {
        let error = RustScopeError::Argument("unsupported report format: html".to_string());

        assert_eq!(error.to_string(), "unsupported report format: html");
    }

    #[test]
    fn converts_io_error() {
        let error = RustScopeError::from(io::Error::new(io::ErrorKind::NotFound, "missing"));

        assert!(error.to_string().contains("I/O error"));
    }
}
