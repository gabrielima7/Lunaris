//! Foliage System
//!
//! Procedural placement and rendering of vegetation.

use glam::{Vec2, Vec3, Mat4, Quat};
use std::collections::HashMap;

/// Foliage type
#[derive(Debug, Clone)]
pub struct FoliageType {
    /// Type ID
    pub id: u64,
    /// Name
    pub name: String,
    /// Mesh ID
    pub mesh_id: u64,
    /// Material ID
    pub material_id: u64,
    /// LOD meshes (distance, mesh_id)
    pub lod_meshes: Vec<(f32, u64)>,
    /// Min scale
    pub min_scale: f32,
    /// Max scale
    pub max_scale: f32,
    /// Random rotation range (degrees)
    pub rotation_range: f32,
    /// Align to surface normal
    pub align_to_normal: bool,
    /// Normal alignment factor (0=up, 1=surface)
    pub normal_blend: f32,
    /// Density (per square meter)
    pub density: f32,
    /// Ground slope min (degrees)
    pub slope_min: f32,
    /// Ground slope max (degrees)
    pub slope_max: f32,
    /// Altitude min
    pub altitude_min: f32,
    /// Altitude max
    pub altitude_max: f32,
    /// Cast shadows
    pub cast_shadows: bool,
    /// Receive shadows
    pub receive_shadows: bool,
    /// Wind reactivity
    pub wind_strength: f32,
    /// Collision enabled
    pub collision: bool,
    /// Cull distance
    pub cull_distance: f32,
}

impl Default for FoliageType {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::from("Foliage"),
            mesh_id: 0,
            material_id: 0,
            lod_meshes: Vec::new(),
            min_scale: 0.8,
            max_scale: 1.2,
            rotation_range: 360.0,
            align_to_normal: true,
            normal_blend: 0.3,
            density: 1.0,
            slope_min: 0.0,
            slope_max: 45.0,
            altitude_min: f32::MIN,
            altitude_max: f32::MAX,
            cast_shadows: true,
            receive_shadows: true,
            wind_strength: 1.0,
            collision: false,
            cull_distance: 100.0,
        }
    }
}

/// Foliage instance
#[derive(Debug, Clone, Copy)]
pub struct FoliageInstance {
    /// Position
    pub position: Vec3,
    /// Rotation
    pub rotation: Quat,
    /// Scale
    pub scale: f32,
    /// Type ID
    pub type_id: u64,
    /// Random seed (for wind variation)
    pub seed: u32,
}

impl FoliageInstance {
    /// Get transform matrix
    #[must_use]
    pub fn transform(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            Vec3::splat(self.scale),
            self.rotation,
            self.position,
        )
    }
}

/// Foliage cell (spatial partition)
pub struct FoliageCell {
    /// Cell position (grid coords)
    pub coords: [i32; 2],
    /// Instances in this cell
    pub instances: Vec<FoliageInstance>,
    /// Is loaded
    pub loaded: bool,
    /// Distance to camera
    pub distance: f32,
}

impl FoliageCell {
    /// Create a new cell
    #[must_use]
    pub fn new(x: i32, z: i32) -> Self {
        Self {
            coords: [x, z],
            instances: Vec::new(),
            loaded: false,
            distance: f32::MAX,
        }
    }

    /// Get world center
    #[must_use]
    pub fn world_center(&self, cell_size: f32) -> Vec3 {
        Vec3::new(
            self.coords[0] as f32 * cell_size + cell_size * 0.5,
            0.0,
            self.coords[1] as f32 * cell_size + cell_size * 0.5,
        )
    }
}

/// Foliage layer
pub struct FoliageLayer {
    /// Layer name
    pub name: String,
    /// Foliage types in this layer
    pub types: Vec<FoliageType>,
    /// Cells
    cells: HashMap<[i32; 2], FoliageCell>,
    /// Cell size
    pub cell_size: f32,
    /// View distance
    pub view_distance: f32,
    /// Max instances per cell
    pub max_per_cell: u32,
    /// Seed for procedural placement
    pub seed: u64,
}

impl Default for FoliageLayer {
    fn default() -> Self {
        Self::new("Default")
    }
}

impl FoliageLayer {
    /// Create a new layer
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            types: Vec::new(),
            cells: HashMap::new(),
            cell_size: 50.0,
            view_distance: 200.0,
            max_per_cell: 1000,
            seed: 12345,
        }
    }

    /// Add foliage type
    pub fn add_type(&mut self, foliage_type: FoliageType) {
        self.types.push(foliage_type);
    }

    /// Generate instances for a cell
    pub fn generate_cell(
        &mut self,
        x: i32,
        z: i32,
        height_fn: &dyn Fn(f32, f32) -> (f32, Vec3),
    ) {
        let cell = self.cells.entry([x, z]).or_insert_with(|| FoliageCell::new(x, z));
        
        if cell.loaded {
            return;
        }

        cell.instances.clear();
        
        let world_x = x as f32 * self.cell_size;
        let world_z = z as f32 * self.cell_size;
        
        for foliage_type in &self.types {
            let count = (foliage_type.density * self.cell_size * self.cell_size) as u32;
            let count = count.min(self.max_per_cell);
            
            for i in 0..count {
                // Pseudo-random position within cell
                let seed = self.seed.wrapping_add(x as u64 * 1000 + z as u64).wrapping_add(i as u64);
                let rx = Self::hash_float(seed);
                let rz = Self::hash_float(seed.wrapping_add(1));
                
                let px = world_x + rx * self.cell_size;
                let pz = world_z + rz * self.cell_size;
                
                let (height, normal) = height_fn(px, pz);
                
                // Check slope
                let slope = normal.y.acos().to_degrees();
                if slope < foliage_type.slope_min || slope > foliage_type.slope_max {
                    continue;
                }
                
                // Check altitude
                if height < foliage_type.altitude_min || height > foliage_type.altitude_max {
                    continue;
                }
                
                // Random scale
                let scale_range = foliage_type.max_scale - foliage_type.min_scale;
                let scale = foliage_type.min_scale + Self::hash_float(seed.wrapping_add(2)) * scale_range;
                
                // Random rotation
                let rotation_y = Self::hash_float(seed.wrapping_add(3)) * foliage_type.rotation_range.to_radians();
                let mut rotation = Quat::from_rotation_y(rotation_y);
                
                // Align to normal
                if foliage_type.align_to_normal {
                    let up = Vec3::Y.lerp(normal, foliage_type.normal_blend).normalize();
                    let align_rotation = Quat::from_rotation_arc(Vec3::Y, up);
                    rotation = align_rotation * rotation;
                }
                
                cell.instances.push(FoliageInstance {
                    position: Vec3::new(px, height, pz),
                    rotation,
                    scale,
                    type_id: foliage_type.id,
                    seed: seed as u32,
                });
            }
        }
        
        cell.loaded = true;
    }

    fn hash_float(seed: u64) -> f32 {
        let x = (seed as f32 * 12.9898).sin() * 43758.5453;
        x.fract()
    }

    /// Update visible cells based on camera position
    pub fn update(&mut self, camera_pos: Vec3, height_fn: &dyn Fn(f32, f32) -> (f32, Vec3)) {
        let cell_radius = (self.view_distance / self.cell_size).ceil() as i32;
        let camera_cell_x = (camera_pos.x / self.cell_size).floor() as i32;
        let camera_cell_z = (camera_pos.z / self.cell_size).floor() as i32;
        
        // Generate needed cells
        for x in (camera_cell_x - cell_radius)..=(camera_cell_x + cell_radius) {
            for z in (camera_cell_z - cell_radius)..=(camera_cell_z + cell_radius) {
                self.generate_cell(x, z, height_fn);
            }
        }
        
        // Update distances and unload far cells
        let cells_to_remove: Vec<[i32; 2]> = self.cells.iter()
            .filter_map(|(coords, cell)| {
                let center = cell.world_center(self.cell_size);
                let dist = (center - camera_pos).length();
                if dist > self.view_distance * 1.5 {
                    Some(*coords)
                } else {
                    None
                }
            })
            .collect();
        
        for coords in cells_to_remove {
            self.cells.remove(&coords);
        }
    }

    /// Get visible instances
    #[must_use]
    pub fn visible_instances(&self, camera_pos: Vec3) -> Vec<&FoliageInstance> {
        let mut instances: Vec<_> = self.cells.values()
            .filter(|cell| {
                let center = cell.world_center(self.cell_size);
                (center - camera_pos).length() <= self.view_distance
            })
            .flat_map(|cell| cell.instances.iter())
            .filter(|inst| {
                let foliage_type = self.types.iter().find(|t| t.id == inst.type_id);
                if let Some(ft) = foliage_type {
                    (inst.position - camera_pos).length() <= ft.cull_distance
                } else {
                    true
                }
            })
            .collect();
        
        // Sort by distance for batching
        instances.sort_by(|a, b| {
            let da = (a.position - camera_pos).length_squared();
            let db = (b.position - camera_pos).length_squared();
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        instances
    }

    /// Get instance count
    #[must_use]
    pub fn instance_count(&self) -> usize {
        self.cells.values().map(|c| c.instances.len()).sum()
    }
}

/// Wind simulation for foliage
#[derive(Debug, Clone)]
pub struct FoliageWind {
    /// Wind direction (XZ normalized)
    pub direction: Vec2,
    /// Wind strength
    pub strength: f32,
    /// Gust frequency
    pub gust_frequency: f32,
    /// Gust strength
    pub gust_strength: f32,
    /// Current time
    time: f32,
}

impl Default for FoliageWind {
    fn default() -> Self {
        Self {
            direction: Vec2::new(1.0, 0.0),
            strength: 1.0,
            gust_frequency: 0.5,
            gust_strength: 0.3,
            time: 0.0,
        }
    }
}

impl FoliageWind {
    /// Update wind
    pub fn update(&mut self, delta_time: f32) {
        self.time += delta_time;
    }

    /// Get wind vector at position
    #[must_use]
    pub fn at_position(&self, position: Vec3, instance_seed: u32) -> Vec3 {
        let phase = (position.x + position.z) * 0.1 + instance_seed as f32 * 0.01;
        let wave = (self.time * 2.0 + phase).sin();
        let gust = (self.time * self.gust_frequency + phase * 0.5).sin() * self.gust_strength;
        
        let strength = self.strength * (1.0 + wave * 0.3 + gust);
        
        Vec3::new(
            self.direction.x * strength,
            0.0,
            self.direction.y * strength,
        )
    }
}
