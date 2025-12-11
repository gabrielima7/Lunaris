//! Subsurface Scattering (SSS)
//!
//! Realistic skin, wax, milk, and translucent materials.

use glam::{Vec3, Vec4};

/// SSS system
pub struct SubsurfaceScattering {
    pub profiles: Vec<SSSProfile>,
    pub settings: SSSSettings,
}

/// SSS profile
pub struct SSSProfile {
    pub name: String,
    pub scatter_color: Vec3,
    pub scatter_radius: f32,
    pub scatter_falloff: Vec3,
    pub sharpness: f32,
    pub transmission_tint: Vec3,
    pub transmission_weight: f32,
    pub profile_type: ProfileType,
}

/// Profile type
pub enum ProfileType { Skin, Wax, Milk, Jade, Marble, Foliage, Custom }

/// SSS settings
pub struct SSSSettings {
    pub enabled: bool,
    pub quality: SSSQuality,
    pub samples: u32,
    pub jitter: f32,
    pub separable_blur: bool,
}

/// SSS quality
pub enum SSSQuality { Low, Medium, High, Ultra }

impl Default for SSSSettings {
    fn default() -> Self {
        Self { enabled: true, quality: SSSQuality::High, samples: 25, jitter: 0.05, separable_blur: true }
    }
}

impl SSSProfile {
    pub fn skin() -> Self {
        Self {
            name: "Skin".into(),
            scatter_color: Vec3::new(0.8, 0.3, 0.2),
            scatter_radius: 0.02,
            scatter_falloff: Vec3::new(1.0, 0.37, 0.15),
            sharpness: 0.5,
            transmission_tint: Vec3::new(1.0, 0.4, 0.2),
            transmission_weight: 0.3,
            profile_type: ProfileType::Skin,
        }
    }

    pub fn wax() -> Self {
        Self {
            name: "Wax".into(),
            scatter_color: Vec3::new(0.98, 0.92, 0.75),
            scatter_radius: 0.03,
            scatter_falloff: Vec3::new(1.0, 0.7, 0.4),
            sharpness: 0.3,
            transmission_tint: Vec3::new(1.0, 0.9, 0.6),
            transmission_weight: 0.6,
            profile_type: ProfileType::Wax,
        }
    }

    pub fn milk() -> Self {
        Self {
            name: "Milk".into(),
            scatter_color: Vec3::new(0.95, 0.95, 0.9),
            scatter_radius: 0.05,
            scatter_falloff: Vec3::new(0.9, 0.9, 0.9),
            sharpness: 0.2,
            transmission_tint: Vec3::new(1.0, 1.0, 0.95),
            transmission_weight: 0.8,
            profile_type: ProfileType::Milk,
        }
    }

    pub fn jade() -> Self {
        Self {
            name: "Jade".into(),
            scatter_color: Vec3::new(0.2, 0.8, 0.3),
            scatter_radius: 0.01,
            scatter_falloff: Vec3::new(0.3, 1.0, 0.4),
            sharpness: 0.7,
            transmission_tint: Vec3::new(0.3, 0.9, 0.4),
            transmission_weight: 0.4,
            profile_type: ProfileType::Jade,
        }
    }

    pub fn foliage() -> Self {
        Self {
            name: "Foliage".into(),
            scatter_color: Vec3::new(0.4, 0.8, 0.2),
            scatter_radius: 0.015,
            scatter_falloff: Vec3::new(0.5, 1.0, 0.3),
            sharpness: 0.4,
            transmission_tint: Vec3::new(0.3, 0.9, 0.2),
            transmission_weight: 0.7,
            profile_type: ProfileType::Foliage,
        }
    }
}

impl SubsurfaceScattering {
    pub fn new() -> Self {
        Self {
            profiles: vec![SSSProfile::skin(), SSSProfile::wax(), SSSProfile::milk(), SSSProfile::jade(), SSSProfile::foliage()],
            settings: SSSSettings::default(),
        }
    }

    pub fn get_profile(&self, name: &str) -> Option<&SSSProfile> {
        self.profiles.iter().find(|p| p.name == name)
    }

    pub fn compute_diffusion(&self, profile: &SSSProfile, distance: f32) -> Vec3 {
        let r = distance / profile.scatter_radius;
        let falloff = profile.scatter_falloff;
        
        Vec3::new(
            gaussian_diffuse(r, falloff.x),
            gaussian_diffuse(r, falloff.y),
            gaussian_diffuse(r, falloff.z),
        ) * profile.scatter_color
    }

    pub fn compute_transmission(&self, profile: &SSSProfile, thickness: f32, light_dir: Vec3, view_dir: Vec3) -> Vec3 {
        let transmittance = (-thickness / profile.scatter_radius).exp();
        let scatter_dot = (-light_dir).dot(view_dir).max(0.0);
        let transmission = scatter_dot.powf(12.0) * transmittance;
        
        profile.transmission_tint * transmission * profile.transmission_weight
    }

    pub fn separable_blur_weights(&self, profile: &SSSProfile, samples: u32) -> Vec<BlurSample> {
        let mut weights = Vec::new();
        let radius = profile.scatter_radius * 10.0;
        
        for i in 0..samples {
            let t = (i as f32 / (samples - 1) as f32) * 2.0 - 1.0;
            let offset = t * radius;
            let weight = gaussian(offset, profile.scatter_radius * 3.0);
            
            weights.push(BlurSample {
                offset,
                weight: Vec3::new(
                    weight * profile.scatter_falloff.x,
                    weight * profile.scatter_falloff.y,
                    weight * profile.scatter_falloff.z,
                ),
            });
        }
        
        // Normalize
        let sum: Vec3 = weights.iter().map(|w| w.weight).sum();
        for w in &mut weights {
            w.weight /= sum;
        }
        
        weights
    }
}

/// Blur sample
pub struct BlurSample {
    pub offset: f32,
    pub weight: Vec3,
}

fn gaussian(x: f32, sigma: f32) -> f32 {
    let a = 1.0 / (sigma * (2.0 * std::f32::consts::PI).sqrt());
    a * (-x * x / (2.0 * sigma * sigma)).exp()
}

fn gaussian_diffuse(r: f32, variance: f32) -> f32 {
    (1.0 / (2.0 * std::f32::consts::PI * variance)) * (-r * r / (2.0 * variance)).exp()
}

/// Burley diffusion profile (used in Unreal)
pub fn burley_diffusion(r: f32, d: f32) -> f32 {
    let s = 1.0 / d;
    let exp1 = (-r * s).exp();
    let exp2 = (-r * s / 3.0).exp();
    s * (exp1 + exp2) / (8.0 * std::f32::consts::PI * r.max(0.0001))
}
