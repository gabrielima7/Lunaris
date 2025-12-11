//! World Builder System
//!
//! Procedural world building with biome painting and ecological rules.

use glam::{Vec2, Vec3};
use std::collections::HashMap;

/// Biome type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BiomeType {
    // Forests
    TemperateForest,
    TropicalRainforest,
    BorealForest,
    Savannah,
    
    // Deserts
    HotDesert,
    ColdDesert,
    
    // Grasslands
    Grassland,
    Steppe,
    Tundra,
    
    // Wetlands
    Swamp,
    Marsh,
    Mangrove,
    
    // Mountains
    Alpine,
    Volcanic,
    
    // Coastal
    Beach,
    Cliffs,
    CoralReef,
    
    // Urban
    City,
    Village,
    Ruins,
    
    // Custom
    Custom(u32),
}

/// Biome configuration
#[derive(Debug, Clone)]
pub struct BiomeConfig {
    /// Biome type
    pub biome_type: BiomeType,
    /// Display name
    pub name: String,
    /// Base color (for visualization)
    pub color: [f32; 3],
    /// Terrain height range
    pub height_range: (f32, f32),
    /// Slope range (0-90 degrees)
    pub slope_range: (f32, f32),
    /// Moisture range (0-1)
    pub moisture_range: (f32, f32),
    /// Temperature range (Celsius)
    pub temperature_range: (f32, f32),
    /// Vegetation layers
    pub vegetation: Vec<VegetationLayer>,
    /// Ground materials
    pub ground_materials: Vec<GroundMaterial>,
    /// Props (rocks, debris, etc.)
    pub props: Vec<PropLayer>,
}

/// Vegetation layer
#[derive(Debug, Clone)]
pub struct VegetationLayer {
    /// Layer name
    pub name: String,
    /// Foliage asset IDs
    pub assets: Vec<u64>,
    /// Density (instances per m²)
    pub density: f32,
    /// Min/max scale
    pub scale_range: (f32, f32),
    /// Random rotation
    pub random_rotation: bool,
    /// Align to normal
    pub align_to_normal: bool,
    /// Slope limit (degrees)
    pub max_slope: f32,
    /// Clustering factor (0 = uniform, 1 = clustered)
    pub clustering: f32,
    /// Exclusion radius (no overlap)
    pub exclusion_radius: f32,
    /// LOD distances
    pub lod_distances: Vec<f32>,
}

/// Ground material
#[derive(Debug, Clone)]
pub struct GroundMaterial {
    /// Material asset ID
    pub material_id: u64,
    /// Blend weight
    pub weight: f32,
    /// Texture scale
    pub texture_scale: f32,
}

/// Prop layer
#[derive(Debug, Clone)]
pub struct PropLayer {
    /// Prop asset IDs
    pub assets: Vec<u64>,
    /// Density
    pub density: f32,
    /// Scale range
    pub scale_range: (f32, f32),
    /// Random rotation
    pub random_rotation: bool,
    /// Sink into ground
    pub ground_offset: f32,
}

/// Ecological rule for vegetation interaction
#[derive(Debug, Clone)]
pub struct EcologicalRule {
    /// Rule name
    pub name: String,
    /// Source layer
    pub source: String,
    /// Target layer
    pub target: String,
    /// Relationship
    pub relationship: EcologicalRelation,
    /// Distance threshold
    pub distance: f32,
    /// Effect strength (0-1)
    pub strength: f32,
}

/// Ecological relationship
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcologicalRelation {
    /// Plants grow near each other
    Symbiotic,
    /// Plants avoid each other
    Competitive,
    /// One grows in shadow of other
    Understory,
    /// One wraps around other
    Parasitic,
}

/// Brush tool for painting
#[derive(Debug, Clone)]
pub struct WorldBrush {
    /// Brush type
    pub brush_type: BrushType,
    /// Brush size (radius)
    pub size: f32,
    /// Brush strength (0-1)
    pub strength: f32,
    /// Falloff type
    pub falloff: BrushFalloff,
    /// Current biome
    pub biome: BiomeType,
    /// Paint mode
    pub mode: PaintMode,
}

/// Brush type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrushType {
    Circle,
    Square,
    Custom,
}

/// Brush falloff
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrushFalloff {
    Constant,
    Linear,
    Smooth,
    Spherical,
}

/// Paint mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaintMode {
    /// Add biome
    Paint,
    /// Remove/erase
    Erase,
    /// Smooth transition
    Blend,
    /// Replace specific biome
    Replace,
}

impl Default for WorldBrush {
    fn default() -> Self {
        Self {
            brush_type: BrushType::Circle,
            size: 10.0,
            strength: 1.0,
            falloff: BrushFalloff::Smooth,
            biome: BiomeType::Grassland,
            mode: PaintMode::Paint,
        }
    }
}

/// Terrain sculpting tools
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SculptTool {
    Raise,
    Lower,
    Flatten,
    Smooth,
    Noise,
    Erode,
    Hydraulic,
    Thermal,
}

/// Spline for roads/rivers
#[derive(Debug, Clone)]
pub struct WorldSpline {
    /// Unique ID
    pub id: u64,
    /// Spline type
    pub spline_type: SplineType,
    /// Control points
    pub points: Vec<SplinePoint>,
    /// Width
    pub width: f32,
    /// Deform terrain
    pub deform_terrain: bool,
    /// Deform strength
    pub deform_strength: f32,
    /// Material
    pub material_id: u64,
}

/// Spline type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplineType {
    Road,
    Path,
    River,
    Wall,
    Fence,
    Railroad,
    PowerLine,
    Custom,
}

/// Spline control point
#[derive(Debug, Clone)]
pub struct SplinePoint {
    /// Position
    pub position: Vec3,
    /// Width at this point
    pub width: f32,
    /// Tangent
    pub tangent: Vec3,
    /// Roll angle
    pub roll: f32,
}

/// World biome map
#[derive(Debug, Clone)]
pub struct BiomeMap {
    /// Resolution (cells per axis)
    pub resolution: u32,
    /// World size
    pub world_size: Vec2,
    /// Biome weights per cell (biome -> weight)
    cells: Vec<HashMap<BiomeType, f32>>,
}

impl BiomeMap {
    /// Create new biome map
    #[must_use]
    pub fn new(resolution: u32, world_size: Vec2) -> Self {
        let cell_count = (resolution * resolution) as usize;
        let mut cells = Vec::with_capacity(cell_count);
        for _ in 0..cell_count {
            cells.push(HashMap::new());
        }
        
        Self {
            resolution,
            world_size,
            cells,
        }
    }

    /// Get cell index from world position
    fn cell_index(&self, pos: Vec2) -> Option<usize> {
        let normalized = pos / self.world_size + Vec2::splat(0.5);
        if normalized.x < 0.0 || normalized.x >= 1.0 || normalized.y < 0.0 || normalized.y >= 1.0 {
            return None;
        }
        
        let x = (normalized.x * self.resolution as f32) as usize;
        let y = (normalized.y * self.resolution as f32) as usize;
        let idx = y * self.resolution as usize + x;
        
        if idx < self.cells.len() {
            Some(idx)
        } else {
            None
        }
    }

    /// Paint biome at position
    pub fn paint(&mut self, pos: Vec2, biome: BiomeType, weight: f32) {
        if let Some(idx) = self.cell_index(pos) {
            let cell = &mut self.cells[idx];
            let current = cell.entry(biome).or_insert(0.0);
            *current = (*current + weight).min(1.0);
            
            // Normalize weights
            let total: f32 = cell.values().sum();
            if total > 1.0 {
                for w in cell.values_mut() {
                    *w /= total;
                }
            }
        }
    }

    /// Get dominant biome at position
    #[must_use]
    pub fn get_biome(&self, pos: Vec2) -> Option<BiomeType> {
        self.cell_index(pos).and_then(|idx| {
            self.cells[idx].iter()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(&biome, _)| biome)
        })
    }

    /// Get biome weights at position
    #[must_use]
    pub fn get_weights(&self, pos: Vec2) -> HashMap<BiomeType, f32> {
        self.cell_index(pos)
            .map(|idx| self.cells[idx].clone())
            .unwrap_or_default()
    }
}

/// World builder
pub struct WorldBuilder {
    /// Biome configurations
    biomes: HashMap<BiomeType, BiomeConfig>,
    /// Biome map
    pub biome_map: BiomeMap,
    /// Current brush
    pub brush: WorldBrush,
    /// Sculpt tool
    pub sculpt_tool: SculptTool,
    /// Splines
    splines: Vec<WorldSpline>,
    /// Ecological rules
    pub rules: Vec<EcologicalRule>,
    /// Random seed
    pub seed: u64,
    /// Next spline ID
    next_spline_id: u64,
}

impl WorldBuilder {
    /// Create new world builder
    #[must_use]
    pub fn new(world_size: Vec2, resolution: u32) -> Self {
        let mut builder = Self {
            biomes: HashMap::new(),
            biome_map: BiomeMap::new(resolution, world_size),
            brush: WorldBrush::default(),
            sculpt_tool: SculptTool::Raise,
            splines: Vec::new(),
            rules: Vec::new(),
            seed: 42,
            next_spline_id: 1,
        };

        // Register default biomes
        builder.register_default_biomes();
        builder
    }

    fn register_default_biomes(&mut self) {
        // Grassland
        self.register_biome(BiomeConfig {
            biome_type: BiomeType::Grassland,
            name: "Grassland".to_string(),
            color: [0.4, 0.7, 0.3],
            height_range: (0.0, 100.0),
            slope_range: (0.0, 30.0),
            moisture_range: (0.3, 0.7),
            temperature_range: (10.0, 30.0),
            vegetation: vec![
                VegetationLayer {
                    name: "Grass".to_string(),
                    assets: vec![1, 2, 3],
                    density: 50.0,
                    scale_range: (0.8, 1.2),
                    random_rotation: true,
                    align_to_normal: true,
                    max_slope: 45.0,
                    clustering: 0.3,
                    exclusion_radius: 0.1,
                    lod_distances: vec![50.0, 100.0, 200.0],
                },
            ],
            ground_materials: vec![
                GroundMaterial {
                    material_id: 1,
                    weight: 1.0,
                    texture_scale: 1.0,
                },
            ],
            props: vec![],
        });

        // Forest
        self.register_biome(BiomeConfig {
            biome_type: BiomeType::TemperateForest,
            name: "Temperate Forest".to_string(),
            color: [0.2, 0.5, 0.2],
            height_range: (0.0, 500.0),
            slope_range: (0.0, 45.0),
            moisture_range: (0.5, 1.0),
            temperature_range: (5.0, 25.0),
            vegetation: vec![
                VegetationLayer {
                    name: "Trees".to_string(),
                    assets: vec![10, 11, 12],
                    density: 0.5,
                    scale_range: (0.7, 1.3),
                    random_rotation: true,
                    align_to_normal: false,
                    max_slope: 35.0,
                    clustering: 0.5,
                    exclusion_radius: 3.0,
                    lod_distances: vec![100.0, 300.0, 500.0],
                },
                VegetationLayer {
                    name: "Undergrowth".to_string(),
                    assets: vec![20, 21],
                    density: 10.0,
                    scale_range: (0.5, 1.5),
                    random_rotation: true,
                    align_to_normal: true,
                    max_slope: 45.0,
                    clustering: 0.7,
                    exclusion_radius: 0.5,
                    lod_distances: vec![30.0, 60.0],
                },
            ],
            ground_materials: vec![
                GroundMaterial {
                    material_id: 2,
                    weight: 0.7,
                    texture_scale: 1.0,
                },
                GroundMaterial {
                    material_id: 3,
                    weight: 0.3,
                    texture_scale: 0.5,
                },
            ],
            props: vec![
                PropLayer {
                    assets: vec![100, 101], // Rocks
                    density: 0.05,
                    scale_range: (0.5, 2.0),
                    random_rotation: true,
                    ground_offset: -0.1,
                },
            ],
        });
    }

    /// Register a biome configuration
    pub fn register_biome(&mut self, config: BiomeConfig) {
        self.biomes.insert(config.biome_type, config);
    }

    /// Paint with current brush
    pub fn paint(&mut self, center: Vec2) {
        let size = self.brush.size;
        let strength = self.brush.strength;
        let biome = self.brush.biome;

        // Sample points within brush radius
        let step = self.biome_map.world_size / self.biome_map.resolution as f32;
        let steps = (size / step.x).ceil() as i32;

        for dy in -steps..=steps {
            for dx in -steps..=steps {
                let offset = Vec2::new(dx as f32 * step.x, dy as f32 * step.y);
                let pos = center + offset;
                let dist = offset.length();

                if dist <= size {
                    let falloff = match self.brush.falloff {
                        BrushFalloff::Constant => 1.0,
                        BrushFalloff::Linear => 1.0 - dist / size,
                        BrushFalloff::Smooth => {
                            let t = dist / size;
                            1.0 - t * t * (3.0 - 2.0 * t)
                        }
                        BrushFalloff::Spherical => (1.0 - (dist / size).powi(2)).sqrt(),
                    };

                    let weight = strength * falloff;

                    match self.brush.mode {
                        PaintMode::Paint => {
                            self.biome_map.paint(pos, biome, weight);
                        }
                        PaintMode::Erase => {
                            // Remove biome weight
                        }
                        PaintMode::Blend | PaintMode::Replace => {
                            // Other modes
                        }
                    }
                }
            }
        }
    }

    /// Create spline
    pub fn create_spline(&mut self, spline_type: SplineType) -> u64 {
        let id = self.next_spline_id;
        self.next_spline_id += 1;

        let spline = WorldSpline {
            id,
            spline_type,
            points: Vec::new(),
            width: 4.0,
            deform_terrain: true,
            deform_strength: 1.0,
            material_id: 0,
        };

        self.splines.push(spline);
        id
    }

    /// Add point to spline
    pub fn add_spline_point(&mut self, spline_id: u64, point: SplinePoint) {
        if let Some(spline) = self.splines.iter_mut().find(|s| s.id == spline_id) {
            spline.points.push(point);
        }
    }

    /// Generate vegetation instances for a region
    #[must_use]
    pub fn generate_vegetation(&self, min: Vec2, max: Vec2) -> Vec<VegetationInstance> {
        let mut instances = Vec::new();
        let mut rng = SimpleRng::new(self.seed);

        // Sample the region
        let step = 1.0; // 1 meter steps
        let mut y = min.y;
        while y < max.y {
            let mut x = min.x;
            while x < max.x {
                let pos = Vec2::new(x, y);
                
                // Get biome at this position
                if let Some(biome_type) = self.biome_map.get_biome(pos) {
                    if let Some(config) = self.biomes.get(&biome_type) {
                        // Generate vegetation for each layer
                        for layer in &config.vegetation {
                            if rng.next_f32() < layer.density * step * step {
                                let scale = layer.scale_range.0 
                                    + rng.next_f32() * (layer.scale_range.1 - layer.scale_range.0);
                                
                                let rotation = if layer.random_rotation {
                                    rng.next_f32() * std::f32::consts::TAU
                                } else {
                                    0.0
                                };

                                let asset_idx = rng.next_u32() as usize % layer.assets.len();

                                instances.push(VegetationInstance {
                                    position: Vec3::new(pos.x, 0.0, pos.y),
                                    rotation,
                                    scale,
                                    asset_id: layer.assets[asset_idx],
                                    biome: biome_type,
                                });
                            }
                        }
                    }
                }
                
                x += step;
            }
            y += step;
        }

        instances
    }

    /// Get biomes
    #[must_use]
    pub fn biomes(&self) -> &HashMap<BiomeType, BiomeConfig> {
        &self.biomes
    }

    /// Get splines
    #[must_use]
    pub fn splines(&self) -> &[WorldSpline] {
        &self.splines
    }
}

/// Vegetation instance
#[derive(Debug, Clone)]
pub struct VegetationInstance {
    /// Position
    pub position: Vec3,
    /// Rotation (Y axis)
    pub rotation: f32,
    /// Scale
    pub scale: f32,
    /// Asset ID
    pub asset_id: u64,
    /// Source biome
    pub biome: BiomeType,
}

/// Simple RNG for deterministic generation
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }

    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    fn next_f32(&mut self) -> f32 {
        (self.next_u32() as f32) / (u32::MAX as f32)
    }
}
