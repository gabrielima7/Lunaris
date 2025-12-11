//! Ray Traced Shadows
//!
//! Area light soft shadows, contact hardening, denoising.

use glam::{Vec3, Vec2, Mat4};

/// Ray traced shadows system
pub struct RTShadows {
    pub settings: RTShadowSettings,
    pub denoiser: ShadowDenoiser,
    pub lights: Vec<AreaLight>,
}

/// RT shadow settings
pub struct RTShadowSettings {
    pub enabled: bool,
    pub samples_per_pixel: u32,
    pub max_ray_distance: f32,
    pub bias: f32,
    pub normal_bias: f32,
    pub contact_hardening: bool,
    pub penumbra_scale: f32,
    pub temporal_accumulation: bool,
    pub temporal_weight: f32,
}

impl Default for RTShadowSettings {
    fn default() -> Self {
        Self {
            enabled: true, samples_per_pixel: 1, max_ray_distance: 1000.0, bias: 0.001, normal_bias: 0.01,
            contact_hardening: true, penumbra_scale: 1.0, temporal_accumulation: true, temporal_weight: 0.9,
        }
    }
}

/// Area light
pub struct AreaLight {
    pub id: u64,
    pub light_type: AreaLightType,
    pub position: Vec3,
    pub direction: Vec3,
    pub color: Vec3,
    pub intensity: f32,
    pub cast_shadows: bool,
}

/// Area light type
pub enum AreaLightType {
    Sphere { radius: f32 },
    Disk { radius: f32, normal: Vec3 },
    Rectangle { width: f32, height: f32, right: Vec3, up: Vec3 },
    Tube { length: f32, radius: f32, axis: Vec3 },
}

impl RTShadows {
    pub fn new() -> Self {
        Self { settings: RTShadowSettings::default(), denoiser: ShadowDenoiser::new(), lights: Vec::new() }
    }

    pub fn add_light(&mut self, light: AreaLight) {
        self.lights.push(light);
    }

    pub fn sample_light(&self, light: &AreaLight, random: Vec2) -> Vec3 {
        match &light.light_type {
            AreaLightType::Sphere { radius } => {
                let theta = random.x * std::f32::consts::TAU;
                let phi = (random.y * 2.0 - 1.0).acos();
                let dir = Vec3::new(phi.sin() * theta.cos(), phi.sin() * theta.sin(), phi.cos());
                light.position + dir * *radius
            }
            AreaLightType::Disk { radius, normal } => {
                let r = random.x.sqrt() * *radius;
                let theta = random.y * std::f32::consts::TAU;
                let tangent = if normal.y.abs() < 0.999 { Vec3::Y.cross(*normal).normalize() } else { Vec3::X };
                let bitangent = normal.cross(tangent);
                light.position + tangent * theta.cos() * r + bitangent * theta.sin() * r
            }
            AreaLightType::Rectangle { width, height, right, up } => {
                let u = random.x - 0.5;
                let v = random.y - 0.5;
                light.position + *right * u * *width + *up * v * *height
            }
            AreaLightType::Tube { length, radius, axis } => {
                let t = random.x - 0.5;
                let theta = random.y * std::f32::consts::TAU;
                let perp = if axis.y.abs() < 0.999 { Vec3::Y.cross(*axis).normalize() } else { Vec3::X };
                let perp2 = axis.cross(perp);
                light.position + *axis * t * *length + (perp * theta.cos() + perp2 * theta.sin()) * *radius
            }
        }
    }

    pub fn trace_shadow(&self, origin: Vec3, light_pos: Vec3, normal: Vec3) -> ShadowResult {
        let biased_origin = origin + normal * self.settings.normal_bias;
        let direction = (light_pos - biased_origin).normalize();
        let distance = (light_pos - biased_origin).length();
        
        // Would trace ray here
        ShadowResult { visibility: 1.0, hit_distance: distance, penumbra: 0.0 }
    }

    pub fn compute_pcss_penumbra(&self, blocker_distance: f32, receiver_distance: f32, light_size: f32) -> f32 {
        if blocker_distance <= 0.0 { return 0.0; }
        let penumbra = (receiver_distance - blocker_distance) / blocker_distance * light_size * self.settings.penumbra_scale;
        penumbra.clamp(0.0, 1.0)
    }
}

/// Shadow result
pub struct ShadowResult {
    pub visibility: f32,
    pub hit_distance: f32,
    pub penumbra: f32,
}

/// Shadow denoiser
pub struct ShadowDenoiser {
    pub enabled: bool,
    pub spatial_sigma: f32,
    pub temporal_sigma: f32,
    pub history: Option<Vec<f32>>,
    pub history_valid: Vec<bool>,
}

impl ShadowDenoiser {
    pub fn new() -> Self {
        Self { enabled: true, spatial_sigma: 1.0, temporal_sigma: 0.1, history: None, history_valid: Vec::new() }
    }

    pub fn denoise(&mut self, current: &[f32], motion_vectors: &[Vec2], width: u32, height: u32) -> Vec<f32> {
        let mut result = current.to_vec();
        
        // Temporal accumulation
        if let Some(history) = &self.history {
            for i in 0..result.len() {
                let mv = if i < motion_vectors.len() { motion_vectors[i] } else { Vec2::ZERO };
                let prev_x = ((i as u32 % width) as f32 - mv.x) as i32;
                let prev_y = ((i as u32 / width) as f32 - mv.y) as i32;
                
                if prev_x >= 0 && prev_x < width as i32 && prev_y >= 0 && prev_y < height as i32 {
                    let prev_idx = (prev_y as u32 * width + prev_x as u32) as usize;
                    if prev_idx < history.len() {
                        result[i] = result[i] * (1.0 - self.temporal_sigma) + history[prev_idx] * self.temporal_sigma;
                    }
                }
            }
        }
        
        // Spatial blur (simplified box filter)
        let mut blurred = result.clone();
        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                let idx = (y * width + x) as usize;
                let mut sum = 0.0;
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        let ni = ((y as i32 + dy) as u32 * width + (x as i32 + dx) as u32) as usize;
                        sum += result[ni];
                    }
                }
                blurred[idx] = sum / 9.0;
            }
        }
        
        self.history = Some(blurred.clone());
        blurred
    }
}

/// Shadow mask generation
pub struct ShadowMask {
    pub width: u32,
    pub height: u32,
    pub data: Vec<f32>,
}

impl ShadowMask {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height, data: vec![1.0; (width * height) as usize] }
    }

    pub fn clear(&mut self) { self.data.fill(1.0); }
    
    pub fn set(&mut self, x: u32, y: u32, value: f32) {
        if x < self.width && y < self.height {
            self.data[(y * self.width + x) as usize] = value;
        }
    }

    pub fn get(&self, x: u32, y: u32) -> f32 {
        if x < self.width && y < self.height { self.data[(y * self.width + x) as usize] }
        else { 1.0 }
    }
}
