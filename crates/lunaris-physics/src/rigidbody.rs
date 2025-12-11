//! Rigid body types and components

use lunaris_core::{id::Id, math::Vec3};

/// Handle to a rigidbody in the physics world
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RigidbodyHandle(pub Id);

/// Rigidbody type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RigidbodyType {
    /// Dynamic body - fully simulated with forces and collisions
    #[default]
    Dynamic,
    /// Kinematic body - moved by code, affects dynamic bodies
    Kinematic,
    /// Static body - never moves, infinite mass
    Static,
}

/// Rigidbody properties
#[derive(Debug, Clone)]
pub struct RigidbodyProperties {
    /// Body type
    pub body_type: RigidbodyType,
    /// Mass in kg (ignored for static/kinematic)
    pub mass: f32,
    /// Linear damping (air resistance)
    pub linear_damping: f32,
    /// Angular damping (rotational resistance)
    pub angular_damping: f32,
    /// Gravity scale (1.0 = normal gravity)
    pub gravity_scale: f32,
    /// Lock rotation on X axis
    pub lock_rotation_x: bool,
    /// Lock rotation on Y axis
    pub lock_rotation_y: bool,
    /// Lock rotation on Z axis
    pub lock_rotation_z: bool,
    /// Is the body a sensor (trigger)
    pub is_sensor: bool,
    /// Continuous collision detection
    pub ccd_enabled: bool,
}

impl Default for RigidbodyProperties {
    fn default() -> Self {
        Self {
            body_type: RigidbodyType::Dynamic,
            mass: 1.0,
            linear_damping: 0.0,
            angular_damping: 0.05,
            gravity_scale: 1.0,
            lock_rotation_x: false,
            lock_rotation_y: false,
            lock_rotation_z: false,
            is_sensor: false,
            ccd_enabled: false,
        }
    }
}

impl RigidbodyProperties {
    /// Create a dynamic rigidbody
    #[must_use]
    pub fn dynamic() -> Self {
        Self::default()
    }

    /// Create a kinematic rigidbody
    #[must_use]
    pub fn kinematic() -> Self {
        Self {
            body_type: RigidbodyType::Kinematic,
            ..Default::default()
        }
    }

    /// Create a static rigidbody
    #[must_use]
    pub fn static_body() -> Self {
        Self {
            body_type: RigidbodyType::Static,
            ..Default::default()
        }
    }

    /// Set mass
    #[must_use]
    pub const fn with_mass(mut self, mass: f32) -> Self {
        self.mass = mass;
        self
    }

    /// Set gravity scale
    #[must_use]
    pub const fn with_gravity_scale(mut self, scale: f32) -> Self {
        self.gravity_scale = scale;
        self
    }

    /// Lock all rotation axes
    #[must_use]
    pub const fn with_locked_rotation(mut self) -> Self {
        self.lock_rotation_x = true;
        self.lock_rotation_y = true;
        self.lock_rotation_z = true;
        self
    }

    /// Enable CCD
    #[must_use]
    pub const fn with_ccd(mut self) -> Self {
        self.ccd_enabled = true;
        self
    }
}

/// Rigidbody state (read from physics world)
#[derive(Debug, Clone, Copy, Default)]
pub struct RigidbodyState {
    /// Position in world space
    pub position: Vec3,
    /// Rotation (euler angles in radians)
    pub rotation: Vec3,
    /// Linear velocity
    pub linear_velocity: Vec3,
    /// Angular velocity
    pub angular_velocity: Vec3,
}

/// Forces to apply to a rigidbody
#[derive(Debug, Clone, Copy)]
pub enum ForceMode {
    /// Apply as a force (affected by mass)
    Force,
    /// Apply as an impulse (instant velocity change)
    Impulse,
    /// Apply as an acceleration (not affected by mass)
    Acceleration,
    /// Apply as a velocity change
    VelocityChange,
}

/// Collider properties
#[derive(Debug, Clone)]
pub struct ColliderProperties {
    /// Friction coefficient (0-1)
    pub friction: f32,
    /// Restitution/bounciness (0-1)
    pub restitution: f32,
    /// Density (for mass calculation)
    pub density: f32,
    /// Collision layers
    pub layers: super::collision::CollisionLayers,
}

impl Default for ColliderProperties {
    fn default() -> Self {
        Self {
            friction: 0.5,
            restitution: 0.0,
            density: 1.0,
            layers: super::collision::CollisionLayers::default(),
        }
    }
}

impl ColliderProperties {
    /// Bouncy material
    #[must_use]
    pub fn bouncy() -> Self {
        Self {
            restitution: 0.9,
            ..Default::default()
        }
    }

    /// Slippery material (ice)
    #[must_use]
    pub fn slippery() -> Self {
        Self {
            friction: 0.05,
            ..Default::default()
        }
    }

    /// Rough material
    #[must_use]
    pub fn rough() -> Self {
        Self {
            friction: 0.9,
            ..Default::default()
        }
    }
}
