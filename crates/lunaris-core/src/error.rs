//! Error types for Lunaris Engine

use thiserror::Error;

/// Main error type for Lunaris operations
#[derive(Error, Debug)]
pub enum Error {
    /// Initialization error
    #[error("Initialization failed: {0}")]
    Init(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Asset loading error
    #[error("Asset error: {0}")]
    Asset(String),

    /// Scripting error
    #[error("Script error: {0}")]
    Script(String),

    /// Renderer error
    #[error("Renderer error: {0}")]
    Renderer(String),

    /// Window error
    #[error("Window error: {0}")]
    Window(String),

    /// Plugin error
    #[error("Plugin error: {0}")]
    Plugin(String),

    /// Configuration error
    #[error("Config error: {0}")]
    Config(String),

    /// Generic internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type alias using Lunaris Error
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        let err = Error::Init("test".to_string());
        assert!(err.to_string().contains("Initialization failed"));
    }
}
