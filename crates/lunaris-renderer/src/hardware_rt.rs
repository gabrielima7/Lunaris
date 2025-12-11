//! Hardware Ray Tracing
//!
//! GPU-accelerated ray tracing using DXR, Vulkan RT, and Metal RT.

use glam::{Vec3, Mat4};
use std::collections::HashMap;

/// Hardware RT backend
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareRTBackend {
    /// DirectX Raytracing (Windows, Xbox)
    DXR,
    /// Vulkan Ray Tracing
    VulkanRT,
    /// Metal Ray Tracing (Apple)
    MetalRT,
    /// PlayStation RT
    PSRT,
    /// Software fallback
    Software,
}

/// Ray tracing tier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum RTTier {
    /// No RT support
    None = 0,
    /// Basic RT (shadows only)
    Tier1 = 1,
    /// Mid RT (shadows + reflections)
    Tier2 = 2,
    /// Full RT (shadows + reflections + GI)
    Tier3 = 3,
    /// Path tracing capable
    Tier4 = 4,
}

/// Bottom Level Acceleration Structure (mesh)
#[derive(Debug, Clone)]
pub struct BLAS {
    /// Unique ID
    pub id: u64,
    /// Mesh ID reference
    pub mesh_id: u64,
    /// Flags
    pub flags: BLASFlags,
    /// Is built
    pub built: bool,
    /// Memory size (bytes)
    pub memory_bytes: u64,
    /// Primitive count
    pub primitive_count: u32,
}

/// BLAS build flags
#[derive(Debug, Clone, Copy, Default)]
pub struct BLASFlags {
    /// Allow updates (for animated meshes)
    pub allow_update: bool,
    /// Allow compaction
    pub allow_compaction: bool,
    /// Prefer fast trace
    pub prefer_fast_trace: bool,
    /// Prefer fast build
    pub prefer_fast_build: bool,
    /// Low memory
    pub low_memory: bool,
}

/// Top Level Acceleration Structure (scene)
#[derive(Debug, Clone)]
pub struct TLAS {
    /// Unique ID
    pub id: u64,
    /// Instances
    pub instances: Vec<TLASInstance>,
    /// Is built
    pub built: bool,
    /// Memory size (bytes)
    pub memory_bytes: u64,
}

/// TLAS instance
#[derive(Debug, Clone)]
pub struct TLASInstance {
    /// BLAS reference
    pub blas_id: u64,
    /// Transform
    pub transform: Mat4,
    /// Instance ID (custom data)
    pub instance_id: u32,
    /// Visibility mask
    pub visibility_mask: u8,
    /// Flags
    pub flags: TLASInstanceFlags,
    /// Shader binding table offset
    pub sbt_offset: u32,
}

/// TLAS instance flags
#[derive(Debug, Clone, Copy, Default)]
pub struct TLASInstanceFlags {
    /// Force opaque
    pub force_opaque: bool,
    /// Force no opaque
    pub force_no_opaque: bool,
    /// Disable triangle cull
    pub disable_triangle_cull: bool,
    /// Front CCW
    pub front_ccw: bool,
}

/// Ray tracing pipeline
#[derive(Debug, Clone)]
pub struct RTPipeline {
    /// Pipeline ID
    pub id: u64,
    /// Ray generation shader
    pub raygen_shader: u64,
    /// Miss shaders
    pub miss_shaders: Vec<u64>,
    /// Hit groups
    pub hit_groups: Vec<RTHitGroup>,
    /// Max recursion depth
    pub max_recursion: u32,
    /// Max payload size
    pub max_payload_size: u32,
    /// Max attribute size
    pub max_attribute_size: u32,
}

/// Ray tracing hit group
#[derive(Debug, Clone)]
pub struct RTHitGroup {
    /// Closest hit shader
    pub closest_hit: Option<u64>,
    /// Any hit shader
    pub any_hit: Option<u64>,
    /// Intersection shader (procedural)
    pub intersection: Option<u64>,
    /// Type
    pub hit_type: HitGroupType,
}

/// Hit group type
#[derive(Debug, Clone, Copy)]
pub enum HitGroupType {
    /// Triangle geometry
    Triangles,
    /// Procedural geometry
    Procedural,
}

/// Shader Binding Table
#[derive(Debug, Clone)]
pub struct ShaderBindingTable {
    /// Ray gen entries
    pub raygen_entries: Vec<SBTEntry>,
    /// Miss entries
    pub miss_entries: Vec<SBTEntry>,
    /// Hit group entries
    pub hit_entries: Vec<SBTEntry>,
    /// Callable entries
    pub callable_entries: Vec<SBTEntry>,
}

/// SBT Entry
#[derive(Debug, Clone)]
pub struct SBTEntry {
    /// Shader ID
    pub shader_id: u64,
    /// Inline data
    pub data: Vec<u8>,
}

/// Hardware RT manager
pub struct HardwareRTManager {
    /// Backend
    pub backend: HardwareRTBackend,
    /// Tier
    pub tier: RTTier,
    /// BLAS registry
    blas_registry: HashMap<u64, BLAS>,
    /// TLAS registry
    tlas_registry: HashMap<u64, TLAS>,
    /// Pipeline registry
    pipelines: HashMap<u64, RTPipeline>,
    /// Next ID
    next_id: u64,
    /// Max recursion supported
    pub max_recursion_depth: u32,
    /// Ray query support
    pub ray_query_support: bool,
}

impl HardwareRTManager {
    /// Create new manager
    #[must_use]
    pub fn new(backend: HardwareRTBackend) -> Self {
        let tier = match backend {
            HardwareRTBackend::DXR | HardwareRTBackend::VulkanRT => RTTier::Tier4,
            HardwareRTBackend::MetalRT => RTTier::Tier3,
            HardwareRTBackend::PSRT => RTTier::Tier3,
            HardwareRTBackend::Software => RTTier::Tier2,
        };

        Self {
            backend,
            tier,
            blas_registry: HashMap::new(),
            tlas_registry: HashMap::new(),
            pipelines: HashMap::new(),
            next_id: 1,
            max_recursion_depth: 31,
            ray_query_support: true,
        }
    }

    /// Create BLAS
    pub fn create_blas(&mut self, mesh_id: u64, flags: BLASFlags) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let blas = BLAS {
            id,
            mesh_id,
            flags,
            built: false,
            memory_bytes: 0,
            primitive_count: 0,
        };

        self.blas_registry.insert(id, blas);
        id
    }

    /// Build BLAS
    pub fn build_blas(&mut self, id: u64, primitive_count: u32) {
        if let Some(blas) = self.blas_registry.get_mut(&id) {
            blas.built = true;
            blas.primitive_count = primitive_count;
            // Estimate memory: ~64 bytes per primitive for BVH
            blas.memory_bytes = primitive_count as u64 * 64;
        }
    }

    /// Create TLAS
    pub fn create_tlas(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let tlas = TLAS {
            id,
            instances: Vec::new(),
            built: false,
            memory_bytes: 0,
        };

        self.tlas_registry.insert(id, tlas);
        id
    }

    /// Add instance to TLAS
    pub fn add_instance(&mut self, tlas_id: u64, instance: TLASInstance) {
        if let Some(tlas) = self.tlas_registry.get_mut(&tlas_id) {
            tlas.instances.push(instance);
            tlas.built = false;
        }
    }

    /// Build TLAS
    pub fn build_tlas(&mut self, id: u64) {
        if let Some(tlas) = self.tlas_registry.get_mut(&id) {
            tlas.built = true;
            // Estimate: ~128 bytes per instance
            tlas.memory_bytes = tlas.instances.len() as u64 * 128;
        }
    }

    /// Create RT pipeline
    pub fn create_pipeline(&mut self, pipeline: RTPipeline) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.pipelines.insert(id, pipeline);
        id
    }

    /// Get total acceleration structure memory
    #[must_use]
    pub fn total_as_memory(&self) -> u64 {
        let blas_mem: u64 = self.blas_registry.values().map(|b| b.memory_bytes).sum();
        let tlas_mem: u64 = self.tlas_registry.values().map(|t| t.memory_bytes).sum();
        blas_mem + tlas_mem
    }

    /// Get BLAS count
    #[must_use]
    pub fn blas_count(&self) -> usize {
        self.blas_registry.len()
    }

    /// Get total instance count
    #[must_use]
    pub fn instance_count(&self) -> usize {
        self.tlas_registry.values().map(|t| t.instances.len()).sum()
    }
}

/// Path tracer configuration
#[derive(Debug, Clone)]
pub struct PathTracerConfig {
    /// Samples per pixel
    pub spp: u32,
    /// Max bounces
    pub max_bounces: u32,
    /// Russian roulette start depth
    pub rr_start_depth: u32,
    /// Accumulation enabled
    pub accumulate: bool,
    /// Denoiser enabled
    pub denoise: bool,
    /// Firefly filter
    pub firefly_filter: bool,
    /// Adaptive sampling
    pub adaptive: bool,
    /// Variance threshold
    pub variance_threshold: f32,
}

impl Default for PathTracerConfig {
    fn default() -> Self {
        Self {
            spp: 1,
            max_bounces: 8,
            rr_start_depth: 3,
            accumulate: true,
            denoise: true,
            firefly_filter: true,
            adaptive: true,
            variance_threshold: 0.01,
        }
    }
}

/// Real-time path tracer
pub struct PathTracer {
    /// Configuration
    pub config: PathTracerConfig,
    /// Current sample count
    sample_count: u32,
    /// Camera changed flag
    camera_dirty: bool,
    /// Accumulated samples
    accumulated_samples: u32,
}

impl Default for PathTracer {
    fn default() -> Self {
        Self::new(PathTracerConfig::default())
    }
}

impl PathTracer {
    /// Create new path tracer
    #[must_use]
    pub fn new(config: PathTracerConfig) -> Self {
        Self {
            config,
            sample_count: 0,
            camera_dirty: true,
            accumulated_samples: 0,
        }
    }

    /// Reset accumulation
    pub fn reset(&mut self) {
        self.accumulated_samples = 0;
        self.camera_dirty = true;
    }

    /// Notify camera changed
    pub fn camera_moved(&mut self) {
        self.camera_dirty = true;
        if !self.config.accumulate {
            self.accumulated_samples = 0;
        }
    }

    /// Get accumulated samples
    #[must_use]
    pub fn accumulated(&self) -> u32 {
        self.accumulated_samples
    }

    /// Should continue accumulating
    #[must_use]
    pub fn should_accumulate(&self) -> bool {
        self.config.accumulate && !self.camera_dirty
    }

    /// Advance frame
    pub fn advance(&mut self) {
        if self.should_accumulate() {
            self.accumulated_samples += self.config.spp;
        } else {
            self.accumulated_samples = self.config.spp;
            self.camera_dirty = false;
        }
    }
}
