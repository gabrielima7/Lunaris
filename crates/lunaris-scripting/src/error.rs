//! Script error types

use thiserror::Error;

/// Errors that can occur during script execution
#[derive(Error, Debug)]
pub enum ScriptError {
    /// Lua runtime error
    #[error("Lua error: {0}")]
    Lua(#[from] mlua::Error),

    /// Script exceeded resource limits
    #[error("Resource limit exceeded: {0}")]
    ResourceLimit(String),

    /// Attempted to access forbidden capability
    #[error("Capability denied: {0}")]
    CapabilityDenied(String),

    /// Script execution timeout
    #[error("Script timeout after {0} instructions")]
    Timeout(u64),

    /// Memory limit exceeded
    #[error("Memory limit exceeded: {used} bytes > {limit} bytes")]
    MemoryLimit {
        /// Bytes used
        used: usize,
        /// Limit in bytes
        limit: usize,
    },

    /// Sandbox initialization error
    #[error("Sandbox init error: {0}")]
    SandboxInit(String),

    /// Script compilation error
    #[error("Compilation error: {0}")]
    Compile(String),
}

/// Result type for script operations
pub type ScriptResult<T> = Result<T, ScriptError>;
