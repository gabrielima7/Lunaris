//! Screen Space Effects
//!
//! SSAO, SSR, SSGI, and motion blur.

use glam::{Vec2, Vec3, Vec4, Mat4};

/// Screen space effects
pub struct ScreenSpaceEffects {
    pub ssao: SSAO,
    pub ssr: SSR,
    pub ssgi: SSGI,
    pub motion_blur: MotionBlur,
}

/// Screen Space Ambient Occlusion
pub struct SSAO {
    pub enabled: bool,
    pub radius: f32,
    pub bias: f32,
    pub intensity: f32,
    pub samples: u32,
    pub blur_size: u32,
    pub power: f32,
}

impl Default for SSAO {
    fn default() -> Self {
        Self { enabled: true, radius: 0.5, bias: 0.025, intensity: 1.0, samples: 32, blur_size: 4, power: 2.0 }
    }
}

impl SSAO {
    pub fn compute(&self, position: Vec3, normal: Vec3, sample_kernel: &[Vec3], noise: Vec3) -> f32 {
        let mut occlusion = 0.0;

        for sample in sample_kernel.iter().take(self.samples as usize) {
            // Orient sample
            let tangent = (noise - normal * normal.dot(noise)).normalize();
            let bitangent = normal.cross(tangent);
            let tbn = Mat4::from_cols(
                tangent.extend(0.0),
                bitangent.extend(0.0),
                normal.extend(0.0),
                Vec4::W,
            );
            
            let sample_pos = position + (tbn.transform_vector3(*sample)) * self.radius;
            
            // Sample depth at position (would be from depth buffer)
            let sample_depth = sample_pos.z;
            let range_check = ((position.z - sample_depth).abs() / self.radius).min(1.0);
            
            occlusion += if sample_depth >= position.z + self.bias { range_check } else { 0.0 };
        }

        let result = 1.0 - (occlusion / self.samples as f32);
        result.powf(self.power) * self.intensity
    }

    pub fn generate_kernel(samples: u32) -> Vec<Vec3> {
        (0..samples).map(|i| {
            let scale = i as f32 / samples as f32;
            let scale = lerp(0.1, 1.0, scale * scale);
            Vec3::new(rand() * 2.0 - 1.0, rand() * 2.0 - 1.0, rand()).normalize() * scale
        }).collect()
    }
}

/// Screen Space Reflections
pub struct SSR {
    pub enabled: bool,
    pub max_distance: f32,
    pub resolution: f32,
    pub thickness: f32,
    pub max_steps: u32,
    pub binary_search_steps: u32,
    pub jitter: f32,
    pub fade_start: f32,
    pub fade_end: f32,
}

impl Default for SSR {
    fn default() -> Self {
        Self {
            enabled: true, max_distance: 100.0, resolution: 0.5, thickness: 0.5,
            max_steps: 64, binary_search_steps: 8, jitter: 0.1, fade_start: 0.7, fade_end: 1.0,
        }
    }
}

impl SSR {
    pub fn trace(&self, position: Vec3, reflect_dir: Vec3, depth_fn: impl Fn(Vec2) -> f32, proj: Mat4) -> Option<Vec2> {
        let max_dist = self.max_distance;
        let step = max_dist / self.max_steps as f32;

        for i in 0..self.max_steps {
            let t = (i as f32 + rand() * self.jitter) * step;
            let sample_pos = position + reflect_dir * t;
            
            // Project to screen space
            let clip = proj * sample_pos.extend(1.0);
            let ndc = clip.truncate() / clip.w;
            let screen_uv = (ndc.truncate() + Vec2::ONE) * 0.5;
            
            if screen_uv.x < 0.0 || screen_uv.x > 1.0 || screen_uv.y < 0.0 || screen_uv.y > 1.0 {
                return None;
            }
            
            let scene_depth = depth_fn(screen_uv);
            if sample_pos.z > scene_depth && sample_pos.z < scene_depth + self.thickness {
                return Some(self.binary_search(position, reflect_dir, t - step, t, depth_fn, proj));
            }
        }
        None
    }

    fn binary_search(&self, origin: Vec3, dir: Vec3, mut t_min: f32, mut t_max: f32, depth_fn: impl Fn(Vec2) -> f32, proj: Mat4) -> Vec2 {
        for _ in 0..self.binary_search_steps {
            let t = (t_min + t_max) * 0.5;
            let pos = origin + dir * t;
            let clip = proj * pos.extend(1.0);
            let ndc = clip.truncate() / clip.w;
            let uv = (ndc.truncate() + Vec2::ONE) * 0.5;
            let depth = depth_fn(uv);
            
            if pos.z > depth { t_max = t; } else { t_min = t; }
        }
        let t = (t_min + t_max) * 0.5;
        let pos = origin + dir * t;
        let clip = proj * pos.extend(1.0);
        let ndc = clip.truncate() / clip.w;
        (ndc.truncate() + Vec2::ONE) * 0.5
    }
}

/// Screen Space Global Illumination
pub struct SSGI {
    pub enabled: bool,
    pub intensity: f32,
    pub radius: f32,
    pub samples: u32,
    pub thickness: f32,
    pub falloff: f32,
}

impl Default for SSGI {
    fn default() -> Self {
        Self { enabled: true, intensity: 1.0, radius: 2.0, samples: 16, thickness: 0.3, falloff: 2.0 }
    }
}

impl SSGI {
    pub fn compute(&self, position: Vec3, normal: Vec3, color_fn: impl Fn(Vec2) -> Vec3) -> Vec3 {
        let mut gi = Vec3::ZERO;
        
        for i in 0..self.samples {
            let angle = (i as f32 / self.samples as f32) * std::f32::consts::TAU;
            let r = rand() * self.radius;
            let offset = Vec2::new(angle.cos() * r, angle.sin() * r);
            
            let sample_uv = Vec2::new(position.x, position.y) + offset;
            let sample_color = color_fn(sample_uv);
            
            let weight = 1.0 / (1.0 + r.powf(self.falloff));
            gi += sample_color * weight;
        }
        
        gi / self.samples as f32 * self.intensity
    }
}

/// Motion blur
pub struct MotionBlur {
    pub enabled: bool,
    pub intensity: f32,
    pub samples: u32,
    pub max_velocity: f32,
    pub object_blur: bool,
    pub camera_blur: bool,
}

impl Default for MotionBlur {
    fn default() -> Self {
        Self { enabled: true, intensity: 1.0, samples: 8, max_velocity: 40.0, object_blur: true, camera_blur: true }
    }
}

impl MotionBlur {
    pub fn compute(&self, uv: Vec2, velocity: Vec2, color_fn: impl Fn(Vec2) -> Vec3) -> Vec3 {
        let vel = velocity.clamp_length_max(self.max_velocity) * self.intensity;
        let mut color = Vec3::ZERO;
        
        for i in 0..self.samples {
            let t = i as f32 / (self.samples - 1) as f32 - 0.5;
            let sample_uv = uv + vel * t;
            color += color_fn(sample_uv);
        }
        
        color / self.samples as f32
    }
}

impl ScreenSpaceEffects {
    pub fn new() -> Self {
        Self { ssao: SSAO::default(), ssr: SSR::default(), ssgi: SSGI::default(), motion_blur: MotionBlur::default() }
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 { a + (b - a) * t }
fn rand() -> f32 { 0.5 }
