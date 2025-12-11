//! Terrain System
//!
//! Procedural and heightmap-based terrain rendering.

use lunaris_core::math::{Color, Vec2, Vec3};

/// Terrain configuration
#[derive(Debug, Clone)]
pub struct TerrainConfig {
    /// Width in world units
    pub width: f32,
    /// Length in world units
    pub length: f32,
    /// Height scale
    pub height_scale: f32,
    /// Resolution (vertices per side)
    pub resolution: u32,
    /// Chunk size
    pub chunk_size: u32,
    /// LOD levels
    pub lod_levels: u32,
}

impl Default for TerrainConfig {
    fn default() -> Self {
        Self {
            width: 1024.0,
            length: 1024.0,
            height_scale: 100.0,
            resolution: 256,
            chunk_size: 32,
            lod_levels: 4,
        }
    }
}

/// Terrain layer (texture + properties)
#[derive(Debug, Clone)]
pub struct TerrainLayer {
    /// Layer name
    pub name: String,
    /// Diffuse texture path
    pub diffuse_texture: String,
    /// Normal texture path
    pub normal_texture: Option<String>,
    /// Tile scale
    pub tile_scale: Vec2,
    /// Metallic
    pub metallic: f32,
    /// Smoothness
    pub smoothness: f32,
}

impl Default for TerrainLayer {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            diffuse_texture: String::new(),
            normal_texture: None,
            tile_scale: Vec2::new(10.0, 10.0),
            metallic: 0.0,
            smoothness: 0.5,
        }
    }
}

/// Terrain chunk
#[derive(Debug, Clone)]
pub struct TerrainChunk {
    /// Chunk X index
    pub x: u32,
    /// Chunk Z index
    pub z: u32,
    /// Current LOD level
    pub lod: u32,
    /// Is visible
    pub visible: bool,
    /// Vertices
    pub vertices: Vec<Vec3>,
    /// Normals
    pub normals: Vec<Vec3>,
    /// UVs
    pub uvs: Vec<Vec2>,
    /// Indices
    pub indices: Vec<u32>,
}

/// Terrain heightmap
#[derive(Debug, Clone)]
pub struct Heightmap {
    /// Width
    pub width: u32,
    /// Height
    pub height: u32,
    /// Height data (0.0 - 1.0)
    pub data: Vec<f32>,
}

impl Heightmap {
    /// Create a flat heightmap
    #[must_use]
    pub fn flat(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0.0; (width * height) as usize],
        }
    }

    /// Create from noise (simplified perlin-like)
    #[must_use]
    pub fn from_noise(width: u32, height: u32, scale: f32, octaves: u32) -> Self {
        let mut data = Vec::with_capacity((width * height) as usize);
        
        for z in 0..height {
            for x in 0..width {
                let mut value = 0.0;
                let mut amplitude = 1.0;
                let mut frequency = 1.0;
                
                for _ in 0..octaves {
                    // Simplified noise (sine-based)
                    let nx = x as f32 * frequency / scale;
                    let nz = z as f32 * frequency / scale;
                    let noise = (nx.sin() * nz.cos() + (nx * 2.0).sin() * 0.5 + (nz * 2.0).cos() * 0.5) / 2.0;
                    value += noise * amplitude;
                    
                    amplitude *= 0.5;
                    frequency *= 2.0;
                }
                
                data.push((value + 1.0) / 2.0);
            }
        }
        
        Self { width, height, data }
    }

    /// Get height at position
    #[must_use]
    pub fn get(&self, x: u32, z: u32) -> f32 {
        if x >= self.width || z >= self.height {
            return 0.0;
        }
        self.data[(z * self.width + x) as usize]
    }

    /// Get interpolated height
    #[must_use]
    pub fn sample(&self, u: f32, v: f32) -> f32 {
        let x = (u * (self.width - 1) as f32).clamp(0.0, (self.width - 2) as f32);
        let z = (v * (self.height - 1) as f32).clamp(0.0, (self.height - 2) as f32);
        
        let x0 = x as u32;
        let z0 = z as u32;
        let fx = x.fract();
        let fz = z.fract();
        
        let h00 = self.get(x0, z0);
        let h10 = self.get(x0 + 1, z0);
        let h01 = self.get(x0, z0 + 1);
        let h11 = self.get(x0 + 1, z0 + 1);
        
        // Bilinear interpolation
        let h0 = h00 + (h10 - h00) * fx;
        let h1 = h01 + (h11 - h01) * fx;
        h0 + (h1 - h0) * fz
    }
}

/// Terrain splatmap (blend weights for layers)
#[derive(Debug, Clone)]
pub struct Splatmap {
    /// Width
    pub width: u32,
    /// Height
    pub height: u32,
    /// Layer weights (RGBA = 4 layers per texel)
    pub data: Vec<[f32; 4]>,
}

impl Splatmap {
    /// Create a default splatmap (all first layer)
    #[must_use]
    pub fn default_splat(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![[1.0, 0.0, 0.0, 0.0]; (width * height) as usize],
        }
    }

    /// Get layer weights at position
    #[must_use]
    pub fn get(&self, x: u32, z: u32) -> [f32; 4] {
        if x >= self.width || z >= self.height {
            return [1.0, 0.0, 0.0, 0.0];
        }
        self.data[(z * self.width + x) as usize]
    }

    /// Set layer weights
    pub fn set(&mut self, x: u32, z: u32, weights: [f32; 4]) {
        if x < self.width && z < self.height {
            self.data[(z * self.width + x) as usize] = weights;
        }
    }
}

/// Terrain system
pub struct Terrain {
    /// Configuration
    pub config: TerrainConfig,
    /// Heightmap
    pub heightmap: Heightmap,
    /// Splatmap
    pub splatmap: Splatmap,
    /// Layers
    pub layers: Vec<TerrainLayer>,
    /// Chunks
    pub chunks: Vec<TerrainChunk>,
    /// Is dirty (needs rebuild)
    dirty: bool,
}

impl Terrain {
    /// Create a new terrain
    #[must_use]
    pub fn new(config: TerrainConfig) -> Self {
        let res = config.resolution;
        let mut terrain = Self {
            config,
            heightmap: Heightmap::flat(res, res),
            splatmap: Splatmap::default_splat(res, res),
            layers: vec![TerrainLayer::default()],
            chunks: Vec::new(),
            dirty: true,
        };
        terrain.build_chunks();
        terrain
    }

    /// Create with noise
    #[must_use]
    pub fn with_noise(config: TerrainConfig, noise_scale: f32, octaves: u32) -> Self {
        let res = config.resolution;
        let mut terrain = Self {
            config,
            heightmap: Heightmap::from_noise(res, res, noise_scale, octaves),
            splatmap: Splatmap::default_splat(res, res),
            layers: vec![TerrainLayer::default()],
            chunks: Vec::new(),
            dirty: true,
        };
        terrain.build_chunks();
        terrain
    }

    fn build_chunks(&mut self) {
        self.chunks.clear();
        
        let chunks_x = self.config.resolution / self.config.chunk_size;
        let chunks_z = self.config.resolution / self.config.chunk_size;
        
        for cz in 0..chunks_z {
            for cx in 0..chunks_x {
                let chunk = self.build_chunk(cx, cz, 0);
                self.chunks.push(chunk);
            }
        }
        
        self.dirty = false;
    }

    fn build_chunk(&self, chunk_x: u32, chunk_z: u32, lod: u32) -> TerrainChunk {
        let step = 1 << lod;
        let size = self.config.chunk_size;
        let scale_x = self.config.width / self.config.resolution as f32;
        let scale_z = self.config.length / self.config.resolution as f32;
        
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();
        
        let start_x = chunk_x * size;
        let start_z = chunk_z * size;
        
        // Generate vertices
        for z in (0..=size).step_by(step as usize) {
            for x in (0..=size).step_by(step as usize) {
                let gx = start_x + x;
                let gz = start_z + z;
                
                let u = gx as f32 / self.config.resolution as f32;
                let v = gz as f32 / self.config.resolution as f32;
                
                let height = self.heightmap.sample(u, v) * self.config.height_scale;
                
                vertices.push(Vec3::new(gx as f32 * scale_x, height, gz as f32 * scale_z));
                uvs.push(Vec2::new(u, v));
                
                // Calculate normal
                let h_l = self.heightmap.sample((gx as f32 - 1.0) / self.config.resolution as f32, v) * self.config.height_scale;
                let h_r = self.heightmap.sample((gx as f32 + 1.0) / self.config.resolution as f32, v) * self.config.height_scale;
                let h_u = self.heightmap.sample(u, (gz as f32 - 1.0) / self.config.resolution as f32) * self.config.height_scale;
                let h_d = self.heightmap.sample(u, (gz as f32 + 1.0) / self.config.resolution as f32) * self.config.height_scale;
                
                let normal = Vec3::new(h_l - h_r, 2.0, h_u - h_d).normalize();
                normals.push(normal);
            }
        }
        
        // Generate indices
        let verts_per_row = size / step + 1;
        for z in 0..(verts_per_row - 1) {
            for x in 0..(verts_per_row - 1) {
                let i = z * verts_per_row + x;
                indices.push(i);
                indices.push(i + verts_per_row);
                indices.push(i + 1);
                indices.push(i + 1);
                indices.push(i + verts_per_row);
                indices.push(i + verts_per_row + 1);
            }
        }
        
        TerrainChunk {
            x: chunk_x,
            z: chunk_z,
            lod,
            visible: true,
            vertices,
            normals,
            uvs,
            indices,
        }
    }

    /// Get height at world position
    #[must_use]
    pub fn get_height(&self, x: f32, z: f32) -> f32 {
        let u = x / self.config.width;
        let v = z / self.config.length;
        self.heightmap.sample(u, v) * self.config.height_scale
    }

    /// Get normal at world position
    #[must_use]
    pub fn get_normal(&self, x: f32, z: f32) -> Vec3 {
        let u = x / self.config.width;
        let v = z / self.config.length;
        let step = 1.0 / self.config.resolution as f32;
        
        let h_l = self.heightmap.sample(u - step, v) * self.config.height_scale;
        let h_r = self.heightmap.sample(u + step, v) * self.config.height_scale;
        let h_u = self.heightmap.sample(u, v - step) * self.config.height_scale;
        let h_d = self.heightmap.sample(u, v + step) * self.config.height_scale;
        
        Vec3::new(h_l - h_r, 2.0, h_u - h_d).normalize()
    }

    /// Update LOD based on camera position
    pub fn update_lod(&mut self, camera_pos: Vec3) {
        for chunk in &mut self.chunks {
            let chunk_center = Vec3::new(
                (chunk.x as f32 + 0.5) * self.config.chunk_size as f32 * (self.config.width / self.config.resolution as f32),
                0.0,
                (chunk.z as f32 + 0.5) * self.config.chunk_size as f32 * (self.config.length / self.config.resolution as f32),
            );
            
            let distance = (camera_pos - chunk_center).length();
            
            // Calculate LOD based on distance
            let new_lod = if distance < 50.0 { 0 }
            else if distance < 100.0 { 1 }
            else if distance < 200.0 { 2 }
            else { 3 };
            
            if new_lod != chunk.lod {
                *chunk = self.build_chunk(chunk.x, chunk.z, new_lod.min(self.config.lod_levels - 1));
            }
        }
    }

    /// Add a terrain layer
    pub fn add_layer(&mut self, layer: TerrainLayer) {
        self.layers.push(layer);
    }
}
