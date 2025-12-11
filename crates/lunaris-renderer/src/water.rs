//! Water System
//!
//! Ocean, rivers, and underwater rendering.

use glam::{Vec2, Vec3, Vec4};

/// Water body
pub struct WaterBody {
    pub id: u64,
    pub water_type: WaterType,
    pub surface: WaterSurface,
    pub material: WaterMaterial,
    pub physics: WaterPhysics,
    pub bounds: WaterBounds,
}

/// Water type
pub enum WaterType {
    Ocean { depth: f32, wave_direction: Vec2 },
    Lake { depth: f32 },
    River { flow_speed: f32, flow_direction: Vec2, width: f32 },
    Pool { depth: f32 },
}

/// Water surface
pub struct WaterSurface {
    pub wave_amplitude: f32,
    pub wave_frequency: f32,
    pub wave_speed: f32,
    pub wave_steepness: f32,
    pub detail_waves: bool,
    pub foam_amount: f32,
    pub caustics_intensity: f32,
}

impl Default for WaterSurface {
    fn default() -> Self {
        Self { wave_amplitude: 0.5, wave_frequency: 0.1, wave_speed: 1.0, wave_steepness: 0.5, detail_waves: true, foam_amount: 0.3, caustics_intensity: 0.5 }
    }
}

/// Water material
pub struct WaterMaterial {
    pub shallow_color: Vec4,
    pub deep_color: Vec4,
    pub absorption: Vec3,
    pub scattering: f32,
    pub refraction_strength: f32,
    pub reflection_strength: f32,
    pub fresnel_power: f32,
    pub specular_power: f32,
    pub normal_strength: f32,
}

impl Default for WaterMaterial {
    fn default() -> Self {
        Self {
            shallow_color: Vec4::new(0.1, 0.4, 0.5, 0.8),
            deep_color: Vec4::new(0.0, 0.1, 0.2, 1.0),
            absorption: Vec3::new(0.5, 0.2, 0.1),
            scattering: 0.3,
            refraction_strength: 0.5,
            reflection_strength: 0.8,
            fresnel_power: 5.0,
            specular_power: 256.0,
            normal_strength: 1.0,
        }
    }
}

/// Water physics
pub struct WaterPhysics {
    pub density: f32,
    pub viscosity: f32,
    pub buoyancy_damping: f32,
    pub splash_particles: bool,
    pub ripple_simulation: bool,
}

impl Default for WaterPhysics {
    fn default() -> Self {
        Self { density: 1000.0, viscosity: 1.0, buoyancy_damping: 0.5, splash_particles: true, ripple_simulation: true }
    }
}

/// Water bounds
pub struct WaterBounds {
    pub min: Vec3,
    pub max: Vec3,
    pub surface_height: f32,
}

impl WaterBody {
    pub fn ocean(height: f32, size: f32) -> Self {
        Self {
            id: 1,
            water_type: WaterType::Ocean { depth: 100.0, wave_direction: Vec2::new(1.0, 0.5).normalize() },
            surface: WaterSurface::default(),
            material: WaterMaterial::default(),
            physics: WaterPhysics::default(),
            bounds: WaterBounds { min: Vec3::new(-size, -100.0, -size), max: Vec3::new(size, height, size), surface_height: height },
        }
    }

    pub fn get_height(&self, pos: Vec2, time: f32) -> f32 {
        let base = self.bounds.surface_height;
        let wave = (pos.x * self.surface.wave_frequency + time * self.surface.wave_speed).sin() * self.surface.wave_amplitude;
        let wave2 = (pos.y * self.surface.wave_frequency * 0.7 + time * self.surface.wave_speed * 0.8).sin() * self.surface.wave_amplitude * 0.5;
        base + wave + wave2
    }

    pub fn get_buoyancy(&self, pos: Vec3, volume: f32) -> Vec3 {
        let water_height = self.get_height(Vec2::new(pos.x, pos.z), 0.0);
        if pos.y < water_height {
            let submerged = (water_height - pos.y).min(1.0);
            Vec3::Y * self.physics.density * volume * 9.81 * submerged
        } else {
            Vec3::ZERO
        }
    }

    pub fn is_underwater(&self, pos: Vec3, time: f32) -> bool {
        pos.y < self.get_height(Vec2::new(pos.x, pos.z), time)
    }
}

/// Underwater effects
pub struct UnderwaterEffects {
    pub fog_density: f32,
    pub fog_color: Vec3,
    pub caustics_enabled: bool,
    pub distortion: f32,
    pub god_rays: bool,
    pub bubble_particles: bool,
}

impl Default for UnderwaterEffects {
    fn default() -> Self {
        Self { fog_density: 0.02, fog_color: Vec3::new(0.0, 0.2, 0.3), caustics_enabled: true, distortion: 0.02, god_rays: true, bubble_particles: true }
    }
}
