//! Save System
//!
//! Cloud saves, slots, auto-save, and cross-platform sync.

use std::collections::HashMap;
use std::path::PathBuf;

/// Save system
pub struct SaveSystem {
    pub slots: Vec<SaveSlot>,
    pub max_slots: usize,
    pub auto_save: AutoSaveConfig,
    pub cloud: CloudSaveConfig,
    pub current_slot: Option<usize>,
}

/// Save slot
pub struct SaveSlot {
    pub index: usize,
    pub name: String,
    pub timestamp: u64,
    pub play_time: f64,
    pub thumbnail: Option<Vec<u8>>,
    pub metadata: SaveMetadata,
    pub data: Vec<u8>,
    pub cloud_synced: bool,
}

/// Save metadata
pub struct SaveMetadata {
    pub version: u32,
    pub level: String,
    pub progress: f32,
    pub custom: HashMap<String, String>,
}

/// Auto-save config
pub struct AutoSaveConfig {
    pub enabled: bool,
    pub interval_seconds: f32,
    pub slot_index: usize,
    pub on_checkpoint: bool,
    pub on_level_change: bool,
    pub last_save: f32,
}

/// Cloud save config
pub struct CloudSaveConfig {
    pub enabled: bool,
    pub provider: CloudProvider,
    pub sync_on_save: bool,
    pub sync_on_load: bool,
    pub conflict_resolution: ConflictResolution,
}

/// Cloud provider
pub enum CloudProvider { Steam, PlayStation, Xbox, Nintendo, Epic, Custom(String) }

/// Conflict resolution
pub enum ConflictResolution { UseLocal, UseCloud, UseMostRecent, AskUser }

impl Default for AutoSaveConfig {
    fn default() -> Self {
        Self { enabled: true, interval_seconds: 300.0, slot_index: 0, on_checkpoint: true, on_level_change: true, last_save: 0.0 }
    }
}

impl Default for CloudSaveConfig {
    fn default() -> Self {
        Self { enabled: true, provider: CloudProvider::Steam, sync_on_save: true, sync_on_load: true, conflict_resolution: ConflictResolution::UseMostRecent }
    }
}

impl SaveSystem {
    pub fn new(max_slots: usize) -> Self {
        Self { slots: Vec::new(), max_slots, auto_save: AutoSaveConfig::default(), cloud: CloudSaveConfig::default(), current_slot: None }
    }

    pub fn save(&mut self, slot: usize, name: &str, data: Vec<u8>, metadata: SaveMetadata) -> Result<(), String> {
        if slot >= self.max_slots { return Err("Invalid slot".into()); }
        
        // Remove existing slot if present
        self.slots.retain(|s| s.index != slot);
        
        self.slots.push(SaveSlot {
            index: slot, name: name.into(), timestamp: 0, play_time: 0.0, thumbnail: None,
            metadata, data, cloud_synced: false,
        });
        
        self.current_slot = Some(slot);
        
        if self.cloud.enabled && self.cloud.sync_on_save {
            self.sync_to_cloud(slot)?;
        }
        
        Ok(())
    }

    pub fn load(&mut self, slot: usize) -> Result<&SaveSlot, String> {
        if self.cloud.enabled && self.cloud.sync_on_load {
            self.sync_from_cloud(slot)?;
        }
        
        self.slots.iter().find(|s| s.index == slot).ok_or("Slot not found".into())
    }

    pub fn delete(&mut self, slot: usize) -> Result<(), String> {
        self.slots.retain(|s| s.index != slot);
        Ok(())
    }

    pub fn auto_save_tick(&mut self, dt: f32, game_data: Vec<u8>, metadata: SaveMetadata) {
        if !self.auto_save.enabled { return; }
        
        self.auto_save.last_save += dt;
        if self.auto_save.last_save >= self.auto_save.interval_seconds {
            let _ = self.save(self.auto_save.slot_index, "Auto Save", game_data, metadata);
            self.auto_save.last_save = 0.0;
        }
    }

    pub fn checkpoint(&mut self, game_data: Vec<u8>, metadata: SaveMetadata) {
        if self.auto_save.on_checkpoint {
            let _ = self.save(self.auto_save.slot_index, "Checkpoint", game_data, metadata);
        }
    }

    fn sync_to_cloud(&mut self, slot: usize) -> Result<(), String> {
        // Would upload to cloud provider
        if let Some(s) = self.slots.iter_mut().find(|s| s.index == slot) {
            s.cloud_synced = true;
        }
        Ok(())
    }

    fn sync_from_cloud(&mut self, slot: usize) -> Result<(), String> {
        // Would download from cloud provider
        Ok(())
    }

    pub fn get_slots(&self) -> &[SaveSlot] { &self.slots }
    pub fn has_save(&self, slot: usize) -> bool { self.slots.iter().any(|s| s.index == slot) }
}

/// Quick save/load
impl SaveSystem {
    pub fn quick_save(&mut self, data: Vec<u8>, metadata: SaveMetadata) -> Result<(), String> {
        self.save(self.max_slots - 1, "Quick Save", data, metadata)
    }

    pub fn quick_load(&mut self) -> Result<&SaveSlot, String> {
        self.load(self.max_slots - 1)
    }
}
