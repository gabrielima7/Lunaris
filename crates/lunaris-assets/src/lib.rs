//! # Lunaris Assets
//!
//! Asset loading, caching, and hot-reloading for the Lunaris Game Engine.

#![warn(missing_docs)]
#![warn(clippy::all)]

use lunaris_core::Result;

/// Initialize the asset subsystem
///
/// # Errors
///
/// Returns an error if initialization fails
pub fn init() -> Result<()> {
    tracing::info!("Asset subsystem initialized");
    Ok(())
}
