//! Screen Space Reflections (SSR)
//!
//! High-quality real-time reflections using screen-space ray marching.

use glam::{Vec2, Vec3, Vec4, Mat4};

/// SSR quality preset
#[derive(Debug, Clone, Copy, Default)]
pub enum SSRQuality {
    /// Low quality - 8 steps
    Low,
    /// Medium quality - 16 steps
    #[default]
    Medium,
    /// High quality - 32 steps
    High,
    /// Ultra quality - 64 steps
    Ultra,
}

impl SSRQuality {
    /// Get max steps
    #[must_use]
    pub fn max_steps(&self) -> u32 {
        match self {
            SSRQuality::Low => 8,
            SSRQuality::Medium => 16,
            SSRQuality::High => 32,
            SSRQuality::Ultra => 64,
        }
    }
}

/// SSR configuration
#[derive(Debug, Clone)]
pub struct SSRConfig {
    /// Enabled
    pub enabled: bool,
    /// Quality
    pub quality: SSRQuality,
    /// Max distance
    pub max_distance: f32,
    /// Step size
    pub step_size: f32,
    /// Thickness
    pub thickness: f32,
    /// Max roughness
    pub max_roughness: f32,
    /// Edge fade
    pub edge_fade: f32,
    /// Temporal filtering
    pub temporal: bool,
    /// Binary search refinement
    pub refinement_steps: u32,
}

impl Default for SSRConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            quality: SSRQuality::Medium,
            max_distance: 100.0,
            step_size: 0.1,
            thickness: 0.5,
            max_roughness: 0.5,
            edge_fade: 0.1,
            temporal: true,
            refinement_steps: 4,
        }
    }
}

/// SSR hit result
#[derive(Debug, Clone)]
pub struct SSRHit {
    /// UV coordinates of hit
    pub uv: Vec2,
    /// Confidence (0-1)
    pub confidence: f32,
    /// PDF for importance sampling
    pub pdf: f32,
}

/// Screen-space ray marching
pub struct ScreenSpaceReflections {
    /// Configuration
    pub config: SSRConfig,
    /// History buffer for temporal
    history_uv: Vec<Vec2>,
    /// History confidence
    history_confidence: Vec<f32>,
    /// Frame count
    frame: u64,
}

impl Default for ScreenSpaceReflections {
    fn default() -> Self {
        Self::new(SSRConfig::default())
    }
}

impl ScreenSpaceReflections {
    /// Create a new SSR system
    #[must_use]
    pub fn new(config: SSRConfig) -> Self {
        Self {
            config,
            history_uv: Vec::new(),
            history_confidence: Vec::new(),
            frame: 0,
        }
    }

    /// Ray march in screen space
    #[must_use]
    pub fn ray_march(
        &self,
        position: Vec3,
        normal: Vec3,
        view_dir: Vec3,
        roughness: f32,
        view_proj: Mat4,
        inv_view_proj: Mat4,
        depth_buffer: &[f32],
        width: u32,
        height: u32,
    ) -> Option<SSRHit> {
        if !self.config.enabled || roughness > self.config.max_roughness {
            return None;
        }

        // Calculate reflection direction
        let reflect_dir = view_dir - 2.0 * normal.dot(view_dir) * normal;
        
        // Start position in screen space
        let start_clip = view_proj * Vec4::new(position.x, position.y, position.z, 1.0);
        let start_ndc = start_clip.truncate() / start_clip.w;
        let mut ray_pos = position;

        let max_steps = self.config.quality.max_steps();
        
        for step in 0..max_steps {
            ray_pos += reflect_dir * self.config.step_size;
            
            // Project to screen
            let clip = view_proj * Vec4::new(ray_pos.x, ray_pos.y, ray_pos.z, 1.0);
            let ndc = clip.truncate() / clip.w;
            
            // Check if off screen
            if ndc.x < -1.0 || ndc.x > 1.0 || ndc.y < -1.0 || ndc.y > 1.0 || ndc.z < 0.0 {
                break;
            }
            
            // Convert to UV
            let uv = Vec2::new(ndc.x * 0.5 + 0.5, ndc.y * 0.5 + 0.5);
            
            // Sample depth buffer
            let x = (uv.x * width as f32) as usize;
            let y = (uv.y * height as f32) as usize;
            let idx = y * width as usize + x;
            
            if idx >= depth_buffer.len() {
                break;
            }
            
            let scene_depth = depth_buffer[idx];
            let ray_depth = ndc.z;
            
            // Check for intersection
            if ray_depth > scene_depth && ray_depth - scene_depth < self.config.thickness {
                // Binary search refinement
                let refined_uv = self.binary_search_refinement(
                    position,
                    reflect_dir,
                    step,
                    view_proj,
                    depth_buffer,
                    width,
                    height,
                );

                let confidence = self.calculate_confidence(refined_uv, roughness);
                
                return Some(SSRHit {
                    uv: refined_uv,
                    confidence,
                    pdf: 1.0,
                });
            }
        }

        None
    }

    fn binary_search_refinement(
        &self,
        start: Vec3,
        direction: Vec3,
        step: u32,
        view_proj: Mat4,
        depth_buffer: &[f32],
        width: u32,
        height: u32,
    ) -> Vec2 {
        let mut low = (step as f32 - 1.0) * self.config.step_size;
        let mut high = step as f32 * self.config.step_size;
        
        for _ in 0..self.config.refinement_steps {
            let mid = (low + high) * 0.5;
            let pos = start + direction * mid;
            
            let clip = view_proj * Vec4::new(pos.x, pos.y, pos.z, 1.0);
            let ndc = clip.truncate() / clip.w;
            let uv = Vec2::new(ndc.x * 0.5 + 0.5, ndc.y * 0.5 + 0.5);
            
            let x = (uv.x * width as f32) as usize;
            let y = (uv.y * height as f32) as usize;
            let idx = y * width as usize + x;
            
            if idx < depth_buffer.len() && ndc.z > depth_buffer[idx] {
                high = mid;
            } else {
                low = mid;
            }
        }
        
        let final_pos = start + direction * ((low + high) * 0.5);
        let clip = view_proj * Vec4::new(final_pos.x, final_pos.y, final_pos.z, 1.0);
        let ndc = clip.truncate() / clip.w;
        Vec2::new(ndc.x * 0.5 + 0.5, ndc.y * 0.5 + 0.5)
    }

    fn calculate_confidence(&self, uv: Vec2, roughness: f32) -> f32 {
        let mut confidence = 1.0;
        
        // Fade at screen edges
        let edge_x = (0.5 - (uv.x - 0.5).abs()) / self.config.edge_fade;
        let edge_y = (0.5 - (uv.y - 0.5).abs()) / self.config.edge_fade;
        confidence *= edge_x.clamp(0.0, 1.0) * edge_y.clamp(0.0, 1.0);
        
        // Fade with roughness
        let roughness_fade = 1.0 - (roughness / self.config.max_roughness);
        confidence *= roughness_fade.clamp(0.0, 1.0);
        
        confidence
    }

    /// Update temporal
    pub fn update(&mut self) {
        self.frame += 1;
    }
}

/// SSAO (Screen-Space Ambient Occlusion)
pub struct SSAO {
    /// Enabled
    pub enabled: bool,
    /// Radius
    pub radius: f32,
    /// Bias
    pub bias: f32,
    /// Intensity
    pub intensity: f32,
    /// Sample count
    pub samples: u32,
    /// Kernel samples
    kernel: Vec<Vec3>,
    /// Noise texture values
    noise: Vec<Vec3>,
}

impl Default for SSAO {
    fn default() -> Self {
        let mut ssao = Self {
            enabled: true,
            radius: 0.5,
            bias: 0.025,
            intensity: 1.0,
            samples: 64,
            kernel: Vec::new(),
            noise: Vec::new(),
        };
        ssao.generate_kernel();
        ssao.generate_noise();
        ssao
    }
}

impl SSAO {
    fn generate_kernel(&mut self) {
        self.kernel.clear();
        
        for i in 0..self.samples {
            // Random direction in hemisphere
            let x = (i as f32 * 12.9898).sin() * 43758.5453;
            let y = (i as f32 * 78.233).sin() * 43758.5453;
            let z = (i as f32 * 45.164).sin() * 43758.5453;
            
            let mut sample = Vec3::new(
                x.fract() * 2.0 - 1.0,
                y.fract() * 2.0 - 1.0,
                z.fract().abs(),
            ).normalize();
            
            // Scale to distribute more samples near origin
            let scale = i as f32 / self.samples as f32;
            let scale = 0.1 + scale * scale * 0.9;
            sample *= scale;
            
            self.kernel.push(sample);
        }
    }

    fn generate_noise(&mut self) {
        self.noise.clear();
        
        for i in 0..16 {
            let x = (i as f32 * 12.9898).sin() * 43758.5453;
            let y = (i as f32 * 78.233).sin() * 43758.5453;
            
            self.noise.push(Vec3::new(
                x.fract() * 2.0 - 1.0,
                y.fract() * 2.0 - 1.0,
                0.0,
            ).normalize());
        }
    }

    /// Calculate AO at a point
    #[must_use]
    pub fn calculate(
        &self,
        position: Vec3,
        normal: Vec3,
        view: Mat4,
        projection: Mat4,
        depth_buffer: &[f32],
        width: u32,
        height: u32,
    ) -> f32 {
        if !self.enabled {
            return 1.0;
        }

        let mut occlusion = 0.0;

        // Create TBN matrix
        let tangent = if normal.y.abs() < 0.99 {
            normal.cross(Vec3::Y).normalize()
        } else {
            normal.cross(Vec3::X).normalize()
        };
        let bitangent = normal.cross(tangent);

        for (i, sample) in self.kernel.iter().enumerate() {
            // Transform sample to world space
            let offset = tangent * sample.x + bitangent * sample.y + normal * sample.z;
            let sample_pos = position + offset * self.radius;

            // Project to screen
            let view_pos = view * Vec4::new(sample_pos.x, sample_pos.y, sample_pos.z, 1.0);
            let clip_pos = projection * view_pos;
            let ndc = clip_pos.truncate() / clip_pos.w;
            
            let uv = Vec2::new(ndc.x * 0.5 + 0.5, 1.0 - (ndc.y * 0.5 + 0.5));
            
            // Sample depth
            let x = (uv.x * width as f32) as usize;
            let y = (uv.y * height as f32) as usize;
            let idx = y * width as usize + x;
            
            if idx < depth_buffer.len() {
                let scene_depth = depth_buffer[idx];
                let sample_depth = ndc.z;
                
                // Range check
                let range_check = ((position.z - scene_depth).abs() / self.radius).clamp(0.0, 1.0);
                
                if sample_depth >= scene_depth + self.bias {
                    occlusion += range_check;
                }
            }
        }

        let ao = 1.0 - (occlusion / self.samples as f32) * self.intensity;
        ao.clamp(0.0, 1.0)
    }
}
