//! 3D Physics helpers (when rapier3d feature is enabled)

#[cfg(feature = "3d")]
pub use rapier3d::prelude::*;

/// 3D Physics world wrapper (placeholder for Rapier integration)
#[cfg(not(feature = "3d"))]
pub struct Physics3D;
