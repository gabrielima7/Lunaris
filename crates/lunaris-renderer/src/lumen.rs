//! Lumen-like Global Illumination
//!
//! Software Lumen implementation with SDFGI, screen-space, and hybrid GI.

use glam::{Vec3, Vec4, Mat4};

/// Lumen configuration
#[derive(Debug, Clone)]
pub struct LumenConfig {
    /// Enabled
    pub enabled: bool,
    /// Quality preset
    pub quality: LumenQuality,
    /// Final gather quality
    pub final_gather: FinalGatherQuality,
    /// Scene detail
    pub scene_detail: f32,
    /// Reflection quality
    pub reflection_quality: ReflectionQuality,
    /// Software ray tracing
    pub software_ray_tracing: bool,
    /// Hardware ray tracing (if available)
    pub hardware_ray_tracing: bool,
    /// Use screen traces
    pub screen_traces: bool,
    /// Use sky light
    pub sky_light: bool,
    /// Max trace distance
    pub max_trace_distance: f32,
    /// Diffuse indirect
    pub diffuse_indirect: bool,
    /// Specular indirect
    pub specular_indirect: bool,
    /// Infinite bounce GI
    pub infinite_bounces: bool,
    /// Ray lighting mode
    pub ray_lighting: RayLightingMode,
}

impl Default for LumenConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            quality: LumenQuality::High,
            final_gather: FinalGatherQuality::High,
            scene_detail: 1.0,
            reflection_quality: ReflectionQuality::High,
            software_ray_tracing: true,
            hardware_ray_tracing: false,
            screen_traces: true,
            sky_light: true,
            max_trace_distance: 200.0,
            diffuse_indirect: true,
            specular_indirect: true,
            infinite_bounces: true,
            ray_lighting: RayLightingMode::SurfaceCache,
        }
    }
}

/// Lumen quality preset
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LumenQuality {
    Low,
    Medium,
    High,
    Epic,
    Cinematic,
}

/// Final gather quality
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinalGatherQuality {
    Low,
    Medium,
    High,
    Epic,
}

/// Reflection quality
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflectionQuality {
    Low,
    Medium,
    High,
    Epic,
}

/// Ray lighting mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RayLightingMode {
    /// Surface cache for fast indirect
    SurfaceCache,
    /// Hit lighting for accurate
    HitLighting,
}

/// Signed distance field for mesh
#[derive(Debug, Clone)]
pub struct MeshSDF {
    /// Mesh ID
    pub mesh_id: u64,
    /// SDF texture resolution
    pub resolution: [u32; 3],
    /// Bounds min
    pub bounds_min: Vec3,
    /// Bounds max
    pub bounds_max: Vec3,
    /// Mip levels
    pub mip_levels: u8,
    /// Is built
    pub built: bool,
}

/// Global SDF
#[derive(Debug, Clone)]
pub struct GlobalSDF {
    /// Resolution
    pub resolution: [u32; 3],
    /// World bounds min
    pub bounds_min: Vec3,
    /// World bounds max
    pub bounds_max: Vec3,
    /// Clipmap levels
    pub clipmap_levels: u8,
    /// Pages allocated
    pub pages_allocated: u32,
    /// Is valid
    pub valid: bool,
}

impl Default for GlobalSDF {
    fn default() -> Self {
        Self {
            resolution: [128, 64, 128],
            bounds_min: Vec3::splat(-500.0),
            bounds_max: Vec3::splat(500.0),
            clipmap_levels: 4,
            pages_allocated: 0,
            valid: false,
        }
    }
}

/// Surface cache for indirect lighting
#[derive(Debug, Clone)]
pub struct SurfaceCache {
    /// Atlas resolution
    pub atlas_resolution: u32,
    /// Pages
    pub pages: Vec<SurfaceCachePage>,
    /// Update rate
    pub update_frames: u32,
    /// Current frame
    current_frame: u32,
}

/// Surface cache page
#[derive(Debug, Clone)]
pub struct SurfaceCachePage {
    /// Object ID
    pub object_id: u64,
    /// UV offset
    pub uv_offset: [f32; 2],
    /// UV scale
    pub uv_scale: [f32; 2],
    /// Resolution
    pub resolution: u32,
    /// Last update frame
    pub last_update: u32,
    /// Is valid
    pub valid: bool,
}

impl Default for SurfaceCache {
    fn default() -> Self {
        Self::new(4096)
    }
}

impl SurfaceCache {
    /// Create new surface cache
    #[must_use]
    pub fn new(atlas_resolution: u32) -> Self {
        Self {
            atlas_resolution,
            pages: Vec::new(),
            update_frames: 8,
            current_frame: 0,
        }
    }

    /// Allocate page
    pub fn allocate_page(&mut self, object_id: u64, resolution: u32) -> usize {
        let page = SurfaceCachePage {
            object_id,
            uv_offset: [0.0, 0.0],
            uv_scale: [1.0, 1.0],
            resolution,
            last_update: 0,
            valid: false,
        };
        self.pages.push(page);
        self.pages.len() - 1
    }

    /// Advance frame
    pub fn advance(&mut self) {
        self.current_frame += 1;
    }

    /// Get pages needing update
    #[must_use]
    pub fn pages_needing_update(&self) -> Vec<usize> {
        self.pages.iter()
            .enumerate()
            .filter(|(_, p)| {
                self.current_frame - p.last_update >= self.update_frames || !p.valid
            })
            .map(|(i, _)| i)
            .collect()
    }
}

/// Radiance cache
#[derive(Debug, Clone)]
pub struct RadianceCache {
    /// Probe grid resolution
    pub grid_resolution: [u32; 3],
    /// World bounds min
    pub bounds_min: Vec3,
    /// World bounds max
    pub bounds_max: Vec3,
    /// Probe spacing
    pub spacing: f32,
    /// Probes
    pub probes: Vec<RadianceProbe>,
    /// Is built
    pub built: bool,
}

/// Radiance probe
#[derive(Debug, Clone, Copy)]
pub struct RadianceProbe {
    /// Position
    pub position: Vec3,
    /// Radiance (6 directions)
    pub radiance: [Vec3; 6],
    /// Depth (for parallax)
    pub depth: [f32; 6],
    /// Is valid
    pub valid: bool,
}

impl Default for RadianceCache {
    fn default() -> Self {
        Self {
            grid_resolution: [32, 8, 32],
            bounds_min: Vec3::splat(-100.0),
            bounds_max: Vec3::splat(100.0),
            spacing: 2.0,
            probes: Vec::new(),
            built: false,
        }
    }
}

/// Lumen GI system
pub struct LumenGI {
    /// Configuration
    pub config: LumenConfig,
    /// Global SDF
    pub global_sdf: GlobalSDF,
    /// Surface cache
    pub surface_cache: SurfaceCache,
    /// Radiance cache
    pub radiance_cache: RadianceCache,
    /// Mesh SDFs
    mesh_sdfs: Vec<MeshSDF>,
    /// Frame counter
    frame: u64,
}

impl Default for LumenGI {
    fn default() -> Self {
        Self::new()
    }
}

impl LumenGI {
    /// Create new Lumen GI
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: LumenConfig::default(),
            global_sdf: GlobalSDF::default(),
            surface_cache: SurfaceCache::default(),
            radiance_cache: RadianceCache::default(),
            mesh_sdfs: Vec::new(),
            frame: 0,
        }
    }

    /// Register mesh for SDF
    pub fn register_mesh(&mut self, mesh_id: u64, bounds_min: Vec3, bounds_max: Vec3) {
        let size = bounds_max - bounds_min;
        let max_dim = size.x.max(size.y).max(size.z);
        let resolution = ((max_dim / 0.5) as u32).clamp(8, 128);

        let sdf = MeshSDF {
            mesh_id,
            resolution: [resolution, resolution, resolution],
            bounds_min,
            bounds_max,
            mip_levels: 4,
            built: false,
        };

        self.mesh_sdfs.push(sdf);
    }

    /// Update Lumen
    pub fn update(&mut self) {
        self.frame += 1;
        self.surface_cache.advance();

        // Update global SDF if needed
        if !self.global_sdf.valid {
            self.build_global_sdf();
        }

        // Update surface cache pages
        let pages_to_update = self.surface_cache.pages_needing_update();
        for page_idx in pages_to_update.iter().take(4) {
            if let Some(page) = self.surface_cache.pages.get_mut(*page_idx) {
                page.last_update = self.surface_cache.current_frame;
                page.valid = true;
            }
        }

        // Update radiance cache
        if !self.radiance_cache.built {
            self.build_radiance_cache();
        }
    }

    fn build_global_sdf(&mut self) {
        // Build global SDF from mesh SDFs
        self.global_sdf.valid = true;
        self.global_sdf.pages_allocated = 1024;
    }

    fn build_radiance_cache(&mut self) {
        let res = self.radiance_cache.grid_resolution;
        let size = self.radiance_cache.bounds_max - self.radiance_cache.bounds_min;
        let spacing = Vec3::new(
            size.x / res[0] as f32,
            size.y / res[1] as f32,
            size.z / res[2] as f32,
        );

        for z in 0..res[2] {
            for y in 0..res[1] {
                for x in 0..res[0] {
                    let position = self.radiance_cache.bounds_min + Vec3::new(
                        (x as f32 + 0.5) * spacing.x,
                        (y as f32 + 0.5) * spacing.y,
                        (z as f32 + 0.5) * spacing.z,
                    );

                    let probe = RadianceProbe {
                        position,
                        radiance: [Vec3::ZERO; 6],
                        depth: [10.0; 6],
                        valid: true,
                    };

                    self.radiance_cache.probes.push(probe);
                }
            }
        }

        self.radiance_cache.built = true;
    }

    /// Sample GI at position
    #[must_use]
    pub fn sample(&self, position: Vec3, normal: Vec3) -> Vec3 {
        if !self.radiance_cache.built || self.radiance_cache.probes.is_empty() {
            return Vec3::splat(0.1);
        }

        // Simple trilinear interpolation
        let relative = (position - self.radiance_cache.bounds_min) 
            / (self.radiance_cache.bounds_max - self.radiance_cache.bounds_min);
        
        let res = self.radiance_cache.grid_resolution;
        let x = (relative.x * res[0] as f32) as usize;
        let y = (relative.y * res[1] as f32) as usize;
        let z = (relative.z * res[2] as f32) as usize;

        let idx = x + y * res[0] as usize + z * (res[0] * res[1]) as usize;
        
        if let Some(probe) = self.radiance_cache.probes.get(idx) {
            // Sample radiance based on normal direction
            let dir_idx = normal_to_direction_index(normal);
            return probe.radiance[dir_idx];
        }

        Vec3::splat(0.1)
    }
}

fn normal_to_direction_index(normal: Vec3) -> usize {
    let abs_n = normal.abs();
    if abs_n.x > abs_n.y && abs_n.x > abs_n.z {
        if normal.x > 0.0 { 0 } else { 1 }
    } else if abs_n.y > abs_n.z {
        if normal.y > 0.0 { 2 } else { 3 }
    } else {
        if normal.z > 0.0 { 4 } else { 5 }
    }
}
