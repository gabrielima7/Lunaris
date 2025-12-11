//! Mesh Shaders
//!
//! GPU-driven rendering, meshlet culling, variable rate shading.

use glam::{Vec3, Vec4, Mat4};

/// Mesh shader system
pub struct MeshShaderSystem {
    pub meshlet_data: MeshletData,
    pub culling: MeshletCulling,
    pub vrs: VariableRateShading,
    pub settings: MeshShaderSettings,
}

/// Meshlet data
pub struct MeshletData {
    pub meshlets: Vec<Meshlet>,
    pub vertices: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub meshlet_vertices: Vec<u32>,
    pub meshlet_triangles: Vec<u8>,
}

/// Meshlet (small mesh chunk)
pub struct Meshlet {
    pub vertex_offset: u32,
    pub vertex_count: u32,
    pub triangle_offset: u32,
    pub triangle_count: u32,
    pub center: Vec3,
    pub radius: f32,
    pub cone_axis: Vec3,
    pub cone_cutoff: f32,
}

/// Meshlet culling
pub struct MeshletCulling {
    pub frustum_culling: bool,
    pub occlusion_culling: bool,
    pub backface_culling: bool,
    pub small_primitive_culling: bool,
    pub culled_count: u32,
    pub visible_count: u32,
}

impl Default for MeshletCulling {
    fn default() -> Self {
        Self { frustum_culling: true, occlusion_culling: true, backface_culling: true, small_primitive_culling: true, culled_count: 0, visible_count: 0 }
    }
}

/// Variable Rate Shading
pub struct VariableRateShading {
    pub enabled: bool,
    pub mode: VRSMode,
    pub base_rate: ShadingRate,
    pub image_rate: Option<Vec<ShadingRate>>,
    pub image_size: (u32, u32),
    pub tile_size: u32,
}

/// VRS mode
pub enum VRSMode { PerDraw, PerPrimitive, Image }

/// Shading rate
#[derive(Clone, Copy)]
pub enum ShadingRate { R1x1, R1x2, R2x1, R2x2, R2x4, R4x2, R4x4 }

impl Default for VariableRateShading {
    fn default() -> Self {
        Self { enabled: true, mode: VRSMode::Image, base_rate: ShadingRate::R1x1, image_rate: None, image_size: (0, 0), tile_size: 16 }
    }
}

/// Mesh shader settings
pub struct MeshShaderSettings {
    pub max_vertices_per_meshlet: u32,
    pub max_triangles_per_meshlet: u32,
    pub workgroup_size: u32,
    pub cone_culling: bool,
}

impl Default for MeshShaderSettings {
    fn default() -> Self {
        Self { max_vertices_per_meshlet: 64, max_triangles_per_meshlet: 126, workgroup_size: 32, cone_culling: true }
    }
}

impl MeshShaderSystem {
    pub fn new() -> Self {
        Self { meshlet_data: MeshletData { meshlets: Vec::new(), vertices: Vec::new(), indices: Vec::new(), meshlet_vertices: Vec::new(), meshlet_triangles: Vec::new() }, culling: MeshletCulling::default(), vrs: VariableRateShading::default(), settings: MeshShaderSettings::default() }
    }

    /// Build meshlets from mesh
    pub fn build_meshlets(&mut self, vertices: &[Vec3], indices: &[u32]) {
        self.meshlet_data.vertices = vertices.to_vec();
        self.meshlet_data.indices = indices.to_vec();
        self.meshlet_data.meshlets.clear();

        let max_verts = self.settings.max_vertices_per_meshlet as usize;
        let max_tris = self.settings.max_triangles_per_meshlet as usize;

        let mut i = 0;
        while i < indices.len() {
            let start_vert = self.meshlet_data.meshlet_vertices.len() as u32;
            let start_tri = self.meshlet_data.meshlet_triangles.len() as u32;
            
            let mut meshlet_verts: Vec<u32> = Vec::new();
            let mut meshlet_tris: Vec<u8> = Vec::new();
            let mut bounds_min = Vec3::splat(f32::MAX);
            let mut bounds_max = Vec3::splat(f32::MIN);

            while i < indices.len() && meshlet_verts.len() < max_verts && meshlet_tris.len() / 3 < max_tris {
                for j in 0..3 {
                    let idx = indices[i + j];
                    let local_idx = meshlet_verts.iter().position(|&v| v == idx);
                    
                    let local = if let Some(l) = local_idx { l as u8 }
                    else { 
                        if meshlet_verts.len() >= max_verts { break; }
                        meshlet_verts.push(idx);
                        let pos = vertices[idx as usize];
                        bounds_min = bounds_min.min(pos);
                        bounds_max = bounds_max.max(pos);
                        (meshlet_verts.len() - 1) as u8
                    };
                    meshlet_tris.push(local);
                }
                i += 3;
            }

            let center = (bounds_min + bounds_max) * 0.5;
            let radius = (bounds_max - center).length();

            self.meshlet_data.meshlets.push(Meshlet {
                vertex_offset: start_vert,
                vertex_count: meshlet_verts.len() as u32,
                triangle_offset: start_tri,
                triangle_count: (meshlet_tris.len() / 3) as u32,
                center,
                radius,
                cone_axis: Vec3::Y,
                cone_cutoff: 1.0,
            });

            self.meshlet_data.meshlet_vertices.extend(meshlet_verts);
            self.meshlet_data.meshlet_triangles.extend(meshlet_tris);
        }
    }

    /// Cull meshlets against camera
    pub fn cull(&mut self, view_proj: Mat4, camera_pos: Vec3) {
        self.culling.culled_count = 0;
        self.culling.visible_count = 0;

        for meshlet in &self.meshlet_data.meshlets {
            let mut visible = true;

            // Frustum culling
            if self.culling.frustum_culling {
                let clip = view_proj * meshlet.center.extend(1.0);
                let ndc = clip.truncate() / clip.w.abs();
                if ndc.x.abs() > 1.0 + meshlet.radius || ndc.y.abs() > 1.0 + meshlet.radius || clip.w < 0.0 {
                    visible = false;
                }
            }

            // Backface cone culling
            if visible && self.culling.backface_culling && self.settings.cone_culling {
                let view_dir = (meshlet.center - camera_pos).normalize();
                if view_dir.dot(meshlet.cone_axis) < meshlet.cone_cutoff {
                    visible = false;
                }
            }

            if visible { self.culling.visible_count += 1; }
            else { self.culling.culled_count += 1; }
        }
    }

    /// Update VRS image
    pub fn update_vrs_image(&mut self, motion_vectors: &[Vec2], luminance: &[f32], width: u32, height: u32) {
        let tile_size = self.vrs.tile_size;
        let tiles_x = (width + tile_size - 1) / tile_size;
        let tiles_y = (height + tile_size - 1) / tile_size;
        
        self.vrs.image_size = (tiles_x, tiles_y);
        self.vrs.image_rate = Some(vec![ShadingRate::R1x1; (tiles_x * tiles_y) as usize]);

        if let Some(rates) = &mut self.vrs.image_rate {
            for ty in 0..tiles_y {
                for tx in 0..tiles_x {
                    let tile_idx = (ty * tiles_x + tx) as usize;
                    let px = (tx * tile_size + tile_size / 2).min(width - 1) as usize;
                    let py = (ty * tile_size + tile_size / 2).min(height - 1) as usize;
                    let pix_idx = py * width as usize + px;

                    let motion = if pix_idx < motion_vectors.len() { motion_vectors[pix_idx].length() } else { 0.0 };
                    let luma = if pix_idx < luminance.len() { luminance[pix_idx] } else { 0.5 };

                    // Higher motion and lower luminance = lower shading rate
                    rates[tile_idx] = if motion > 10.0 || luma < 0.1 { ShadingRate::R4x4 }
                        else if motion > 5.0 || luma < 0.3 { ShadingRate::R2x2 }
                        else { ShadingRate::R1x1 };
                }
            }
        }
    }

    pub fn meshlet_count(&self) -> usize { self.meshlet_data.meshlets.len() }
    pub fn triangle_count(&self) -> u32 { self.meshlet_data.meshlets.iter().map(|m| m.triangle_count).sum() }
}

use glam::Vec2;
