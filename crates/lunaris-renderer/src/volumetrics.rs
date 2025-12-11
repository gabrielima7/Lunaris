//! Volumetric Effects
//!
//! Ray-marched fog, clouds, god rays, and volumetric shadows.

use glam::{Vec3, Vec4, Mat4};

/// Volumetric system
pub struct VolumetricSystem {
    pub fog: VolumetricFog,
    pub clouds: VolumetricClouds,
    pub god_rays: GodRays,
    pub settings: VolumetricSettings,
}

/// Volumetric fog
pub struct VolumetricFog {
    pub enabled: bool,
    pub density: f32,
    pub height_falloff: f32,
    pub base_height: f32,
    pub color: Vec3,
    pub scattering: f32,
    pub absorption: f32,
    pub anisotropy: f32,
    pub noise_scale: f32,
    pub noise_intensity: f32,
    pub temporal_reprojection: bool,
}

impl Default for VolumetricFog {
    fn default() -> Self {
        Self {
            enabled: true, density: 0.01, height_falloff: 0.5, base_height: 0.0,
            color: Vec3::new(0.5, 0.6, 0.7), scattering: 0.5, absorption: 0.1,
            anisotropy: 0.5, noise_scale: 0.1, noise_intensity: 0.3, temporal_reprojection: true,
        }
    }
}

impl VolumetricFog {
    pub fn sample(&self, position: Vec3, view_dir: Vec3, light_dir: Vec3) -> Vec4 {
        let height_density = (-((position.y - self.base_height) * self.height_falloff).max(0.0)).exp();
        let base_density = self.density * height_density;
        
        // Phase function (Henyey-Greenstein)
        let cos_angle = view_dir.dot(light_dir);
        let g = self.anisotropy;
        let phase = (1.0 - g * g) / (4.0 * std::f32::consts::PI * (1.0 + g * g - 2.0 * g * cos_angle).powf(1.5));
        
        let in_scatter = self.scattering * phase;
        let out_scatter = self.absorption + self.scattering;
        
        Vec4::new(self.color.x * in_scatter, self.color.y * in_scatter, self.color.z * in_scatter, base_density * out_scatter)
    }

    pub fn raymarch(&self, ray_origin: Vec3, ray_dir: Vec3, max_dist: f32, steps: u32) -> (Vec3, f32) {
        let step_size = max_dist / steps as f32;
        let mut accumulated_color = Vec3::ZERO;
        let mut transmittance = 1.0;

        for i in 0..steps {
            let t = (i as f32 + 0.5) * step_size;
            let pos = ray_origin + ray_dir * t;
            
            let sample = self.sample(pos, ray_dir, Vec3::new(0.5, 1.0, 0.3).normalize());
            let density = sample.w * step_size;
            
            let sample_transmittance = (-density).exp();
            accumulated_color += Vec3::new(sample.x, sample.y, sample.z) * transmittance * (1.0 - sample_transmittance);
            transmittance *= sample_transmittance;
            
            if transmittance < 0.01 { break; }
        }

        (accumulated_color, transmittance)
    }
}

/// Volumetric clouds
pub struct VolumetricClouds {
    pub enabled: bool,
    pub coverage: f32,
    pub altitude_min: f32,
    pub altitude_max: f32,
    pub density: f32,
    pub shape_scale: f32,
    pub detail_scale: f32,
    pub wind_speed: Vec3,
    pub time: f32,
    pub silver_lining: f32,
    pub ambient_light: Vec3,
}

impl Default for VolumetricClouds {
    fn default() -> Self {
        Self {
            enabled: true, coverage: 0.5, altitude_min: 1500.0, altitude_max: 4000.0,
            density: 0.3, shape_scale: 0.0001, detail_scale: 0.001, wind_speed: Vec3::new(10.0, 0.0, 5.0),
            time: 0.0, silver_lining: 0.5, ambient_light: Vec3::splat(0.3),
        }
    }
}

impl VolumetricClouds {
    pub fn update(&mut self, dt: f32) {
        self.time += dt;
    }

    pub fn sample_density(&self, position: Vec3) -> f32 {
        let height_fraction = ((position.y - self.altitude_min) / (self.altitude_max - self.altitude_min)).clamp(0.0, 1.0);
        let height_gradient = height_fraction * (1.0 - height_fraction) * 4.0;
        
        let wind_offset = self.wind_speed * self.time;
        let sample_pos = position + wind_offset;
        
        let noise = fbm(sample_pos * self.shape_scale, 4);
        let coverage = ((noise - (1.0 - self.coverage)) / self.coverage).max(0.0);
        
        coverage * height_gradient * self.density
    }

    pub fn raymarch(&self, ray_origin: Vec3, ray_dir: Vec3, steps: u32) -> (Vec3, f32) {
        let t_min = (self.altitude_min - ray_origin.y) / ray_dir.y;
        let t_max = (self.altitude_max - ray_origin.y) / ray_dir.y;
        
        if t_min > t_max || t_max < 0.0 { return (Vec3::ZERO, 1.0); }
        
        let start = t_min.max(0.0);
        let end = t_max.min(50000.0);
        let step_size = (end - start) / steps as f32;
        
        let mut accumulated = Vec3::ZERO;
        let mut transmittance = 1.0;

        for i in 0..steps {
            let t = start + (i as f32 + 0.5) * step_size;
            let pos = ray_origin + ray_dir * t;
            
            let density = self.sample_density(pos);
            if density > 0.001 {
                let sample_transmittance = (-density * step_size).exp();
                let light = self.ambient_light + Vec3::splat(self.silver_lining);
                accumulated += light * density * transmittance * (1.0 - sample_transmittance);
                transmittance *= sample_transmittance;
            }
            
            if transmittance < 0.01 { break; }
        }

        (accumulated, transmittance)
    }
}

fn fbm(p: Vec3, octaves: u32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    let mut frequency = 1.0;
    for _ in 0..octaves {
        value += amplitude * noise3d(p * frequency);
        frequency *= 2.0;
        amplitude *= 0.5;
    }
    value
}

fn noise3d(p: Vec3) -> f32 {
    ((p.x * 12.9898 + p.y * 78.233 + p.z * 45.164).sin() * 43758.5453).fract()
}

/// God rays
pub struct GodRays {
    pub enabled: bool,
    pub intensity: f32,
    pub decay: f32,
    pub density: f32,
    pub samples: u32,
    pub light_position: Vec3,
}

impl Default for GodRays {
    fn default() -> Self {
        Self { enabled: true, intensity: 1.0, decay: 0.95, density: 0.5, samples: 64, light_position: Vec3::ZERO }
    }
}

impl GodRays {
    pub fn compute(&self, screen_pos: Vec3, depth_fn: impl Fn(Vec3) -> f32) -> f32 {
        let ray_dir = (screen_pos - self.light_position).normalize();
        let step = 1.0 / self.samples as f32;
        
        let mut illumination = 0.0;
        let mut decay_factor = 1.0;

        for i in 0..self.samples {
            let t = i as f32 * step;
            let sample_pos = self.light_position + ray_dir * t;
            let depth = depth_fn(sample_pos);
            
            if depth > t {
                illumination += decay_factor * self.density;
            }
            
            decay_factor *= self.decay;
        }

        illumination * self.intensity
    }
}

/// Volumetric settings
pub struct VolumetricSettings {
    pub resolution_scale: f32,
    pub max_steps: u32,
    pub temporal_weight: f32,
    pub jitter: bool,
}

impl Default for VolumetricSettings {
    fn default() -> Self {
        Self { resolution_scale: 0.5, max_steps: 64, temporal_weight: 0.9, jitter: true }
    }
}

impl VolumetricSystem {
    pub fn new() -> Self {
        Self { fog: VolumetricFog::default(), clouds: VolumetricClouds::default(), god_rays: GodRays::default(), settings: VolumetricSettings::default() }
    }

    pub fn update(&mut self, dt: f32) {
        self.clouds.update(dt);
    }
}
