use std::fmt;

#[derive(Debug)]
pub enum BootstrapError {
    #[allow(dead_code)]
    MissingPatchPath,

    IOError(std::io::Error),
    TOMLError(toml_edit::TomlError),

    #[allow(dead_code)]
    InvalidPatch(String),
    Other(String),
}

impl fmt::Display for BootstrapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BootstrapError::MissingPatchPath => write!(f, "❌ No path was supplied for `--patch-pyproject`"),
            BootstrapError::IOError(e) => write!(f, "❌ I/O error: {}", e),
            BootstrapError::TOMLError(e) => write!(f, "❌ TOML parsing error: {}", e),
            BootstrapError::InvalidPatch(msg) => write!(f, "❌ Invalid patch operation: {}", msg),
            BootstrapError::Other(msg) => write!(f, "❌ Error: {}", msg),
        }
    }
}

impl std::error::Error for BootstrapError {}

impl From<std::io::Error> for BootstrapError {
    fn from(err: std::io::Error) -> Self {
        BootstrapError::IOError(err)
    }
}

impl From<toml_edit::TomlError> for BootstrapError {
    fn from(err: toml_edit::TomlError) -> Self {
        BootstrapError::TOMLError(err)
    }
}

impl From<&str> for BootstrapError {
    fn from(msg: &str) -> Self {
        BootstrapError::Other(msg.to_string())
    }
}

impl From<String> for BootstrapError {
    fn from(msg: String) -> Self {
        BootstrapError::Other(msg)
    }
}
