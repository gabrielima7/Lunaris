//! GPU Instancing System
//!
//! Efficient rendering of many objects with hardware instancing.

use glam::{Vec3, Vec4, Mat4};
use std::collections::HashMap;

/// Instance data for GPU
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct InstanceData {
    /// Model matrix (4x4 = 16 floats)
    pub model_matrix: [[f32; 4]; 4],
    /// Instance color/tint
    pub color: [f32; 4],
    /// Custom data (material ID, animation frame, etc.)
    pub custom: [f32; 4],
}

impl Default for InstanceData {
    fn default() -> Self {
        Self {
            model_matrix: Mat4::IDENTITY.to_cols_array_2d(),
            color: [1.0, 1.0, 1.0, 1.0],
            custom: [0.0; 4],
        }
    }
}

impl InstanceData {
    /// Create from transform
    #[must_use]
    pub fn from_transform(position: Vec3, rotation: glam::Quat, scale: Vec3) -> Self {
        let mat = Mat4::from_scale_rotation_translation(scale, rotation, position);
        Self {
            model_matrix: mat.to_cols_array_2d(),
            color: [1.0, 1.0, 1.0, 1.0],
            custom: [0.0; 4],
        }
    }

    /// Set color
    #[must_use]
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color.to_array();
        self
    }

    /// Set custom data
    #[must_use]
    pub fn with_custom(mut self, custom: [f32; 4]) -> Self {
        self.custom = custom;
        self
    }
}

/// Instance batch
pub struct InstanceBatch {
    /// Mesh ID
    pub mesh_id: u64,
    /// Material ID
    pub material_id: u64,
    /// Instance data
    pub instances: Vec<InstanceData>,
    /// Is dirty (needs buffer update)
    pub dirty: bool,
    /// Max instances (buffer capacity)
    pub capacity: usize,
}

impl InstanceBatch {
    /// Create a new batch
    #[must_use]
    pub fn new(mesh_id: u64, material_id: u64, capacity: usize) -> Self {
        Self {
            mesh_id,
            material_id,
            instances: Vec::with_capacity(capacity),
            dirty: true,
            capacity,
        }
    }

    /// Add instance
    pub fn add(&mut self, instance: InstanceData) {
        if self.instances.len() < self.capacity {
            self.instances.push(instance);
            self.dirty = true;
        }
    }

    /// Clear all instances
    pub fn clear(&mut self) {
        self.instances.clear();
        self.dirty = true;
    }

    /// Instance count
    #[must_use]
    pub fn count(&self) -> usize {
        self.instances.len()
    }

    /// Is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }

    /// Get data as bytes for GPU upload
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.instances)
    }
}

/// Instance manager
pub struct InstanceManager {
    /// Batches by key (mesh_id, material_id)
    batches: HashMap<(u64, u64), InstanceBatch>,
    /// Default capacity
    pub default_capacity: usize,
}

impl Default for InstanceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl InstanceManager {
    /// Create a new manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            batches: HashMap::new(),
            default_capacity: 1000,
        }
    }

    /// Add an instance
    pub fn add(&mut self, mesh_id: u64, material_id: u64, instance: InstanceData) {
        let batch = self.batches.entry((mesh_id, material_id))
            .or_insert_with(|| InstanceBatch::new(mesh_id, material_id, self.default_capacity));
        batch.add(instance);
    }

    /// Get or create batch
    pub fn get_batch(&mut self, mesh_id: u64, material_id: u64) -> &mut InstanceBatch {
        self.batches.entry((mesh_id, material_id))
            .or_insert_with(|| InstanceBatch::new(mesh_id, material_id, self.default_capacity))
    }

    /// Clear all batches
    pub fn clear(&mut self) {
        for batch in self.batches.values_mut() {
            batch.clear();
        }
    }

    /// Get all batches for rendering
    #[must_use]
    pub fn batches(&self) -> Vec<&InstanceBatch> {
        self.batches.values().filter(|b| !b.is_empty()).collect()
    }

    /// Total instance count
    #[must_use]
    pub fn total_instances(&self) -> usize {
        self.batches.values().map(|b| b.count()).sum()
    }
}

/// Indirect draw command (for GPU-driven rendering)
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct DrawIndirectCommand {
    /// Vertex count per instance
    pub vertex_count: u32,
    /// Instance count
    pub instance_count: u32,
    /// First vertex
    pub first_vertex: u32,
    /// First instance
    pub first_instance: u32,
}

/// Indexed indirect draw command
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct DrawIndexedIndirectCommand {
    /// Index count per instance
    pub index_count: u32,
    /// Instance count
    pub instance_count: u32,
    /// First index
    pub first_index: u32,
    /// Base vertex
    pub base_vertex: i32,
    /// First instance
    pub first_instance: u32,
}
