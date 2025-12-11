//! # Lunaris Physics
//!
//! Physics simulation for the Lunaris Game Engine.
//!
//! Built on Rapier for high-performance 2D and 3D physics.
//!
//! ## Features
//!
//! - Rigid body dynamics
//! - Collision detection and response
//! - Joints and constraints
//! - Character controllers
//! - Spatial queries (raycasting, shapecasting)

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod character;
pub mod collision;
pub mod ragdoll;
pub mod rigidbody;
pub mod world;

#[cfg(feature = "2d")]
pub mod physics2d;

#[cfg(feature = "3d")]
pub mod physics3d;

pub use character::{CharacterController2D, CharacterController3D};
pub use collision::{ColliderShape, CollisionEvent, CollisionLayers};
pub use ragdoll::{Joint, JointConfig, JointType, RagdollConfig, RagdollController};
pub use rigidbody::{RigidbodyHandle, RigidbodyType};
pub use world::PhysicsWorld;

use lunaris_core::Result;

/// Physics configuration
#[derive(Debug, Clone)]
pub struct PhysicsConfig {
    /// Gravity vector
    pub gravity: lunaris_core::math::Vec3,
    /// Physics timestep (fixed)
    pub timestep: f32,
    /// Maximum substeps per frame
    pub max_substeps: u32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity: lunaris_core::math::Vec3::new(0.0, -9.81, 0.0),
            timestep: 1.0 / 60.0,
            max_substeps: 4,
        }
    }
}

/// Initialize the physics subsystem
///
/// # Errors
///
/// Returns an error if initialization fails
pub fn init() -> Result<()> {
    tracing::info!("Physics subsystem initialized");
    Ok(())
}
