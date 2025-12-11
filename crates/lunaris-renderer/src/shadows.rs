//! Contact Shadows
//!
//! Screen-space contact shadows for fine shadow detail.

use glam::{Vec3, Mat4};

/// Contact shadows configuration
#[derive(Debug, Clone)]
pub struct ContactShadows {
    /// Enabled
    pub enabled: bool,
    /// Max distance (world units)
    pub max_distance: f32,
    /// Step count
    pub steps: u32,
    /// Thickness (world units)
    pub thickness: f32,
    /// Bias
    pub bias: f32,
    /// Fade distance start
    pub fade_start: f32,
    /// Fade distance end
    pub fade_end: f32,
    /// Use temporal filtering
    pub temporal: bool,
    /// Dither pattern index
    pub dither_frame: u32,
}

impl Default for ContactShadows {
    fn default() -> Self {
        Self {
            enabled: true,
            max_distance: 0.5,
            steps: 16,
            thickness: 0.01,
            bias: 0.01,
            fade_start: 0.3,
            fade_end: 0.5,
            temporal: true,
            dither_frame: 0,
        }
    }
}

impl ContactShadows {
    /// Ray march for contact shadow
    ///
    /// Returns shadow factor (0 = in shadow, 1 = lit)
    #[must_use]
    pub fn trace(
        &self,
        position: Vec3,
        normal: Vec3,
        light_dir: Vec3,
        view_proj: &Mat4,
        depth_sample: &dyn Fn(f32, f32) -> f32,
        screen_size: [f32; 2],
    ) -> f32 {
        if !self.enabled {
            return 1.0;
        }

        // Offset to avoid self-shadowing
        let start_pos = position + normal * self.bias;
        
        // March toward light
        let step_size = self.max_distance / self.steps as f32;
        
        for i in 0..self.steps {
            // Add dithering for temporal stability
            let dither = if self.temporal {
                Self::dither(i, self.dither_frame)
            } else {
                0.0
            };
            
            let t = (i as f32 + dither) * step_size;
            let sample_pos = start_pos + light_dir * t;
            
            // Project to screen space
            let clip = *view_proj * sample_pos.extend(1.0);
            if clip.w <= 0.0 {
                continue;
            }
            
            let ndc = clip.truncate() / clip.w;
            let uv = (ndc.truncate() * 0.5 + 0.5).to_array();
            
            // Check screen bounds
            if uv[0] < 0.0 || uv[0] > 1.0 || uv[1] < 0.0 || uv[1] > 1.0 {
                continue;
            }
            
            // Sample depth buffer
            let depth = depth_sample(uv[0], uv[1]);
            let sample_depth = ndc.z * 0.5 + 0.5;
            
            // Check occlusion
            let depth_diff = sample_depth - depth;
            if depth_diff > 0.0 && depth_diff < self.thickness {
                // Fade based on distance
                let fade = if t > self.fade_start {
                    1.0 - ((t - self.fade_start) / (self.fade_end - self.fade_start)).clamp(0.0, 1.0)
                } else {
                    1.0
                };
                
                return 1.0 - fade;
            }
        }
        
        1.0
    }

    fn dither(step: u32, frame: u32) -> f32 {
        let pattern = [0.0, 0.5, 0.25, 0.75, 0.125, 0.625, 0.375, 0.875];
        let idx = (step + frame) as usize % pattern.len();
        pattern[idx]
    }
}

/// Cascaded shadow maps configuration
#[derive(Debug, Clone)]
pub struct CascadedShadowMaps {
    /// Number of cascades
    pub cascade_count: u32,
    /// Cascade split lambda (0 = uniform, 1 = logarithmic)
    pub split_lambda: f32,
    /// Shadow map resolution per cascade
    pub resolution: u32,
    /// Shadow bias
    pub bias: f32,
    /// Normal offset bias
    pub normal_bias: f32,
    /// Soft shadow filter size
    pub filter_size: f32,
    /// PCF samples
    pub pcf_samples: u32,
    /// Blend between cascades
    pub blend_cascades: bool,
    /// Blend distance
    pub blend_distance: f32,
    /// Cascade distances
    cascade_distances: Vec<f32>,
}

impl Default for CascadedShadowMaps {
    fn default() -> Self {
        Self::new(4, 100.0)
    }
}

impl CascadedShadowMaps {
    /// Create new CSM config
    #[must_use]
    pub fn new(cascade_count: u32, max_distance: f32) -> Self {
        let mut csm = Self {
            cascade_count,
            split_lambda: 0.75,
            resolution: 2048,
            bias: 0.005,
            normal_bias: 0.01,
            filter_size: 2.0,
            pcf_samples: 16,
            blend_cascades: true,
            blend_distance: 0.1,
            cascade_distances: Vec::new(),
        };
        csm.calculate_splits(0.1, max_distance);
        csm
    }

    /// Calculate cascade split distances
    pub fn calculate_splits(&mut self, near: f32, far: f32) {
        self.cascade_distances.clear();
        
        for i in 0..self.cascade_count {
            let p = (i + 1) as f32 / self.cascade_count as f32;
            
            // Logarithmic split
            let log_split = near * (far / near).powf(p);
            
            // Uniform split
            let uniform_split = near + (far - near) * p;
            
            // Blend
            let split = self.split_lambda * log_split + (1.0 - self.split_lambda) * uniform_split;
            self.cascade_distances.push(split);
        }
    }

    /// Get cascade for depth
    #[must_use]
    pub fn get_cascade(&self, depth: f32) -> u32 {
        for (i, &dist) in self.cascade_distances.iter().enumerate() {
            if depth < dist {
                return i as u32;
            }
        }
        self.cascade_count - 1
    }

    /// Get cascade distance
    #[must_use]
    pub fn cascade_distance(&self, index: u32) -> f32 {
        self.cascade_distances.get(index as usize).copied().unwrap_or(1000.0)
    }

    /// Calculate cascade blend weight
    #[must_use]
    pub fn cascade_blend(&self, depth: f32, cascade: u32) -> f32 {
        if !self.blend_cascades {
            return 1.0;
        }

        let cascade_end = self.cascade_distance(cascade);
        let blend_start = cascade_end * (1.0 - self.blend_distance);
        
        if depth > blend_start {
            1.0 - (depth - blend_start) / (cascade_end - blend_start)
        } else {
            1.0
        }
    }
}

/// Variance shadow maps
#[derive(Debug, Clone)]
pub struct VarianceShadowMaps {
    /// Enabled
    pub enabled: bool,
    /// Resolution
    pub resolution: u32,
    /// Light bleed reduction
    pub light_bleed_reduction: f32,
    /// Min variance
    pub min_variance: f32,
    /// Blur kernel size
    pub blur_size: u32,
}

impl Default for VarianceShadowMaps {
    fn default() -> Self {
        Self {
            enabled: true,
            resolution: 1024,
            light_bleed_reduction: 0.2,
            min_variance: 0.00001,
            blur_size: 3,
        }
    }
}

impl VarianceShadowMaps {
    /// Calculate shadow using Chebyshev's inequality
    #[must_use]
    pub fn calculate_shadow(&self, depth: f32, moments: [f32; 2]) -> f32 {
        let mean = moments[0];
        let variance = moments[1] - mean * mean;
        let variance = variance.max(self.min_variance);
        
        let d = depth - mean;
        let p_max = variance / (variance + d * d);
        
        // Light bleed reduction
        let p = ((p_max - self.light_bleed_reduction) / (1.0 - self.light_bleed_reduction)).clamp(0.0, 1.0);
        
        if depth <= mean {
            1.0
        } else {
            p
        }
    }
}
