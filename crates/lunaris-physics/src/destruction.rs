//! Destruction System
//!
//! Real-time destructible environments and objects.

use glam::{Vec3, Quat};
use std::collections::HashMap;

/// Destruction chunk
#[derive(Debug, Clone)]
pub struct DestructionChunk {
    /// Chunk ID
    pub id: u64,
    /// Mesh ID
    pub mesh_id: u64,
    /// Local position
    pub position: Vec3,
    /// Local rotation
    pub rotation: Quat,
    /// Scale
    pub scale: Vec3,
    /// Mass
    pub mass: f32,
    /// Health
    pub health: f32,
    /// Is detached
    pub detached: bool,
    /// Velocity (when detached)
    pub velocity: Vec3,
    /// Angular velocity
    pub angular_velocity: Vec3,
    /// Connection indices
    pub connections: Vec<u64>,
}

/// Destructible object
#[derive(Debug, Clone)]
pub struct Destructible {
    /// Object ID
    pub id: u64,
    /// Chunks
    pub chunks: HashMap<u64, DestructionChunk>,
    /// Is destroyed
    pub destroyed: bool,
    /// Destruction threshold
    pub health: f32,
    /// Max health
    pub max_health: f32,
    /// Fracture pattern
    pub pattern: FracturePattern,
    /// Physics enabled
    pub physics_enabled: bool,
    /// Debris lifetime
    pub debris_lifetime: f32,
    /// On destroy callback
    pub on_destroy: Option<fn(u64)>,
}

/// Fracture pattern
#[derive(Debug, Clone, Copy, Default)]
pub enum FracturePattern {
    /// Voronoi fracture
    #[default]
    Voronoi,
    /// Uniform grid
    Uniform,
    /// Radial from impact
    Radial,
    /// Slice planes
    Slice,
    /// Pre-fractured
    PreFractured,
}

impl Destructible {
    /// Create a new destructible
    #[must_use]
    pub fn new(health: f32) -> Self {
        Self {
            id: 0,
            chunks: HashMap::new(),
            destroyed: false,
            health,
            max_health: health,
            pattern: FracturePattern::Voronoi,
            physics_enabled: true,
            debris_lifetime: 10.0,
            on_destroy: None,
        }
    }

    /// Apply damage
    pub fn apply_damage(&mut self, amount: f32, impact_point: Vec3, impact_force: Vec3) {
        self.health -= amount;
        
        if self.health <= 0.0 && !self.destroyed {
            self.fracture(impact_point, impact_force);
        }
    }

    /// Fracture the object
    pub fn fracture(&mut self, impact_point: Vec3, impact_force: Vec3) {
        self.destroyed = true;
        
        // Detach all chunks
        for chunk in self.chunks.values_mut() {
            chunk.detached = true;
            
            // Apply impulse from impact
            let to_chunk = chunk.position - impact_point;
            let dist = to_chunk.length().max(0.1);
            let force = impact_force / (dist * dist);
            chunk.velocity = force / chunk.mass;
            
            // Add some random angular velocity
            chunk.angular_velocity = Vec3::new(
                (chunk.id as f32 * 12.9898).sin(),
                (chunk.id as f32 * 78.233).sin(),
                (chunk.id as f32 * 45.164).sin(),
            ) * 5.0;
        }

        if let Some(callback) = self.on_destroy {
            callback(self.id);
        }
    }

    /// Update physics for chunks
    pub fn update(&mut self, delta_time: f32, gravity: Vec3) {
        if !self.physics_enabled {
            return;
        }

        for chunk in self.chunks.values_mut() {
            if chunk.detached {
                // Apply gravity
                chunk.velocity += gravity * delta_time;
                chunk.position += chunk.velocity * delta_time;
                
                // Apply angular velocity
                let angle = chunk.angular_velocity.length() * delta_time;
                if angle > 0.0 {
                    let axis = chunk.angular_velocity.normalize();
                    chunk.rotation = Quat::from_axis_angle(axis, angle) * chunk.rotation;
                }
                
                // Damping
                chunk.velocity *= 0.99;
                chunk.angular_velocity *= 0.98;
            }
        }
    }

    /// Add a chunk
    pub fn add_chunk(&mut self, chunk: DestructionChunk) {
        self.chunks.insert(chunk.id, chunk);
    }
}

/// Voronoi cell generator
pub struct VoronoiFracturer {
    /// Number of cells
    pub cell_count: u32,
    /// Seed
    pub seed: u64,
    /// Min cell size
    pub min_size: f32,
}

impl Default for VoronoiFracturer {
    fn default() -> Self {
        Self {
            cell_count: 10,
            seed: 0,
            min_size: 0.1,
        }
    }
}

impl VoronoiFracturer {
    /// Generate Voronoi points
    #[must_use]
    pub fn generate_points(&self, bounds_min: Vec3, bounds_max: Vec3) -> Vec<Vec3> {
        let mut points = Vec::new();
        let size = bounds_max - bounds_min;
        
        for i in 0..self.cell_count {
            let seed = self.seed.wrapping_add(i as u64);
            let x = Self::hash(seed) * size.x + bounds_min.x;
            let y = Self::hash(seed.wrapping_add(1)) * size.y + bounds_min.y;
            let z = Self::hash(seed.wrapping_add(2)) * size.z + bounds_min.z;
            points.push(Vec3::new(x, y, z));
        }
        
        points
    }

    fn hash(seed: u64) -> f32 {
        let x = (seed as f32 * 12.9898).sin() * 43758.5453;
        x.fract()
    }
}

/// Destruction manager
pub struct DestructionManager {
    /// All destructibles
    destructibles: HashMap<u64, Destructible>,
    /// Next ID
    next_id: u64,
    /// Fracturer
    pub fracturer: VoronoiFracturer,
    /// Max debris count
    pub max_debris: usize,
    /// Auto cleanup
    pub auto_cleanup: bool,
}

impl Default for DestructionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DestructionManager {
    /// Create a new manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            destructibles: HashMap::new(),
            next_id: 1,
            fracturer: VoronoiFracturer::default(),
            max_debris: 500,
            auto_cleanup: true,
        }
    }

    /// Register a destructible
    pub fn register(&mut self, mut destructible: Destructible) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        destructible.id = id;
        self.destructibles.insert(id, destructible);
        id
    }

    /// Apply damage to destructible
    pub fn damage(&mut self, id: u64, amount: f32, impact_point: Vec3, impact_force: Vec3) {
        if let Some(destructible) = self.destructibles.get_mut(&id) {
            destructible.apply_damage(amount, impact_point, impact_force);
        }
    }

    /// Update all destructibles
    pub fn update(&mut self, delta_time: f32, gravity: Vec3) {
        for destructible in self.destructibles.values_mut() {
            destructible.update(delta_time, gravity);
        }

        // Cleanup if needed
        if self.auto_cleanup {
            self.cleanup_debris();
        }
    }

    fn cleanup_debris(&mut self) {
        // Remove debris that has fallen too far
        for destructible in self.destructibles.values_mut() {
            destructible.chunks.retain(|_, chunk| {
                !chunk.detached || chunk.position.y > -100.0
            });
        }
    }

    /// Get destructible
    #[must_use]
    pub fn get(&self, id: u64) -> Option<&Destructible> {
        self.destructibles.get(&id)
    }

    /// Get debris count
    #[must_use]
    pub fn debris_count(&self) -> usize {
        self.destructibles.values()
            .flat_map(|d| d.chunks.values())
            .filter(|c| c.detached)
            .count()
    }
}
