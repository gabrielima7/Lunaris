//! Nanite-like Virtualized Geometry System
//!
//! GPU-driven mesh rendering with automatic LOD and streaming.

use glam::{Vec3, Mat4};
use std::collections::HashMap;

/// Cluster - Basic unit of virtualized geometry
#[derive(Debug, Clone)]
pub struct MeshCluster {
    /// Cluster ID
    pub id: u64,
    /// Mesh ID
    pub mesh_id: u64,
    /// LOD level
    pub lod_level: u8,
    /// Bounding sphere center
    pub bounds_center: Vec3,
    /// Bounding sphere radius
    pub bounds_radius: f32,
    /// Parent cluster (for hierarchy)
    pub parent: Option<u64>,
    /// Child clusters
    pub children: Vec<u64>,
    /// Triangle count
    pub triangle_count: u32,
    /// Vertex offset in buffer
    pub vertex_offset: u32,
    /// Index offset in buffer
    pub index_offset: u32,
    /// Error metric for LOD selection
    pub lod_error: f32,
    /// Is leaf cluster
    pub is_leaf: bool,
}

/// Virtualized mesh
#[derive(Debug, Clone)]
pub struct VirtualizedMesh {
    /// Mesh ID
    pub id: u64,
    /// Name
    pub name: String,
    /// All clusters
    pub clusters: Vec<MeshCluster>,
    /// LOD count
    pub lod_count: u8,
    /// Total triangle count (full LOD)
    pub total_triangles: u32,
    /// Bounding box min
    pub bounds_min: Vec3,
    /// Bounding box max
    pub bounds_max: Vec3,
    /// Is streamed
    pub streamed: bool,
    /// Memory resident (bytes)
    pub resident_memory: u64,
    /// Total memory (bytes)
    pub total_memory: u64,
}

/// Instance of virtualized mesh
#[derive(Debug, Clone)]
pub struct VirtualInstance {
    /// Instance ID
    pub id: u64,
    /// Mesh ID
    pub mesh_id: u64,
    /// Transform
    pub transform: Mat4,
    /// Visibility flags
    pub visible: bool,
    /// Force LOD (-1 = auto)
    pub force_lod: i8,
    /// Custom data
    pub custom_id: u32,
}

/// Cluster culling result
#[derive(Debug, Clone, Copy)]
pub enum ClusterVisibility {
    /// Fully visible
    Visible,
    /// Not visible (culled)
    Culled,
    /// Parent visible instead
    OccludedByParent,
    /// Unknown (needs test)
    Unknown,
}

/// GPU-driven culling pass
#[derive(Debug, Clone)]
pub struct CullingPass {
    /// Frustum culling enabled
    pub frustum_cull: bool,
    /// Occlusion culling enabled
    pub occlusion_cull: bool,
    /// Two-pass occlusion (HZB)
    pub two_pass_occlusion: bool,
    /// LOD selection enabled
    pub lod_selection: bool,
    /// Error threshold for LOD
    pub error_threshold: f32,
    /// Max screen error (pixels)
    pub max_screen_error: f32,
    /// Visible clusters output
    visible_clusters: Vec<u64>,
    /// Total clusters tested
    pub clusters_tested: u32,
    /// Clusters passed
    pub clusters_passed: u32,
    /// Triangles rendered
    pub triangles_rendered: u64,
}

impl Default for CullingPass {
    fn default() -> Self {
        Self {
            frustum_cull: true,
            occlusion_cull: true,
            two_pass_occlusion: true,
            lod_selection: true,
            error_threshold: 1.0,
            max_screen_error: 1.0,
            visible_clusters: Vec::new(),
            clusters_tested: 0,
            clusters_passed: 0,
            triangles_rendered: 0,
        }
    }
}

impl CullingPass {
    /// Calculate screen error for cluster
    #[must_use]
    pub fn calculate_screen_error(
        &self,
        cluster_radius: f32,
        cluster_error: f32,
        distance: f32,
        screen_height: f32,
        fov: f32,
    ) -> f32 {
        if distance < 0.001 {
            return f32::MAX;
        }
        
        let projected_size = (cluster_radius / distance) * screen_height / fov.tan();
        projected_size * cluster_error
    }

    /// Should use parent LOD
    #[must_use]
    pub fn should_use_parent(&self, screen_error: f32) -> bool {
        screen_error < self.max_screen_error
    }

    /// Get visible clusters
    #[must_use]
    pub fn visible_clusters(&self) -> &[u64] {
        &self.visible_clusters
    }

    /// Get cull efficiency
    #[must_use]
    pub fn cull_efficiency(&self) -> f32 {
        if self.clusters_tested == 0 {
            return 0.0;
        }
        1.0 - (self.clusters_passed as f32 / self.clusters_tested as f32)
    }
}

/// Streaming priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StreamingPriority {
    /// Critical - visible now
    Critical = 0,
    /// High - will be visible soon
    High = 1,
    /// Normal - might be visible
    Normal = 2,
    /// Low - background loading
    Low = 3,
    /// Unload candidate
    Unload = 4,
}

/// Streaming request
#[derive(Debug, Clone)]
pub struct StreamingRequest {
    /// Mesh ID
    pub mesh_id: u64,
    /// Target LOD
    pub target_lod: u8,
    /// Priority
    pub priority: StreamingPriority,
    /// Distance to camera
    pub distance: f32,
}

/// Virtualized geometry manager
pub struct VirtualGeometryManager {
    /// All meshes
    meshes: HashMap<u64, VirtualizedMesh>,
    /// All instances
    instances: HashMap<u64, VirtualInstance>,
    /// Culling pass
    pub culling: CullingPass,
    /// Streaming requests
    streaming_queue: Vec<StreamingRequest>,
    /// Next ID
    next_id: u64,
    /// Memory budget (bytes)
    pub memory_budget: u64,
    /// Current memory usage
    pub current_memory: u64,
    /// GPU-driven enabled
    pub gpu_driven: bool,
    /// Mesh shader support
    pub mesh_shaders: bool,
}

impl Default for VirtualGeometryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualGeometryManager {
    /// Create new manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            meshes: HashMap::new(),
            instances: HashMap::new(),
            culling: CullingPass::default(),
            streaming_queue: Vec::new(),
            next_id: 1,
            memory_budget: 4 * 1024 * 1024 * 1024, // 4GB
            current_memory: 0,
            gpu_driven: true,
            mesh_shaders: true,
        }
    }

    /// Register mesh
    pub fn register_mesh(&mut self, mesh: VirtualizedMesh) -> u64 {
        let id = mesh.id;
        self.meshes.insert(id, mesh);
        id
    }

    /// Create instance
    pub fn create_instance(&mut self, mesh_id: u64, transform: Mat4) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let instance = VirtualInstance {
            id,
            mesh_id,
            transform,
            visible: true,
            force_lod: -1,
            custom_id: 0,
        };

        self.instances.insert(id, instance);
        id
    }

    /// Update instance transform
    pub fn set_transform(&mut self, instance_id: u64, transform: Mat4) {
        if let Some(instance) = self.instances.get_mut(&instance_id) {
            instance.transform = transform;
        }
    }

    /// Queue streaming request
    pub fn request_streaming(&mut self, mesh_id: u64, target_lod: u8, distance: f32) {
        let priority = if distance < 10.0 {
            StreamingPriority::Critical
        } else if distance < 50.0 {
            StreamingPriority::High
        } else if distance < 200.0 {
            StreamingPriority::Normal
        } else {
            StreamingPriority::Low
        };

        self.streaming_queue.push(StreamingRequest {
            mesh_id,
            target_lod,
            priority,
            distance,
        });
    }

    /// Process streaming queue
    pub fn process_streaming(&mut self, max_bytes_per_frame: u64) {
        // Sort by priority
        self.streaming_queue.sort_by(|a, b| {
            a.priority.cmp(&b.priority)
                .then(a.distance.partial_cmp(&b.distance).unwrap_or(std::cmp::Ordering::Equal))
        });

        let mut bytes_loaded = 0u64;

        for request in &self.streaming_queue {
            if bytes_loaded >= max_bytes_per_frame {
                break;
            }

            if let Some(mesh) = self.meshes.get_mut(&request.mesh_id) {
                // Simulate loading
                let bytes_needed = mesh.total_memory / (mesh.lod_count as u64 + 1);
                if bytes_loaded + bytes_needed <= max_bytes_per_frame {
                    mesh.resident_memory = mesh.resident_memory.saturating_add(bytes_needed);
                    mesh.streamed = true;
                    bytes_loaded += bytes_needed;
                    self.current_memory += bytes_needed;
                }
            }
        }

        self.streaming_queue.clear();

        // Unload if over budget
        self.evict_if_needed();
    }

    fn evict_if_needed(&mut self) {
        while self.current_memory > self.memory_budget {
            // Find mesh to evict (furthest, lowest priority)
            let to_evict = self.meshes.values()
                .filter(|m| m.streamed && m.resident_memory > 0)
                .min_by(|a, b| a.resident_memory.cmp(&b.resident_memory))
                .map(|m| m.id);

            if let Some(id) = to_evict {
                if let Some(mesh) = self.meshes.get_mut(&id) {
                    self.current_memory -= mesh.resident_memory;
                    mesh.resident_memory = 0;
                    mesh.streamed = false;
                }
            } else {
                break;
            }
        }
    }

    /// Get statistics
    #[must_use]
    pub fn stats(&self) -> VirtualGeometryStats {
        let total_triangles: u64 = self.meshes.values()
            .map(|m| m.total_triangles as u64)
            .sum();

        VirtualGeometryStats {
            mesh_count: self.meshes.len(),
            instance_count: self.instances.len(),
            total_triangles,
            rendered_triangles: self.culling.triangles_rendered,
            memory_used: self.current_memory,
            memory_budget: self.memory_budget,
            cull_efficiency: self.culling.cull_efficiency(),
        }
    }
}

/// Statistics
#[derive(Debug, Clone)]
pub struct VirtualGeometryStats {
    /// Mesh count
    pub mesh_count: usize,
    /// Instance count
    pub instance_count: usize,
    /// Total triangles
    pub total_triangles: u64,
    /// Rendered triangles
    pub rendered_triangles: u64,
    /// Memory used
    pub memory_used: u64,
    /// Memory budget
    pub memory_budget: u64,
    /// Cull efficiency (0-1)
    pub cull_efficiency: f32,
}
