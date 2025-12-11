//! # Lunaris Runtime
//!
//! Game runtime that orchestrates all engine subsystems.

#![warn(missing_docs)]
#![warn(clippy::all)]

use lunaris_core::Result;

/// Initialize the complete runtime
///
/// # Errors
///
/// Returns an error if any subsystem fails to initialize
pub fn init() -> Result<()> {
    lunaris_core::init()?;
    tracing::info!("Lunaris Runtime initialized");
    Ok(())
}
