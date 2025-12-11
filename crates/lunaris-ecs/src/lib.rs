//! # Lunaris ECS
//!
//! Entity Component System wrapper and extensions for the Lunaris Game Engine.
//!
//! Built on top of bevy_ecs for maximum performance and ergonomics.

#![warn(missing_docs)]
#![warn(clippy::all)]

pub use bevy_ecs::prelude::*;

/// Re-export bevy_ecs for direct access when needed
pub mod ecs {
    pub use bevy_ecs::*;
}
