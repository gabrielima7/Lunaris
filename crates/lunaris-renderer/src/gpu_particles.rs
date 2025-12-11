//! GPU Particles
//!
//! Millions of particles with collision and vector fields.

use glam::{Vec2, Vec3, Vec4, Mat4};

/// GPU particle system
pub struct GPUParticleSystem {
    pub emitters: Vec<GPUEmitter>,
    pub settings: GPUParticleSettings,
    pub vector_fields: Vec<VectorField>,
    pub collision: ParticleCollision,
}

/// GPU emitter
pub struct GPUEmitter {
    pub id: u64,
    pub name: String,
    pub position: Vec3,
    pub rotation: Vec3,
    pub max_particles: u32,
    pub spawn_rate: f32,
    pub spawn_burst: Option<SpawnBurst>,
    pub lifetime: ParticleRange,
    pub velocity: VelocityModule,
    pub size: SizeModule,
    pub color: ColorModule,
    pub gravity: f32,
    pub drag: f32,
    pub active: bool,
    // Runtime
    pub particle_count: u32,
    pub time: f32,
}

/// Spawn burst
pub struct SpawnBurst {
    pub count: u32,
    pub time: f32,
    pub cycles: u32,
    pub interval: f32,
}

/// Particle range
pub struct ParticleRange {
    pub min: f32,
    pub max: f32,
}

impl ParticleRange {
    pub fn constant(v: f32) -> Self { Self { min: v, max: v } }
    pub fn range(min: f32, max: f32) -> Self { Self { min, max } }
    pub fn sample(&self, t: f32) -> f32 { self.min + (self.max - self.min) * t }
}

/// Velocity module
pub struct VelocityModule {
    pub initial: VelocityType,
    pub over_lifetime: Option<Vec3Curve>,
    pub orbital: Option<OrbitalVelocity>,
    pub inherit_velocity: f32,
}

/// Velocity type
pub enum VelocityType {
    Directional { direction: Vec3, speed: ParticleRange },
    Cone { direction: Vec3, angle: f32, speed: ParticleRange },
    Sphere { speed: ParticleRange },
    Hemisphere { direction: Vec3, speed: ParticleRange },
}

/// Orbital velocity
pub struct OrbitalVelocity {
    pub center: Vec3,
    pub speed: f32,
    pub radial: f32,
}

/// Vec3 curve
pub struct Vec3Curve {
    pub keys: Vec<(f32, Vec3)>,
}

impl Vec3Curve {
    pub fn evaluate(&self, t: f32) -> Vec3 {
        if self.keys.is_empty() { return Vec3::ZERO; }
        if t <= self.keys[0].0 { return self.keys[0].1; }
        if t >= self.keys.last().unwrap().0 { return self.keys.last().unwrap().1; }
        
        for i in 0..self.keys.len() - 1 {
            if t >= self.keys[i].0 && t < self.keys[i + 1].0 {
                let local_t = (t - self.keys[i].0) / (self.keys[i + 1].0 - self.keys[i].0);
                return self.keys[i].1.lerp(self.keys[i + 1].1, local_t);
            }
        }
        Vec3::ZERO
    }
}

/// Size module
pub struct SizeModule {
    pub initial: ParticleRange,
    pub over_lifetime: Option<FloatCurve>,
    pub by_speed: Option<(f32, f32, ParticleRange)>,
}

/// Float curve
pub struct FloatCurve {
    pub keys: Vec<(f32, f32)>,
}

impl FloatCurve {
    pub fn evaluate(&self, t: f32) -> f32 {
        if self.keys.is_empty() { return 1.0; }
        if t <= self.keys[0].0 { return self.keys[0].1; }
        if t >= self.keys.last().unwrap().0 { return self.keys.last().unwrap().1; }
        
        for i in 0..self.keys.len() - 1 {
            if t >= self.keys[i].0 && t < self.keys[i + 1].0 {
                let local_t = (t - self.keys[i].0) / (self.keys[i + 1].0 - self.keys[i].0);
                return self.keys[i].1 + (self.keys[i + 1].1 - self.keys[i].1) * local_t;
            }
        }
        1.0
    }
}

/// Color module
pub struct ColorModule {
    pub initial: ColorType,
    pub over_lifetime: Option<GradientCurve>,
}

/// Color type
pub enum ColorType { Constant(Vec4), Random { min: Vec4, max: Vec4 }, Gradient(GradientCurve) }

/// Gradient curve
pub struct GradientCurve {
    pub keys: Vec<(f32, Vec4)>,
}

impl GradientCurve {
    pub fn evaluate(&self, t: f32) -> Vec4 {
        if self.keys.is_empty() { return Vec4::ONE; }
        if t <= self.keys[0].0 { return self.keys[0].1; }
        if t >= self.keys.last().unwrap().0 { return self.keys.last().unwrap().1; }
        
        for i in 0..self.keys.len() - 1 {
            if t >= self.keys[i].0 && t < self.keys[i + 1].0 {
                let local_t = (t - self.keys[i].0) / (self.keys[i + 1].0 - self.keys[i].0);
                return self.keys[i].1.lerp(self.keys[i + 1].1, local_t);
            }
        }
        Vec4::ONE
    }
}

/// GPU particle settings
pub struct GPUParticleSettings {
    pub simulation_space: SimulationSpace,
    pub sort_mode: SortMode,
    pub render_mode: ParticleRenderMode,
    pub culling: bool,
    pub bounds_padding: f32,
}

/// Simulation space
pub enum SimulationSpace { Local, World }

/// Sort mode
pub enum SortMode { None, ByDistance, OldestFirst, YoungestFirst }

/// Particle render mode
pub enum ParticleRenderMode { Billboard, StretchedBillboard { speed_stretch: f32 }, Mesh { mesh_id: u64 }, Trail { width: f32, segments: u32 } }

impl Default for GPUParticleSettings {
    fn default() -> Self {
        Self { simulation_space: SimulationSpace::World, sort_mode: SortMode::ByDistance, render_mode: ParticleRenderMode::Billboard, culling: true, bounds_padding: 1.0 }
    }
}

/// Vector field
pub struct VectorField {
    pub id: u64,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub resolution: (u32, u32, u32),
    pub data: Vec<Vec3>,
    pub intensity: f32,
    pub tightness: f32,
}

impl VectorField {
    pub fn sample(&self, world_pos: Vec3) -> Vec3 {
        let local = (world_pos - self.position) / self.scale;
        let uvw = (local + Vec3::splat(0.5)).clamp(Vec3::ZERO, Vec3::ONE);
        
        let x = (uvw.x * (self.resolution.0 - 1) as f32) as usize;
        let y = (uvw.y * (self.resolution.1 - 1) as f32) as usize;
        let z = (uvw.z * (self.resolution.2 - 1) as f32) as usize;
        
        let idx = z * (self.resolution.0 * self.resolution.1) as usize + y * self.resolution.0 as usize + x;
        
        if idx < self.data.len() { self.data[idx] * self.intensity }
        else { Vec3::ZERO }
    }
}

/// Particle collision
pub struct ParticleCollision {
    pub enabled: bool,
    pub mode: CollisionMode,
    pub bounce: f32,
    pub lifetime_loss: f32,
    pub radius_scale: f32,
    pub depth_buffer: bool,
}

/// Collision mode
pub enum CollisionMode { World, Planes(Vec<CollisionPlane>), DepthBuffer }

/// Collision plane
pub struct CollisionPlane {
    pub normal: Vec3,
    pub distance: f32,
}

impl Default for ParticleCollision {
    fn default() -> Self {
        Self { enabled: true, mode: CollisionMode::DepthBuffer, bounce: 0.5, lifetime_loss: 0.0, radius_scale: 1.0, depth_buffer: true }
    }
}

impl GPUParticleSystem {
    pub fn new() -> Self {
        Self { emitters: Vec::new(), settings: GPUParticleSettings::default(), vector_fields: Vec::new(), collision: ParticleCollision::default() }
    }

    pub fn create_emitter(&mut self, name: &str, max_particles: u32) -> usize {
        let id = self.emitters.len();
        self.emitters.push(GPUEmitter {
            id: id as u64, name: name.into(), position: Vec3::ZERO, rotation: Vec3::ZERO,
            max_particles, spawn_rate: 100.0, spawn_burst: None, lifetime: ParticleRange::range(1.0, 3.0),
            velocity: VelocityModule { initial: VelocityType::Sphere { speed: ParticleRange::range(1.0, 5.0) }, over_lifetime: None, orbital: None, inherit_velocity: 0.0 },
            size: SizeModule { initial: ParticleRange::range(0.1, 0.3), over_lifetime: None, by_speed: None },
            color: ColorModule { initial: ColorType::Constant(Vec4::ONE), over_lifetime: None },
            gravity: 1.0, drag: 0.0, active: true, particle_count: 0, time: 0.0,
        });
        id
    }

    pub fn update(&mut self, dt: f32) {
        for emitter in &mut self.emitters {
            if !emitter.active { continue; }
            emitter.time += dt;
            
            // Spawn particles
            let spawn_count = (emitter.spawn_rate * dt) as u32;
            emitter.particle_count = (emitter.particle_count + spawn_count).min(emitter.max_particles);
        }
    }

    pub fn total_particles(&self) -> u32 {
        self.emitters.iter().map(|e| e.particle_count).sum()
    }
}
