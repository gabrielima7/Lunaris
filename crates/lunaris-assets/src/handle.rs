//! Asset handles and identifiers

use lunaris_core::id::Id;
use std::sync::Arc;

/// Unique identifier for an asset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetId(pub Id);

impl AssetId {
    /// Generate a new asset ID
    #[must_use]
    pub fn new() -> Self {
        Self(Id::new())
    }

    /// Create from path hash
    #[must_use]
    pub fn from_path(path: &str) -> Self {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        path.hash(&mut hasher);
        Self(Id::from_raw(hasher.finish()))
    }
}

impl Default for AssetId {
    fn default() -> Self {
        Self::new()
    }
}

/// Asset loading state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetState {
    /// Not loaded
    NotLoaded,
    /// Currently loading
    Loading,
    /// Successfully loaded
    Loaded,
    /// Failed to load
    Failed,
    /// Unloaded (was loaded, now released)
    Unloaded,
}

/// Handle to a loaded asset
#[derive(Debug)]
pub struct AssetHandle<T> {
    /// Asset ID
    pub id: AssetId,
    /// Asset path
    pub path: String,
    /// Current state
    pub state: AssetState,
    /// The actual asset data (None if not loaded)
    pub data: Option<Arc<T>>,
}

impl<T> AssetHandle<T> {
    /// Create a new handle for an asset path
    #[must_use]
    pub fn new(path: impl Into<String>) -> Self {
        let path = path.into();
        Self {
            id: AssetId::from_path(&path),
            path,
            state: AssetState::NotLoaded,
            data: None,
        }
    }

    /// Check if the asset is loaded
    #[must_use]
    pub fn is_loaded(&self) -> bool {
        self.state == AssetState::Loaded && self.data.is_some()
    }

    /// Get the asset data (if loaded)
    #[must_use]
    pub fn get(&self) -> Option<&Arc<T>> {
        self.data.as_ref()
    }

    /// Get the asset path
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }
}

impl<T> Clone for AssetHandle<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            path: self.path.clone(),
            state: self.state,
            data: self.data.clone(),
        }
    }
}

/// Strong handle that keeps asset loaded
pub type StrongHandle<T> = AssetHandle<T>;

/// Weak handle that doesn't keep asset loaded
#[derive(Debug, Clone)]
pub struct WeakHandle {
    /// Asset ID
    pub id: AssetId,
    /// Asset path
    pub path: String,
}

impl WeakHandle {
    /// Create a weak handle from a strong handle
    #[must_use]
    pub fn from_strong<T>(handle: &AssetHandle<T>) -> Self {
        Self {
            id: handle.id,
            path: handle.path.clone(),
        }
    }
}
