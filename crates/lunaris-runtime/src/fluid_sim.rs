//! Fluid Simulation (SPH/FLIP)
//!
//! Smoothed Particle Hydrodynamics and real-time liquids.

use glam::Vec3;

/// Fluid simulation system
pub struct FluidSimulation {
    pub particles: Vec<FluidParticle>,
    pub settings: FluidSettings,
    pub grid: FluidGrid,
    pub boundaries: Vec<Boundary>,
}

/// Fluid particle
pub struct FluidParticle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub force: Vec3,
    pub density: f32,
    pub pressure: f32,
    pub mass: f32,
    pub type_id: u32,
}

/// Fluid settings
pub struct FluidSettings {
    pub rest_density: f32,
    pub gas_constant: f32,
    pub viscosity: f32,
    pub surface_tension: f32,
    pub gravity: Vec3,
    pub smoothing_radius: f32,
    pub time_step: f32,
    pub max_particles: usize,
    pub solver: FluidSolver,
}

/// Fluid solver type
pub enum FluidSolver { SPH, PCISPH, FLIP, APIC }

impl Default for FluidSettings {
    fn default() -> Self {
        Self {
            rest_density: 1000.0, gas_constant: 50.0, viscosity: 0.1, surface_tension: 0.0728,
            gravity: Vec3::new(0.0, -9.81, 0.0), smoothing_radius: 0.04, time_step: 1.0 / 120.0,
            max_particles: 100000, solver: FluidSolver::SPH,
        }
    }
}

/// Fluid grid (for neighbor search)
pub struct FluidGrid {
    pub cell_size: f32,
    pub cells: Vec<Vec<usize>>,
    pub grid_size: (usize, usize, usize),
    pub origin: Vec3,
}

impl FluidGrid {
    pub fn new(cell_size: f32, size: (usize, usize, usize), origin: Vec3) -> Self {
        Self { cell_size, cells: vec![Vec::new(); size.0 * size.1 * size.2], grid_size: size, origin }
    }

    pub fn clear(&mut self) { for cell in &mut self.cells { cell.clear(); } }

    pub fn insert(&mut self, idx: usize, position: Vec3) {
        let cell_idx = self.get_cell_index(position);
        if let Some(cell) = self.cells.get_mut(cell_idx) { cell.push(idx); }
    }

    pub fn get_cell_index(&self, position: Vec3) -> usize {
        let local = position - self.origin;
        let x = (local.x / self.cell_size) as usize;
        let y = (local.y / self.cell_size) as usize;
        let z = (local.z / self.cell_size) as usize;
        let x = x.min(self.grid_size.0 - 1);
        let y = y.min(self.grid_size.1 - 1);
        let z = z.min(self.grid_size.2 - 1);
        z * self.grid_size.0 * self.grid_size.1 + y * self.grid_size.0 + x
    }

    pub fn get_neighbors(&self, position: Vec3) -> Vec<usize> {
        let mut neighbors = Vec::new();
        let local = position - self.origin;
        let cx = (local.x / self.cell_size) as i32;
        let cy = (local.y / self.cell_size) as i32;
        let cz = (local.z / self.cell_size) as i32;

        for dz in -1..=1 {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let x = (cx + dx).clamp(0, self.grid_size.0 as i32 - 1) as usize;
                    let y = (cy + dy).clamp(0, self.grid_size.1 as i32 - 1) as usize;
                    let z = (cz + dz).clamp(0, self.grid_size.2 as i32 - 1) as usize;
                    let idx = z * self.grid_size.0 * self.grid_size.1 + y * self.grid_size.0 + x;
                    if let Some(cell) = self.cells.get(idx) {
                        neighbors.extend(cell.iter().copied());
                    }
                }
            }
        }
        neighbors
    }
}

/// Boundary
pub struct Boundary {
    pub boundary_type: BoundaryType,
    pub position: Vec3,
    pub normal: Vec3,
    pub friction: f32,
    pub restitution: f32,
}

/// Boundary type
pub enum BoundaryType { Plane, Sphere { radius: f32 }, Box { half_extents: Vec3 } }

impl FluidSimulation {
    pub fn new(settings: FluidSettings) -> Self {
        let cell_size = settings.smoothing_radius;
        let grid = FluidGrid::new(cell_size, (64, 64, 64), Vec3::splat(-5.0));
        Self { particles: Vec::new(), settings, grid, boundaries: Vec::new() }
    }

    pub fn add_particle(&mut self, position: Vec3, velocity: Vec3) {
        if self.particles.len() >= self.settings.max_particles { return; }
        self.particles.push(FluidParticle {
            position, velocity, force: Vec3::ZERO, density: 0.0, pressure: 0.0,
            mass: 1.0, type_id: 0,
        });
    }

    pub fn add_block(&mut self, min: Vec3, max: Vec3, spacing: f32) {
        let mut x = min.x;
        while x < max.x {
            let mut y = min.y;
            while y < max.y {
                let mut z = min.z;
                while z < max.z {
                    self.add_particle(Vec3::new(x, y, z), Vec3::ZERO);
                    z += spacing;
                }
                y += spacing;
            }
            x += spacing;
        }
    }

    pub fn step(&mut self) {
        self.build_grid();
        self.compute_density_pressure();
        self.compute_forces();
        self.integrate();
        self.handle_boundaries();
    }

    fn build_grid(&mut self) {
        self.grid.clear();
        for (i, p) in self.particles.iter().enumerate() {
            self.grid.insert(i, p.position);
        }
    }

    fn compute_density_pressure(&mut self) {
        let h = self.settings.smoothing_radius;
        let h2 = h * h;
        let poly6 = 315.0 / (64.0 * std::f32::consts::PI * h.powi(9));

        for i in 0..self.particles.len() {
            let pos = self.particles[i].position;
            let neighbors = self.grid.get_neighbors(pos);
            
            let mut density = 0.0;
            for &j in &neighbors {
                let r = pos - self.particles[j].position;
                let r2 = r.length_squared();
                if r2 < h2 {
                    density += self.particles[j].mass * poly6 * (h2 - r2).powi(3);
                }
            }

            self.particles[i].density = density.max(self.settings.rest_density);
            self.particles[i].pressure = self.settings.gas_constant * (self.particles[i].density - self.settings.rest_density);
        }
    }

    fn compute_forces(&mut self) {
        let h = self.settings.smoothing_radius;
        let spiky = -45.0 / (std::f32::consts::PI * h.powi(6));
        let visc = 45.0 / (std::f32::consts::PI * h.powi(6));

        for i in 0..self.particles.len() {
            let pos = self.particles[i].position;
            let neighbors = self.grid.get_neighbors(pos);
            
            let mut pressure_force = Vec3::ZERO;
            let mut viscosity_force = Vec3::ZERO;

            for &j in &neighbors {
                if i == j { continue; }
                
                let r = pos - self.particles[j].position;
                let r_len = r.length();
                if r_len < h && r_len > 0.0001 {
                    let r_norm = r / r_len;
                    
                    // Pressure
                    let pressure_avg = (self.particles[i].pressure + self.particles[j].pressure) / 2.0;
                    pressure_force -= r_norm * self.particles[j].mass * pressure_avg / self.particles[j].density * spiky * (h - r_len).powi(2);
                    
                    // Viscosity
                    let vel_diff = self.particles[j].velocity - self.particles[i].velocity;
                    viscosity_force += vel_diff * self.particles[j].mass / self.particles[j].density * visc * (h - r_len);
                }
            }

            self.particles[i].force = pressure_force + viscosity_force * self.settings.viscosity + self.settings.gravity * self.particles[i].density;
        }
    }

    fn integrate(&mut self) {
        let dt = self.settings.time_step;
        for p in &mut self.particles {
            p.velocity += p.force / p.density * dt;
            p.position += p.velocity * dt;
        }
    }

    fn handle_boundaries(&mut self) {
        for p in &mut self.particles {
            for boundary in &self.boundaries {
                match boundary.boundary_type {
                    BoundaryType::Plane => {
                        let dist = (p.position - boundary.position).dot(boundary.normal);
                        if dist < 0.0 {
                            p.position -= boundary.normal * dist;
                            let vn = p.velocity.dot(boundary.normal);
                            if vn < 0.0 {
                                p.velocity -= boundary.normal * vn * (1.0 + boundary.restitution);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn particle_count(&self) -> usize { self.particles.len() }
}
