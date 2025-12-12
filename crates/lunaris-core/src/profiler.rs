//! Profiler
//!
//! GPU, CPU, and memory profiling with frame timeline.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Profiler
pub struct Profiler {
    pub enabled: bool,
    pub frame_data: Vec<FrameData>,
    pub current_frame: FrameData,
    pub zones: HashMap<String, ZoneStats>,
    pub gpu_profiler: GpuProfiler,
    pub memory_profiler: MemoryProfiler,
    pub max_frames: usize,
}

/// Frame data
#[derive(Clone, Default)]
pub struct FrameData {
    pub frame_number: u64,
    pub total_time: Duration,
    pub cpu_time: Duration,
    pub gpu_time: Duration,
    pub zones: Vec<ZoneData>,
    pub draw_calls: u32,
    pub triangles: u64,
    pub memory_used: u64,
}

/// Zone data
#[derive(Clone)]
pub struct ZoneData {
    pub name: String,
    pub start: Duration,
    pub duration: Duration,
    pub _depth: usize,
    pub color: [f32; 3],
}

/// Zone stats
pub struct ZoneStats {
    pub total_time: Duration,
    pub call_count: u64,
    pub min_time: Duration,
    pub max_time: Duration,
}

/// GPU profiler
pub struct GpuProfiler {
    pub queries: Vec<GpuQuery>,
    pub frame_time: Duration,
}

/// GPU query
pub struct GpuQuery {
    pub name: String,
    pub time: Duration,
}

/// Memory profiler
pub struct MemoryProfiler {
    pub allocations: HashMap<String, AllocationInfo>,
    pub total_allocated: u64,
    pub total_freed: u64,
    pub peak_usage: u64,
    pub current_usage: u64,
}

/// Allocation info
pub struct AllocationInfo {
    pub size: u64,
    pub count: u64,
    pub category: String,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            enabled: true,
            frame_data: Vec::new(),
            current_frame: FrameData::default(),
            zones: HashMap::new(),
            gpu_profiler: GpuProfiler { queries: Vec::new(), frame_time: Duration::ZERO },
            memory_profiler: MemoryProfiler { allocations: HashMap::new(), total_allocated: 0, total_freed: 0, peak_usage: 0, current_usage: 0 },
            max_frames: 300,
        }
    }

    pub fn begin_frame(&mut self, frame: u64) {
        self.current_frame = FrameData { frame_number: frame, ..Default::default() };
    }

    pub fn end_frame(&mut self) {
        self.frame_data.push(self.current_frame.clone());
        if self.frame_data.len() > self.max_frames { self.frame_data.remove(0); }
    }

    pub fn zone(&mut self, name: &str) -> ZoneGuard {
        ZoneGuard { profiler: self as *mut _, name: name.into(), start: Instant::now(), _depth: 0 }
    }

    pub fn record_zone(&mut self, name: &str, duration: Duration) {
        self.current_frame.zones.push(ZoneData { name: name.into(), start: Duration::ZERO, duration, _depth: 0, color: [0.5, 0.5, 1.0] });
        self.zones.entry(name.into()).or_insert(ZoneStats { total_time: Duration::ZERO, call_count: 0, min_time: Duration::MAX, max_time: Duration::ZERO })
            .add(duration);
    }

    pub fn avg_frame_time(&self) -> Duration {
        if self.frame_data.is_empty() { Duration::ZERO }
        else { self.frame_data.iter().map(|f| f.total_time).sum::<Duration>() / self.frame_data.len() as u32 }
    }

    pub fn fps(&self) -> f32 {
        let avg = self.avg_frame_time().as_secs_f32();
        if avg > 0.0 { 1.0 / avg } else { 0.0 }
    }
}

impl ZoneStats {
    fn add(&mut self, duration: Duration) {
        self.total_time += duration;
        self.call_count += 1;
        self.min_time = self.min_time.min(duration);
        self.max_time = self.max_time.max(duration);
    }

    pub fn avg(&self) -> Duration {
        if self.call_count > 0 { self.total_time / self.call_count as u32 } else { Duration::ZERO }
    }
}

/// Zone guard for RAII profiling
pub struct ZoneGuard {
    profiler: *mut Profiler,
    name: String,
    start: Instant,
    _depth: u32,
}

impl Drop for ZoneGuard {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        unsafe { (*self.profiler).record_zone(&self.name, duration); }
    }
}

/// Network debugger
pub struct NetworkDebugger {
    pub packets_sent: u64,
    pub packets_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub latency: Duration,
    pub packet_loss: f32,
    pub history: Vec<NetworkSample>,
}

/// Network sample
pub struct NetworkSample {
    pub time: f64,
    pub in_bandwidth: f32,
    pub out_bandwidth: f32,
    pub latency: f32,
}

impl NetworkDebugger {
    pub fn new() -> Self {
        Self { packets_sent: 0, packets_received: 0, bytes_sent: 0, bytes_received: 0, latency: Duration::ZERO, packet_loss: 0.0, history: Vec::new() }
    }
}
