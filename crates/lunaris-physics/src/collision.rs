//! Collision detection and shapes

use lunaris_core::math::{Vec2, Vec3};

/// Collision layers for filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollisionLayers {
    /// Groups this collider belongs to
    pub membership: u32,
    /// Groups this collider can collide with
    pub filter: u32,
}

impl Default for CollisionLayers {
    fn default() -> Self {
        Self {
            membership: 0xFFFFFFFF,
            filter: 0xFFFFFFFF,
        }
    }
}

impl CollisionLayers {
    /// Create layers with specific membership
    #[must_use]
    pub const fn new(membership: u32, filter: u32) -> Self {
        Self { membership, filter }
    }

    /// Check if two layers can interact
    #[must_use]
    pub fn can_interact(self, other: Self) -> bool {
        (self.membership & other.filter) != 0 && (other.membership & self.filter) != 0
    }

    /// Predefined: Default layer
    pub const DEFAULT: Self = Self { membership: 1, filter: 0xFFFFFFFF };
    /// Predefined: Player layer
    pub const PLAYER: Self = Self { membership: 2, filter: 0xFFFFFFFF };
    /// Predefined: Enemy layer
    pub const ENEMY: Self = Self { membership: 4, filter: 0xFFFFFFFF };
    /// Predefined: Projectile layer
    pub const PROJECTILE: Self = Self { membership: 8, filter: 0xFFFFFFFF };
    /// Predefined: Environment layer
    pub const ENVIRONMENT: Self = Self { membership: 16, filter: 0xFFFFFFFF };
    /// Predefined: Trigger layer (no physical collision)
    pub const TRIGGER: Self = Self { membership: 32, filter: 0 };
}

/// 2D Collider shapes
#[derive(Debug, Clone)]
pub enum ColliderShape2D {
    /// Circle with radius
    Circle { radius: f32 },
    /// Rectangle with half-extents
    Rectangle { half_width: f32, half_height: f32 },
    /// Capsule (rounded rectangle)
    Capsule { half_height: f32, radius: f32 },
    /// Convex polygon
    ConvexPolygon { vertices: Vec<Vec2> },
    /// Compound shape (multiple shapes)
    Compound { shapes: Vec<(Vec2, f32, Box<ColliderShape2D>)> },
}

impl ColliderShape2D {
    /// Create a circle
    #[must_use]
    pub const fn circle(radius: f32) -> Self {
        Self::Circle { radius }
    }

    /// Create a rectangle
    #[must_use]
    pub const fn rectangle(width: f32, height: f32) -> Self {
        Self::Rectangle {
            half_width: width / 2.0,
            half_height: height / 2.0,
        }
    }

    /// Create a capsule
    #[must_use]
    pub const fn capsule(height: f32, radius: f32) -> Self {
        Self::Capsule {
            half_height: height / 2.0,
            radius,
        }
    }
}

/// 3D Collider shapes
#[derive(Debug, Clone)]
pub enum ColliderShape3D {
    /// Sphere with radius
    Sphere { radius: f32 },
    /// Box with half-extents
    Box { half_extents: Vec3 },
    /// Capsule
    Capsule { half_height: f32, radius: f32 },
    /// Cylinder
    Cylinder { half_height: f32, radius: f32 },
    /// Convex hull
    ConvexHull { vertices: Vec<Vec3> },
    /// Triangle mesh (for static geometry)
    TriMesh { vertices: Vec<Vec3>, indices: Vec<[u32; 3]> },
    /// Compound shape
    Compound { shapes: Vec<(Vec3, Vec3, Box<ColliderShape3D>)> },
}

impl ColliderShape3D {
    /// Create a sphere
    #[must_use]
    pub const fn sphere(radius: f32) -> Self {
        Self::Sphere { radius }
    }

    /// Create a box
    #[must_use]
    pub fn cube(size: f32) -> Self {
        Self::Box {
            half_extents: Vec3::new(size / 2.0, size / 2.0, size / 2.0),
        }
    }

    /// Create a box with dimensions
    #[must_use]
    pub fn box_shape(width: f32, height: f32, depth: f32) -> Self {
        Self::Box {
            half_extents: Vec3::new(width / 2.0, height / 2.0, depth / 2.0),
        }
    }

    /// Create a capsule
    #[must_use]
    pub const fn capsule(height: f32, radius: f32) -> Self {
        Self::Capsule {
            half_height: height / 2.0,
            radius,
        }
    }
}

/// Generic collider shape enum
#[derive(Debug, Clone)]
pub enum ColliderShape {
    /// 2D shape
    Shape2D(ColliderShape2D),
    /// 3D shape
    Shape3D(ColliderShape3D),
}

/// Collision event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionEventType {
    /// Collision started
    Started,
    /// Collision ongoing (for continuous detection)
    Ongoing,
    /// Collision ended
    Ended,
}

/// Collision event data
#[derive(Debug, Clone)]
pub struct CollisionEvent {
    /// Type of event
    pub event_type: CollisionEventType,
    /// First collider entity
    pub entity_a: lunaris_core::id::Id,
    /// Second collider entity
    pub entity_b: lunaris_core::id::Id,
    /// Contact points (world space)
    pub contacts: Vec<ContactPoint>,
}

/// Contact point information
#[derive(Debug, Clone, Copy)]
pub struct ContactPoint {
    /// World position of contact
    pub position: Vec3,
    /// Contact normal (pointing from A to B)
    pub normal: Vec3,
    /// Penetration depth
    pub depth: f32,
}

/// Raycast hit result
#[derive(Debug, Clone)]
pub struct RaycastHit {
    /// Entity that was hit
    pub entity: lunaris_core::id::Id,
    /// Hit position in world space
    pub point: Vec3,
    /// Surface normal at hit point
    pub normal: Vec3,
    /// Distance from ray origin
    pub distance: f32,
}

/// Raycast query
#[derive(Debug, Clone)]
pub struct RaycastQuery {
    /// Ray origin
    pub origin: Vec3,
    /// Ray direction (normalized)
    pub direction: Vec3,
    /// Maximum distance
    pub max_distance: f32,
    /// Collision layers to query
    pub layers: CollisionLayers,
}

impl RaycastQuery {
    /// Create a new raycast query
    #[must_use]
    pub fn new(origin: Vec3, direction: Vec3, max_distance: f32) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
            max_distance,
            layers: CollisionLayers::default(),
        }
    }
}
