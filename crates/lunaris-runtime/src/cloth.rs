//! Cloth and Hair Simulation
//!
//! Physics-based cloth and strand simulation.

use glam::{Vec3, Vec4};

/// Cloth simulation
pub struct ClothSimulation {
    pub particles: Vec<ClothParticle>,
    pub constraints: Vec<ClothConstraint>,
    pub settings: ClothSettings,
    pub bounds: ClothBounds,
}

/// Cloth particle
pub struct ClothParticle {
    pub position: Vec3,
    pub prev_position: Vec3,
    pub velocity: Vec3,
    pub mass: f32,
    pub pinned: bool,
}

/// Cloth constraint
pub struct ClothConstraint {
    pub particle_a: usize,
    pub particle_b: usize,
    pub rest_length: f32,
    pub stiffness: f32,
}

/// Cloth settings
pub struct ClothSettings {
    pub gravity: Vec3,
    pub damping: f32,
    pub iterations: u32,
    pub wind: Vec3,
    pub wind_turbulence: f32,
}

impl Default for ClothSettings {
    fn default() -> Self {
        Self { gravity: Vec3::new(0.0, -9.81, 0.0), damping: 0.98, iterations: 5, wind: Vec3::ZERO, wind_turbulence: 0.0 }
    }
}

/// Cloth bounds
pub struct ClothBounds {
    pub min: Vec3,
    pub max: Vec3,
}

impl ClothSimulation {
    pub fn new(width: u32, height: u32, spacing: f32) -> Self {
        let mut particles = Vec::new();
        let mut constraints = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let pos = Vec3::new(x as f32 * spacing, 0.0, y as f32 * spacing);
                particles.push(ClothParticle {
                    position: pos, prev_position: pos, velocity: Vec3::ZERO, mass: 1.0, pinned: y == 0,
                });
            }
        }

        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) as usize;
                if x < width - 1 {
                    constraints.push(ClothConstraint { particle_a: idx, particle_b: idx + 1, rest_length: spacing, stiffness: 1.0 });
                }
                if y < height - 1 {
                    constraints.push(ClothConstraint { particle_a: idx, particle_b: idx + width as usize, rest_length: spacing, stiffness: 1.0 });
                }
            }
        }

        Self { particles, constraints, settings: ClothSettings::default(), bounds: ClothBounds { min: Vec3::ZERO, max: Vec3::splat(spacing * width as f32) } }
    }

    pub fn simulate(&mut self, dt: f32) {
        for p in &mut self.particles {
            if p.pinned { continue; }
            let acc = self.settings.gravity + self.settings.wind;
            let new_pos = p.position + (p.position - p.prev_position) * self.settings.damping + acc * dt * dt;
            p.prev_position = p.position;
            p.position = new_pos;
        }

        for _ in 0..self.settings.iterations {
            for c in &self.constraints {
                let pa = self.particles[c.particle_a].position;
                let pb = self.particles[c.particle_b].position;
                let delta = pb - pa;
                let len = delta.length();
                let diff = (len - c.rest_length) / len * 0.5 * c.stiffness;
                if !self.particles[c.particle_a].pinned { self.particles[c.particle_a].position += delta * diff; }
                if !self.particles[c.particle_b].pinned { self.particles[c.particle_b].position -= delta * diff; }
            }
        }
    }
}

/// Hair/Groom system
pub struct GroomSystem {
    pub strands: Vec<HairStrand>,
    pub settings: GroomSettings,
}

/// Hair strand
pub struct HairStrand {
    pub points: Vec<Vec3>,
    pub root_position: Vec3,
    pub thickness: f32,
    pub color: Vec4,
}

/// Groom settings
pub struct GroomSettings {
    pub gravity: f32,
    pub stiffness: f32,
    pub damping: f32,
    pub wind_response: f32,
    pub length_variance: f32,
    pub thickness_tip_scale: f32,
}

impl Default for GroomSettings {
    fn default() -> Self {
        Self { gravity: 1.0, stiffness: 0.5, damping: 0.9, wind_response: 0.5, length_variance: 0.2, thickness_tip_scale: 0.1 }
    }
}

impl GroomSystem {
    pub fn new() -> Self {
        Self { strands: Vec::new(), settings: GroomSettings::default() }
    }

    pub fn add_strand(&mut self, root: Vec3, length: f32, segments: u32) {
        let mut points = Vec::new();
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            points.push(root + Vec3::new(0.0, -length * t, 0.0));
        }
        self.strands.push(HairStrand { points, root_position: root, thickness: 0.01, color: Vec4::new(0.2, 0.1, 0.05, 1.0) });
    }

    pub fn simulate(&mut self, dt: f32, wind: Vec3) {
        for strand in &mut self.strands {
            strand.points[0] = strand.root_position;
            for i in 1..strand.points.len() {
                let gravity = Vec3::new(0.0, -self.settings.gravity * dt * dt, 0.0);
                let wind_force = wind * self.settings.wind_response * dt * dt;
                strand.points[i] += gravity + wind_force;
                
                let prev = strand.points[i - 1];
                let curr = strand.points[i];
                let rest_len = 0.02;
                let delta = curr - prev;
                let len = delta.length();
                if len > 0.001 {
                    strand.points[i] = prev + delta.normalize() * rest_len;
                }
            }
        }
    }
}
