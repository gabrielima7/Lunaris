//! Resource Management
//!
//! Memory pools, streaming, and resource lifecycle.

use lunaris_core::id::Id;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Resource handle
#[derive(Debug, Clone)]
pub struct ResourceHandle<T> {
    id: Id,
    data: Arc<RwLock<Option<T>>>,
    state: Arc<RwLock<ResourceState>>,
}

impl<T> ResourceHandle<T> {
    /// Check if loaded
    #[must_use]
    pub fn is_loaded(&self) -> bool {
        matches!(*self.state.read().unwrap(), ResourceState::Loaded)
    }

    /// Get resource (if loaded)
    #[must_use]
    pub fn get(&self) -> Option<impl std::ops::Deref<Target = T> + '_> {
        let guard = self.data.read().ok()?;
        if guard.is_some() {
            Some(std::sync::RwLockReadGuard::map(guard, |opt| opt.as_ref().unwrap()))
        } else {
            None
        }
    }

    /// Get resource ID
    #[must_use]
    pub fn id(&self) -> Id {
        self.id
    }
}

/// Resource loading state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResourceState {
    /// Not loaded
    #[default]
    Unloaded,
    /// Loading in progress
    Loading,
    /// Loaded and ready
    Loaded,
    /// Failed to load
    Failed,
    /// Unloading
    Unloading,
}

/// Resource priority for streaming
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ResourcePriority {
    /// Critical (always loaded)
    Critical = 0,
    /// High priority
    High = 1,
    /// Normal priority
    #[default]
    Normal = 2,
    /// Low priority
    Low = 3,
    /// Background (stream when idle)
    Background = 4,
}

/// Memory pool for fast allocations
pub struct MemoryPool<T: Default + Clone> {
    /// Pool data
    data: Vec<T>,
    /// Free indices
    free: Vec<usize>,
    /// Pool capacity
    capacity: usize,
}

impl<T: Default + Clone> MemoryPool<T> {
    /// Create a new memory pool
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![T::default(); capacity],
            free: (0..capacity).rev().collect(),
            capacity,
        }
    }

    /// Allocate from pool
    pub fn allocate(&mut self) -> Option<usize> {
        self.free.pop()
    }

    /// Free to pool
    pub fn free(&mut self, index: usize) {
        if index < self.capacity && !self.free.contains(&index) {
            self.free.push(index);
            self.data[index] = T::default();
        }
    }

    /// Get item
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    /// Get item mutably
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }

    /// Available slots
    #[must_use]
    pub fn available(&self) -> usize {
        self.free.len()
    }

    /// Used slots
    #[must_use]
    pub fn used(&self) -> usize {
        self.capacity - self.free.len()
    }

    /// Clear pool
    pub fn clear(&mut self) {
        self.free = (0..self.capacity).rev().collect();
        for item in &mut self.data {
            *item = T::default();
        }
    }
}

/// Streaming chunk
#[derive(Debug, Clone)]
pub struct StreamingChunk {
    /// Chunk ID
    pub id: u64,
    /// Position (for distance-based streaming)
    pub position: lunaris_core::math::Vec3,
    /// Radius
    pub radius: f32,
    /// Priority
    pub priority: ResourcePriority,
    /// State
    pub state: ResourceState,
    /// Resources in this chunk
    pub resources: Vec<Id>,
}

/// Streaming system
pub struct StreamingSystem {
    /// Chunks
    chunks: HashMap<u64, StreamingChunk>,
    /// Loaded chunks
    loaded: Vec<u64>,
    /// Loading queue
    loading_queue: Vec<u64>,
    /// Max concurrent loads
    max_concurrent: usize,
    /// Streaming distance
    streaming_distance: f32,
    /// Reference position
    reference_position: lunaris_core::math::Vec3,
}

impl Default for StreamingSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamingSystem {
    /// Create a new streaming system
    #[must_use]
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            loaded: Vec::new(),
            loading_queue: Vec::new(),
            max_concurrent: 4,
            streaming_distance: 100.0,
            reference_position: lunaris_core::math::Vec3::ZERO,
        }
    }

    /// Set streaming distance
    pub fn set_streaming_distance(&mut self, distance: f32) {
        self.streaming_distance = distance;
    }

    /// Add a streaming chunk
    pub fn add_chunk(&mut self, chunk: StreamingChunk) {
        self.chunks.insert(chunk.id, chunk);
    }

    /// Remove a chunk
    pub fn remove_chunk(&mut self, id: u64) {
        self.chunks.remove(&id);
        self.loaded.retain(|&x| x != id);
        self.loading_queue.retain(|&x| x != id);
    }

    /// Update reference position (usually camera)
    pub fn update_reference(&mut self, position: lunaris_core::math::Vec3) {
        self.reference_position = position;
    }

    /// Update streaming
    pub fn update(&mut self) -> StreamingResult {
        let mut to_load = Vec::new();
        let mut to_unload = Vec::new();

        // Check each chunk
        for (id, chunk) in &mut self.chunks {
            let distance = (chunk.position - self.reference_position).length();
            let in_range = distance < self.streaming_distance + chunk.radius;

            match chunk.state {
                ResourceState::Unloaded if in_range => {
                    to_load.push(*id);
                }
                ResourceState::Loaded if !in_range => {
                    to_unload.push(*id);
                }
                _ => {}
            }
        }

        // Sort by priority and distance
        to_load.sort_by(|a, b| {
            let chunk_a = self.chunks.get(a).unwrap();
            let chunk_b = self.chunks.get(b).unwrap();
            
            chunk_a.priority.cmp(&chunk_b.priority)
                .then_with(|| {
                    let dist_a = (chunk_a.position - self.reference_position).length();
                    let dist_b = (chunk_b.position - self.reference_position).length();
                    dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
                })
        });

        // Queue loading
        for id in to_load.iter().take(self.max_concurrent - self.loading_queue.len()) {
            if !self.loading_queue.contains(id) {
                self.loading_queue.push(*id);
                if let Some(chunk) = self.chunks.get_mut(id) {
                    chunk.state = ResourceState::Loading;
                }
            }
        }

        StreamingResult {
            to_load: to_load.into_iter().take(self.max_concurrent).collect(),
            to_unload,
            loading: self.loading_queue.len(),
            loaded: self.loaded.len(),
        }
    }

    /// Mark chunk as loaded
    pub fn mark_loaded(&mut self, id: u64) {
        if let Some(chunk) = self.chunks.get_mut(&id) {
            chunk.state = ResourceState::Loaded;
            self.loaded.push(id);
            self.loading_queue.retain(|&x| x != id);
        }
    }

    /// Mark chunk as unloaded
    pub fn mark_unloaded(&mut self, id: u64) {
        if let Some(chunk) = self.chunks.get_mut(&id) {
            chunk.state = ResourceState::Unloaded;
            self.loaded.retain(|&x| x != id);
        }
    }
}

/// Streaming update result
#[derive(Debug, Clone, Default)]
pub struct StreamingResult {
    /// Chunks to load
    pub to_load: Vec<u64>,
    /// Chunks to unload
    pub to_unload: Vec<u64>,
    /// Currently loading count
    pub loading: usize,
    /// Currently loaded count
    pub loaded: usize,
}

/// Resource cache with LRU eviction
pub struct ResourceCache<T> {
    /// Cached items
    cache: HashMap<Id, CacheEntry<T>>,
    /// Access order (most recent at back)
    access_order: Vec<Id>,
    /// Max cache size
    max_size: usize,
}

struct CacheEntry<T> {
    data: T,
    size: usize,
}

impl<T> ResourceCache<T> {
    /// Create a new cache
    #[must_use]
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            access_order: Vec::new(),
            max_size,
        }
    }

    /// Insert into cache
    pub fn insert(&mut self, id: Id, data: T, size: usize) {
        // Evict if needed
        while self.current_size() + size > self.max_size && !self.access_order.is_empty() {
            let oldest = self.access_order.remove(0);
            self.cache.remove(&oldest);
        }

        self.cache.insert(id, CacheEntry { data, size });
        self.access_order.push(id);
    }

    /// Get from cache
    #[must_use]
    pub fn get(&mut self, id: &Id) -> Option<&T> {
        if self.cache.contains_key(id) {
            // Update access order
            self.access_order.retain(|x| x != id);
            self.access_order.push(*id);
            self.cache.get(id).map(|e| &e.data)
        } else {
            None
        }
    }

    /// Remove from cache
    pub fn remove(&mut self, id: &Id) {
        self.cache.remove(id);
        self.access_order.retain(|x| x != id);
    }

    /// Current size
    #[must_use]
    pub fn current_size(&self) -> usize {
        self.cache.values().map(|e| e.size).sum()
    }

    /// Clear cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.access_order.clear();
    }
}
