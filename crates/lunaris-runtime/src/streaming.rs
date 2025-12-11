//! World Streaming
//!
//! World partition, level streaming, and seamless loading.

use glam::{Vec2, Vec3, IVec2};
use std::collections::{HashMap, HashSet};

/// World partition
pub struct WorldPartition {
    pub cells: HashMap<IVec2, WorldCell>,
    pub cell_size: f32,
    pub load_distance: f32,
    pub unload_distance: f32,
    pub current_cell: IVec2,
    pub loaded_cells: HashSet<IVec2>,
    pub streaming_state: StreamingState,
}

/// World cell
pub struct WorldCell {
    pub coord: IVec2,
    pub layers: Vec<StreamingLayer>,
    pub state: CellState,
    pub bounds: CellBounds,
    pub priority: i32,
}

/// Streaming layer
pub struct StreamingLayer {
    pub name: String,
    pub asset_path: String,
    pub loaded: bool,
    pub streaming_distance: f32,
}

/// Cell state
#[derive(Clone, Copy, PartialEq)]
pub enum CellState { Unloaded, Loading, Loaded, Unloading }

/// Cell bounds
pub struct CellBounds {
    pub min: Vec3,
    pub max: Vec3,
}

/// Streaming state
pub struct StreamingState {
    pub loading_queue: Vec<IVec2>,
    pub unloading_queue: Vec<IVec2>,
    pub async_loads: u32,
    pub max_async_loads: u32,
}

impl WorldPartition {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cells: HashMap::new(),
            cell_size,
            load_distance: cell_size * 2.0,
            unload_distance: cell_size * 3.0,
            current_cell: IVec2::ZERO,
            loaded_cells: HashSet::new(),
            streaming_state: StreamingState { loading_queue: Vec::new(), unloading_queue: Vec::new(), async_loads: 0, max_async_loads: 4 },
        }
    }

    pub fn update(&mut self, player_position: Vec3) {
        let new_cell = self.world_to_cell(player_position);
        if new_cell != self.current_cell {
            self.current_cell = new_cell;
            self.update_streaming();
        }
        self.process_queue();
    }

    fn world_to_cell(&self, pos: Vec3) -> IVec2 {
        IVec2::new((pos.x / self.cell_size).floor() as i32, (pos.z / self.cell_size).floor() as i32)
    }

    fn update_streaming(&mut self) {
        let radius = (self.load_distance / self.cell_size).ceil() as i32;
        let mut to_load = HashSet::new();

        for dx in -radius..=radius {
            for dz in -radius..=radius {
                let cell = self.current_cell + IVec2::new(dx, dz);
                let dist = ((dx * dx + dz * dz) as f32).sqrt() * self.cell_size;
                if dist <= self.load_distance {
                    to_load.insert(cell);
                }
            }
        }

        // Queue loads
        for cell in &to_load {
            if !self.loaded_cells.contains(cell) && !self.streaming_state.loading_queue.contains(cell) {
                self.streaming_state.loading_queue.push(*cell);
            }
        }

        // Queue unloads
        for cell in self.loaded_cells.iter() {
            let dist = ((*cell - self.current_cell).as_vec2().length()) * self.cell_size;
            if dist > self.unload_distance {
                self.streaming_state.unloading_queue.push(*cell);
            }
        }
    }

    fn process_queue(&mut self) {
        // Process loads
        while self.streaming_state.async_loads < self.streaming_state.max_async_loads {
            if let Some(cell) = self.streaming_state.loading_queue.pop() {
                self.load_cell(cell);
                self.streaming_state.async_loads += 1;
            } else { break; }
        }

        // Process unloads
        for cell in self.streaming_state.unloading_queue.drain(..).collect::<Vec<_>>() {
            self.unload_cell(cell);
        }
    }

    fn load_cell(&mut self, coord: IVec2) {
        if let Some(cell) = self.cells.get_mut(&coord) {
            cell.state = CellState::Loading;
            // Would async load assets
            cell.state = CellState::Loaded;
            self.loaded_cells.insert(coord);
        }
        self.streaming_state.async_loads = self.streaming_state.async_loads.saturating_sub(1);
    }

    fn unload_cell(&mut self, coord: IVec2) {
        if let Some(cell) = self.cells.get_mut(&coord) {
            cell.state = CellState::Unloading;
            // Would unload assets
            cell.state = CellState::Unloaded;
            self.loaded_cells.remove(&coord);
        }
    }

    pub fn add_cell(&mut self, coord: IVec2) {
        let min = Vec3::new(coord.x as f32 * self.cell_size, -1000.0, coord.y as f32 * self.cell_size);
        let max = min + Vec3::new(self.cell_size, 2000.0, self.cell_size);
        self.cells.insert(coord, WorldCell { coord, layers: Vec::new(), state: CellState::Unloaded, bounds: CellBounds { min, max }, priority: 0 });
    }

    pub fn loaded_count(&self) -> usize { self.loaded_cells.len() }
}

/// Level streaming
pub struct LevelStreaming {
    pub levels: Vec<StreamingLevel>,
    pub loaded: HashSet<String>,
}

/// Streaming level
pub struct StreamingLevel {
    pub name: String,
    pub path: String,
    pub bounds: LevelBounds,
    pub always_loaded: bool,
    pub blueprint_class: Option<String>,
}

/// Level bounds
pub struct LevelBounds {
    pub center: Vec3,
    pub extents: Vec3,
}

impl LevelStreaming {
    pub fn new() -> Self { Self { levels: Vec::new(), loaded: HashSet::new() } }

    pub fn load(&mut self, name: &str) -> Result<(), String> {
        if self.loaded.contains(name) { return Ok(()); }
        if self.levels.iter().any(|l| l.name == name) {
            self.loaded.insert(name.into());
            Ok(())
        } else { Err("Level not found".into()) }
    }

    pub fn unload(&mut self, name: &str) {
        self.loaded.remove(name);
    }

    pub fn is_loaded(&self, name: &str) -> bool { self.loaded.contains(name) }
}
