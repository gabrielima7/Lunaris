//! Particle System
//!
//! GPU-accelerated particle effects for games.

use lunaris_core::{
    id::{Id, TypedId},
    math::{Color, Vec2, Vec3},
};
use std::collections::HashMap;

/// Particle emitter identifier
pub type EmitterId = TypedId<ParticleEmitter>;

/// A single particle
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Particle {
    /// Position
    pub position: Vec3,
    /// Velocity
    pub velocity: Vec3,
    /// Age (0 to lifetime)
    pub age: f32,
    /// Lifetime
    pub lifetime: f32,
    /// Size
    pub size: f32,
    /// Rotation
    pub rotation: f32,
    /// Color
    pub color: Color,
    /// Is alive
    pub alive: bool,
}

unsafe impl bytemuck::Pod for Particle {}
unsafe impl bytemuck::Zeroable for Particle {}

/// Particle emission shape
#[derive(Debug, Clone, Copy)]
pub enum EmissionShape {
    /// Point emission
    Point,
    /// Sphere emission
    Sphere { radius: f32 },
    /// Box emission
    Box { half_extents: Vec3 },
    /// Cone emission
    Cone { angle: f32, radius: f32 },
    /// Circle emission (2D)
    Circle { radius: f32 },
    /// Edge emission (line)
    Edge { start: Vec3, end: Vec3 },
}

impl Default for EmissionShape {
    fn default() -> Self {
        Self::Point
    }
}

/// Particle velocity mode
#[derive(Debug, Clone, Copy, Default)]
pub enum VelocityMode {
    /// Random in unit sphere
    #[default]
    Random,
    /// Away from emitter center
    Outward,
    /// Toward a target
    Toward(Vec3),
    /// Inherit emitter velocity
    Inherit,
}

/// Value over lifetime (constant or curve)
#[derive(Debug, Clone)]
pub enum ValueOverLifetime<T> {
    /// Constant value
    Constant(T),
    /// Linear from start to end
    Linear { start: T, end: T },
    /// Curve (keyframes)
    Curve(Vec<(f32, T)>),
    /// Random between two values
    Random { min: T, max: T },
}

impl<T: Clone> ValueOverLifetime<T> {
    /// Sample at normalized time (0-1)
    pub fn sample(&self, t: f32) -> T
    where
        T: std::ops::Add<Output = T> + std::ops::Mul<f32, Output = T> + Copy,
    {
        match self {
            Self::Constant(v) => *v,
            Self::Linear { start, end } => *start * (1.0 - t) + *end * t,
            Self::Curve(keyframes) => {
                if keyframes.is_empty() {
                    panic!("Empty curve");
                }
                if keyframes.len() == 1 {
                    return keyframes[0].1;
                }
                // Find keyframes
                for i in 1..keyframes.len() {
                    if keyframes[i].0 >= t {
                        let prev = &keyframes[i - 1];
                        let next = &keyframes[i];
                        let local_t = (t - prev.0) / (next.0 - prev.0);
                        return prev.1 * (1.0 - local_t) + next.1 * local_t;
                    }
                }
                keyframes.last().unwrap().1
            }
            Self::Random { min, max } => {
                // Would use random, for now just return min
                *min
            }
        }
    }
}

/// Particle emitter configuration
#[derive(Debug, Clone)]
pub struct EmitterConfig {
    /// Max particles
    pub max_particles: u32,
    /// Emission rate (particles per second)
    pub emission_rate: f32,
    /// Burst emission
    pub bursts: Vec<ParticleBurst>,
    /// Emission shape
    pub shape: EmissionShape,
    /// Particle lifetime
    pub lifetime: ValueOverLifetime<f32>,
    /// Start speed
    pub start_speed: ValueOverLifetime<f32>,
    /// Start size
    pub start_size: ValueOverLifetime<f32>,
    /// Start rotation
    pub start_rotation: ValueOverLifetime<f32>,
    /// Start color
    pub start_color: Color,
    /// Color over lifetime
    pub color_over_lifetime: Option<ValueOverLifetime<Color>>,
    /// Size over lifetime
    pub size_over_lifetime: Option<ValueOverLifetime<f32>>,
    /// Speed over lifetime
    pub speed_over_lifetime: Option<ValueOverLifetime<f32>>,
    /// Gravity modifier
    pub gravity: f32,
    /// Drag
    pub drag: f32,
    /// Looping
    pub looping: bool,
    /// Duration
    pub duration: f32,
    /// Play on start
    pub play_on_start: bool,
}

impl Default for EmitterConfig {
    fn default() -> Self {
        Self {
            max_particles: 1000,
            emission_rate: 50.0,
            bursts: Vec::new(),
            shape: EmissionShape::default(),
            lifetime: ValueOverLifetime::Constant(2.0),
            start_speed: ValueOverLifetime::Constant(5.0),
            start_size: ValueOverLifetime::Constant(0.1),
            start_rotation: ValueOverLifetime::Constant(0.0),
            start_color: Color::WHITE,
            color_over_lifetime: None,
            size_over_lifetime: None,
            speed_over_lifetime: None,
            gravity: 0.0,
            drag: 0.0,
            looping: true,
            duration: 5.0,
            play_on_start: true,
        }
    }
}

/// Particle burst configuration
#[derive(Debug, Clone)]
pub struct ParticleBurst {
    /// Time to emit burst
    pub time: f32,
    /// Number of particles
    pub count: u32,
    /// Cycles (0 = once)
    pub cycles: u32,
    /// Interval between cycles
    pub interval: f32,
}

/// Particle emitter
pub struct ParticleEmitter {
    /// Emitter ID
    pub id: EmitterId,
    /// Configuration
    pub config: EmitterConfig,
    /// Position
    pub position: Vec3,
    /// Rotation
    pub rotation: Vec3,
    /// Particles
    particles: Vec<Particle>,
    /// Active particle count
    active_count: usize,
    /// Emission accumulator
    emission_accumulator: f32,
    /// Current time
    time: f32,
    /// Is playing
    playing: bool,
}

impl ParticleEmitter {
    /// Create a new emitter
    #[must_use]
    pub fn new(config: EmitterConfig) -> Self {
        let max = config.max_particles as usize;
        Self {
            id: EmitterId::new(),
            config,
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            particles: vec![Particle::default(); max],
            active_count: 0,
            emission_accumulator: 0.0,
            time: 0.0,
            playing: true,
        }
    }

    /// Play the emitter
    pub fn play(&mut self) {
        self.playing = true;
        self.time = 0.0;
    }

    /// Stop the emitter
    pub fn stop(&mut self) {
        self.playing = false;
    }

    /// Pause the emitter
    pub fn pause(&mut self) {
        self.playing = false;
    }

    /// Is playing
    #[must_use]
    pub fn is_playing(&self) -> bool {
        self.playing
    }

    /// Active particle count
    #[must_use]
    pub fn active_count(&self) -> usize {
        self.active_count
    }

    /// Update the emitter
    pub fn update(&mut self, delta_time: f32) {
        if !self.playing {
            return;
        }

        self.time += delta_time;

        // Check duration
        if !self.config.looping && self.time >= self.config.duration {
            self.playing = false;
        }

        // Emit new particles
        self.emission_accumulator += self.config.emission_rate * delta_time;
        while self.emission_accumulator >= 1.0 && self.active_count < self.particles.len() {
            self.emit_particle();
            self.emission_accumulator -= 1.0;
        }

        // Update particles
        for particle in &mut self.particles {
            if !particle.alive {
                continue;
            }

            particle.age += delta_time;
            if particle.age >= particle.lifetime {
                particle.alive = false;
                continue;
            }

            // Apply gravity
            particle.velocity.y -= self.config.gravity * delta_time;

            // Apply drag
            particle.velocity = particle.velocity * (1.0 - self.config.drag * delta_time);

            // Update position
            particle.position = particle.position + particle.velocity * delta_time;

            // Update color/size over lifetime
            let t = particle.age / particle.lifetime;
            
            if let Some(ref color_curve) = self.config.color_over_lifetime {
                particle.color = color_curve.sample(t);
            }
            
            if let Some(ref size_curve) = self.config.size_over_lifetime {
                particle.size = size_curve.sample(t);
            }
        }

        // Count active
        self.active_count = self.particles.iter().filter(|p| p.alive).count();
    }

    fn emit_particle(&mut self) {
        // Find dead particle
        let particle = match self.particles.iter_mut().find(|p| !p.alive) {
            Some(p) => p,
            None => return,
        };

        // Position based on shape
        let offset = match self.config.shape {
            EmissionShape::Point => Vec3::ZERO,
            EmissionShape::Sphere { radius } => {
                // Would use random, simplified
                Vec3::new(0.0, radius * 0.5, 0.0)
            }
            EmissionShape::Box { half_extents } => {
                Vec3::new(half_extents.x * 0.5, half_extents.y * 0.5, half_extents.z * 0.5)
            }
            EmissionShape::Cone { angle, radius } => {
                Vec3::new(0.0, radius, 0.0)
            }
            EmissionShape::Circle { radius } => {
                Vec3::new(radius * 0.5, 0.0, 0.0)
            }
            EmissionShape::Edge { start, end } => {
                (start + end) * 0.5
            }
        };

        let start_t = 0.0; // Would be random
        let lifetime = match &self.config.lifetime {
            ValueOverLifetime::Constant(v) => *v,
            _ => 2.0,
        };
        let speed = match &self.config.start_speed {
            ValueOverLifetime::Constant(v) => *v,
            _ => 5.0,
        };
        let size = match &self.config.start_size {
            ValueOverLifetime::Constant(v) => *v,
            _ => 0.1,
        };

        *particle = Particle {
            position: self.position + offset,
            velocity: Vec3::new(0.0, speed, 0.0),
            age: 0.0,
            lifetime,
            size,
            rotation: 0.0,
            color: self.config.start_color,
            alive: true,
        };
    }

    /// Get particles for rendering
    #[must_use]
    pub fn particles(&self) -> &[Particle] {
        &self.particles
    }

    /// Get alive particles
    pub fn alive_particles(&self) -> impl Iterator<Item = &Particle> {
        self.particles.iter().filter(|p| p.alive)
    }
}

/// Particle system manager
pub struct ParticleSystem {
    /// Emitters
    emitters: HashMap<EmitterId, ParticleEmitter>,
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl ParticleSystem {
    /// Create a new particle system
    #[must_use]
    pub fn new() -> Self {
        Self {
            emitters: HashMap::new(),
        }
    }

    /// Create an emitter
    pub fn create_emitter(&mut self, config: EmitterConfig) -> EmitterId {
        let emitter = ParticleEmitter::new(config);
        let id = emitter.id;
        self.emitters.insert(id, emitter);
        id
    }

    /// Get an emitter
    #[must_use]
    pub fn get(&self, id: EmitterId) -> Option<&ParticleEmitter> {
        self.emitters.get(&id)
    }

    /// Get an emitter mutably
    pub fn get_mut(&mut self, id: EmitterId) -> Option<&mut ParticleEmitter> {
        self.emitters.get_mut(&id)
    }

    /// Remove an emitter
    pub fn remove(&mut self, id: EmitterId) {
        self.emitters.remove(&id);
    }

    /// Update all emitters
    pub fn update(&mut self, delta_time: f32) {
        for emitter in self.emitters.values_mut() {
            emitter.update(delta_time);
        }
    }

    /// Get total active particle count
    #[must_use]
    pub fn total_particles(&self) -> usize {
        self.emitters.values().map(|e| e.active_count()).sum()
    }
}

/// Preset emitter configurations
pub mod presets {
    use super::*;

    /// Fire effect
    #[must_use]
    pub fn fire() -> EmitterConfig {
        EmitterConfig {
            max_particles: 500,
            emission_rate: 100.0,
            shape: EmissionShape::Cone { angle: 15.0, radius: 0.2 },
            lifetime: ValueOverLifetime::Linear { start: 1.0, end: 2.0 },
            start_speed: ValueOverLifetime::Linear { start: 3.0, end: 5.0 },
            start_size: ValueOverLifetime::Constant(0.3),
            start_color: Color::new(1.0, 0.5, 0.0, 1.0),
            gravity: -2.0, // Fire rises
            drag: 0.5,
            ..Default::default()
        }
    }

    /// Smoke effect
    #[must_use]
    pub fn smoke() -> EmitterConfig {
        EmitterConfig {
            max_particles: 200,
            emission_rate: 30.0,
            shape: EmissionShape::Sphere { radius: 0.3 },
            lifetime: ValueOverLifetime::Constant(4.0),
            start_speed: ValueOverLifetime::Constant(1.0),
            start_size: ValueOverLifetime::Constant(0.5),
            start_color: Color::new(0.5, 0.5, 0.5, 0.5),
            gravity: -0.5,
            drag: 0.8,
            ..Default::default()
        }
    }

    /// Sparks effect
    #[must_use]
    pub fn sparks() -> EmitterConfig {
        EmitterConfig {
            max_particles: 100,
            emission_rate: 50.0,
            shape: EmissionShape::Point,
            lifetime: ValueOverLifetime::Constant(0.5),
            start_speed: ValueOverLifetime::Linear { start: 5.0, end: 10.0 },
            start_size: ValueOverLifetime::Constant(0.05),
            start_color: Color::new(1.0, 0.8, 0.2, 1.0),
            gravity: 9.8,
            ..Default::default()
        }
    }

    /// Rain effect
    #[must_use]
    pub fn rain() -> EmitterConfig {
        EmitterConfig {
            max_particles: 2000,
            emission_rate: 500.0,
            shape: EmissionShape::Box { half_extents: Vec3::new(10.0, 0.0, 10.0) },
            lifetime: ValueOverLifetime::Constant(2.0),
            start_speed: ValueOverLifetime::Constant(20.0),
            start_size: ValueOverLifetime::Constant(0.02),
            start_color: Color::new(0.7, 0.8, 1.0, 0.5),
            gravity: 0.0, // Already falling fast
            ..Default::default()
        }
    }

    /// Explosion burst
    #[must_use]
    pub fn explosion() -> EmitterConfig {
        EmitterConfig {
            max_particles: 200,
            emission_rate: 0.0,
            bursts: vec![ParticleBurst {
                time: 0.0,
                count: 200,
                cycles: 0,
                interval: 0.0,
            }],
            shape: EmissionShape::Sphere { radius: 0.1 },
            lifetime: ValueOverLifetime::Constant(1.0),
            start_speed: ValueOverLifetime::Linear { start: 10.0, end: 20.0 },
            start_size: ValueOverLifetime::Constant(0.2),
            start_color: Color::new(1.0, 0.6, 0.1, 1.0),
            gravity: 5.0,
            drag: 2.0,
            looping: false,
            duration: 2.0,
            ..Default::default()
        }
    }
}
