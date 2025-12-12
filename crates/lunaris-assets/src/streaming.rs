//! Asset Streaming System
//!
//! Background streaming of assets with priority and LOD management.

use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;

/// Streaming priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StreamPriority {
    /// Immediate - block until loaded
    Immediate = 0,
    /// Critical - load next frame
    Critical = 1,
    /// High - load soon
    High = 2,
    /// Normal - background load
    Normal = 3,
    /// Low - load when idle
    Low = 4,
    /// Prefetch - speculative load
    Prefetch = 5,
}

/// Streaming state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamState {
    /// Not loaded
    Unloaded,
    /// Queued for load
    Queued,
    /// Currently loading
    Loading,
    /// Loaded and ready
    Loaded,
    /// Load failed
    Failed,
    /// Queued for unload
    Unloading,
}

/// Streamable asset
#[derive(Debug, Clone)]
pub struct StreamableAsset {
    /// Asset ID
    pub id: u64,
    /// Path
    pub path: PathBuf,
    /// Asset type
    pub asset_type: StreamAssetType,
    /// State
    pub state: StreamState,
    /// Priority
    pub priority: StreamPriority,
    /// Size in memory (bytes)
    pub memory_size: u64,
    /// Size on disk (bytes)
    pub disk_size: u64,
    /// LOD levels available
    pub lod_levels: u8,
    /// Currently loaded LOD
    pub loaded_lod: Option<u8>,
    /// Last access time
    pub last_access: std::time::Instant,
    /// Reference count
    pub ref_count: u32,
    /// Distance to camera (for priority)
    pub distance: f32,
}

/// Streamable asset type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamAssetType {
    Texture,
    Mesh,
    Audio,
    Animation,
    Material,
    Prefab,
    Scene,
    Terrain,
    Video,
}

/// Streaming request
#[derive(Debug, Clone)]
pub struct StreamRequest {
    /// Asset ID
    pub asset_id: u64,
    /// Target LOD
    pub target_lod: u8,
    /// Priority
    pub priority: StreamPriority,
    /// Callback on complete
    pub on_complete: Option<fn(u64, bool)>,
}

/// Streaming manager
pub struct StreamingManager {
    /// All assets
    assets: HashMap<u64, StreamableAsset>,
    /// Load queue
    load_queue: VecDeque<StreamRequest>,
    /// Unload queue
    unload_queue: Vec<u64>,
    /// Currently loading
    loading: Vec<u64>,
    /// Next ID
    next_id: u64,
    /// Memory budget
    pub memory_budget: u64,
    /// Current memory
    pub current_memory: u64,
    /// Max concurrent loads
    pub max_concurrent: usize,
    /// Bytes per frame limit
    pub bytes_per_frame: u64,
    /// Distance for unload
    pub unload_distance: f32,
    /// Min time before unload (seconds)
    pub min_loaded_time: f32,
}

impl Default for StreamingManager {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamingManager {
    /// Create new manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
            load_queue: VecDeque::new(),
            unload_queue: Vec::new(),
            loading: Vec::new(),
            next_id: 1,
            memory_budget: 4 * 1024 * 1024 * 1024, // 4GB
            current_memory: 0,
            max_concurrent: 8,
            bytes_per_frame: 32 * 1024 * 1024, // 32MB per frame
            unload_distance: 500.0,
            min_loaded_time: 5.0,
        }
    }

    /// Register asset
    pub fn register(&mut self, path: PathBuf, asset_type: StreamAssetType, disk_size: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let asset = StreamableAsset {
            id,
            path,
            asset_type,
            state: StreamState::Unloaded,
            priority: StreamPriority::Normal,
            memory_size: 0,
            disk_size,
            lod_levels: 1,
            loaded_lod: None,
            last_access: std::time::Instant::now(),
            ref_count: 0,
            distance: f32::MAX,
        };

        self.assets.insert(id, asset);
        id
    }

    /// Request load
    pub fn request(&mut self, asset_id: u64, priority: StreamPriority) {
        if let Some(asset) = self.assets.get_mut(&asset_id) {
            if asset.state == StreamState::Unloaded || asset.state == StreamState::Failed {
                asset.state = StreamState::Queued;
                asset.priority = priority;
                self.load_queue.push_back(StreamRequest {
                    asset_id,
                    target_lod: 0,
                    priority,
                    on_complete: None,
                });
            }
        }
    }

    /// Request load with callback
    pub fn request_with_callback(
        &mut self,
        asset_id: u64,
        priority: StreamPriority,
        callback: fn(u64, bool),
    ) {
        if let Some(asset) = self.assets.get_mut(&asset_id) {
            if asset.state == StreamState::Unloaded || asset.state == StreamState::Failed {
                asset.state = StreamState::Queued;
                asset.priority = priority;
                self.load_queue.push_back(StreamRequest {
                    asset_id,
                    target_lod: 0,
                    priority,
                    on_complete: Some(callback),
                });
            }
        }
    }

    /// Update streaming
    pub fn update(&mut self, _camera_position: glam::Vec3) {
        // Update distances
        for asset in self.assets.values_mut() {
            // Simplified - would use asset position
            asset.distance = 100.0; // Placeholder
        }

        // Sort queue by priority and distance
        let mut queue_vec: Vec<_> = self.load_queue.drain(..).collect();
        queue_vec.sort_by(|a, b| {
            a.priority.cmp(&b.priority).then_with(|| {
                let dist_a = self.assets.get(&a.asset_id).map(|a| a.distance).unwrap_or(f32::MAX);
                let dist_b = self.assets.get(&b.asset_id).map(|a| a.distance).unwrap_or(f32::MAX);
                dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
            })
        });
        self.load_queue = queue_vec.into();

        // Process loads
        let mut bytes_this_frame = 0u64;

        while self.loading.len() < self.max_concurrent && !self.load_queue.is_empty() {
            if let Some(request) = self.load_queue.pop_front() {
                if let Some(asset) = self.assets.get_mut(&request.asset_id) {
                    if bytes_this_frame + asset.disk_size <= self.bytes_per_frame {
                        asset.state = StreamState::Loading;
                        self.loading.push(request.asset_id);
                        bytes_this_frame += asset.disk_size;
                    } else {
                        // Put back in queue
                        self.load_queue.push_front(request);
                        break;
                    }
                }
            }
        }

        // Complete loads (simulated)
        let completed: Vec<_> = self.loading.drain(..).collect();
        for id in completed {
            if let Some(asset) = self.assets.get_mut(&id) {
                asset.state = StreamState::Loaded;
                asset.loaded_lod = Some(0);
                asset.memory_size = asset.disk_size; // Simplified
                asset.last_access = std::time::Instant::now();
                self.current_memory += asset.memory_size;
            }
        }

        // Check for unloads
        self.check_unloads();
    }

    fn check_unloads(&mut self) {
        // Find assets to unload
        if self.current_memory > self.memory_budget {
            let now = std::time::Instant::now();
            let mut candidates: Vec<_> = self.assets.values()
                .filter(|a| {
                    a.state == StreamState::Loaded 
                    && a.ref_count == 0
                    && now.duration_since(a.last_access).as_secs_f32() > self.min_loaded_time
                })
                .map(|a| (a.id, a.distance, a.last_access))
                .collect();

            // Sort by distance (furthest first)
            candidates.sort_by(|a, b| {
                b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
            });

            // Unload until under budget
            for (id, _, _) in candidates {
                if self.current_memory <= self.memory_budget {
                    break;
                }
                if let Some(asset) = self.assets.get_mut(&id) {
                    self.current_memory -= asset.memory_size;
                    asset.state = StreamState::Unloaded;
                    asset.loaded_lod = None;
                    asset.memory_size = 0;
                }
            }
        }
    }

    /// Get asset
    #[must_use]
    pub fn get(&self, id: u64) -> Option<&StreamableAsset> {
        self.assets.get(&id)
    }

    /// Is asset loaded
    #[must_use]
    pub fn is_loaded(&self, id: u64) -> bool {
        self.assets.get(&id).map_or(false, |a| a.state == StreamState::Loaded)
    }

    /// Add reference
    pub fn add_ref(&mut self, id: u64) {
        if let Some(asset) = self.assets.get_mut(&id) {
            asset.ref_count += 1;
            asset.last_access = std::time::Instant::now();
        }
    }

    /// Remove reference
    pub fn release(&mut self, id: u64) {
        if let Some(asset) = self.assets.get_mut(&id) {
            asset.ref_count = asset.ref_count.saturating_sub(1);
        }
    }

    /// Get statistics
    #[must_use]
    pub fn stats(&self) -> StreamingStats {
        StreamingStats {
            total_assets: self.assets.len(),
            loaded_assets: self.assets.values().filter(|a| a.state == StreamState::Loaded).count(),
            loading_assets: self.loading.len(),
            queued_assets: self.load_queue.len(),
            memory_used: self.current_memory,
            memory_budget: self.memory_budget,
        }
    }
}

/// Streaming statistics
#[derive(Debug, Clone)]
pub struct StreamingStats {
    pub total_assets: usize,
    pub loaded_assets: usize,
    pub loading_assets: usize,
    pub queued_assets: usize,
    pub memory_used: u64,
    pub memory_budget: u64,
}
