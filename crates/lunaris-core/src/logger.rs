//! Logging utilities for Lunaris Engine

use tracing::Level;

/// Log level configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    /// Trace level - most verbose
    Trace,
    /// Debug level
    Debug,
    /// Info level - default
    Info,
    /// Warning level
    Warn,
    /// Error level - least verbose
    Error,
}

impl LogLevel {
    /// Convert to tracing Level
    #[must_use]
    pub const fn to_tracing_level(self) -> Level {
        match self {
            Self::Trace => Level::TRACE,
            Self::Debug => Level::DEBUG,
            Self::Info => Level::INFO,
            Self::Warn => Level::WARN,
            Self::Error => Level::ERROR,
        }
    }
}

/// Logger configuration
#[derive(Debug)]
pub struct Logger {
    level: LogLevel,
}

impl Logger {
    /// Create a new logger with default INFO level
    #[must_use]
    pub const fn new() -> Self {
        Self {
            level: LogLevel::Info,
        }
    }

    /// Create a logger with specified level
    #[must_use]
    pub const fn with_level(level: LogLevel) -> Self {
        Self { level }
    }

    /// Get the current log level
    #[must_use]
    pub const fn level(&self) -> LogLevel {
        self.level
    }

    /// Set the log level
    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}
