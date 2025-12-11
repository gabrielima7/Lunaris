//! Cloth Physics Simulation
//!
//! Real-time cloth with constraints and collisions.

use glam::Vec3;

/// Cloth particle
#[derive(Debug, Clone)]
pub struct ClothParticle {
    /// Current position
    pub position: Vec3,
    /// Previous position (Verlet)
    pub prev_position: Vec3,
    /// Acceleration
    pub acceleration: Vec3,
    /// Mass (0 = fixed)
    pub mass: f32,
    /// Is pinned
    pub pinned: bool,
    /// UV coordinates
    pub uv: [f32; 2],
}

impl ClothParticle {
    /// Create a new particle
    #[must_use]
    pub fn new(position: Vec3, mass: f32) -> Self {
        Self {
            position,
            prev_position: position,
            acceleration: Vec3::ZERO,
            mass,
            pinned: false,
            uv: [0.0, 0.0],
        }
    }

    /// Apply force
    pub fn apply_force(&mut self, force: Vec3) {
        if self.mass > 0.0 {
            self.acceleration += force / self.mass;
        }
    }

    /// Update position (Verlet integration)
    pub fn update(&mut self, delta_time: f32, damping: f32) {
        if self.pinned || self.mass <= 0.0 {
            return;
        }

        let velocity = (self.position - self.prev_position) * damping;
        self.prev_position = self.position;
        self.position += velocity + self.acceleration * delta_time * delta_time;
        self.acceleration = Vec3::ZERO;
    }
}

/// Distance constraint
#[derive(Debug, Clone)]
pub struct DistanceConstraint {
    /// First particle index
    pub p1: usize,
    /// Second particle index
    pub p2: usize,
    /// Rest length
    pub rest_length: f32,
    /// Stiffness (0-1)
    pub stiffness: f32,
}

/// Bend constraint
#[derive(Debug, Clone)]
pub struct BendConstraint {
    /// Center particle
    pub center: usize,
    /// Connected particles
    pub p1: usize,
    pub p2: usize,
    /// Rest angle
    pub rest_angle: f32,
    /// Stiffness
    pub stiffness: f32,
}

/// Cloth configuration
#[derive(Debug, Clone)]
pub struct ClothConfig {
    /// Width in particles
    pub width: u32,
    /// Height in particles
    pub height: u32,
    /// Particle spacing
    pub spacing: f32,
    /// Mass per particle
    pub mass: f32,
    /// Structural stiffness
    pub structural_stiffness: f32,
    /// Shear stiffness
    pub shear_stiffness: f32,
    /// Bend stiffness
    pub bend_stiffness: f32,
    /// Damping
    pub damping: f32,
    /// Solver iterations
    pub iterations: u32,
    /// Collision radius
    pub collision_radius: f32,
}

impl Default for ClothConfig {
    fn default() -> Self {
        Self {
            width: 20,
            height: 20,
            spacing: 0.1,
            mass: 0.1,
            structural_stiffness: 1.0,
            shear_stiffness: 0.5,
            bend_stiffness: 0.1,
            damping: 0.99,
            iterations: 10,
            collision_radius: 0.02,
        }
    }
}

/// Cloth simulation
pub struct Cloth {
    /// Configuration
    pub config: ClothConfig,
    /// Particles
    pub particles: Vec<ClothParticle>,
    /// Distance constraints
    pub distance_constraints: Vec<DistanceConstraint>,
    /// Bend constraints
    pub bend_constraints: Vec<BendConstraint>,
    /// External forces
    gravity: Vec3,
    wind: Vec3,
}

impl Cloth {
    /// Create a new cloth grid
    #[must_use]
    pub fn new(config: ClothConfig, origin: Vec3) -> Self {
        let mut particles = Vec::new();
        let mut distance_constraints = Vec::new();
        let bend_constraints = Vec::new();

        // Create particles
        for y in 0..config.height {
            for x in 0..config.width {
                let pos = origin + Vec3::new(
                    x as f32 * config.spacing,
                    0.0,
                    y as f32 * config.spacing,
                );
                
                let mut particle = ClothParticle::new(pos, config.mass);
                particle.uv = [
                    x as f32 / (config.width - 1) as f32,
                    y as f32 / (config.height - 1) as f32,
                ];
                
                particles.push(particle);
            }
        }

        // Create structural constraints (horizontal and vertical)
        for y in 0..config.height {
            for x in 0..config.width {
                let idx = (y * config.width + x) as usize;
                
                // Horizontal
                if x < config.width - 1 {
                    distance_constraints.push(DistanceConstraint {
                        p1: idx,
                        p2: idx + 1,
                        rest_length: config.spacing,
                        stiffness: config.structural_stiffness,
                    });
                }
                
                // Vertical
                if y < config.height - 1 {
                    distance_constraints.push(DistanceConstraint {
                        p1: idx,
                        p2: idx + config.width as usize,
                        rest_length: config.spacing,
                        stiffness: config.structural_stiffness,
                    });
                }
                
                // Shear diagonal
                if x < config.width - 1 && y < config.height - 1 {
                    let diag_len = config.spacing * std::f32::consts::SQRT_2;
                    distance_constraints.push(DistanceConstraint {
                        p1: idx,
                        p2: idx + config.width as usize + 1,
                        rest_length: diag_len,
                        stiffness: config.shear_stiffness,
                    });
                    distance_constraints.push(DistanceConstraint {
                        p1: idx + 1,
                        p2: idx + config.width as usize,
                        rest_length: diag_len,
                        stiffness: config.shear_stiffness,
                    });
                }
            }
        }

        Self {
            config,
            particles,
            distance_constraints,
            bend_constraints,
            gravity: Vec3::new(0.0, -9.81, 0.0),
            wind: Vec3::ZERO,
        }
    }

    /// Pin a particle
    pub fn pin(&mut self, x: u32, y: u32) {
        let idx = (y * self.config.width + x) as usize;
        if let Some(particle) = self.particles.get_mut(idx) {
            particle.pinned = true;
        }
    }

    /// Unpin a particle
    pub fn unpin(&mut self, x: u32, y: u32) {
        let idx = (y * self.config.width + x) as usize;
        if let Some(particle) = self.particles.get_mut(idx) {
            particle.pinned = false;
        }
    }

    /// Set wind
    pub fn set_wind(&mut self, wind: Vec3) {
        self.wind = wind;
    }

    /// Set gravity
    pub fn set_gravity(&mut self, gravity: Vec3) {
        self.gravity = gravity;
    }

    /// Update simulation
    pub fn update(&mut self, delta_time: f32) {
        // Apply forces
        for particle in &mut self.particles {
            if !particle.pinned {
                particle.apply_force(self.gravity * particle.mass);
                particle.apply_force(self.wind);
            }
        }

        // Update particles
        for particle in &mut self.particles {
            particle.update(delta_time, self.config.damping);
        }

        // Solve constraints
        for _ in 0..self.config.iterations {
            self.solve_constraints();
        }
    }

    fn solve_constraints(&mut self) {
        // Distance constraints
        for constraint in &self.distance_constraints {
            let p1 = self.particles[constraint.p1].position;
            let p2 = self.particles[constraint.p2].position;
            
            let delta = p2 - p1;
            let distance = delta.length();
            
            if distance < 0.0001 {
                continue;
            }
            
            let diff = (distance - constraint.rest_length) / distance;
            let correction = delta * diff * 0.5 * constraint.stiffness;
            
            if !self.particles[constraint.p1].pinned {
                self.particles[constraint.p1].position += correction;
            }
            if !self.particles[constraint.p2].pinned {
                self.particles[constraint.p2].position -= correction;
            }
        }
    }

    /// Collide with sphere
    pub fn collide_sphere(&mut self, center: Vec3, radius: f32) {
        let total_radius = radius + self.config.collision_radius;
        
        for particle in &mut self.particles {
            if particle.pinned {
                continue;
            }
            
            let delta = particle.position - center;
            let distance = delta.length();
            
            if distance < total_radius {
                let correction = delta.normalize() * (total_radius - distance);
                particle.position += correction;
            }
        }
    }

    /// Collide with plane
    pub fn collide_plane(&mut self, point: Vec3, normal: Vec3) {
        for particle in &mut self.particles {
            if particle.pinned {
                continue;
            }
            
            let dist = (particle.position - point).dot(normal);
            if dist < self.config.collision_radius {
                particle.position += normal * (self.config.collision_radius - dist);
            }
        }
    }

    /// Get particle count
    #[must_use]
    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }

    /// Get triangle indices
    #[must_use]
    pub fn get_indices(&self) -> Vec<u32> {
        let mut indices = Vec::new();
        
        for y in 0..(self.config.height - 1) {
            for x in 0..(self.config.width - 1) {
                let i00 = y * self.config.width + x;
                let i10 = i00 + 1;
                let i01 = i00 + self.config.width;
                let i11 = i01 + 1;
                
                indices.push(i00);
                indices.push(i01);
                indices.push(i10);
                
                indices.push(i10);
                indices.push(i01);
                indices.push(i11);
            }
        }
        
        indices
    }

    /// Calculate normals
    #[must_use]
    pub fn calculate_normals(&self) -> Vec<Vec3> {
        let mut normals = vec![Vec3::ZERO; self.particles.len()];
        
        for y in 0..(self.config.height - 1) {
            for x in 0..(self.config.width - 1) {
                let i00 = (y * self.config.width + x) as usize;
                let i10 = i00 + 1;
                let i01 = i00 + self.config.width as usize;
                let i11 = i01 + 1;
                
                let p00 = self.particles[i00].position;
                let p10 = self.particles[i10].position;
                let p01 = self.particles[i01].position;
                let p11 = self.particles[i11].position;
                
                let n1 = (p01 - p00).cross(p10 - p00).normalize();
                let n2 = (p10 - p11).cross(p01 - p11).normalize();
                
                normals[i00] += n1;
                normals[i10] += n1 + n2;
                normals[i01] += n1 + n2;
                normals[i11] += n2;
            }
        }
        
        for normal in &mut normals {
            *normal = normal.normalize();
        }
        
        normals
    }
}

/// Soft body simulation (simplified)
pub struct SoftBody {
    /// Particles
    pub particles: Vec<ClothParticle>,
    /// Constraints
    pub constraints: Vec<DistanceConstraint>,
    /// Volume constraint
    pub target_volume: f32,
    /// Pressure
    pub pressure: f32,
    /// Damping
    pub damping: f32,
    /// Iterations
    pub iterations: u32,
}

impl SoftBody {
    /// Create a sphere soft body
    #[must_use]
    pub fn sphere(center: Vec3, radius: f32, segments: u32, mass: f32) -> Self {
        let mut particles = Vec::new();
        let mut constraints = Vec::new();

        // Create sphere particles
        for i in 0..segments {
            let theta = (i as f32 / segments as f32) * std::f32::consts::PI;
            
            for j in 0..(segments * 2) {
                let phi = (j as f32 / (segments * 2) as f32) * std::f32::consts::TAU;
                
                let pos = center + Vec3::new(
                    radius * theta.sin() * phi.cos(),
                    radius * theta.cos(),
                    radius * theta.sin() * phi.sin(),
                );
                
                particles.push(ClothParticle::new(pos, mass));
            }
        }

        // Create constraints between neighbors
        let width = segments * 2;
        for i in 0..particles.len() {
            let row = i / width as usize;
            let col = i % width as usize;
            
            // Connect to right neighbor
            if col < width as usize - 1 {
                let next = i + 1;
                let len = (particles[i].position - particles[next].position).length();
                constraints.push(DistanceConstraint {
                    p1: i,
                    p2: next,
                    rest_length: len,
                    stiffness: 1.0,
                });
            }
            
            // Connect to bottom neighbor
            if row < segments as usize - 1 {
                let below = i + width as usize;
                if below < particles.len() {
                    let len = (particles[i].position - particles[below].position).length();
                    constraints.push(DistanceConstraint {
                        p1: i,
                        p2: below,
                        rest_length: len,
                        stiffness: 1.0,
                    });
                }
            }
        }

        let volume = (4.0 / 3.0) * std::f32::consts::PI * radius.powi(3);

        Self {
            particles,
            constraints,
            target_volume: volume,
            pressure: 1.0,
            damping: 0.99,
            iterations: 10,
        }
    }

    /// Update simulation
    pub fn update(&mut self, delta_time: f32, gravity: Vec3) {
        // Apply gravity
        for particle in &mut self.particles {
            particle.apply_force(gravity * particle.mass);
        }

        // Verlet integration
        for particle in &mut self.particles {
            particle.update(delta_time, self.damping);
        }

        // Solve constraints
        for _ in 0..self.iterations {
            for constraint in &self.constraints {
                let p1 = self.particles[constraint.p1].position;
                let p2 = self.particles[constraint.p2].position;
                
                let delta = p2 - p1;
                let distance = delta.length();
                
                if distance > 0.0001 {
                    let diff = (distance - constraint.rest_length) / distance;
                    let correction = delta * diff * 0.5 * constraint.stiffness;
                    
                    self.particles[constraint.p1].position += correction;
                    self.particles[constraint.p2].position -= correction;
                }
            }
        }
    }
}
