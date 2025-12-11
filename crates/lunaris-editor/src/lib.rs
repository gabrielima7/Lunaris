//! # Lunaris Editor
//!
//! Visual editor for creating games with the Lunaris Game Engine.

#![warn(missing_docs)]
#![warn(clippy::all)]

use lunaris_core::Result;

/// Initialize the editor
///
/// # Errors
///
/// Returns an error if initialization fails
pub fn init() -> Result<()> {
    tracing::info!("Editor initialized");
    Ok(())
}
