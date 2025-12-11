//! Save/Load System
//!
//! Game state serialization for saves and checkpoints.

use lunaris_core::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Save file metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveMetadata {
    /// Save name
    pub name: String,
    /// Save description
    pub description: String,
    /// Timestamp
    pub timestamp: u64,
    /// Play time (seconds)
    pub play_time: u64,
    /// Game version
    pub game_version: String,
    /// Save version
    pub save_version: u32,
    /// Screenshot path (if any)
    pub screenshot: Option<String>,
    /// Custom metadata
    pub custom: HashMap<String, String>,
}

impl Default for SaveMetadata {
    fn default() -> Self {
        Self {
            name: String::from("Save"),
            description: String::new(),
            timestamp: 0,
            play_time: 0,
            game_version: String::from("1.0.0"),
            save_version: 1,
            screenshot: None,
            custom: HashMap::new(),
        }
    }
}

/// Save data container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    /// Metadata
    pub metadata: SaveMetadata,
    /// Scene data
    pub scene: String,
    /// Player data
    pub player: PlayerSaveData,
    /// World data
    pub world: WorldSaveData,
    /// Custom data sections
    pub sections: HashMap<String, serde_json::Value>,
}

/// Player save data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlayerSaveData {
    /// Position
    pub position: [f32; 3],
    /// Rotation
    pub rotation: [f32; 3],
    /// Health
    pub health: f32,
    /// Inventory
    pub inventory: Vec<InventoryItem>,
    /// Stats
    pub stats: HashMap<String, f32>,
    /// Unlocked abilities
    pub abilities: Vec<String>,
}

/// Inventory item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    /// Item ID
    pub id: String,
    /// Item count
    pub count: u32,
    /// Item data
    pub data: HashMap<String, serde_json::Value>,
}

/// World save data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorldSaveData {
    /// World time
    pub time: f32,
    /// Weather state
    pub weather: String,
    /// Destroyed objects
    pub destroyed: Vec<u64>,
    /// Spawned objects
    pub spawned: Vec<SpawnedObject>,
    /// Quest states
    pub quests: HashMap<String, QuestState>,
    /// World flags
    pub flags: HashMap<String, bool>,
}

/// Spawned object data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnedObject {
    /// Object type
    pub object_type: String,
    /// Position
    pub position: [f32; 3],
    /// Rotation
    pub rotation: [f32; 3],
    /// Custom data
    pub data: HashMap<String, serde_json::Value>,
}

/// Quest state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestState {
    /// Quest ID
    pub id: String,
    /// Status
    pub status: QuestStatus,
    /// Current objective
    pub objective: u32,
    /// Progress
    pub progress: HashMap<String, i32>,
}

/// Quest status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestStatus {
    /// Not started
    NotStarted,
    /// In progress
    InProgress,
    /// Completed
    Completed,
    /// Failed
    Failed,
}

impl SaveData {
    /// Create a new save data
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            metadata: SaveMetadata {
                name: name.into(),
                ..Default::default()
            },
            scene: String::new(),
            player: PlayerSaveData::default(),
            world: WorldSaveData::default(),
            sections: HashMap::new(),
        }
    }

    /// Save to file
    ///
    /// # Errors
    ///
    /// Returns error if saving fails
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| lunaris_core::Error::Asset(e.to_string()))?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load from file
    ///
    /// # Errors
    ///
    /// Returns error if loading fails
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        serde_json::from_str(&json).map_err(|e| lunaris_core::Error::Asset(e.to_string()))
    }

    /// Add custom section
    pub fn add_section(&mut self, name: impl Into<String>, data: impl Serialize) {
        if let Ok(value) = serde_json::to_value(data) {
            self.sections.insert(name.into(), value);
        }
    }

    /// Get custom section
    pub fn get_section<T: for<'de> Deserialize<'de>>(&self, name: &str) -> Option<T> {
        self.sections.get(name).and_then(|v| serde_json::from_value(v.clone()).ok())
    }
}

/// Save system
pub struct SaveSystem {
    /// Save directory
    save_dir: String,
    /// Autosave enabled
    pub autosave_enabled: bool,
    /// Autosave interval (seconds)
    pub autosave_interval: f32,
    /// Max saves
    pub max_saves: usize,
    /// Current slot
    current_slot: Option<u32>,
    /// Time since last autosave
    autosave_timer: f32,
}

impl Default for SaveSystem {
    fn default() -> Self {
        Self::new("saves")
    }
}

impl SaveSystem {
    /// Create a new save system
    #[must_use]
    pub fn new(save_dir: &str) -> Self {
        Self {
            save_dir: save_dir.to_string(),
            autosave_enabled: true,
            autosave_interval: 300.0, // 5 minutes
            max_saves: 100,
            current_slot: None,
            autosave_timer: 0.0,
        }
    }

    /// Update autosave timer
    pub fn update(&mut self, delta_time: f32) -> bool {
        if !self.autosave_enabled {
            return false;
        }

        self.autosave_timer += delta_time;
        if self.autosave_timer >= self.autosave_interval {
            self.autosave_timer = 0.0;
            true
        } else {
            false
        }
    }

    /// Get save file path
    #[must_use]
    pub fn save_path(&self, slot: u32) -> String {
        format!("{}/save_{}.json", self.save_dir, slot)
    }

    /// List all saves
    ///
    /// # Errors
    ///
    /// Returns error if reading directory fails
    pub fn list_saves(&self) -> Result<Vec<(u32, SaveMetadata)>> {
        let mut saves = Vec::new();
        let dir = std::path::Path::new(&self.save_dir);
        
        if !dir.exists() {
            return Ok(saves);
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |e| e == "json") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if let Some(slot_str) = stem.strip_prefix("save_") {
                        if let Ok(slot) = slot_str.parse::<u32>() {
                            if let Ok(save) = SaveData::load(&path) {
                                saves.push((slot, save.metadata));
                            }
                        }
                    }
                }
            }
        }

        saves.sort_by(|a, b| b.1.timestamp.cmp(&a.1.timestamp));
        Ok(saves)
    }

    /// Delete a save
    ///
    /// # Errors
    ///
    /// Returns error if deletion fails
    pub fn delete_save(&self, slot: u32) -> Result<()> {
        let path = self.save_path(slot);
        std::fs::remove_file(path)?;
        Ok(())
    }

    /// Get current slot
    #[must_use]
    pub fn current_slot(&self) -> Option<u32> {
        self.current_slot
    }

    /// Set current slot
    pub fn set_slot(&mut self, slot: u32) {
        self.current_slot = Some(slot);
    }
}
