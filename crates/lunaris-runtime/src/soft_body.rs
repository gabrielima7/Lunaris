//! Soft Body Physics
//!
//! Finite Element Method, shape matching, tetrahedral meshes.

use glam::{Vec3, Mat3};

/// Soft body system
pub struct SoftBodySystem {
    pub bodies: Vec<SoftBody>,
    pub settings: SoftBodySettings,
}

/// Soft body
pub struct SoftBody {
    pub id: u64,
    pub name: String,
    pub particles: Vec<SoftParticle>,
    pub tetrahedra: Vec<Tetrahedron>,
    pub constraints: Vec<SoftConstraint>,
    pub material: SoftMaterial,
    pub method: SimulationMethod,
}

/// Soft particle
pub struct SoftParticle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub rest_position: Vec3,
    pub mass: f32,
    pub inv_mass: f32,
    pub pinned: bool,
}

/// Tetrahedron
pub struct Tetrahedron {
    pub indices: [usize; 4],
    pub rest_matrix: Mat3,
    pub inv_rest_matrix: Mat3,
    pub rest_volume: f32,
}

impl Tetrahedron {
    pub fn new(particles: &[SoftParticle], i0: usize, i1: usize, i2: usize, i3: usize) -> Self {
        let p0 = particles[i0].rest_position;
        let p1 = particles[i1].rest_position;
        let p2 = particles[i2].rest_position;
        let p3 = particles[i3].rest_position;
        
        let dm = Mat3::from_cols(p1 - p0, p2 - p0, p3 - p0);
        let rest_volume = dm.determinant().abs() / 6.0;
        let inv_dm = dm.inverse();
        
        Self { indices: [i0, i1, i2, i3], rest_matrix: dm, inv_rest_matrix: inv_dm, rest_volume }
    }

    pub fn compute_deformation(&self, particles: &[SoftParticle]) -> Mat3 {
        let p0 = particles[self.indices[0]].position;
        let p1 = particles[self.indices[1]].position;
        let p2 = particles[self.indices[2]].position;
        let p3 = particles[self.indices[3]].position;
        
        let ds = Mat3::from_cols(p1 - p0, p2 - p0, p3 - p0);
        ds * self.inv_rest_matrix
    }
}

/// Soft constraint
pub enum SoftConstraint {
    Distance { i: usize, j: usize, rest_length: f32, stiffness: f32 },
    Volume { tet_index: usize, stiffness: f32 },
    ShapeMatch { indices: Vec<usize>, stiffness: f32, rest_positions: Vec<Vec3> },
}

/// Soft material
pub struct SoftMaterial {
    pub youngs_modulus: f32,
    pub poisson_ratio: f32,
    pub damping: f32,
    pub friction: f32,
}

impl Default for SoftMaterial {
    fn default() -> Self {
        Self { youngs_modulus: 10000.0, poisson_ratio: 0.3, damping: 0.99, friction: 0.5 }
    }
}

/// Simulation method
pub enum SimulationMethod { MassSpring, FEM, ShapeMatching, XPBD }

/// Soft body settings
pub struct SoftBodySettings {
    pub gravity: Vec3,
    pub substeps: u32,
    pub iterations: u32,
    pub time_step: f32,
}

impl Default for SoftBodySettings {
    fn default() -> Self {
        Self { gravity: Vec3::new(0.0, -9.81, 0.0), substeps: 4, iterations: 10, time_step: 1.0 / 60.0 }
    }
}

impl SoftBodySystem {
    pub fn new() -> Self {
        Self { bodies: Vec::new(), settings: SoftBodySettings::default() }
    }

    pub fn create_body(&mut self, name: &str, method: SimulationMethod) -> usize {
        let id = self.bodies.len();
        self.bodies.push(SoftBody { id: id as u64, name: name.into(), particles: Vec::new(), tetrahedra: Vec::new(), constraints: Vec::new(), material: SoftMaterial::default(), method });
        id
    }

    pub fn create_cube(&mut self, center: Vec3, size: f32, subdivisions: u32) -> usize {
        let body_id = self.create_body("cube", SimulationMethod::ShapeMatching);
        let body = &mut self.bodies[body_id];
        
        let step = size / subdivisions as f32;
        let half = size / 2.0;
        
        // Create particles
        for z in 0..=subdivisions {
            for y in 0..=subdivisions {
                for x in 0..=subdivisions {
                    let pos = center + Vec3::new(x as f32 * step - half, y as f32 * step - half, z as f32 * step - half);
                    body.particles.push(SoftParticle { position: pos, velocity: Vec3::ZERO, rest_position: pos, mass: 1.0, inv_mass: 1.0, pinned: false });
                }
            }
        }

        // Create distance constraints
        let n = subdivisions + 1;
        for z in 0..n {
            for y in 0..n {
                for x in 0..n {
                    let i = (z * n * n + y * n + x) as usize;
                    
                    if x + 1 < n {
                        let j = (z * n * n + y * n + x + 1) as usize;
                        let rest_len = (body.particles[i].rest_position - body.particles[j].rest_position).length();
                        body.constraints.push(SoftConstraint::Distance { i, j, rest_length: rest_len, stiffness: 0.5 });
                    }
                    if y + 1 < n {
                        let j = (z * n * n + (y + 1) * n + x) as usize;
                        let rest_len = (body.particles[i].rest_position - body.particles[j].rest_position).length();
                        body.constraints.push(SoftConstraint::Distance { i, j, rest_length: rest_len, stiffness: 0.5 });
                    }
                    if z + 1 < n {
                        let j = ((z + 1) * n * n + y * n + x) as usize;
                        let rest_len = (body.particles[i].rest_position - body.particles[j].rest_position).length();
                        body.constraints.push(SoftConstraint::Distance { i, j, rest_length: rest_len, stiffness: 0.5 });
                    }
                }
            }
        }

        // Add shape matching constraint
        let all_indices: Vec<usize> = (0..body.particles.len()).collect();
        let rest_positions: Vec<Vec3> = body.particles.iter().map(|p| p.rest_position).collect();
        body.constraints.push(SoftConstraint::ShapeMatch { indices: all_indices, stiffness: 0.5, rest_positions });

        body_id
    }

    pub fn step(&mut self) {
        let dt = self.settings.time_step / self.settings.substeps as f32;
        
        for _ in 0..self.settings.substeps {
            for body in &mut self.bodies {
                // Apply gravity and predict positions
                for p in &mut body.particles {
                    if p.pinned { continue; }
                    p.velocity += self.settings.gravity * dt;
                    p.position += p.velocity * dt;
                }

                // Solve constraints
                for _ in 0..self.settings.iterations {
                    Self::solve_constraints(body);
                }

                // Update velocities
                for p in &mut body.particles {
                    if p.pinned { continue; }
                    p.velocity *= body.material.damping;
                }
            }
        }
    }

    fn solve_constraints(body: &mut SoftBody) {
        for constraint in &body.constraints {
            match constraint {
                SoftConstraint::Distance { i, j, rest_length, stiffness } => {
                    let p1 = body.particles[*i].position;
                    let p2 = body.particles[*j].position;
                    let diff = p2 - p1;
                    let dist = diff.length();
                    if dist < 0.0001 { continue; }
                    
                    let error = (dist - *rest_length) / dist;
                    let correction = diff * error * *stiffness * 0.5;
                    
                    if !body.particles[*i].pinned { body.particles[*i].position += correction; }
                    if !body.particles[*j].pinned { body.particles[*j].position -= correction; }
                }
                SoftConstraint::ShapeMatch { indices, stiffness, rest_positions } => {
                    Self::solve_shape_matching(body, indices, rest_positions, *stiffness);
                }
                _ => {}
            }
        }
    }

    fn solve_shape_matching(body: &mut SoftBody, indices: &[usize], rest: &[Vec3], stiffness: f32) {
        // Compute center of mass
        let mut com = Vec3::ZERO;
        let mut rest_com = Vec3::ZERO;
        let mut total_mass = 0.0;
        
        for (i, &idx) in indices.iter().enumerate() {
            let m = body.particles[idx].mass;
            com += body.particles[idx].position * m;
            rest_com += rest[i] * m;
            total_mass += m;
        }
        com /= total_mass;
        rest_com /= total_mass;

        // Compute optimal rotation matrix using polar decomposition (simplified)
        let mut apq = Mat3::ZERO;
        for (i, &idx) in indices.iter().enumerate() {
            let q = rest[i] - rest_com;
            let p = body.particles[idx].position - com;
            apq += Mat3::from_cols(p * q.x, p * q.y, p * q.z);
        }
        
        // Simplified: just use the polar decomposition's rotation part
        let rotation = extract_rotation(apq);

        // Apply goal positions
        for (i, &idx) in indices.iter().enumerate() {
            if body.particles[idx].pinned { continue; }
            let goal = com + rotation * (rest[i] - rest_com);
            body.particles[idx].position += (goal - body.particles[idx].position) * stiffness;
        }
    }
}

fn extract_rotation(m: Mat3) -> Mat3 {
    // Simplified polar decomposition
    let mut r = m;
    for _ in 0..5 {
        let r_inv_t = r.inverse().transpose();
        r = (r + r_inv_t) * 0.5;
    }
    r
}
