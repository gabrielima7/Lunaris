//! 2D Physics helpers (when rapier2d feature is enabled)

#[cfg(feature = "2d")]
pub use rapier2d::prelude::*;

/// 2D Physics world wrapper (placeholder for Rapier integration)
#[cfg(not(feature = "2d"))]
pub struct Physics2D;
