//! Subsurface Scattering
//!
//! Realistic skin, wax, leaves, and translucent materials.

use glam::Vec3;

/// SSS profile type
#[derive(Debug, Clone, Copy, Default)]
pub enum SSSProfile {
    /// Human skin
    #[default]
    Skin,
    /// Jade/marble
    Jade,
    /// Milk
    Milk,
    /// Wax/candle
    Wax,
    /// Plant leaf
    Leaf,
    /// Custom
    Custom,
}

/// Subsurface scattering configuration
#[derive(Debug, Clone)]
pub struct SubsurfaceScattering {
    /// Profile type
    pub profile: SSSProfile,
    /// Scatter color
    pub scatter_color: Vec3,
    /// Scatter radius (world units)
    pub scatter_radius: f32,
    /// Scatter falloff
    pub falloff: Vec3,
    /// Thickness map multiplier
    pub thickness_scale: f32,
    /// Translucency
    pub translucency: f32,
    /// Normal distortion
    pub normal_distortion: f32,
    /// Ambient
    pub ambient: f32,
    /// Quality (samples)
    pub quality: SSSQuality,
}

/// SSS quality level
#[derive(Debug, Clone, Copy, Default)]
pub enum SSSQuality {
    /// Low (4 samples)
    Low,
    /// Medium (8 samples)
    #[default]
    Medium,
    /// High (16 samples)
    High,
    /// Ultra (32 samples)
    Ultra,
}

impl SSSQuality {
    /// Get sample count
    #[must_use]
    pub fn samples(&self) -> u32 {
        match self {
            SSSQuality::Low => 4,
            SSSQuality::Medium => 8,
            SSSQuality::High => 16,
            SSSQuality::Ultra => 32,
        }
    }
}

impl Default for SubsurfaceScattering {
    fn default() -> Self {
        Self::skin()
    }
}

impl SubsurfaceScattering {
    /// Human skin profile
    #[must_use]
    pub fn skin() -> Self {
        Self {
            profile: SSSProfile::Skin,
            scatter_color: Vec3::new(1.0, 0.35, 0.15),
            scatter_radius: 0.012,
            falloff: Vec3::new(1.0, 0.37, 0.3),
            thickness_scale: 1.0,
            translucency: 0.5,
            normal_distortion: 0.1,
            ambient: 0.1,
            quality: SSSQuality::Medium,
        }
    }

    /// Jade/marble profile
    #[must_use]
    pub fn jade() -> Self {
        Self {
            profile: SSSProfile::Jade,
            scatter_color: Vec3::new(0.4, 1.0, 0.5),
            scatter_radius: 0.03,
            falloff: Vec3::new(0.8, 1.0, 0.6),
            thickness_scale: 1.0,
            translucency: 0.8,
            normal_distortion: 0.0,
            ambient: 0.2,
            quality: SSSQuality::Medium,
        }
    }

    /// Milk profile
    #[must_use]
    pub fn milk() -> Self {
        Self {
            profile: SSSProfile::Milk,
            scatter_color: Vec3::new(0.95, 0.93, 0.88),
            scatter_radius: 0.05,
            falloff: Vec3::splat(1.0),
            thickness_scale: 1.0,
            translucency: 0.9,
            normal_distortion: 0.0,
            ambient: 0.3,
            quality: SSSQuality::Medium,
        }
    }

    /// Wax profile
    #[must_use]
    pub fn wax() -> Self {
        Self {
            profile: SSSProfile::Wax,
            scatter_color: Vec3::new(1.0, 0.9, 0.7),
            scatter_radius: 0.02,
            falloff: Vec3::new(1.0, 0.8, 0.5),
            thickness_scale: 1.0,
            translucency: 0.7,
            normal_distortion: 0.05,
            ambient: 0.15,
            quality: SSSQuality::Medium,
        }
    }

    /// Leaf profile
    #[must_use]
    pub fn leaf() -> Self {
        Self {
            profile: SSSProfile::Leaf,
            scatter_color: Vec3::new(0.5, 1.0, 0.2),
            scatter_radius: 0.005,
            falloff: Vec3::new(0.5, 1.0, 0.3),
            thickness_scale: 1.0,
            translucency: 0.6,
            normal_distortion: 0.2,
            ambient: 0.1,
            quality: SSSQuality::Low,
        }
    }

    /// Calculate SSS contribution
    #[must_use]
    pub fn calculate(
        &self,
        n_dot_l: f32,
        light_color: Vec3,
        thickness: f32,
        view_dot_light: f32,
    ) -> Vec3 {
        // Wrap lighting for soft terminator
        let wrap = 0.5;
        let wrapped_diffuse = ((n_dot_l + wrap) / (1.0 + wrap)).max(0.0);
        
        // Thickness-based attenuation
        let thickness_atten = (-thickness * self.thickness_scale).exp();
        
        // Translucency (back-lighting)
        let translucent_dot = (-view_dot_light).max(0.0).powf(2.0);
        let translucency = translucent_dot * self.translucency * thickness_atten;
        
        // Scatter contribution
        let scatter_falloff = Vec3::new(
            (-thickness / self.falloff.x).exp(),
            (-thickness / self.falloff.y).exp(),
            (-thickness / self.falloff.z).exp(),
        );
        
        let scatter = self.scatter_color * scatter_falloff;
        
        // Combine
        let diffuse = light_color * wrapped_diffuse;
        let sss = light_color * scatter * (wrapped_diffuse + translucency);
        let ambient_term = self.scatter_color * self.ambient;
        
        diffuse * 0.5 + sss * 0.5 + ambient_term
    }

    /// Generate Gaussian blur kernel for screen-space SSS
    #[must_use]
    pub fn generate_kernel(&self) -> Vec<(f32, Vec3)> {
        let samples = self.quality.samples() as usize;
        let mut kernel = Vec::with_capacity(samples);
        
        // Separable Gaussian with different falloffs per channel
        for i in 0..samples {
            let x = (i as f32 / (samples - 1) as f32) * 2.0 - 1.0;
            let offset = x * self.scatter_radius;
            
            let weight = Vec3::new(
                Self::gaussian(x, self.falloff.x),
                Self::gaussian(x, self.falloff.y),
                Self::gaussian(x, self.falloff.z),
            );
            
            kernel.push((offset, weight));
        }
        
        // Normalize
        let sum: Vec3 = kernel.iter().map(|(_, w)| *w).fold(Vec3::ZERO, |a, b| a + b);
        for (_, ref mut weight) in &mut kernel {
            *weight /= sum;
        }
        
        kernel
    }

    fn gaussian(x: f32, sigma: f32) -> f32 {
        let sigma2 = sigma * sigma;
        (-(x * x) / (2.0 * sigma2)).exp() / (2.0 * std::f32::consts::PI * sigma2).sqrt()
    }
}

/// Pre-integrated skin lookup table
pub struct SkinLUT {
    /// LUT data (n_dot_l, curvature) -> color
    data: Vec<Vec3>,
    /// Resolution
    resolution: u32,
}

impl SkinLUT {
    /// Generate skin LUT
    #[must_use]
    pub fn generate(resolution: u32, sss: &SubsurfaceScattering) -> Self {
        let mut data = Vec::with_capacity((resolution * resolution) as usize);
        
        for y in 0..resolution {
            for x in 0..resolution {
                let n_dot_l = (x as f32 / (resolution - 1) as f32) * 2.0 - 1.0;
                let curvature = y as f32 / (resolution - 1) as f32;
                
                // Simplified skin BRDF
                let thickness = curvature * 2.0;
                let color = sss.calculate(n_dot_l, Vec3::ONE, thickness, 0.0);
                
                data.push(color);
            }
        }
        
        Self { data, resolution }
    }

    /// Sample LUT
    #[must_use]
    pub fn sample(&self, n_dot_l: f32, curvature: f32) -> Vec3 {
        let u = ((n_dot_l * 0.5 + 0.5) * (self.resolution - 1) as f32) as usize;
        let v = (curvature.clamp(0.0, 1.0) * (self.resolution - 1) as f32) as usize;
        let u = u.min(self.resolution as usize - 1);
        let v = v.min(self.resolution as usize - 1);
        
        let idx = v * self.resolution as usize + u;
        self.data.get(idx).copied().unwrap_or(Vec3::ZERO)
    }
}
