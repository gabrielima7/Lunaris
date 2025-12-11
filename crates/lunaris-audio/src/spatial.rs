//! Spatial Audio System
//!
//! 3D positional audio with HRTF, reverb, and occlusion.

use glam::Vec3;
use std::collections::HashMap;

/// Audio source component
#[derive(Debug, Clone)]
pub struct AudioSource3D {
    /// Source ID
    pub id: u64,
    /// Position
    pub position: Vec3,
    /// Velocity (for Doppler)
    pub velocity: Vec3,
    /// Sound ID
    pub sound_id: u64,
    /// Volume (0-1)
    pub volume: f32,
    /// Pitch
    pub pitch: f32,
    /// Min distance (full volume)
    pub min_distance: f32,
    /// Max distance (silent)
    pub max_distance: f32,
    /// Rolloff mode
    pub rolloff: AttenuationMode,
    /// Is looping
    pub looping: bool,
    /// Is playing
    pub playing: bool,
    /// Spatialize
    pub spatial: bool,
    /// Occlusion factor (0=clear, 1=fully occluded)
    pub occlusion: f32,
    /// Reverb send
    pub reverb_send: f32,
    /// Priority (higher = more important)
    pub priority: u32,
}

/// Distance attenuation mode
#[derive(Debug, Clone, Copy, Default)]
pub enum AttenuationMode {
    /// Linear falloff
    #[default]
    Linear,
    /// Inverse distance
    Inverse,
    /// Inverse squared
    InverseSquared,
    /// Logarithmic
    Logarithmic,
    /// Custom curve
    Custom,
}

impl Default for AudioSource3D {
    fn default() -> Self {
        Self {
            id: 0,
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            sound_id: 0,
            volume: 1.0,
            pitch: 1.0,
            min_distance: 1.0,
            max_distance: 100.0,
            rolloff: AttenuationMode::InverseSquared,
            looping: false,
            playing: false,
            spatial: true,
            occlusion: 0.0,
            reverb_send: 0.3,
            priority: 50,
        }
    }
}

impl AudioSource3D {
    /// Calculate attenuation at distance
    #[must_use]
    pub fn attenuation(&self, distance: f32) -> f32 {
        if distance <= self.min_distance {
            return 1.0;
        }
        if distance >= self.max_distance {
            return 0.0;
        }

        let range = self.max_distance - self.min_distance;
        let d = (distance - self.min_distance) / range;

        match self.rolloff {
            AttenuationMode::Linear => 1.0 - d,
            AttenuationMode::Inverse => 1.0 / (1.0 + d),
            AttenuationMode::InverseSquared => 1.0 / (1.0 + d * d),
            AttenuationMode::Logarithmic => (1.0 - d.ln().max(0.0) / 4.0).max(0.0),
            AttenuationMode::Custom => 1.0 - d, // Default to linear
        }
    }
}

/// Audio listener
#[derive(Debug, Clone)]
pub struct AudioListener3D {
    /// Position
    pub position: Vec3,
    /// Forward direction
    pub forward: Vec3,
    /// Up direction
    pub up: Vec3,
    /// Velocity (for Doppler)
    pub velocity: Vec3,
}

impl Default for AudioListener3D {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            forward: Vec3::NEG_Z,
            up: Vec3::Y,
            velocity: Vec3::ZERO,
        }
    }
}

impl AudioListener3D {
    /// Get right vector
    #[must_use]
    pub fn right(&self) -> Vec3 {
        self.forward.cross(self.up).normalize()
    }

    /// Calculate panning (-1 left, +1 right)
    #[must_use]
    pub fn calculate_pan(&self, source_pos: Vec3) -> f32 {
        let to_source = (source_pos - self.position).normalize();
        let right = self.right();
        to_source.dot(right).clamp(-1.0, 1.0)
    }

    /// Calculate elevation angle
    #[must_use]
    pub fn calculate_elevation(&self, source_pos: Vec3) -> f32 {
        let to_source = (source_pos - self.position).normalize();
        to_source.dot(self.up).asin()
    }

    /// Calculate azimuth angle
    #[must_use]
    pub fn calculate_azimuth(&self, source_pos: Vec3) -> f32 {
        let to_source = source_pos - self.position;
        let forward_proj = to_source - self.up * to_source.dot(self.up);
        let forward_proj = forward_proj.normalize();
        
        let cos_angle = forward_proj.dot(self.forward);
        let sin_angle = forward_proj.dot(self.right());
        
        sin_angle.atan2(cos_angle)
    }
}

/// HRTF (Head-Related Transfer Function) data
#[derive(Debug, Clone)]
pub struct HRTF {
    /// Azimuth angles (degrees)
    pub azimuths: Vec<f32>,
    /// Elevation angles (degrees)
    pub elevations: Vec<f32>,
    /// ITD (Interaural Time Difference) in samples
    pub itd: Vec<Vec<f32>>,
    /// ILD (Interaural Level Difference) in dB
    pub ild: Vec<Vec<f32>>,
    /// Sample rate
    pub sample_rate: u32,
}

impl HRTF {
    /// Get ITD and ILD for direction
    #[must_use]
    pub fn get_parameters(&self, azimuth: f32, elevation: f32) -> (f32, f32) {
        // Simplified lookup - would use bilinear interpolation in real implementation
        let az_idx = self.find_nearest_index(&self.azimuths, azimuth);
        let el_idx = self.find_nearest_index(&self.elevations, elevation);
        
        let itd = self.itd.get(el_idx).and_then(|row| row.get(az_idx)).copied().unwrap_or(0.0);
        let ild = self.ild.get(el_idx).and_then(|row| row.get(az_idx)).copied().unwrap_or(0.0);
        
        (itd, ild)
    }

    fn find_nearest_index(&self, values: &[f32], target: f32) -> usize {
        values.iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                let da = (*a - target).abs();
                let db = (*b - target).abs();
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
}

/// Reverb zone
#[derive(Debug, Clone)]
pub struct ReverbZone {
    /// Position
    pub position: Vec3,
    /// Inner radius (full reverb)
    pub inner_radius: f32,
    /// Outer radius (no reverb)
    pub outer_radius: f32,
    /// Reverb preset
    pub preset: ReverbPreset,
    /// Priority
    pub priority: u32,
}

/// Reverb preset
#[derive(Debug, Clone)]
pub struct ReverbPreset {
    /// Name
    pub name: String,
    /// Room size
    pub room_size: f32,
    /// Decay time (seconds)
    pub decay_time: f32,
    /// High frequency damping
    pub hf_damping: f32,
    /// Early reflections level
    pub early_level: f32,
    /// Late reverb level
    pub late_level: f32,
    /// Pre-delay (ms)
    pub pre_delay: f32,
    /// Diffusion
    pub diffusion: f32,
    /// Density
    pub density: f32,
}

impl ReverbPreset {
    /// Small room preset
    #[must_use]
    pub fn small_room() -> Self {
        Self {
            name: "Small Room".to_string(),
            room_size: 0.2,
            decay_time: 0.5,
            hf_damping: 0.7,
            early_level: 0.8,
            late_level: 0.6,
            pre_delay: 5.0,
            diffusion: 0.8,
            density: 0.9,
        }
    }

    /// Large hall preset
    #[must_use]
    pub fn large_hall() -> Self {
        Self {
            name: "Large Hall".to_string(),
            room_size: 0.9,
            decay_time: 2.5,
            hf_damping: 0.4,
            early_level: 0.6,
            late_level: 0.8,
            pre_delay: 20.0,
            diffusion: 0.9,
            density: 0.7,
        }
    }

    /// Cave preset
    #[must_use]
    pub fn cave() -> Self {
        Self {
            name: "Cave".to_string(),
            room_size: 0.7,
            decay_time: 3.0,
            hf_damping: 0.2,
            early_level: 0.5,
            late_level: 0.9,
            pre_delay: 30.0,
            diffusion: 0.6,
            density: 0.8,
        }
    }

    /// Outdoor preset
    #[must_use]
    pub fn outdoor() -> Self {
        Self {
            name: "Outdoor".to_string(),
            room_size: 0.0,
            decay_time: 0.3,
            hf_damping: 0.9,
            early_level: 0.3,
            late_level: 0.1,
            pre_delay: 2.0,
            diffusion: 0.5,
            density: 0.3,
        }
    }
}

/// Spatial audio manager
pub struct SpatialAudioManager {
    /// Listener
    pub listener: AudioListener3D,
    /// Sources
    sources: HashMap<u64, AudioSource3D>,
    /// Reverb zones
    reverb_zones: Vec<ReverbZone>,
    /// Next source ID
    next_id: u64,
    /// Speed of sound (m/s)
    pub speed_of_sound: f32,
    /// Doppler factor
    pub doppler_factor: f32,
    /// Max simultaneous sources
    pub max_sources: usize,
    /// HRTF enabled
    pub hrtf_enabled: bool,
}

impl Default for SpatialAudioManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SpatialAudioManager {
    /// Create a new manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            listener: AudioListener3D::default(),
            sources: HashMap::new(),
            reverb_zones: Vec::new(),
            next_id: 1,
            speed_of_sound: 343.0,
            doppler_factor: 1.0,
            max_sources: 32,
            hrtf_enabled: true,
        }
    }

    /// Create a source
    pub fn create_source(&mut self, sound_id: u64, position: Vec3) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        
        let source = AudioSource3D {
            id,
            position,
            sound_id,
            ..Default::default()
        };
        
        self.sources.insert(id, source);
        id
    }

    /// Get source
    #[must_use]
    pub fn get_source(&self, id: u64) -> Option<&AudioSource3D> {
        self.sources.get(&id)
    }

    /// Get source mut
    pub fn get_source_mut(&mut self, id: u64) -> Option<&mut AudioSource3D> {
        self.sources.get_mut(&id)
    }

    /// Play source
    pub fn play(&mut self, id: u64) {
        if let Some(source) = self.sources.get_mut(&id) {
            source.playing = true;
        }
    }

    /// Stop source
    pub fn stop(&mut self, id: u64) {
        if let Some(source) = self.sources.get_mut(&id) {
            source.playing = false;
        }
    }

    /// Remove source
    pub fn remove(&mut self, id: u64) {
        self.sources.remove(&id);
    }

    /// Add reverb zone
    pub fn add_reverb_zone(&mut self, zone: ReverbZone) {
        self.reverb_zones.push(zone);
    }

    /// Calculate Doppler shift
    #[must_use]
    pub fn calculate_doppler(&self, source: &AudioSource3D) -> f32 {
        let listener_velocity_toward = self.listener.velocity.dot(
            (source.position - self.listener.position).normalize()
        );
        let source_velocity_toward = source.velocity.dot(
            (self.listener.position - source.position).normalize()
        );
        
        let speed = self.speed_of_sound;
        let pitch_shift = (speed + listener_velocity_toward * self.doppler_factor) 
                        / (speed + source_velocity_toward * self.doppler_factor);
        
        pitch_shift.clamp(0.5, 2.0)
    }

    /// Get active reverb at position
    #[must_use]
    pub fn get_reverb_at(&self, position: Vec3) -> Option<(&ReverbPreset, f32)> {
        let mut best: Option<(&ReverbZone, f32)> = None;
        
        for zone in &self.reverb_zones {
            let distance = (position - zone.position).length();
            
            if distance <= zone.outer_radius {
                let blend = if distance <= zone.inner_radius {
                    1.0
                } else {
                    1.0 - (distance - zone.inner_radius) / (zone.outer_radius - zone.inner_radius)
                };
                
                if best.is_none() || zone.priority > best.unwrap().0.priority {
                    best = Some((zone, blend));
                }
            }
        }
        
        best.map(|(zone, blend)| (&zone.preset, blend))
    }

    /// Update all sources
    pub fn update(&mut self, delta_time: f32) {
        // Virtualization: sort by priority and distance
        let mut sorted_ids: Vec<u64> = self.sources.keys().copied().collect();
        sorted_ids.sort_by(|a, b| {
            let sa = &self.sources[a];
            let sb = &self.sources[b];
            
            let dist_a = (sa.position - self.listener.position).length();
            let dist_b = (sb.position - self.listener.position).length();
            
            // Compare by priority first, then distance
            sb.priority.cmp(&sa.priority)
                .then(dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal))
        });

        // Mark sources beyond max as virtualized (not rendered)
        for (i, id) in sorted_ids.iter().enumerate() {
            if let Some(source) = self.sources.get_mut(id) {
                source.playing = i < self.max_sources && source.playing;
            }
        }
    }

    /// Get active source count
    #[must_use]
    pub fn active_count(&self) -> usize {
        self.sources.values().filter(|s| s.playing).count()
    }
}
