//! Asset manager for loading and caching assets

use crate::{AssetHandle, AssetId, AssetState, AssetType};
use lunaris_core::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Asset manager handles loading, caching, and unloading of assets
pub struct AssetManager {
    /// Base path for assets
    base_path: PathBuf,
    /// Asset metadata cache
    metadata: HashMap<AssetId, AssetMetadata>,
    /// Pending load requests
    pending: Vec<AssetId>,
    /// Hot reload enabled
    hot_reload: bool,
}

/// Asset metadata
#[derive(Debug, Clone)]
struct AssetMetadata {
    path: String,
    asset_type: AssetType,
    state: AssetState,
    load_time: Option<std::time::Instant>,
    file_modified: Option<std::time::SystemTime>,
}

impl AssetManager {
    /// Create a new asset manager
    #[must_use]
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
            metadata: HashMap::new(),
            pending: Vec::new(),
            hot_reload: cfg!(debug_assertions),
        }
    }

    /// Set the base path for assets
    pub fn set_base_path(&mut self, path: impl Into<PathBuf>) {
        self.base_path = path.into();
    }

    /// Get the base path
    #[must_use]
    pub fn base_path(&self) -> &Path {
        &self.base_path
    }

    /// Enable or disable hot reloading
    pub fn set_hot_reload(&mut self, enabled: bool) {
        self.hot_reload = enabled;
    }

    /// Request an asset to be loaded
    pub fn load<T>(&mut self, path: &str) -> AssetHandle<T> {
        let full_path = self.base_path.join(path);
        let id = AssetId::from_path(path);

        // Check if already tracked
        if !self.metadata.contains_key(&id) {
            let asset_type = full_path
                .extension()
                .and_then(|e| e.to_str())
                .and_then(AssetType::from_extension)
                .unwrap_or(AssetType::Binary);

            self.metadata.insert(
                id,
                AssetMetadata {
                    path: path.to_string(),
                    asset_type,
                    state: AssetState::NotLoaded,
                    load_time: None,
                    file_modified: None,
                },
            );

            self.pending.push(id);
        }

        AssetHandle {
            id,
            path: path.to_string(),
            state: AssetState::Loading,
            data: None,
        }
    }

    /// Load an asset synchronously
    pub fn load_sync<T: Default>(&mut self, path: &str) -> Result<AssetHandle<T>> {
        let full_path = self.base_path.join(path);
        let id = AssetId::from_path(path);

        // Read file
        let bytes = std::fs::read(&full_path)
            .map_err(|e| lunaris_core::Error::Asset(format!("Failed to read {}: {}", path, e)))?;

        tracing::info!("Loaded asset: {} ({} bytes)", path, bytes.len());

        let asset_type = full_path
            .extension()
            .and_then(|e| e.to_str())
            .and_then(AssetType::from_extension)
            .unwrap_or(AssetType::Binary);

        self.metadata.insert(
            id,
            AssetMetadata {
                path: path.to_string(),
                asset_type,
                state: AssetState::Loaded,
                load_time: Some(std::time::Instant::now()),
                file_modified: std::fs::metadata(&full_path).ok().and_then(|m| m.modified().ok()),
            },
        );

        // In real implementation, would use appropriate loader
        Ok(AssetHandle {
            id,
            path: path.to_string(),
            state: AssetState::Loaded,
            data: Some(Arc::new(T::default())),
        })
    }

    /// Unload an asset
    pub fn unload(&mut self, id: AssetId) {
        if let Some(meta) = self.metadata.get_mut(&id) {
            meta.state = AssetState::Unloaded;
        }
    }

    /// Check if an asset is loaded
    #[must_use]
    pub fn is_loaded(&self, id: AssetId) -> bool {
        self.metadata
            .get(&id)
            .map(|m| m.state == AssetState::Loaded)
            .unwrap_or(false)
    }

    /// Get asset state
    #[must_use]
    pub fn get_state(&self, id: AssetId) -> AssetState {
        self.metadata
            .get(&id)
            .map(|m| m.state)
            .unwrap_or(AssetState::NotLoaded)
    }

    /// Process pending loads (call each frame)
    pub fn update(&mut self) {
        // Process pending loads
        let pending: Vec<_> = self.pending.drain(..).collect();
        for id in pending {
            if let Some(meta) = self.metadata.get_mut(&id) {
                let full_path = self.base_path.join(&meta.path);
                
                match std::fs::read(&full_path) {
                    Ok(bytes) => {
                        tracing::debug!("Loaded: {} ({} bytes)", meta.path, bytes.len());
                        meta.state = AssetState::Loaded;
                        meta.load_time = Some(std::time::Instant::now());
                        meta.file_modified = std::fs::metadata(&full_path)
                            .ok()
                            .and_then(|m| m.modified().ok());
                    }
                    Err(e) => {
                        tracing::error!("Failed to load {}: {}", meta.path, e);
                        meta.state = AssetState::Failed;
                    }
                }
            }
        }

        // Check for hot reload
        if self.hot_reload {
            self.check_hot_reload();
        }
    }

    /// Check for modified files and reload
    fn check_hot_reload(&mut self) {
        let mut to_reload = Vec::new();

        for (id, meta) in &self.metadata {
            if meta.state != AssetState::Loaded {
                continue;
            }

            let full_path = self.base_path.join(&meta.path);
            if let Ok(file_meta) = std::fs::metadata(&full_path) {
                if let Ok(modified) = file_meta.modified() {
                    if let Some(old_modified) = meta.file_modified {
                        if modified > old_modified {
                            to_reload.push(*id);
                        }
                    }
                }
            }
        }

        for id in to_reload {
            if let Some(meta) = self.metadata.get_mut(&id) {
                tracing::info!("Hot reloading: {}", meta.path);
                meta.state = AssetState::NotLoaded;
                self.pending.push(id);
            }
        }
    }

    /// Get number of loaded assets
    #[must_use]
    pub fn loaded_count(&self) -> usize {
        self.metadata
            .values()
            .filter(|m| m.state == AssetState::Loaded)
            .count()
    }

    /// Get total number of tracked assets
    #[must_use]
    pub fn total_count(&self) -> usize {
        self.metadata.len()
    }
}

impl Default for AssetManager {
    fn default() -> Self {
        Self::new("assets")
    }
}
