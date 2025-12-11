//! Volumetric Rendering
//!
//! Fog, clouds, and volumetric lighting.

use glam::{Vec3, Vec4, Mat4};

/// Volumetric fog configuration
#[derive(Debug, Clone)]
pub struct VolumetricFog {
    /// Enabled
    pub enabled: bool,
    /// Fog color
    pub color: Vec3,
    /// Density
    pub density: f32,
    /// Height falloff
    pub height_falloff: f32,
    /// Base height
    pub base_height: f32,
    /// Scattering coefficient
    pub scattering: f32,
    /// Absorption coefficient
    pub absorption: f32,
    /// Anisotropy (Henyey-Greenstein g parameter, -1 to 1)
    pub anisotropy: f32,
    /// Max distance
    pub max_distance: f32,
    /// Light contribution
    pub light_intensity: f32,
    /// Temporal reprojection
    pub temporal: bool,
}

impl Default for VolumetricFog {
    fn default() -> Self {
        Self {
            enabled: true,
            color: Vec3::new(0.5, 0.6, 0.7),
            density: 0.02,
            height_falloff: 0.1,
            base_height: 0.0,
            scattering: 0.5,
            absorption: 0.1,
            anisotropy: 0.5,
            max_distance: 500.0,
            light_intensity: 1.0,
            temporal: true,
        }
    }
}

impl VolumetricFog {
    /// Calculate fog density at height
    #[must_use]
    pub fn density_at_height(&self, height: f32) -> f32 {
        let height_factor = (-self.height_falloff * (height - self.base_height).max(0.0)).exp();
        self.density * height_factor
    }

    /// Henyey-Greenstein phase function
    #[must_use]
    pub fn phase(&self, cos_theta: f32) -> f32 {
        let g = self.anisotropy;
        let g2 = g * g;
        let denom = 1.0 + g2 - 2.0 * g * cos_theta;
        (1.0 - g2) / (4.0 * std::f32::consts::PI * denom.powf(1.5))
    }

    /// Ray march through fog
    #[must_use]
    pub fn ray_march(
        &self,
        origin: Vec3,
        direction: Vec3,
        max_dist: f32,
        steps: u32,
        light_dir: Vec3,
        light_color: Vec3,
    ) -> (Vec3, f32) {
        let step_size = max_dist.min(self.max_distance) / steps as f32;
        let mut transmittance = 1.0;
        let mut in_scattered = Vec3::ZERO;

        for i in 0..steps {
            let t = (i as f32 + 0.5) * step_size;
            let pos = origin + direction * t;
            
            let local_density = self.density_at_height(pos.y);
            let extinction = (self.scattering + self.absorption) * local_density;
            
            // Beer-Lambert
            let step_transmittance = (-extinction * step_size).exp();
            
            // Phase function for light scattering
            let cos_theta = (-direction).dot(light_dir);
            let phase = self.phase(cos_theta);
            
            // In-scattering
            let scattering_amount = local_density * self.scattering * phase * self.light_intensity;
            in_scattered += light_color * scattering_amount * transmittance * step_size;
            
            transmittance *= step_transmittance;
            
            if transmittance < 0.01 {
                break;
            }
        }

        (in_scattered + self.color * (1.0 - transmittance), transmittance)
    }
}

/// Volumetric cloud layer
#[derive(Debug, Clone)]
pub struct CloudLayer {
    /// Enabled
    pub enabled: bool,
    /// Base altitude (meters)
    pub altitude: f32,
    /// Layer thickness
    pub thickness: f32,
    /// Coverage (0-1)
    pub coverage: f32,
    /// Density
    pub density: f32,
    /// Wind direction and speed
    pub wind: Vec3,
    /// Detail scale
    pub detail_scale: f32,
    /// Cloud type (0=stratus, 1=cumulus)
    pub cloud_type: f32,
    /// Silver lining intensity
    pub silver_intensity: f32,
}

impl Default for CloudLayer {
    fn default() -> Self {
        Self {
            enabled: true,
            altitude: 2000.0,
            thickness: 1000.0,
            coverage: 0.5,
            density: 0.3,
            wind: Vec3::new(10.0, 0.0, 5.0),
            detail_scale: 0.001,
            cloud_type: 0.5,
            silver_intensity: 0.3,
        }
    }
}

impl CloudLayer {
    /// Sample cloud density (simplified)
    #[must_use]
    pub fn sample_density(&self, world_pos: Vec3, time: f32) -> f32 {
        let height_fraction = ((world_pos.y - self.altitude) / self.thickness).clamp(0.0, 1.0);
        
        // Height gradient (round bottom, flat top for cumulus)
        let height_gradient = if self.cloud_type > 0.5 {
            // Cumulus
            let lower = (height_fraction * 4.0).clamp(0.0, 1.0);
            let upper = 1.0 - ((height_fraction - 0.5) * 2.0).clamp(0.0, 1.0);
            lower * upper
        } else {
            // Stratus
            1.0 - (height_fraction * 2.0 - 1.0).abs()
        };

        // Animated position
        let animated_pos = world_pos + self.wind * time;
        
        // Simplified noise (would use actual 3D noise textures)
        let noise = self.pseudo_noise_3d(animated_pos * self.detail_scale);
        let shape = self.pseudo_noise_3d(animated_pos * self.detail_scale * 0.1);
        
        // Combine
        let density = (shape + noise * 0.5 - (1.0 - self.coverage)) * height_gradient;
        (density * self.density).max(0.0)
    }

    fn pseudo_noise_3d(&self, pos: Vec3) -> f32 {
        let x = (pos.x.sin() * 43758.5453).fract();
        let y = (pos.y.sin() * 28462.1234).fract();
        let z = (pos.z.sin() * 12986.7890).fract();
        ((x + y + z) / 3.0 * 2.0 - 1.0).abs()
    }

    /// Ray march clouds
    #[must_use]
    pub fn ray_march(
        &self,
        origin: Vec3,
        direction: Vec3,
        time: f32,
        sun_dir: Vec3,
        sun_color: Vec3,
        steps: u32,
    ) -> (Vec3, f32) {
        // Find intersection with layer bounds
        let t_bottom = (self.altitude - origin.y) / direction.y;
        let t_top = (self.altitude + self.thickness - origin.y) / direction.y;
        
        let t_min = t_bottom.min(t_top).max(0.0);
        let t_max = t_bottom.max(t_top);
        
        if t_max < 0.0 || t_min > 50000.0 {
            return (Vec3::ZERO, 1.0);
        }

        let step_size = (t_max - t_min) / steps as f32;
        let mut transmittance = 1.0;
        let mut light_energy = Vec3::ZERO;

        for i in 0..steps {
            let t = t_min + (i as f32 + 0.5) * step_size;
            let pos = origin + direction * t;
            
            let density = self.sample_density(pos, time);
            if density <= 0.0 {
                continue;
            }

            // Light sampling
            let light_samples = 6;
            let mut light_transmittance = 1.0;
            for j in 0..light_samples {
                let light_pos = pos + sun_dir * (j as f32 * 50.0);
                let light_density = self.sample_density(light_pos, time);
                light_transmittance *= (-light_density * 50.0).exp();
            }

            // Beer-powder approximation
            let powder = 1.0 - (-density * 2.0).exp();
            let beer = (-density * step_size).exp();
            
            // Silver lining
            let cos_angle = direction.dot(-sun_dir);
            let silver = if cos_angle > 0.7 {
                self.silver_intensity * (cos_angle - 0.7) / 0.3
            } else {
                0.0
            };

            let scattering = sun_color * light_transmittance * (1.0 + silver);
            light_energy += scattering * density * powder * transmittance * step_size;
            
            transmittance *= beer;
            
            if transmittance < 0.01 {
                break;
            }
        }

        // Ambient
        let ambient = Vec3::splat(0.3) * (1.0 - transmittance);

        (light_energy + ambient, transmittance)
    }
}

/// God rays / volumetric light shafts
#[derive(Debug, Clone)]
pub struct VolumetricLightShafts {
    /// Enabled
    pub enabled: bool,
    /// Number of samples
    pub samples: u32,
    /// Density
    pub density: f32,
    /// Weight
    pub weight: f32,
    /// Decay
    pub decay: f32,
    /// Exposure
    pub exposure: f32,
}

impl Default for VolumetricLightShafts {
    fn default() -> Self {
        Self {
            enabled: true,
            samples: 64,
            density: 1.0,
            weight: 0.01,
            decay: 0.96,
            exposure: 0.3,
        }
    }
}

impl VolumetricLightShafts {
    /// Compute god rays (screen space)
    #[must_use]
    pub fn compute(
        &self,
        screen_uv: [f32; 2],
        light_screen_pos: [f32; 2],
        occlusion_samples: &dyn Fn(f32, f32) -> f32,
    ) -> f32 {
        let delta = [
            (screen_uv[0] - light_screen_pos[0]) / self.samples as f32 * self.density,
            (screen_uv[1] - light_screen_pos[1]) / self.samples as f32 * self.density,
        ];

        let mut uv = screen_uv;
        let mut illumination_decay = 1.0;
        let mut result = 0.0;

        for _ in 0..self.samples {
            uv[0] -= delta[0];
            uv[1] -= delta[1];
            
            // Sample occlusion (1 = lit, 0 = occluded)
            let sample = occlusion_samples(uv[0], uv[1]);
            result += sample * illumination_decay * self.weight;
            illumination_decay *= self.decay;
        }

        result * self.exposure
    }
}

/// Atmospheric scattering
#[derive(Debug, Clone)]
pub struct Atmosphere {
    /// Planet radius (km)
    pub planet_radius: f32,
    /// Atmosphere height (km)
    pub atmosphere_height: f32,
    /// Rayleigh scattering coefficients
    pub rayleigh: Vec3,
    /// Rayleigh scale height
    pub rayleigh_height: f32,
    /// Mie scattering coefficient
    pub mie: f32,
    /// Mie scale height
    pub mie_height: f32,
    /// Mie anisotropy
    pub mie_anisotropy: f32,
    /// Sun intensity
    pub sun_intensity: f32,
}

impl Default for Atmosphere {
    fn default() -> Self {
        Self {
            planet_radius: 6371.0,
            atmosphere_height: 100.0,
            rayleigh: Vec3::new(5.8e-6, 13.5e-6, 33.1e-6),
            rayleigh_height: 8.0,
            mie: 21e-6,
            mie_height: 1.2,
            mie_anisotropy: 0.758,
            sun_intensity: 22.0,
        }
    }
}

impl Atmosphere {
    /// Calculate sky color (simplified)
    #[must_use]
    pub fn sky_color(&self, view_dir: Vec3, sun_dir: Vec3) -> Vec3 {
        let cos_theta = view_dir.dot(sun_dir);
        
        // Simple gradient based on view angle
        let horizon = view_dir.y.max(0.0);
        let zenith = 1.0 - horizon;
        
        // Rayleigh (blue scatter)
        let rayleigh_color = self.rayleigh * self.sun_intensity;
        
        // Mie (sun glow)
        let mie_phase = self.henyey_greenstein(cos_theta, self.mie_anisotropy);
        let mie_color = Vec3::splat(self.mie * mie_phase * self.sun_intensity);
        
        // Combine
        let sky = rayleigh_color * zenith * 10000.0 + mie_color * 1000.0;
        
        // Sunset colors
        let sunset_factor = (1.0 - sun_dir.y.abs()).powf(2.0);
        let sunset_color = Vec3::new(1.0, 0.5, 0.2) * sunset_factor;
        
        (sky + sunset_color * self.sun_intensity * 0.1).clamp(Vec3::ZERO, Vec3::ONE)
    }

    fn henyey_greenstein(&self, cos_theta: f32, g: f32) -> f32 {
        let g2 = g * g;
        let denom = 1.0 + g2 - 2.0 * g * cos_theta;
        (1.0 - g2) / (4.0 * std::f32::consts::PI * denom.powf(1.5))
    }
}
