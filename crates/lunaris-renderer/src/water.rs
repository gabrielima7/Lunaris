//! Water Rendering
//!
//! Realistic water with reflections, refractions, waves, and caustics.

use glam::{Vec2, Vec3, Vec4, Mat4};

/// Water configuration
#[derive(Debug, Clone)]
pub struct WaterConfig {
    /// Water color (shallow)
    pub shallow_color: Vec3,
    /// Water color (deep)
    pub deep_color: Vec3,
    /// Depth for color transition
    pub color_depth: f32,
    /// Transparency
    pub transparency: f32,
    /// Refraction strength
    pub refraction: f32,
    /// Fresnel power
    pub fresnel_power: f32,
    /// Specular intensity
    pub specular: f32,
    /// Specular power
    pub specular_power: f32,
    /// Foam color
    pub foam_color: Vec3,
    /// Foam distance
    pub foam_distance: f32,
}

impl Default for WaterConfig {
    fn default() -> Self {
        Self {
            shallow_color: Vec3::new(0.0, 0.3, 0.5),
            deep_color: Vec3::new(0.0, 0.1, 0.2),
            color_depth: 5.0,
            transparency: 0.8,
            refraction: 0.02,
            fresnel_power: 5.0,
            specular: 1.0,
            specular_power: 256.0,
            foam_color: Vec3::ONE,
            foam_distance: 0.5,
        }
    }
}

/// Wave parameters
#[derive(Debug, Clone)]
pub struct WaveParams {
    /// Direction (normalized XZ)
    pub direction: Vec2,
    /// Wavelength
    pub wavelength: f32,
    /// Amplitude
    pub amplitude: f32,
    /// Speed
    pub speed: f32,
    /// Steepness (Gerstner, 0-1)
    pub steepness: f32,
}

impl Default for WaveParams {
    fn default() -> Self {
        Self {
            direction: Vec2::new(1.0, 0.0),
            wavelength: 10.0,
            amplitude: 0.5,
            speed: 2.0,
            steepness: 0.5,
        }
    }
}

/// Gerstner wave calculation
pub struct GerstnerWave {
    /// Wave layers
    pub waves: Vec<WaveParams>,
}

impl Default for GerstnerWave {
    fn default() -> Self {
        Self::new()
    }
}

impl GerstnerWave {
    /// Create default wave set
    #[must_use]
    pub fn new() -> Self {
        Self {
            waves: vec![
                WaveParams {
                    direction: Vec2::new(1.0, 0.0).normalize(),
                    wavelength: 20.0,
                    amplitude: 0.3,
                    speed: 2.0,
                    steepness: 0.4,
                },
                WaveParams {
                    direction: Vec2::new(0.7, 0.7).normalize(),
                    wavelength: 10.0,
                    amplitude: 0.15,
                    speed: 1.5,
                    steepness: 0.5,
                },
                WaveParams {
                    direction: Vec2::new(-0.3, 0.9).normalize(),
                    wavelength: 5.0,
                    amplitude: 0.08,
                    speed: 1.0,
                    steepness: 0.6,
                },
                WaveParams {
                    direction: Vec2::new(0.5, -0.8).normalize(),
                    wavelength: 3.0,
                    amplitude: 0.04,
                    speed: 0.8,
                    steepness: 0.3,
                },
            ],
        }
    }

    /// Calculate wave displacement at point
    #[must_use]
    pub fn calculate(&self, position: Vec2, time: f32) -> (Vec3, Vec3) {
        let mut displacement = Vec3::ZERO;
        let mut normal = Vec3::Y;
        let mut tangent = Vec3::X;
        let mut binormal = Vec3::Z;

        for wave in &self.waves {
            let k = 2.0 * std::f32::consts::PI / wave.wavelength;
            let c = wave.speed;
            let d = wave.direction;
            let a = wave.amplitude;
            let q = wave.steepness / (k * a * self.waves.len() as f32);

            let phase = k * (d.x * position.x + d.y * position.y) - c * time;
            let sin_phase = phase.sin();
            let cos_phase = phase.cos();

            // Gerstner displacement
            displacement.x += q * a * d.x * cos_phase;
            displacement.y += a * sin_phase;
            displacement.z += q * a * d.y * cos_phase;

            // Normal calculation
            let wa = k * a;
            let s = sin_phase;
            let c = cos_phase;

            tangent.x -= q * d.x * d.x * wa * s;
            tangent.y += d.x * wa * c;
            tangent.z -= q * d.x * d.y * wa * s;

            binormal.x -= q * d.x * d.y * wa * s;
            binormal.y += d.y * wa * c;
            binormal.z -= q * d.y * d.y * wa * s;
        }

        normal = binormal.cross(tangent).normalize();

        (displacement, normal)
    }

    /// Get displaced position
    #[must_use]
    pub fn displaced_position(&self, position: Vec3, time: f32) -> Vec3 {
        let (disp, _) = self.calculate(Vec2::new(position.x, position.z), time);
        position + disp
    }
}

/// Ocean/water plane
#[derive(Debug, Clone)]
pub struct WaterPlane {
    /// Configuration
    pub config: WaterConfig,
    /// Wave generator
    waves: GerstnerWave,
    /// Water level (Y position)
    pub water_level: f32,
    /// Size (half extents)
    pub size: Vec2,
    /// Tessellation level
    pub tessellation: u32,
    /// Enable reflections
    pub reflections: bool,
    /// Enable refractions
    pub refractions: bool,
    /// Enable caustics
    pub caustics: bool,
    /// Current time
    time: f32,
}

impl Default for WaterPlane {
    fn default() -> Self {
        Self::new(0.0, Vec2::splat(100.0))
    }
}

impl WaterPlane {
    /// Create a new water plane
    #[must_use]
    pub fn new(water_level: f32, size: Vec2) -> Self {
        Self {
            config: WaterConfig::default(),
            waves: GerstnerWave::new(),
            water_level,
            size,
            tessellation: 64,
            reflections: true,
            refractions: true,
            caustics: true,
            time: 0.0,
        }
    }

    /// Update water
    pub fn update(&mut self, delta_time: f32) {
        self.time += delta_time;
    }

    /// Get wave height at position
    #[must_use]
    pub fn height_at(&self, x: f32, z: f32) -> f32 {
        let (disp, _) = self.waves.calculate(Vec2::new(x, z), self.time);
        self.water_level + disp.y
    }

    /// Get wave normal at position
    #[must_use]
    pub fn normal_at(&self, x: f32, z: f32) -> Vec3 {
        let (_, normal) = self.waves.calculate(Vec2::new(x, z), self.time);
        normal
    }

    /// Calculate water color at depth
    #[must_use]
    pub fn color_at_depth(&self, depth: f32) -> Vec3 {
        let t = (depth / self.config.color_depth).clamp(0.0, 1.0);
        self.config.shallow_color.lerp(self.config.deep_color, t)
    }

    /// Calculate fresnel
    #[must_use]
    pub fn fresnel(&self, view_dir: Vec3, normal: Vec3) -> f32 {
        let n_dot_v = normal.dot(view_dir).max(0.0);
        (1.0 - n_dot_v).powf(self.config.fresnel_power)
    }

    /// Get reflection matrix (for planar reflections)
    #[must_use]
    pub fn reflection_matrix(&self) -> Mat4 {
        // Reflect around water plane Y = water_level
        Mat4::from_cols_array(&[
            1.0, 0.0, 0.0, 0.0,
            0.0, -1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 2.0 * self.water_level, 0.0, 1.0,
        ])
    }

    /// Get clipping plane for reflections
    #[must_use]
    pub fn clip_plane_above(&self) -> Vec4 {
        Vec4::new(0.0, 1.0, 0.0, -self.water_level)
    }

    /// Get clipping plane for refractions
    #[must_use]
    pub fn clip_plane_below(&self) -> Vec4 {
        Vec4::new(0.0, -1.0, 0.0, self.water_level)
    }

    /// Is point underwater
    #[must_use]
    pub fn is_underwater(&self, point: Vec3) -> bool {
        point.y < self.height_at(point.x, point.z)
    }

    /// Get current time
    #[must_use]
    pub fn time(&self) -> f32 {
        self.time
    }

    /// Generate grid vertices
    #[must_use]
    pub fn generate_grid(&self) -> (Vec<Vec3>, Vec<Vec3>, Vec<Vec2>, Vec<u32>) {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();

        let step = 2.0 * self.size / self.tessellation as f32;
        
        for z in 0..=self.tessellation {
            for x in 0..=self.tessellation {
                let px = -self.size.x + x as f32 * step.x;
                let pz = -self.size.y + z as f32 * step.y;
                
                let (disp, normal) = self.waves.calculate(Vec2::new(px, pz), self.time);
                
                positions.push(Vec3::new(px + disp.x, self.water_level + disp.y, pz + disp.z));
                normals.push(normal);
                uvs.push(Vec2::new(x as f32 / self.tessellation as f32, z as f32 / self.tessellation as f32));
            }
        }

        // Generate indices
        for z in 0..self.tessellation {
            for x in 0..self.tessellation {
                let i = z * (self.tessellation + 1) + x;
                indices.push(i);
                indices.push(i + self.tessellation + 1);
                indices.push(i + 1);
                indices.push(i + 1);
                indices.push(i + self.tessellation + 1);
                indices.push(i + self.tessellation + 2);
            }
        }

        (positions, normals, uvs, indices)
    }
}

/// Underwater effects
#[derive(Debug, Clone)]
pub struct UnderwaterEffect {
    /// Fog color
    pub fog_color: Vec3,
    /// Fog density
    pub fog_density: f32,
    /// Caustics intensity
    pub caustics_intensity: f32,
    /// Distortion strength
    pub distortion: f32,
    /// Light attenuation
    pub light_attenuation: Vec3,
}

impl Default for UnderwaterEffect {
    fn default() -> Self {
        Self {
            fog_color: Vec3::new(0.0, 0.2, 0.3),
            fog_density: 0.1,
            caustics_intensity: 0.5,
            distortion: 0.01,
            light_attenuation: Vec3::new(0.5, 0.8, 0.95),
        }
    }
}

impl UnderwaterEffect {
    /// Calculate light at depth
    #[must_use]
    pub fn light_at_depth(&self, depth: f32) -> Vec3 {
        let r = (-self.light_attenuation.x * depth).exp();
        let g = (-self.light_attenuation.y * depth).exp();
        let b = (-self.light_attenuation.z * depth).exp();
        Vec3::new(r, g, b)
    }

    /// Calculate fog factor
    #[must_use]
    pub fn fog_factor(&self, distance: f32) -> f32 {
        1.0 - (-self.fog_density * distance).exp()
    }
}
