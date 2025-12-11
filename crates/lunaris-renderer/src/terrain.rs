//! Terrain System
//!
//! Complete terrain editing and rendering with layers, foliage, and sculpting.

use glam::{Vec2, Vec3, Vec4, IVec2};
use std::collections::HashMap;

// ==================== TERRAIN ====================

/// Terrain component
pub struct Terrain {
    /// Terrain ID
    pub id: u64,
    /// Heightmap data
    pub heightmap: Heightmap,
    /// Material layers
    pub layers: Vec<TerrainLayer>,
    /// Splatmap for layer blending
    pub splatmaps: Vec<Splatmap>,
    /// Foliage instances
    pub foliage: FoliageSystem,
    /// Holes/cutouts
    pub holes: HoleMask,
    /// Settings
    pub settings: TerrainSettings,
    /// Chunks for LOD
    pub chunks: Vec<TerrainChunk>,
}

/// Heightmap
pub struct Heightmap {
    /// Resolution (must be power of 2 + 1)
    pub resolution: u32,
    /// Height data (normalized 0-1)
    pub data: Vec<f32>,
    /// World size
    pub world_size: Vec2,
    /// Height scale
    pub height_scale: f32,
    /// Min/max heights
    pub min_height: f32,
    pub max_height: f32,
}

impl Heightmap {
    pub fn new(resolution: u32, world_size: Vec2, height_scale: f32) -> Self {
        let size = (resolution * resolution) as usize;
        Self {
            resolution,
            data: vec![0.0; size],
            world_size,
            height_scale,
            min_height: 0.0,
            max_height: height_scale,
        }
    }

    /// Get height at world position
    pub fn get_height(&self, world_pos: Vec2) -> f32 {
        let uv = self.world_to_uv(world_pos);
        self.sample_bilinear(uv) * self.height_scale
    }

    /// Get normal at world position
    pub fn get_normal(&self, world_pos: Vec2) -> Vec3 {
        let step = self.world_size / (self.resolution as f32 - 1.0);
        
        let h_left = self.get_height(world_pos - Vec2::new(step.x, 0.0));
        let h_right = self.get_height(world_pos + Vec2::new(step.x, 0.0));
        let h_down = self.get_height(world_pos - Vec2::new(0.0, step.y));
        let h_up = self.get_height(world_pos + Vec2::new(0.0, step.y));

        Vec3::new(h_left - h_right, 2.0 * step.x, h_down - h_up).normalize()
    }

    fn world_to_uv(&self, world_pos: Vec2) -> Vec2 {
        Vec2::new(
            world_pos.x / self.world_size.x + 0.5,
            world_pos.y / self.world_size.y + 0.5,
        )
    }

    fn sample_bilinear(&self, uv: Vec2) -> f32 {
        let uv = uv.clamp(Vec2::ZERO, Vec2::ONE);
        let res = self.resolution as f32 - 1.0;
        
        let x = uv.x * res;
        let y = uv.y * res;
        
        let x0 = x.floor() as u32;
        let y0 = y.floor() as u32;
        let x1 = (x0 + 1).min(self.resolution - 1);
        let y1 = (y0 + 1).min(self.resolution - 1);
        
        let fx = x.fract();
        let fy = y.fract();
        
        let h00 = self.get_pixel(x0, y0);
        let h10 = self.get_pixel(x1, y0);
        let h01 = self.get_pixel(x0, y1);
        let h11 = self.get_pixel(x1, y1);
        
        let h0 = h00 * (1.0 - fx) + h10 * fx;
        let h1 = h01 * (1.0 - fx) + h11 * fx;
        
        h0 * (1.0 - fy) + h1 * fy
    }

    fn get_pixel(&self, x: u32, y: u32) -> f32 {
        self.data[(y * self.resolution + x) as usize]
    }

    fn set_pixel(&mut self, x: u32, y: u32, value: f32) {
        self.data[(y * self.resolution + x) as usize] = value.clamp(0.0, 1.0);
    }

    /// Apply brush at position
    pub fn apply_brush(&mut self, world_pos: Vec2, brush: &TerrainBrush, strength: f32) {
        let uv = self.world_to_uv(world_pos);
        let center = (uv * (self.resolution as f32 - 1.0)).as_ivec2();
        let radius = (brush.radius / self.world_size.x * self.resolution as f32) as i32;

        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let px = center.x + dx;
                let py = center.y + dy;

                if px < 0 || py < 0 || px >= self.resolution as i32 || py >= self.resolution as i32 {
                    continue;
                }

                let dist = ((dx * dx + dy * dy) as f32).sqrt() / radius as f32;
                if dist > 1.0 {
                    continue;
                }

                let falloff = brush.falloff_curve(dist);
                let effect = strength * falloff * brush.strength;

                let current = self.get_pixel(px as u32, py as u32);
                let new_value = match brush.mode {
                    BrushMode::Raise => current + effect,
                    BrushMode::Lower => current - effect,
                    BrushMode::Flatten => {
                        let target = brush.target_height / self.height_scale;
                        current + (target - current) * effect
                    }
                    BrushMode::Smooth => {
                        let neighbors = self.sample_neighbors(px as u32, py as u32);
                        current + (neighbors - current) * effect
                    }
                    BrushMode::Noise => {
                        let noise = simple_noise(px as f32 * 0.1, py as f32 * 0.1);
                        current + noise * effect
                    }
                };

                self.set_pixel(px as u32, py as u32, new_value);
            }
        }

        self.recalculate_bounds();
    }

    fn sample_neighbors(&self, x: u32, y: u32) -> f32 {
        let mut sum = 0.0;
        let mut count = 0;

        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && ny >= 0 && nx < self.resolution as i32 && ny < self.resolution as i32 {
                    sum += self.get_pixel(nx as u32, ny as u32);
                    count += 1;
                }
            }
        }

        sum / count as f32
    }

    fn recalculate_bounds(&mut self) {
        self.min_height = self.data.iter().cloned().fold(f32::MAX, f32::min) * self.height_scale;
        self.max_height = self.data.iter().cloned().fold(f32::MIN, f32::max) * self.height_scale;
    }
}

fn simple_noise(x: f32, y: f32) -> f32 {
    ((x * 12.9898 + y * 78.233).sin() * 43758.5453).fract() * 2.0 - 1.0
}

/// Terrain brush
pub struct TerrainBrush {
    pub mode: BrushMode,
    pub radius: f32,
    pub strength: f32,
    pub falloff: BrushFalloff,
    pub target_height: f32,
}

impl Default for TerrainBrush {
    fn default() -> Self {
        Self {
            mode: BrushMode::Raise,
            radius: 10.0,
            strength: 0.1,
            falloff: BrushFalloff::Smooth,
            target_height: 0.0,
        }
    }
}

impl TerrainBrush {
    fn falloff_curve(&self, t: f32) -> f32 {
        match self.falloff {
            BrushFalloff::Linear => 1.0 - t,
            BrushFalloff::Smooth => {
                let t2 = t * t;
                1.0 - t2 * (3.0 - 2.0 * t)
            }
            BrushFalloff::Sphere => {
                (1.0 - t * t).sqrt()
            }
            BrushFalloff::Tip => {
                (1.0 - t).powi(3)
            }
            BrushFalloff::Flat => {
                if t < 0.8 { 1.0 } else { 1.0 - (t - 0.8) * 5.0 }
            }
        }
    }
}

/// Brush mode
#[derive(Debug, Clone, Copy)]
pub enum BrushMode {
    Raise,
    Lower,
    Flatten,
    Smooth,
    Noise,
}

/// Brush falloff
#[derive(Debug, Clone, Copy)]
pub enum BrushFalloff {
    Linear,
    Smooth,
    Sphere,
    Tip,
    Flat,
}

/// Terrain layer (material)
pub struct TerrainLayer {
    pub name: String,
    pub albedo_texture: String,
    pub normal_texture: String,
    pub mask_texture: Option<String>,
    pub tiling: Vec2,
    pub metallic: f32,
    pub roughness: f32,
    pub height_blend: f32,
    pub slope_blend: SlopeBlend,
}

/// Slope-based blending
pub struct SlopeBlend {
    pub enabled: bool,
    pub min_angle: f32,
    pub max_angle: f32,
    pub falloff: f32,
}

/// Splatmap for layer blending
pub struct Splatmap {
    pub resolution: u32,
    /// RGBA channels = 4 layers per splatmap
    pub data: Vec<[u8; 4]>,
}

impl Splatmap {
    pub fn new(resolution: u32) -> Self {
        let size = (resolution * resolution) as usize;
        // Default: first layer = 100%
        Self {
            resolution,
            data: vec![[255, 0, 0, 0]; size],
        }
    }

    /// Paint layer at position
    pub fn paint(&mut self, uv: Vec2, layer: usize, brush: &TerrainBrush) {
        let center = (uv * (self.resolution as f32 - 1.0)).as_ivec2();
        let radius = (brush.radius * self.resolution as f32 / 100.0) as i32;

        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let px = center.x + dx;
                let py = center.y + dy;

                if px < 0 || py < 0 || px >= self.resolution as i32 || py >= self.resolution as i32 {
                    continue;
                }

                let dist = ((dx * dx + dy * dy) as f32).sqrt() / radius as f32;
                if dist > 1.0 {
                    continue;
                }

                let falloff = brush.falloff_curve(dist);
                let effect = (brush.strength * falloff * 255.0) as i32;

                let idx = (py * self.resolution as i32 + px) as usize;
                let pixel = &mut self.data[idx];

                // Add to target layer, normalize
                let layer_idx = layer % 4;
                let current = pixel[layer_idx] as i32;
                pixel[layer_idx] = (current + effect).clamp(0, 255) as u8;

                // Normalize so sum = 255
                let sum: i32 = pixel.iter().map(|&v| v as i32).sum();
                if sum > 0 {
                    for i in 0..4 {
                        pixel[i] = ((pixel[i] as i32 * 255) / sum) as u8;
                    }
                }
            }
        }
    }
}

/// Hole mask
pub struct HoleMask {
    pub resolution: u32,
    pub data: Vec<bool>,
}

impl HoleMask {
    pub fn new(resolution: u32) -> Self {
        Self {
            resolution,
            data: vec![false; (resolution * resolution) as usize],
        }
    }

    pub fn set_hole(&mut self, uv: Vec2, is_hole: bool) {
        let x = (uv.x * (self.resolution - 1) as f32) as u32;
        let y = (uv.y * (self.resolution - 1) as f32) as u32;
        self.data[(y * self.resolution + x) as usize] = is_hole;
    }

    pub fn is_hole(&self, uv: Vec2) -> bool {
        let x = (uv.x * (self.resolution - 1) as f32) as u32;
        let y = (uv.y * (self.resolution - 1) as f32) as u32;
        self.data[(y * self.resolution + x) as usize]
    }
}

/// Foliage system
pub struct FoliageSystem {
    pub types: Vec<FoliageType>,
    pub instances: HashMap<u64, Vec<FoliageInstance>>,
    pub density_maps: HashMap<u64, DensityMap>,
}

/// Foliage type (grass, trees, rocks)
pub struct FoliageType {
    pub id: u64,
    pub name: String,
    pub mesh: String,
    pub materials: Vec<String>,
    pub density: f32,
    pub scale_range: (f32, f32),
    pub rotation_range: (f32, f32),
    pub align_to_normal: bool,
    pub cast_shadows: bool,
    pub receives_shadows: bool,
    pub wind_response: f32,
    pub lod_distances: Vec<f32>,
    pub collision: bool,
}

/// Foliage instance
#[derive(Debug, Clone)]
pub struct FoliageInstance {
    pub position: Vec3,
    pub rotation: f32,
    pub scale: f32,
    pub color_variation: f32,
}

/// Density map for procedural placement
pub struct DensityMap {
    pub resolution: u32,
    pub data: Vec<f32>,
}

impl FoliageSystem {
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            instances: HashMap::new(),
            density_maps: HashMap::new(),
        }
    }

    /// Add foliage type
    pub fn add_type(&mut self, foliage: FoliageType) -> u64 {
        let id = foliage.id;
        self.types.push(foliage);
        self.instances.insert(id, Vec::new());
        id
    }

    /// Paint foliage
    pub fn paint(&mut self, type_id: u64, world_pos: Vec3, radius: f32, density: f32, terrain: &Heightmap) {
        if let Some(instances) = self.instances.get_mut(&type_id) {
            if let Some(foliage_type) = self.types.iter().find(|t| t.id == type_id) {
                // Calculate how many to place based on density
                let area = std::f32::consts::PI * radius * radius;
                let count = (area * density * foliage_type.density) as u32;

                for _ in 0..count {
                    let angle = rand_f32() * std::f32::consts::TAU;
                    let dist = rand_f32().sqrt() * radius;
                    
                    let offset = Vec2::new(angle.cos() * dist, angle.sin() * dist);
                    let pos_2d = Vec2::new(world_pos.x, world_pos.z) + offset;
                    let height = terrain.get_height(pos_2d);

                    let instance = FoliageInstance {
                        position: Vec3::new(pos_2d.x, height, pos_2d.y),
                        rotation: rand_f32() * std::f32::consts::TAU,
                        scale: lerp(foliage_type.scale_range.0, foliage_type.scale_range.1, rand_f32()),
                        color_variation: rand_f32() * 0.2,
                    };

                    instances.push(instance);
                }
            }
        }
    }

    /// Erase foliage in radius
    pub fn erase(&mut self, type_id: u64, world_pos: Vec3, radius: f32) {
        if let Some(instances) = self.instances.get_mut(&type_id) {
            instances.retain(|inst| {
                (inst.position - world_pos).length() > radius
            });
        }
    }

    /// Get total instance count
    pub fn instance_count(&self) -> usize {
        self.instances.values().map(|v| v.len()).sum()
    }
}

fn rand_f32() -> f32 {
    // Placeholder
    0.5
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Terrain chunk for LOD
pub struct TerrainChunk {
    pub bounds: ChunkBounds,
    pub lod_level: u32,
    pub vertex_count: u32,
    pub is_visible: bool,
}

/// Chunk bounds
pub struct ChunkBounds {
    pub min: Vec3,
    pub max: Vec3,
}

/// Terrain settings
pub struct TerrainSettings {
    pub cast_shadows: bool,
    pub receive_shadows: bool,
    pub draw_instanced: bool,
    pub tessellation: bool,
    pub tessellation_factor: f32,
    pub lod_bias: f32,
    pub collision_detail: u32,
    pub streaming_enabled: bool,
}

impl Default for TerrainSettings {
    fn default() -> Self {
        Self {
            cast_shadows: true,
            receive_shadows: true,
            draw_instanced: true,
            tessellation: true,
            tessellation_factor: 32.0,
            lod_bias: 0.0,
            collision_detail: 1,
            streaming_enabled: true,
        }
    }
}

impl Default for Terrain {
    fn default() -> Self {
        Self::new(1024, Vec2::splat(1000.0), 500.0)
    }
}

impl Terrain {
    pub fn new(resolution: u32, world_size: Vec2, height_scale: f32) -> Self {
        Self {
            id: 1,
            heightmap: Heightmap::new(resolution, world_size, height_scale),
            layers: Vec::new(),
            splatmaps: vec![Splatmap::new(512)],
            foliage: FoliageSystem::new(),
            holes: HoleMask::new(512),
            settings: TerrainSettings::default(),
            chunks: Vec::new(),
        }
    }

    /// Add terrain layer
    pub fn add_layer(&mut self, layer: TerrainLayer) {
        self.layers.push(layer);
        
        // Add splatmap if needed
        let needed = (self.layers.len() + 3) / 4;
        while self.splatmaps.len() < needed {
            self.splatmaps.push(Splatmap::new(512));
        }
    }

    /// Get height at position
    pub fn get_height(&self, pos: Vec2) -> f32 {
        self.heightmap.get_height(pos)
    }

    /// Get normal at position
    pub fn get_normal(&self, pos: Vec2) -> Vec3 {
        self.heightmap.get_normal(pos)
    }

    /// Sculpt terrain
    pub fn sculpt(&mut self, pos: Vec2, brush: &TerrainBrush, strength: f32) {
        self.heightmap.apply_brush(pos, brush, strength);
    }

    /// Paint layer
    pub fn paint_layer(&mut self, pos: Vec2, layer_index: usize, brush: &TerrainBrush) {
        let uv = self.heightmap.world_to_uv(pos);
        let splatmap_idx = layer_index / 4;
        
        if splatmap_idx < self.splatmaps.len() {
            self.splatmaps[splatmap_idx].paint(uv, layer_index % 4, brush);
        }
    }

    /// Generate chunks for rendering
    pub fn build_chunks(&mut self, chunk_size: u32) {
        self.chunks.clear();
        
        let chunks_x = (self.heightmap.world_size.x / chunk_size as f32).ceil() as u32;
        let chunks_z = (self.heightmap.world_size.y / chunk_size as f32).ceil() as u32;

        for cz in 0..chunks_z {
            for cx in 0..chunks_x {
                let min = Vec3::new(
                    cx as f32 * chunk_size as f32 - self.heightmap.world_size.x / 2.0,
                    self.heightmap.min_height,
                    cz as f32 * chunk_size as f32 - self.heightmap.world_size.y / 2.0,
                );
                let max = Vec3::new(
                    min.x + chunk_size as f32,
                    self.heightmap.max_height,
                    min.z + chunk_size as f32,
                );

                self.chunks.push(TerrainChunk {
                    bounds: ChunkBounds { min, max },
                    lod_level: 0,
                    vertex_count: chunk_size * chunk_size,
                    is_visible: true,
                });
            }
        }
    }
}
