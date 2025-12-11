//! Performance Profiler
//!
//! Real-time profiling and performance analysis.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Profiler scope
#[derive(Debug, Clone)]
pub struct ProfileScope {
    /// Scope name
    pub name: String,
    /// Start time
    pub start: Instant,
    /// End time
    pub end: Option<Instant>,
    /// Parent scope
    pub parent: Option<String>,
    /// Child scopes
    pub children: Vec<String>,
    /// Call count
    pub call_count: u64,
    /// Total time
    pub total_time: Duration,
    /// Min time
    pub min_time: Duration,
    /// Max time
    pub max_time: Duration,
    /// Category
    pub category: ProfileCategory,
}

/// Profile category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProfileCategory {
    #[default]
    /// General
    General,
    /// Rendering
    Rendering,
    /// Physics
    Physics,
    /// Audio
    Audio,
    /// AI
    AI,
    /// Scripting
    Scripting,
    /// Network
    Network,
    /// Animation
    Animation,
    /// UI
    UI,
    /// Loading
    Loading,
}

/// Frame timing data
#[derive(Debug, Clone, Default)]
pub struct FrameTiming {
    /// Frame number
    pub frame: u64,
    /// Total frame time (ms)
    pub total_ms: f32,
    /// CPU time (ms)
    pub cpu_ms: f32,
    /// GPU time (ms)
    pub gpu_ms: f32,
    /// Physics time (ms)
    pub physics_ms: f32,
    /// Render time (ms)
    pub render_ms: f32,
    /// Script time (ms)
    pub script_ms: f32,
    /// Draw calls
    pub draw_calls: u32,
    /// Triangle count
    pub triangles: u64,
    /// Batch count
    pub batches: u32,
    /// Memory used (KB)
    pub memory_kb: u64,
}

/// Performance statistics
#[derive(Debug, Clone, Default)]
pub struct PerformanceStats {
    /// Average FPS
    pub fps: f32,
    /// Min FPS
    pub min_fps: f32,
    /// Max FPS
    pub max_fps: f32,
    /// Frame time histogram (ms buckets)
    pub histogram: [u32; 20],
    /// 99th percentile frame time
    pub p99_ms: f32,
    /// 95th percentile frame time
    pub p95_ms: f32,
    /// CPU usage (0-100%)
    pub cpu_usage: f32,
    /// GPU usage (0-100%)
    pub gpu_usage: f32,
    /// Memory usage (MB)
    pub memory_mb: f32,
    /// GPU memory usage (MB)
    pub gpu_memory_mb: f32,
}

/// Main profiler
pub struct Profiler {
    /// Is enabled
    pub enabled: bool,
    /// Current frame
    frame: u64,
    /// Frame start time
    frame_start: Instant,
    /// Scopes
    scopes: HashMap<String, ProfileScope>,
    /// Active scope stack
    scope_stack: Vec<String>,
    /// Frame history
    frame_history: Vec<FrameTiming>,
    /// Max history size
    max_history: usize,
    /// GPU timing
    gpu_timing_enabled: bool,
    /// Current frame timing
    current_frame: FrameTiming,
    /// Performance stats
    stats: PerformanceStats,
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Profiler {
    /// Create a new profiler
    #[must_use]
    pub fn new() -> Self {
        Self {
            enabled: true,
            frame: 0,
            frame_start: Instant::now(),
            scopes: HashMap::new(),
            scope_stack: Vec::new(),
            frame_history: Vec::new(),
            max_history: 300,
            gpu_timing_enabled: false,
            current_frame: FrameTiming::default(),
            stats: PerformanceStats::default(),
        }
    }

    /// Begin frame
    pub fn begin_frame(&mut self) {
        if !self.enabled {
            return;
        }

        self.frame += 1;
        self.frame_start = Instant::now();
        self.current_frame = FrameTiming {
            frame: self.frame,
            ..Default::default()
        };
    }

    /// End frame
    pub fn end_frame(&mut self) {
        if !self.enabled {
            return;
        }

        let elapsed = self.frame_start.elapsed();
        self.current_frame.total_ms = elapsed.as_secs_f32() * 1000.0;

        // Store frame timing
        self.frame_history.push(self.current_frame.clone());
        if self.frame_history.len() > self.max_history {
            self.frame_history.remove(0);
        }

        // Update stats
        self.update_stats();
    }

    /// Begin a profiling scope
    pub fn begin_scope(&mut self, name: &str, category: ProfileCategory) {
        if !self.enabled {
            return;
        }

        let parent = self.scope_stack.last().cloned();
        
        let scope = self.scopes.entry(name.to_string()).or_insert_with(|| ProfileScope {
            name: name.to_string(),
            start: Instant::now(),
            end: None,
            parent: parent.clone(),
            children: Vec::new(),
            call_count: 0,
            total_time: Duration::ZERO,
            min_time: Duration::MAX,
            max_time: Duration::ZERO,
            category,
        });
        
        scope.start = Instant::now();
        scope.call_count += 1;
        
        if let Some(ref parent_name) = parent {
            if let Some(parent_scope) = self.scopes.get_mut(parent_name) {
                if !parent_scope.children.contains(&name.to_string()) {
                    parent_scope.children.push(name.to_string());
                }
            }
        }
        
        self.scope_stack.push(name.to_string());
    }

    /// End a profiling scope
    pub fn end_scope(&mut self, name: &str) {
        if !self.enabled {
            return;
        }

        if let Some(scope) = self.scopes.get_mut(name) {
            scope.end = Some(Instant::now());
            let elapsed = scope.end.unwrap() - scope.start;
            scope.total_time += elapsed;
            scope.min_time = scope.min_time.min(elapsed);
            scope.max_time = scope.max_time.max(elapsed);

            // Update category timing
            let ms = elapsed.as_secs_f32() * 1000.0;
            match scope.category {
                ProfileCategory::Rendering => self.current_frame.render_ms += ms,
                ProfileCategory::Physics => self.current_frame.physics_ms += ms,
                ProfileCategory::Scripting => self.current_frame.script_ms += ms,
                _ => {}
            }
        }

        self.scope_stack.pop();
    }

    /// Record draw calls
    pub fn record_draw_calls(&mut self, count: u32) {
        self.current_frame.draw_calls += count;
    }

    /// Record triangle count
    pub fn record_triangles(&mut self, count: u64) {
        self.current_frame.triangles += count;
    }

    /// Record batches
    pub fn record_batches(&mut self, count: u32) {
        self.current_frame.batches += count;
    }

    /// Record memory usage
    pub fn record_memory(&mut self, kb: u64) {
        self.current_frame.memory_kb = kb;
    }

    /// Get scope statistics
    #[must_use]
    pub fn get_scope(&self, name: &str) -> Option<&ProfileScope> {
        self.scopes.get(name)
    }

    /// Get all scopes in a category
    #[must_use]
    pub fn get_scopes_by_category(&self, category: ProfileCategory) -> Vec<&ProfileScope> {
        self.scopes.values()
            .filter(|s| s.category == category)
            .collect()
    }

    /// Get frame history
    #[must_use]
    pub fn frame_history(&self) -> &[FrameTiming] {
        &self.frame_history
    }

    /// Get current stats
    #[must_use]
    pub fn stats(&self) -> &PerformanceStats {
        &self.stats
    }

    /// Get current FPS
    #[must_use]
    pub fn fps(&self) -> f32 {
        self.stats.fps
    }

    /// Get current frame time in ms
    #[must_use]
    pub fn frame_time_ms(&self) -> f32 {
        self.current_frame.total_ms
    }

    fn update_stats(&mut self) {
        if self.frame_history.is_empty() {
            return;
        }

        // Calculate average FPS
        let total_time: f32 = self.frame_history.iter().map(|f| f.total_ms).sum();
        let count = self.frame_history.len() as f32;
        let avg_frame_time = total_time / count;
        self.stats.fps = 1000.0 / avg_frame_time;

        // Min/Max FPS
        let min_frame_time = self.frame_history.iter()
            .map(|f| f.total_ms)
            .fold(f32::MAX, f32::min);
        let max_frame_time = self.frame_history.iter()
            .map(|f| f.total_ms)
            .fold(0.0f32, f32::max);
        
        self.stats.max_fps = 1000.0 / min_frame_time;
        self.stats.min_fps = 1000.0 / max_frame_time;

        // Percentiles
        let mut times: Vec<f32> = self.frame_history.iter().map(|f| f.total_ms).collect();
        times.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        let p95_idx = ((times.len() as f32) * 0.95) as usize;
        let p99_idx = ((times.len() as f32) * 0.99) as usize;
        
        self.stats.p95_ms = times.get(p95_idx.min(times.len() - 1)).copied().unwrap_or(0.0);
        self.stats.p99_ms = times.get(p99_idx.min(times.len() - 1)).copied().unwrap_or(0.0);

        // Histogram (0-100ms in 5ms buckets)
        self.stats.histogram = [0; 20];
        for timing in &self.frame_history {
            let bucket = ((timing.total_ms / 5.0) as usize).min(19);
            self.stats.histogram[bucket] += 1;
        }

        // Memory
        if let Some(last) = self.frame_history.last() {
            self.stats.memory_mb = last.memory_kb as f32 / 1024.0;
        }
    }

    /// Reset all statistics
    pub fn reset(&mut self) {
        self.scopes.clear();
        self.frame_history.clear();
        self.stats = PerformanceStats::default();
    }

    /// Get hotspots (top N slowest scopes)
    #[must_use]
    pub fn hotspots(&self, count: usize) -> Vec<&ProfileScope> {
        let mut scopes: Vec<_> = self.scopes.values().collect();
        scopes.sort_by(|a, b| b.total_time.cmp(&a.total_time));
        scopes.into_iter().take(count).collect()
    }
}

/// RAII scope guard
pub struct ScopeGuard<'a> {
    profiler: &'a mut Profiler,
    name: String,
}

impl<'a> ScopeGuard<'a> {
    /// Create a new scope guard
    pub fn new(profiler: &'a mut Profiler, name: &str, category: ProfileCategory) -> Self {
        profiler.begin_scope(name, category);
        Self {
            profiler,
            name: name.to_string(),
        }
    }
}

impl<'a> Drop for ScopeGuard<'a> {
    fn drop(&mut self) {
        self.profiler.end_scope(&self.name);
    }
}

/// Memory profiler
pub struct MemoryProfiler {
    /// Allocations by category
    allocations: HashMap<String, MemoryAllocation>,
    /// Total allocated
    pub total_allocated: u64,
    /// Peak allocated
    pub peak_allocated: u64,
}

/// Memory allocation info
#[derive(Debug, Clone, Default)]
pub struct MemoryAllocation {
    /// Category name
    pub category: String,
    /// Current bytes
    pub current: u64,
    /// Peak bytes
    pub peak: u64,
    /// Allocation count
    pub allocations: u64,
    /// Deallocation count
    pub deallocations: u64,
}

impl Default for MemoryProfiler {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryProfiler {
    /// Create a new memory profiler
    #[must_use]
    pub fn new() -> Self {
        Self {
            allocations: HashMap::new(),
            total_allocated: 0,
            peak_allocated: 0,
        }
    }

    /// Track an allocation
    pub fn alloc(&mut self, category: &str, bytes: u64) {
        let alloc = self.allocations.entry(category.to_string()).or_insert_with(|| {
            MemoryAllocation {
                category: category.to_string(),
                ..Default::default()
            }
        });
        
        alloc.current += bytes;
        alloc.allocations += 1;
        alloc.peak = alloc.peak.max(alloc.current);
        
        self.total_allocated += bytes;
        self.peak_allocated = self.peak_allocated.max(self.total_allocated);
    }

    /// Track a deallocation
    pub fn dealloc(&mut self, category: &str, bytes: u64) {
        if let Some(alloc) = self.allocations.get_mut(category) {
            alloc.current = alloc.current.saturating_sub(bytes);
            alloc.deallocations += 1;
        }
        self.total_allocated = self.total_allocated.saturating_sub(bytes);
    }

    /// Get allocation info
    #[must_use]
    pub fn get(&self, category: &str) -> Option<&MemoryAllocation> {
        self.allocations.get(category)
    }

    /// Get all allocations
    #[must_use]
    pub fn all(&self) -> &HashMap<String, MemoryAllocation> {
        &self.allocations
    }

    /// Get total in MB
    #[must_use]
    pub fn total_mb(&self) -> f32 {
        self.total_allocated as f32 / (1024.0 * 1024.0)
    }

    /// Get peak in MB
    #[must_use]
    pub fn peak_mb(&self) -> f32 {
        self.peak_allocated as f32 / (1024.0 * 1024.0)
    }
}
