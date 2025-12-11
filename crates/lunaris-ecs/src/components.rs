//! Core ECS components for game development

use bevy_ecs::prelude::*;
use lunaris_core::math::{Color, Vec2, Vec3};

/// Entity name component
#[derive(Component, Debug, Clone)]
pub struct Name(pub String);

impl Name {
    /// Create a new name
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Get the name as a string slice
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 2D Transform component
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Transform2D {
    /// Position
    pub position: Vec2,
    /// Rotation in radians
    pub rotation: f32,
    /// Scale
    pub scale: Vec2,
}

impl Transform2D {
    /// Identity transform
    pub const IDENTITY: Self = Self {
        position: Vec2::ZERO,
        rotation: 0.0,
        scale: Vec2::ONE,
    };

    /// Create from position
    #[must_use]
    pub const fn from_position(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }

    /// Create full transform
    #[must_use]
    pub const fn new(position: Vec2, rotation: f32, scale: Vec2) -> Self {
        Self { position, rotation, scale }
    }

    /// Move by delta
    pub fn translate(&mut self, delta: Vec2) {
        self.position = self.position + delta;
    }

    /// Rotate by delta radians
    pub fn rotate(&mut self, delta: f32) {
        self.rotation += delta;
    }
}

/// 3D Transform component
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Transform3D {
    /// Position
    pub position: Vec3,
    /// Rotation (Euler angles in radians)
    pub rotation: Vec3,
    /// Scale
    pub scale: Vec3,
}

impl Transform3D {
    /// Identity transform
    pub const IDENTITY: Self = Self {
        position: Vec3::ZERO,
        rotation: Vec3::ZERO,
        scale: Vec3::ONE,
    };

    /// Create from position
    #[must_use]
    pub const fn from_position(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vec3::new(x, y, z),
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
        }
    }

    /// Create full transform
    #[must_use]
    pub const fn new(position: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        Self { position, rotation, scale }
    }

    /// Move by delta
    pub fn translate(&mut self, delta: Vec3) {
        self.position = self.position + delta;
    }
}

/// Global transform (computed from hierarchy)
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct GlobalTransform3D {
    /// World position
    pub position: Vec3,
    /// World rotation
    pub rotation: Vec3,
    /// World scale
    pub scale: Vec3,
}

/// Visibility component
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Visibility {
    /// Is this entity visible
    pub is_visible: bool,
}

impl Visibility {
    /// Visible
    pub const VISIBLE: Self = Self { is_visible: true };
    /// Hidden
    pub const HIDDEN: Self = Self { is_visible: false };
}

/// Computed visibility (considering parent visibility)
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct ComputedVisibility {
    /// Is visible in hierarchy
    pub is_visible_in_hierarchy: bool,
    /// Is visible to cameras
    pub is_visible_in_view: bool,
}

/// Sprite component for 2D rendering
#[derive(Component, Debug, Clone)]
pub struct Sprite {
    /// Texture/sprite to render
    pub texture: lunaris_core::id::Id,
    /// Tint color
    pub color: Color,
    /// Flip horizontally
    pub flip_x: bool,
    /// Flip vertically
    pub flip_y: bool,
    /// Custom size (None = use texture size)
    pub custom_size: Option<Vec2>,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            texture: lunaris_core::id::Id::NULL,
            color: Color::WHITE,
            flip_x: false,
            flip_y: false,
            custom_size: None,
        }
    }
}

/// Camera component
#[derive(Component, Debug, Clone)]
pub struct Camera {
    /// Is this the active camera
    pub is_active: bool,
    /// Render priority (higher = renders later)
    pub priority: i32,
    /// Clear color
    pub clear_color: Option<Color>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            is_active: true,
            priority: 0,
            clear_color: Some(Color::BLACK),
        }
    }
}

/// 2D Camera settings
#[derive(Component, Debug, Clone, Copy)]
pub struct Camera2DSettings {
    /// Zoom level
    pub zoom: f32,
}

impl Default for Camera2DSettings {
    fn default() -> Self {
        Self { zoom: 1.0 }
    }
}

/// 3D Camera settings
#[derive(Component, Debug, Clone, Copy)]
pub struct Camera3DSettings {
    /// Field of view in radians
    pub fov: f32,
    /// Near clipping plane
    pub near: f32,
    /// Far clipping plane
    pub far: f32,
}

impl Default for Camera3DSettings {
    fn default() -> Self {
        Self {
            fov: std::f32::consts::FRAC_PI_4,
            near: 0.1,
            far: 1000.0,
        }
    }
}

/// Tag component for player entities
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Player;

/// Tag component for enemy entities
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Enemy;

/// Health component
#[derive(Component, Debug, Clone, Copy)]
pub struct Health {
    /// Current health
    pub current: f32,
    /// Maximum health
    pub max: f32,
}

impl Health {
    /// Create new health
    #[must_use]
    pub const fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    /// Take damage
    pub fn damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    /// Heal
    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    /// Check if dead
    #[must_use]
    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    /// Get health percentage
    #[must_use]
    pub fn percentage(&self) -> f32 {
        if self.max > 0.0 {
            self.current / self.max
        } else {
            0.0
        }
    }
}

/// Velocity component for physics
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Velocity2D {
    /// Linear velocity
    pub linear: Vec2,
    /// Angular velocity (radians per second)
    pub angular: f32,
}

/// Velocity component for 3D physics
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Velocity3D {
    /// Linear velocity
    pub linear: Vec3,
    /// Angular velocity 
    pub angular: Vec3,
}
