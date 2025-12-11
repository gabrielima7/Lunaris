//! Audio Occlusion
//!
//! Raycast-based occlusion, material absorption, and reverb zones.

use glam::Vec3;
use std::collections::HashMap;

/// Audio occlusion system
pub struct AudioOcclusion {
    pub sources: Vec<OccludedSource>,
    pub reverb_zones: Vec<ReverbZone>,
    pub materials: HashMap<String, AudioMaterial>,
    pub settings: OcclusionSettings,
    pub listener_position: Vec3,
}

/// Occluded audio source
pub struct OccludedSource {
    pub source_id: u64,
    pub position: Vec3,
    pub occlusion: f32,
    pub low_pass: f32,
    pub reverb_send: f32,
    pub current_zone: Option<usize>,
}

/// Occlusion settings
pub struct OcclusionSettings {
    pub enabled: bool,
    pub ray_count: u32,
    pub max_distance: f32,
    pub update_rate: f32,
    pub smoothing: f32,
}

impl Default for OcclusionSettings {
    fn default() -> Self {
        Self { enabled: true, ray_count: 8, max_distance: 100.0, update_rate: 20.0, smoothing: 0.1 }
    }
}

/// Audio material
pub struct AudioMaterial {
    pub name: String,
    pub absorption: f32,          // 0-1: how much sound is absorbed
    pub transmission: f32,        // 0-1: how much passes through
    pub low_pass_factor: f32,     // frequency cutoff for transmitted sound
    pub reflection: f32,          // 0-1: how much is reflected
}

impl Default for AudioMaterial {
    fn default() -> Self {
        Self { name: "default".into(), absorption: 0.3, transmission: 0.5, low_pass_factor: 0.5, reflection: 0.5 }
    }
}

/// Reverb zone
pub struct ReverbZone {
    pub id: usize,
    pub center: Vec3,
    pub size: Vec3,
    pub priority: i32,
    pub settings: ReverbSettings,
    pub blend_distance: f32,
}

/// Reverb settings
pub struct ReverbSettings {
    pub preset: ReverbPreset,
    pub room_size: f32,
    pub damping: f32,
    pub wet_level: f32,
    pub dry_level: f32,
    pub early_reflections: f32,
    pub late_reverb: f32,
    pub diffusion: f32,
    pub density: f32,
}

/// Reverb preset
#[derive(Clone, Copy)]
pub enum ReverbPreset { None, Room, Hall, Cave, Outdoor, Underwater, Custom }

impl ReverbSettings {
    pub fn from_preset(preset: ReverbPreset) -> Self {
        match preset {
            ReverbPreset::None => Self { preset, room_size: 0.0, damping: 0.0, wet_level: 0.0, dry_level: 1.0, early_reflections: 0.0, late_reverb: 0.0, diffusion: 0.0, density: 0.0 },
            ReverbPreset::Room => Self { preset, room_size: 0.3, damping: 0.5, wet_level: 0.3, dry_level: 0.7, early_reflections: 0.5, late_reverb: 0.3, diffusion: 0.7, density: 0.5 },
            ReverbPreset::Hall => Self { preset, room_size: 0.8, damping: 0.3, wet_level: 0.5, dry_level: 0.5, early_reflections: 0.3, late_reverb: 0.6, diffusion: 0.9, density: 0.7 },
            ReverbPreset::Cave => Self { preset, room_size: 1.0, damping: 0.1, wet_level: 0.7, dry_level: 0.3, early_reflections: 0.8, late_reverb: 0.9, diffusion: 0.5, density: 0.8 },
            ReverbPreset::Outdoor => Self { preset, room_size: 0.5, damping: 0.8, wet_level: 0.2, dry_level: 0.8, early_reflections: 0.1, late_reverb: 0.1, diffusion: 0.3, density: 0.2 },
            ReverbPreset::Underwater => Self { preset, room_size: 0.6, damping: 0.4, wet_level: 0.8, dry_level: 0.2, early_reflections: 0.4, late_reverb: 0.7, diffusion: 1.0, density: 0.9 },
            ReverbPreset::Custom => Self { preset, room_size: 0.5, damping: 0.5, wet_level: 0.5, dry_level: 0.5, early_reflections: 0.5, late_reverb: 0.5, diffusion: 0.5, density: 0.5 },
        }
    }
}

impl AudioOcclusion {
    pub fn new() -> Self {
        let mut materials = HashMap::new();
        materials.insert("concrete".into(), AudioMaterial { name: "concrete".into(), absorption: 0.2, transmission: 0.1, low_pass_factor: 0.3, reflection: 0.8 });
        materials.insert("wood".into(), AudioMaterial { name: "wood".into(), absorption: 0.4, transmission: 0.3, low_pass_factor: 0.5, reflection: 0.5 });
        materials.insert("glass".into(), AudioMaterial { name: "glass".into(), absorption: 0.1, transmission: 0.6, low_pass_factor: 0.7, reflection: 0.4 });
        materials.insert("fabric".into(), AudioMaterial { name: "fabric".into(), absorption: 0.8, transmission: 0.2, low_pass_factor: 0.4, reflection: 0.1 });
        materials.insert("metal".into(), AudioMaterial { name: "metal".into(), absorption: 0.1, transmission: 0.05, low_pass_factor: 0.2, reflection: 0.9 });
        materials.insert("water".into(), AudioMaterial { name: "water".into(), absorption: 0.3, transmission: 0.7, low_pass_factor: 0.8, reflection: 0.2 });
        
        Self { sources: Vec::new(), reverb_zones: Vec::new(), materials, settings: OcclusionSettings::default(), listener_position: Vec3::ZERO }
    }

    pub fn register_source(&mut self, source_id: u64, position: Vec3) {
        self.sources.push(OccludedSource { source_id, position, occlusion: 0.0, low_pass: 1.0, reverb_send: 0.0, current_zone: None });
    }

    pub fn update(&mut self, raycast: impl Fn(Vec3, Vec3) -> Option<(f32, &str)>) {
        let listener = self.listener_position;
        
        for source in &mut self.sources {
            let direction = (listener - source.position).normalize();
            let distance = (listener - source.position).length();
            
            if distance > self.settings.max_distance {
                source.occlusion = 1.0;
                source.low_pass = 0.0;
                continue;
            }

            let mut total_occlusion = 0.0;
            let mut total_low_pass = 1.0;
            let mut hits = 0;

            // Cast rays
            for i in 0..self.settings.ray_count {
                let angle = (i as f32 / self.settings.ray_count as f32) * std::f32::consts::TAU;
                let offset = Vec3::new(angle.cos() * 0.1, (angle * 2.0).sin() * 0.1, 0.0);
                let ray_end = listener + offset;
                
                if let Some((_, material)) = raycast(source.position, ray_end) {
                    if let Some(mat) = self.materials.get(material) {
                        total_occlusion += 1.0 - mat.transmission;
                        total_low_pass = total_low_pass.min(mat.low_pass_factor);
                        hits += 1;
                    }
                }
            }

            if hits > 0 {
                source.occlusion = (total_occlusion / hits as f32) * self.settings.smoothing + source.occlusion * (1.0 - self.settings.smoothing);
                source.low_pass = total_low_pass * self.settings.smoothing + source.low_pass * (1.0 - self.settings.smoothing);
            } else {
                source.occlusion = source.occlusion * (1.0 - self.settings.smoothing);
                source.low_pass = source.low_pass * self.settings.smoothing + 1.0 * (1.0 - self.settings.smoothing);
            }

            // Find reverb zone
            source.current_zone = self.find_zone(source.position);
            if let Some(zone_idx) = source.current_zone {
                source.reverb_send = self.reverb_zones[zone_idx].settings.wet_level;
            }
        }
    }

    fn find_zone(&self, position: Vec3) -> Option<usize> {
        let mut best_zone: Option<(usize, i32)> = None;
        
        for (i, zone) in self.reverb_zones.iter().enumerate() {
            let local = position - zone.center;
            if local.x.abs() < zone.size.x && local.y.abs() < zone.size.y && local.z.abs() < zone.size.z {
                if best_zone.map(|(_, p)| zone.priority > p).unwrap_or(true) {
                    best_zone = Some((i, zone.priority));
                }
            }
        }
        
        best_zone.map(|(i, _)| i)
    }

    pub fn add_zone(&mut self, center: Vec3, size: Vec3, preset: ReverbPreset) {
        let id = self.reverb_zones.len();
        self.reverb_zones.push(ReverbZone { id, center, size, priority: 0, settings: ReverbSettings::from_preset(preset), blend_distance: 2.0 });
    }

    pub fn get_source(&self, id: u64) -> Option<&OccludedSource> {
        self.sources.iter().find(|s| s.source_id == id)
    }
}
