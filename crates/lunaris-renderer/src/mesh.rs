//! 3D Mesh and Model System
//!
//! Handles 3D mesh loading, rendering, and model management.

use crate::gpu::Vertex3D;
use lunaris_core::{
    id::{Id, TypedId},
    math::{Color, Vec2, Vec3},
    Result,
};
use std::collections::HashMap;

/// Mesh identifier
pub type MeshId = TypedId<Mesh>;

/// A 3D mesh
#[derive(Debug, Clone)]
pub struct Mesh {
    /// Mesh ID
    pub id: MeshId,
    /// Mesh name
    pub name: String,
    /// Vertices
    pub vertices: Vec<Vertex3D>,
    /// Indices
    pub indices: Vec<u32>,
    /// Bounding box min
    pub bounds_min: Vec3,
    /// Bounding box max
    pub bounds_max: Vec3,
    /// Submeshes
    pub submeshes: Vec<SubMesh>,
}

/// A submesh (part of a mesh with its own material)
#[derive(Debug, Clone)]
pub struct SubMesh {
    /// Start index
    pub start_index: u32,
    /// Index count
    pub index_count: u32,
    /// Material index
    pub material_index: u32,
}

impl Mesh {
    /// Create an empty mesh
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: MeshId::new(),
            name: name.into(),
            vertices: Vec::new(),
            indices: Vec::new(),
            bounds_min: Vec3::ZERO,
            bounds_max: Vec3::ZERO,
            submeshes: Vec::new(),
        }
    }

    /// Create a plane mesh
    #[must_use]
    pub fn plane(size: f32) -> Self {
        let half = size / 2.0;
        let vertices = vec![
            Vertex3D {
                position: [-half, 0.0, -half],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex3D {
                position: [half, 0.0, -half],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex3D {
                position: [half, 0.0, half],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex3D {
                position: [-half, 0.0, half],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
        ];

        let indices = vec![0, 1, 2, 0, 2, 3];

        Self {
            id: MeshId::new(),
            name: "Plane".to_string(),
            vertices,
            indices,
            bounds_min: Vec3::new(-half, 0.0, -half),
            bounds_max: Vec3::new(half, 0.0, half),
            submeshes: vec![SubMesh {
                start_index: 0,
                index_count: 6,
                material_index: 0,
            }],
        }
    }

    /// Create a cube mesh
    #[must_use]
    pub fn cube(size: f32) -> Self {
        let half = size / 2.0;
        
        // 6 faces, 4 vertices each = 24 vertices (for proper normals)
        let vertices = vec![
            // Front face
            Vertex3D { position: [-half, -half, half], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0] },
            Vertex3D { position: [half, -half, half], normal: [0.0, 0.0, 1.0], tex_coords: [1.0, 1.0] },
            Vertex3D { position: [half, half, half], normal: [0.0, 0.0, 1.0], tex_coords: [1.0, 0.0] },
            Vertex3D { position: [-half, half, half], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 0.0] },
            // Back face
            Vertex3D { position: [half, -half, -half], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0] },
            Vertex3D { position: [-half, -half, -half], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
            Vertex3D { position: [-half, half, -half], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0] },
            Vertex3D { position: [half, half, -half], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
            // Top face
            Vertex3D { position: [-half, half, half], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 1.0] },
            Vertex3D { position: [half, half, half], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 1.0] },
            Vertex3D { position: [half, half, -half], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0] },
            Vertex3D { position: [-half, half, -half], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },
            // Bottom face
            Vertex3D { position: [-half, -half, -half], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 1.0] },
            Vertex3D { position: [half, -half, -half], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 1.0] },
            Vertex3D { position: [half, -half, half], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 0.0] },
            Vertex3D { position: [-half, -half, half], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 0.0] },
            // Right face
            Vertex3D { position: [half, -half, half], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 1.0] },
            Vertex3D { position: [half, -half, -half], normal: [1.0, 0.0, 0.0], tex_coords: [1.0, 1.0] },
            Vertex3D { position: [half, half, -half], normal: [1.0, 0.0, 0.0], tex_coords: [1.0, 0.0] },
            Vertex3D { position: [half, half, half], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },
            // Left face
            Vertex3D { position: [-half, -half, -half], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 1.0] },
            Vertex3D { position: [-half, -half, half], normal: [-1.0, 0.0, 0.0], tex_coords: [1.0, 1.0] },
            Vertex3D { position: [-half, half, half], normal: [-1.0, 0.0, 0.0], tex_coords: [1.0, 0.0] },
            Vertex3D { position: [-half, half, -half], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },
        ];

        let indices = vec![
            0, 1, 2, 0, 2, 3,       // Front
            4, 5, 6, 4, 6, 7,       // Back
            8, 9, 10, 8, 10, 11,    // Top
            12, 13, 14, 12, 14, 15, // Bottom
            16, 17, 18, 16, 18, 19, // Right
            20, 21, 22, 20, 22, 23, // Left
        ];

        Self {
            id: MeshId::new(),
            name: "Cube".to_string(),
            vertices,
            indices,
            bounds_min: Vec3::new(-half, -half, -half),
            bounds_max: Vec3::new(half, half, half),
            submeshes: vec![SubMesh {
                start_index: 0,
                index_count: 36,
                material_index: 0,
            }],
        }
    }

    /// Create a sphere mesh
    #[must_use]
    pub fn sphere(radius: f32, segments: u32, rings: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for ring in 0..=rings {
            let phi = std::f32::consts::PI * ring as f32 / rings as f32;
            let sin_phi = phi.sin();
            let cos_phi = phi.cos();

            for seg in 0..=segments {
                let theta = 2.0 * std::f32::consts::PI * seg as f32 / segments as f32;
                let sin_theta = theta.sin();
                let cos_theta = theta.cos();

                let x = sin_phi * cos_theta;
                let y = cos_phi;
                let z = sin_phi * sin_theta;

                vertices.push(Vertex3D {
                    position: [x * radius, y * radius, z * radius],
                    normal: [x, y, z],
                    tex_coords: [seg as f32 / segments as f32, ring as f32 / rings as f32],
                });
            }
        }

        for ring in 0..rings {
            for seg in 0..segments {
                let current = ring * (segments + 1) + seg;
                let next = current + segments + 1;

                indices.push(current);
                indices.push(next);
                indices.push(current + 1);

                indices.push(next);
                indices.push(next + 1);
                indices.push(current + 1);
            }
        }

        let index_count = indices.len() as u32;
        Self {
            id: MeshId::new(),
            name: "Sphere".to_string(),
            vertices,
            indices,
            bounds_min: Vec3::new(-radius, -radius, -radius),
            bounds_max: Vec3::new(radius, radius, radius),
            submeshes: vec![SubMesh {
                start_index: 0,
                index_count,
                material_index: 0,
            }],
        }
    }

    /// Create a cylinder mesh
    #[must_use]
    pub fn cylinder(radius: f32, height: f32, segments: u32) -> Self {
        let half_height = height / 2.0;
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Side vertices
        for i in 0..=segments {
            let theta = 2.0 * std::f32::consts::PI * i as f32 / segments as f32;
            let cos = theta.cos();
            let sin = theta.sin();

            // Bottom vertex
            vertices.push(Vertex3D {
                position: [cos * radius, -half_height, sin * radius],
                normal: [cos, 0.0, sin],
                tex_coords: [i as f32 / segments as f32, 1.0],
            });
            // Top vertex
            vertices.push(Vertex3D {
                position: [cos * radius, half_height, sin * radius],
                normal: [cos, 0.0, sin],
                tex_coords: [i as f32 / segments as f32, 0.0],
            });
        }

        // Side indices
        for i in 0..segments {
            let base = i * 2;
            indices.push(base);
            indices.push(base + 1);
            indices.push(base + 2);
            indices.push(base + 1);
            indices.push(base + 3);
            indices.push(base + 2);
        }

        // Top and bottom caps (simplified)
        let top_center = vertices.len() as u32;
        vertices.push(Vertex3D {
            position: [0.0, half_height, 0.0],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [0.5, 0.5],
        });
        let bottom_center = vertices.len() as u32;
        vertices.push(Vertex3D {
            position: [0.0, -half_height, 0.0],
            normal: [0.0, -1.0, 0.0],
            tex_coords: [0.5, 0.5],
        });

        let index_count = indices.len() as u32;
        Self {
            id: MeshId::new(),
            name: "Cylinder".to_string(),
            vertices,
            indices,
            bounds_min: Vec3::new(-radius, -half_height, -radius),
            bounds_max: Vec3::new(radius, half_height, radius),
            submeshes: vec![SubMesh {
                start_index: 0,
                index_count,
                material_index: 0,
            }],
        }
    }

    /// Calculate bounding box from vertices
    pub fn calculate_bounds(&mut self) {
        if self.vertices.is_empty() {
            return;
        }

        let mut min = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max = Vec3::new(f32::MIN, f32::MIN, f32::MIN);

        for v in &self.vertices {
            min.x = min.x.min(v.position[0]);
            min.y = min.y.min(v.position[1]);
            min.z = min.z.min(v.position[2]);
            max.x = max.x.max(v.position[0]);
            max.y = max.y.max(v.position[1]);
            max.z = max.z.max(v.position[2]);
        }

        self.bounds_min = min;
        self.bounds_max = max;
    }

    /// Get vertex count
    #[must_use]
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Get index count
    #[must_use]
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }

    /// Get triangle count
    #[must_use]
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }
}

/// 3D Model (mesh + materials)
#[derive(Debug, Clone)]
pub struct Model {
    /// Model name
    pub name: String,
    /// Meshes
    pub meshes: Vec<Mesh>,
    /// Material names
    pub materials: Vec<String>,
}

impl Model {
    /// Create a new model
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            meshes: Vec::new(),
            materials: Vec::new(),
        }
    }

    /// Create a primitive cube model
    #[must_use]
    pub fn cube(size: f32) -> Self {
        Self {
            name: "Cube".to_string(),
            meshes: vec![Mesh::cube(size)],
            materials: vec!["Default".to_string()],
        }
    }

    /// Create a primitive sphere model
    #[must_use]
    pub fn sphere(radius: f32) -> Self {
        Self {
            name: "Sphere".to_string(),
            meshes: vec![Mesh::sphere(radius, 32, 16)],
            materials: vec!["Default".to_string()],
        }
    }

    /// Get total vertex count
    #[must_use]
    pub fn total_vertices(&self) -> usize {
        self.meshes.iter().map(|m| m.vertex_count()).sum()
    }

    /// Get total triangle count
    #[must_use]
    pub fn total_triangles(&self) -> usize {
        self.meshes.iter().map(|m| m.triangle_count()).sum()
    }
}

/// Mesh manager
pub struct MeshManager {
    meshes: HashMap<MeshId, Mesh>,
    primitives: HashMap<String, MeshId>,
}

impl Default for MeshManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MeshManager {
    /// Create a new mesh manager
    #[must_use]
    pub fn new() -> Self {
        let mut manager = Self {
            meshes: HashMap::new(),
            primitives: HashMap::new(),
        };

        // Pre-create primitives
        let cube = Mesh::cube(1.0);
        manager.primitives.insert("cube".to_string(), cube.id);
        manager.meshes.insert(cube.id, cube);

        let sphere = Mesh::sphere(0.5, 32, 16);
        manager.primitives.insert("sphere".to_string(), sphere.id);
        manager.meshes.insert(sphere.id, sphere);

        let plane = Mesh::plane(1.0);
        manager.primitives.insert("plane".to_string(), plane.id);
        manager.meshes.insert(plane.id, plane);

        let cylinder = Mesh::cylinder(0.5, 1.0, 16);
        manager.primitives.insert("cylinder".to_string(), cylinder.id);
        manager.meshes.insert(cylinder.id, cylinder);

        manager
    }

    /// Get a primitive mesh
    #[must_use]
    pub fn get_primitive(&self, name: &str) -> Option<&Mesh> {
        self.primitives.get(name).and_then(|id| self.meshes.get(id))
    }

    /// Add a mesh
    pub fn add(&mut self, mesh: Mesh) -> MeshId {
        let id = mesh.id;
        self.meshes.insert(id, mesh);
        id
    }

    /// Get a mesh
    #[must_use]
    pub fn get(&self, id: MeshId) -> Option<&Mesh> {
        self.meshes.get(&id)
    }

    /// Remove a mesh
    pub fn remove(&mut self, id: MeshId) {
        self.meshes.remove(&id);
    }
}
