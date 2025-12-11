//! # Lunaris ECS
//!
//! Entity Component System for the Lunaris Game Engine.
//!
//! Built on bevy_ecs with additional game-specific components.
//!
//! ## Core Components
//!
//! - Transform2D / Transform3D - Spatial transforms
//! - Name - Entity naming
//! - Parent/Children - Scene hierarchy
//! - Visibility - Rendering visibility

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod components;
pub mod hierarchy;
pub mod systems;

pub use bevy_ecs::prelude::*;
pub use components::*;
pub use hierarchy::*;

/// Re-export bevy_ecs for direct access
pub mod ecs {
    pub use bevy_ecs::*;
}
