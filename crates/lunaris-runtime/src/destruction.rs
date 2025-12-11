//! Destruction System
//!
//! Real-time fracturing, deformation, and debris.

use glam::{Vec3, Quat};

/// Destructible
pub struct Destructible {
    pub id: u64,
    pub fragments: Vec<Fragment>,
    pub health: f32,
    pub fracture_settings: FractureSettings,
    pub state: DestructibleState,
}

/// Fragment
pub struct Fragment {
    pub id: u64,
    pub mesh_id: u64,
    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
    pub angular_velocity: Vec3,
    pub mass: f32,
    pub debris: bool,
    pub lifetime: f32,
}

/// Fracture settings
pub struct FractureSettings {
    pub pattern: FracturePattern,
    pub fragment_count: u32,
    pub randomness: f32,
    pub interior_material: String,
    pub impact_resistance: f32,
    pub support_depth: u32,
    pub debris_lifetime: f32,
}

/// Fracture pattern
pub enum FracturePattern { Voronoi, Uniform, Radial, Slice, Custom }

/// Destructible state
pub enum DestructibleState { Intact, Fractured, Destroyed }

impl Default for FractureSettings {
    fn default() -> Self {
        Self { pattern: FracturePattern::Voronoi, fragment_count: 20, randomness: 0.5, interior_material: "default".into(), impact_resistance: 100.0, support_depth: 1, debris_lifetime: 5.0 }
    }
}

impl Destructible {
    pub fn new(id: u64) -> Self {
        Self { id, fragments: Vec::new(), health: 100.0, fracture_settings: FractureSettings::default(), state: DestructibleState::Intact }
    }

    pub fn damage(&mut self, amount: f32, impact_point: Vec3, force: Vec3) {
        self.health -= amount;
        if self.health <= 0.0 && matches!(self.state, DestructibleState::Intact) {
            self.fracture(impact_point, force);
        }
    }

    pub fn fracture(&mut self, impact_point: Vec3, force: Vec3) {
        self.state = DestructibleState::Fractured;
        let count = self.fracture_settings.fragment_count;
        
        for i in 0..count {
            let angle = (i as f32 / count as f32) * std::f32::consts::TAU;
            let offset = Vec3::new(angle.cos(), 0.0, angle.sin()) * (1.0 + rand() * self.fracture_settings.randomness);
            let velocity = force.normalize() * 5.0 + offset * 2.0;
            
            self.fragments.push(Fragment {
                id: i as u64,
                mesh_id: 0,
                position: impact_point + offset * 0.5,
                rotation: Quat::from_rotation_y(angle),
                velocity,
                angular_velocity: Vec3::new(rand() - 0.5, rand() - 0.5, rand() - 0.5) * 5.0,
                mass: 1.0 / count as f32,
                debris: i > count / 2,
                lifetime: self.fracture_settings.debris_lifetime,
            });
        }
    }

    pub fn update(&mut self, dt: f32, gravity: Vec3) {
        for frag in &mut self.fragments {
            frag.velocity += gravity * dt;
            frag.position += frag.velocity * dt;
            frag.rotation = frag.rotation * Quat::from_scaled_axis(frag.angular_velocity * dt);
            frag.angular_velocity *= 0.99;
            if frag.debris { frag.lifetime -= dt; }
        }
        self.fragments.retain(|f| !f.debris || f.lifetime > 0.0);
    }
}

fn rand() -> f32 { 0.5 } // Placeholder

/// Deformable mesh
pub struct Deformable {
    pub vertices: Vec<Vec3>,
    pub original: Vec<Vec3>,
    pub deformation_strength: f32,
    pub recovery_rate: f32,
    pub plastic_threshold: f32,
}

impl Deformable {
    pub fn new(vertices: Vec<Vec3>) -> Self {
        Self { original: vertices.clone(), vertices, deformation_strength: 1.0, recovery_rate: 0.0, plastic_threshold: 0.5 }
    }

    pub fn apply_force(&mut self, point: Vec3, force: Vec3, radius: f32) {
        for (i, vert) in self.vertices.iter_mut().enumerate() {
            let dist = (*vert - point).length();
            if dist < radius {
                let factor = 1.0 - dist / radius;
                *vert += force * factor * self.deformation_strength;
            }
        }
    }

    pub fn recover(&mut self, dt: f32) {
        if self.recovery_rate > 0.0 {
            for (i, vert) in self.vertices.iter_mut().enumerate() {
                let diff = self.original[i] - *vert;
                if diff.length() < self.plastic_threshold {
                    *vert += diff * self.recovery_rate * dt;
                }
            }
        }
    }
}
