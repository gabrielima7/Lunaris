//! Analytics System
//!
//! Gameplay telemetry, heatmaps, and A/B testing.

use std::collections::HashMap;

/// Analytics system
pub struct Analytics {
    pub session_id: String,
    pub user_id: Option<String>,
    pub events: Vec<AnalyticsEvent>,
    pub heatmaps: HashMap<String, Heatmap>,
    pub experiments: Vec<ABExperiment>,
    pub config: AnalyticsConfig,
}

/// Analytics event
pub struct AnalyticsEvent {
    pub name: String,
    pub timestamp: u64,
    pub properties: HashMap<String, EventValue>,
    pub session_time: f32,
}

/// Event value
pub enum EventValue { String(String), Int(i64), Float(f64), Bool(bool) }

/// Analytics config
pub struct AnalyticsConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub batch_size: usize,
    pub flush_interval: f32,
    pub sample_rate: f32,
}

impl Default for AnalyticsConfig {
    fn default() -> Self {
        Self { enabled: true, endpoint: "".into(), batch_size: 50, flush_interval: 30.0, sample_rate: 1.0 }
    }
}

impl Analytics {
    pub fn new(session_id: &str) -> Self {
        Self { session_id: session_id.into(), user_id: None, events: Vec::new(), heatmaps: HashMap::new(), experiments: Vec::new(), config: AnalyticsConfig::default() }
    }

    pub fn track(&mut self, name: &str) {
        self.track_with_props(name, HashMap::new());
    }

    pub fn track_with_props(&mut self, name: &str, properties: HashMap<String, EventValue>) {
        if !self.config.enabled { return; }
        self.events.push(AnalyticsEvent { name: name.into(), timestamp: 0, properties, session_time: 0.0 });
        if self.events.len() >= self.config.batch_size { self.flush(); }
    }

    pub fn flush(&mut self) {
        // Would send to analytics server
        self.events.clear();
    }

    pub fn set_user(&mut self, user_id: &str) { self.user_id = Some(user_id.into()); }

    // Convenience methods
    pub fn track_level_start(&mut self, level: &str) {
        self.track_with_props("level_start", [("level".into(), EventValue::String(level.into()))].into());
    }

    pub fn track_level_complete(&mut self, level: &str, time: f32, deaths: i32) {
        self.track_with_props("level_complete", [
            ("level".into(), EventValue::String(level.into())),
            ("time".into(), EventValue::Float(time as f64)),
            ("deaths".into(), EventValue::Int(deaths as i64)),
        ].into());
    }

    pub fn track_purchase(&mut self, item: &str, price: f64, currency: &str) {
        self.track_with_props("purchase", [
            ("item".into(), EventValue::String(item.into())),
            ("price".into(), EventValue::Float(price)),
            ("currency".into(), EventValue::String(currency.into())),
        ].into());
    }

    pub fn track_death(&mut self, cause: &str, position: [f32; 3]) {
        self.track_with_props("death", [
            ("cause".into(), EventValue::String(cause.into())),
            ("x".into(), EventValue::Float(position[0] as f64)),
            ("y".into(), EventValue::Float(position[1] as f64)),
            ("z".into(), EventValue::Float(position[2] as f64)),
        ].into());
        
        self.record_heatmap("deaths", position[0], position[2]);
    }
}

/// Heatmap
pub struct Heatmap {
    pub name: String,
    pub resolution: (u32, u32),
    pub bounds: (f32, f32, f32, f32), // min_x, min_z, max_x, max_z
    pub data: Vec<f32>,
}

impl Heatmap {
    pub fn new(name: &str, resolution: (u32, u32), bounds: (f32, f32, f32, f32)) -> Self {
        let size = (resolution.0 * resolution.1) as usize;
        Self { name: name.into(), resolution, bounds, data: vec![0.0; size] }
    }

    pub fn record(&mut self, x: f32, z: f32) {
        let px = ((x - self.bounds.0) / (self.bounds.2 - self.bounds.0) * self.resolution.0 as f32) as usize;
        let pz = ((z - self.bounds.1) / (self.bounds.3 - self.bounds.1) * self.resolution.1 as f32) as usize;
        if px < self.resolution.0 as usize && pz < self.resolution.1 as usize {
            self.data[pz * self.resolution.0 as usize + px] += 1.0;
        }
    }

    pub fn normalize(&mut self) {
        let max = self.data.iter().cloned().fold(0.0f32, f32::max);
        if max > 0.0 { for v in &mut self.data { *v /= max; } }
    }
}

impl Analytics {
    pub fn create_heatmap(&mut self, name: &str, resolution: (u32, u32), bounds: (f32, f32, f32, f32)) {
        self.heatmaps.insert(name.into(), Heatmap::new(name, resolution, bounds));
    }

    pub fn record_heatmap(&mut self, name: &str, x: f32, z: f32) {
        if let Some(heatmap) = self.heatmaps.get_mut(name) { heatmap.record(x, z); }
    }
}

/// A/B experiment
pub struct ABExperiment {
    pub name: String,
    pub variants: Vec<Variant>,
    pub active: bool,
    pub assigned_variant: Option<String>,
}

/// Variant
pub struct Variant {
    pub name: String,
    pub weight: f32,
    pub config: HashMap<String, String>,
}

impl ABExperiment {
    pub fn new(name: &str) -> Self {
        Self { name: name.into(), variants: Vec::new(), active: true, assigned_variant: None }
    }

    pub fn add_variant(&mut self, name: &str, weight: f32) {
        self.variants.push(Variant { name: name.into(), weight, config: HashMap::new() });
    }

    pub fn assign(&mut self, user_hash: u64) {
        if self.assigned_variant.is_some() { return; }
        let total: f32 = self.variants.iter().map(|v| v.weight).sum();
        let rand = (user_hash % 1000) as f32 / 1000.0 * total;
        let mut cumulative = 0.0;
        for variant in &self.variants {
            cumulative += variant.weight;
            if rand < cumulative {
                self.assigned_variant = Some(variant.name.clone());
                return;
            }
        }
    }

    pub fn get_variant(&self) -> Option<&Variant> {
        self.assigned_variant.as_ref().and_then(|name| self.variants.iter().find(|v| &v.name == name))
    }
}

impl Analytics {
    pub fn create_experiment(&mut self, name: &str) -> &mut ABExperiment {
        self.experiments.push(ABExperiment::new(name));
        self.experiments.last_mut().unwrap()
    }

    pub fn get_variant(&self, experiment: &str) -> Option<&str> {
        self.experiments.iter().find(|e| e.name == experiment)?.assigned_variant.as_deref()
    }
}
