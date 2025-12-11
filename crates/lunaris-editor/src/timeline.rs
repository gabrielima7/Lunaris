//! Timeline and Cinematics System
//!
//! Non-linear animation sequencer for cutscenes and cinematics.

use glam::{Quat, Vec3};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Timeline clip types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClipType {
    /// Animation clip
    Animation { clip_id: u64, speed: f32 },
    /// Audio clip
    Audio { clip_id: u64, volume: f32 },
    /// Camera animation
    Camera { camera_id: u64 },
    /// Event trigger
    Event { name: String, data: String },
    /// Particle effect
    Particle { emitter_id: u64 },
    /// Light animation
    Light { light_id: u64 },
    /// Property animation
    Property { target_id: u64, property: String },
    /// Subtitle
    Subtitle { text: String, speaker: String },
    /// Script
    Script { function: String },
}

/// Timeline clip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineClip {
    /// Unique ID
    pub id: u64,
    /// Clip type
    pub clip_type: ClipType,
    /// Start time (seconds)
    pub start: f32,
    /// Duration (seconds)
    pub duration: f32,
    /// Blend in time
    pub blend_in: f32,
    /// Blend out time
    pub blend_out: f32,
    /// Is enabled
    pub enabled: bool,
    /// Layer (for blending)
    pub layer: u32,
}

impl TimelineClip {
    /// Check if time is within clip
    #[must_use]
    pub fn contains(&self, time: f32) -> bool {
        time >= self.start && time < self.start + self.duration
    }

    /// Get local time within clip
    #[must_use]
    pub fn local_time(&self, time: f32) -> f32 {
        time - self.start
    }

    /// Get blend weight at time
    #[must_use]
    pub fn weight_at(&self, time: f32) -> f32 {
        if !self.contains(time) {
            return 0.0;
        }

        let local = self.local_time(time);
        let blend_in_weight = if self.blend_in > 0.0 {
            (local / self.blend_in).clamp(0.0, 1.0)
        } else {
            1.0
        };

        let remaining = self.duration - local;
        let blend_out_weight = if self.blend_out > 0.0 {
            (remaining / self.blend_out).clamp(0.0, 1.0)
        } else {
            1.0
        };

        blend_in_weight * blend_out_weight
    }

    /// Get end time
    #[must_use]
    pub fn end(&self) -> f32 {
        self.start + self.duration
    }
}

/// Timeline track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineTrack {
    /// Track name
    pub name: String,
    /// Track type
    pub track_type: TrackType,
    /// Clips on this track
    pub clips: Vec<TimelineClip>,
    /// Is muted
    pub muted: bool,
    /// Is locked
    pub locked: bool,
    /// Track color
    pub color: [f32; 3],
}

/// Track type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum TrackType {
    /// Animation track
    #[default]
    Animation,
    /// Audio track
    Audio,
    /// Camera track
    Camera,
    /// Event track
    Event,
    /// Particle track
    Particle,
    /// Subtitle track
    Subtitle,
    /// Control track (for properties)
    Control,
}

impl TimelineTrack {
    /// Create a new track
    #[must_use]
    pub fn new(name: &str, track_type: TrackType) -> Self {
        Self {
            name: name.to_string(),
            track_type,
            clips: Vec::new(),
            muted: false,
            locked: false,
            color: [0.5, 0.5, 0.5],
        }
    }

    /// Add a clip
    pub fn add_clip(&mut self, clip: TimelineClip) {
        self.clips.push(clip);
        self.clips.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap_or(std::cmp::Ordering::Equal));
    }

    /// Get active clips at time
    #[must_use]
    pub fn active_clips(&self, time: f32) -> Vec<&TimelineClip> {
        if self.muted {
            return Vec::new();
        }
        self.clips.iter().filter(|c| c.enabled && c.contains(time)).collect()
    }

    /// Get track duration
    #[must_use]
    pub fn duration(&self) -> f32 {
        self.clips.iter().map(|c| c.end()).fold(0.0f32, f32::max)
    }
}

/// Camera keyframe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraKeyframe {
    /// Time
    pub time: f32,
    /// Position
    pub position: Vec3,
    /// Rotation
    pub rotation: Quat,
    /// Field of view
    pub fov: f32,
    /// Focus distance
    pub focus_distance: f32,
    /// Aperture (for DOF)
    pub aperture: f32,
    /// Easing
    pub easing: EasingType,
}

/// Easing type
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum EasingType {
    /// Linear interpolation
    #[default]
    Linear,
    /// Ease in (slow start)
    EaseIn,
    /// Ease out (slow end)
    EaseOut,
    /// Ease in and out
    EaseInOut,
    /// Smoothstep
    Smoothstep,
    /// Bounce
    Bounce,
    /// Elastic
    Elastic,
}

impl EasingType {
    /// Apply easing to t (0-1)
    #[must_use]
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            EasingType::Linear => t,
            EasingType::EaseIn => t * t,
            EasingType::EaseOut => 1.0 - (1.0 - t).powi(2),
            EasingType::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            EasingType::Smoothstep => t * t * (3.0 - 2.0 * t),
            EasingType::Bounce => {
                let n1 = 7.5625;
                let d1 = 2.75;
                let mut t = t;
                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    t -= 1.5 / d1;
                    n1 * t * t + 0.75
                } else if t < 2.5 / d1 {
                    t -= 2.25 / d1;
                    n1 * t * t + 0.9375
                } else {
                    t -= 2.625 / d1;
                    n1 * t * t + 0.984375
                }
            }
            EasingType::Elastic => {
                let c4 = (2.0 * std::f32::consts::PI) / 3.0;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    2.0f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
                }
            }
        }
    }
}

/// Camera path for cinematics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraPath {
    /// Keyframes
    pub keyframes: Vec<CameraKeyframe>,
    /// Loop mode
    pub looping: bool,
}

impl CameraPath {
    /// Create a new camera path
    #[must_use]
    pub fn new() -> Self {
        Self {
            keyframes: Vec::new(),
            looping: false,
        }
    }

    /// Add keyframe
    pub fn add_keyframe(&mut self, keyframe: CameraKeyframe) {
        self.keyframes.push(keyframe);
        self.keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap_or(std::cmp::Ordering::Equal));
    }

    /// Evaluate at time
    #[must_use]
    pub fn evaluate(&self, time: f32) -> Option<(Vec3, Quat, f32)> {
        if self.keyframes.is_empty() {
            return None;
        }

        let duration = self.duration();
        let time = if self.looping && duration > 0.0 {
            time % duration
        } else {
            time.clamp(0.0, duration)
        };

        // Find surrounding keyframes
        let mut prev = &self.keyframes[0];
        let mut next = &self.keyframes[0];

        for (i, kf) in self.keyframes.iter().enumerate() {
            if kf.time <= time {
                prev = kf;
            }
            if kf.time > time && i > 0 {
                next = kf;
                break;
            }
        }

        if prev.time == next.time {
            return Some((prev.position, prev.rotation, prev.fov));
        }

        // Interpolate
        let t = (time - prev.time) / (next.time - prev.time);
        let eased_t = next.easing.apply(t);

        let position = prev.position.lerp(next.position, eased_t);
        let rotation = prev.rotation.slerp(next.rotation, eased_t);
        let fov = prev.fov + (next.fov - prev.fov) * eased_t;

        Some((position, rotation, fov))
    }

    /// Get duration
    #[must_use]
    pub fn duration(&self) -> f32 {
        self.keyframes.last().map(|k| k.time).unwrap_or(0.0)
    }
}

impl Default for CameraPath {
    fn default() -> Self {
        Self::new()
    }
}

/// Timeline/Sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    /// Sequence name
    pub name: String,
    /// Tracks
    pub tracks: Vec<TimelineTrack>,
    /// Duration (or None for auto)
    pub duration: Option<f32>,
    /// Loop mode
    pub looping: bool,
    /// Playback rate
    pub rate: f32,
    /// Camera paths
    pub camera_paths: HashMap<u64, CameraPath>,
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new("Untitled")
    }
}

impl Timeline {
    /// Create a new timeline
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tracks: Vec::new(),
            duration: None,
            looping: false,
            rate: 1.0,
            camera_paths: HashMap::new(),
        }
    }

    /// Add track
    pub fn add_track(&mut self, track: TimelineTrack) {
        self.tracks.push(track);
    }

    /// Get duration
    #[must_use]
    pub fn get_duration(&self) -> f32 {
        self.duration.unwrap_or_else(|| {
            self.tracks.iter().map(|t| t.duration()).fold(0.0f32, f32::max)
        })
    }

    /// Serialize to JSON
    ///
    /// # Errors
    ///
    /// Returns error if serialization fails
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from JSON
    ///
    /// # Errors
    ///
    /// Returns error if deserialization fails
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// Timeline event
#[derive(Debug, Clone)]
pub struct TimelineEvent {
    /// Event name
    pub name: String,
    /// Event data
    pub data: String,
    /// Event time
    pub time: f32,
}

/// Timeline player
pub struct TimelinePlayer {
    /// Current timeline
    timeline: Option<Timeline>,
    /// Current time
    time: f32,
    /// Is playing
    playing: bool,
    /// Events queue
    events: Vec<TimelineEvent>,
    /// Previous time (for event triggering)
    prev_time: f32,
}

impl Default for TimelinePlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl TimelinePlayer {
    /// Create a new player
    #[must_use]
    pub fn new() -> Self {
        Self {
            timeline: None,
            time: 0.0,
            playing: false,
            events: Vec::new(),
            prev_time: 0.0,
        }
    }

    /// Load timeline
    pub fn load(&mut self, timeline: Timeline) {
        self.timeline = Some(timeline);
        self.time = 0.0;
        self.prev_time = 0.0;
    }

    /// Play
    pub fn play(&mut self) {
        self.playing = true;
    }

    /// Pause
    pub fn pause(&mut self) {
        self.playing = false;
    }

    /// Stop and reset
    pub fn stop(&mut self) {
        self.playing = false;
        self.time = 0.0;
        self.prev_time = 0.0;
    }

    /// Seek to time
    pub fn seek(&mut self, time: f32) {
        self.time = time;
        self.prev_time = time;
    }

    /// Update player
    pub fn update(&mut self, delta_time: f32) {
        if !self.playing {
            return;
        }

        let Some(ref timeline) = self.timeline else { return };
        
        self.prev_time = self.time;
        self.time += delta_time * timeline.rate;

        let duration = timeline.get_duration();

        // Check for events
        self.events.clear();
        for track in &timeline.tracks {
            for clip in &track.clips {
                if matches!(clip.clip_type, ClipType::Event { .. }) {
                    if self.prev_time < clip.start && self.time >= clip.start {
                        if let ClipType::Event { ref name, ref data } = clip.clip_type {
                            self.events.push(TimelineEvent {
                                name: name.clone(),
                                data: data.clone(),
                                time: clip.start,
                            });
                        }
                    }
                }
            }
        }

        // Handle looping or end
        if self.time >= duration {
            if timeline.looping {
                self.time = self.time % duration;
            } else {
                self.time = duration;
                self.playing = false;
            }
        }
    }

    /// Get fired events
    #[must_use]
    pub fn events(&self) -> &[TimelineEvent] {
        &self.events
    }

    /// Get current time
    #[must_use]
    pub fn time(&self) -> f32 {
        self.time
    }

    /// Is playing
    #[must_use]
    pub fn is_playing(&self) -> bool {
        self.playing
    }

    /// Get active animation clips
    #[must_use]
    pub fn active_animation_clips(&self) -> Vec<(u64, f32, f32)> {
        let Some(ref timeline) = self.timeline else { return Vec::new() };
        
        let mut results = Vec::new();
        for track in &timeline.tracks {
            if matches!(track.track_type, TrackType::Animation) {
                for clip in track.active_clips(self.time) {
                    if let ClipType::Animation { clip_id, speed } = clip.clip_type {
                        let weight = clip.weight_at(self.time);
                        results.push((clip_id, clip.local_time(self.time) * speed, weight));
                    }
                }
            }
        }
        results
    }

    /// Get camera transform
    #[must_use]
    pub fn camera_transform(&self, camera_id: u64) -> Option<(Vec3, Quat, f32)> {
        let timeline = self.timeline.as_ref()?;
        let path = timeline.camera_paths.get(&camera_id)?;
        path.evaluate(self.time)
    }
}
