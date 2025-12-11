//! # Lunaris Audio
//!
//! Audio playback and mixing for the Lunaris Game Engine.

#![warn(missing_docs)]
#![warn(clippy::all)]

use lunaris_core::Result;

/// Initialize the audio subsystem
///
/// # Errors
///
/// Returns an error if initialization fails
pub fn init() -> Result<()> {
    tracing::info!("Audio subsystem initialized");
    Ok(())
}
