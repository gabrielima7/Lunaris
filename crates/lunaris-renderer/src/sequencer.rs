//! Sequencer System
//!
//! Timeline-based cinematics and cutscenes.

use glam::{Vec3, Quat, Vec2};
use std::collections::HashMap;

/// Sequence
pub struct Sequence {
    pub id: u64,
    pub name: String,
    pub duration: f32,
    pub frame_rate: f32,
    pub tracks: Vec<Track>,
    pub markers: Vec<Marker>,
    pub playback: PlaybackState,
}

/// Playback state
#[derive(Default)]
pub struct PlaybackState {
    pub time: f32,
    pub playing: bool,
    pub looping: bool,
    pub rate: f32,
}

/// Marker
pub struct Marker {
    pub time: f32,
    pub name: String,
}

/// Track
pub struct Track {
    pub id: u64,
    pub name: String,
    pub binding: String,
    pub track_type: TrackType,
    pub keys: Vec<Keyframe>,
    pub muted: bool,
}

/// Track type
pub enum TrackType {
    Transform,
    Camera,
    Animation,
    Audio,
    Event,
    Property(String),
    Fade,
}

/// Keyframe
pub struct Keyframe {
    pub time: f32,
    pub value: KeyValue,
    pub interpolation: Interpolation,
}

/// Key value
pub enum KeyValue {
    Float(f32),
    Vec3(Vec3),
    Quat(Quat),
    Event(String),
}

/// Interpolation
#[derive(Clone, Copy)]
pub enum Interpolation {
    Constant,
    Linear,
    Cubic,
}

impl Sequence {
    pub fn new(name: &str, duration: f32) -> Self {
        Self {
            id: 1,
            name: name.into(),
            duration,
            frame_rate: 30.0,
            tracks: Vec::new(),
            markers: Vec::new(),
            playback: PlaybackState { rate: 1.0, ..Default::default() },
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.playback.playing {
            self.playback.time += dt * self.playback.rate;
            if self.playback.time >= self.duration {
                if self.playback.looping {
                    self.playback.time = 0.0;
                } else {
                    self.playback.playing = false;
                }
            }
        }
    }

    pub fn play(&mut self) { self.playback.playing = true; }
    pub fn pause(&mut self) { self.playback.playing = false; }
    pub fn stop(&mut self) { self.playback.playing = false; self.playback.time = 0.0; }
    pub fn seek(&mut self, time: f32) { self.playback.time = time.clamp(0.0, self.duration); }

    pub fn evaluate(&self) -> Vec<TrackResult> {
        self.tracks.iter().filter(|t| !t.muted).filter_map(|t| t.evaluate(self.playback.time)).collect()
    }
}

impl Track {
    pub fn evaluate(&self, time: f32) -> Option<TrackResult> {
        if self.keys.is_empty() { return None; }
        
        // Find keys before and after
        let (k0, k1) = self.find_keys(time);
        let t = if k1.time > k0.time { (time - k0.time) / (k1.time - k0.time) } else { 0.0 };
        
        let value = match (&k0.value, &k1.value) {
            (KeyValue::Float(a), KeyValue::Float(b)) => KeyValue::Float(lerp(*a, *b, t)),
            (KeyValue::Vec3(a), KeyValue::Vec3(b)) => KeyValue::Vec3(a.lerp(*b, t)),
            (KeyValue::Quat(a), KeyValue::Quat(b)) => KeyValue::Quat(a.slerp(*b, t)),
            _ => k0.value.clone(),
        };
        
        Some(TrackResult { binding: self.binding.clone(), track_type: self.track_type.clone(), value })
    }

    fn find_keys(&self, time: f32) -> (&Keyframe, &Keyframe) {
        let mut k0 = &self.keys[0];
        let mut k1 = &self.keys[0];
        for key in &self.keys {
            if key.time <= time { k0 = key; }
            if key.time > time { k1 = key; break; }
        }
        (k0, k1)
    }
}

impl Clone for KeyValue {
    fn clone(&self) -> Self {
        match self {
            Self::Float(v) => Self::Float(*v),
            Self::Vec3(v) => Self::Vec3(*v),
            Self::Quat(v) => Self::Quat(*v),
            Self::Event(s) => Self::Event(s.clone()),
        }
    }
}

impl Clone for TrackType {
    fn clone(&self) -> Self {
        match self {
            Self::Transform => Self::Transform,
            Self::Camera => Self::Camera,
            Self::Animation => Self::Animation,
            Self::Audio => Self::Audio,
            Self::Event => Self::Event,
            Self::Property(s) => Self::Property(s.clone()),
            Self::Fade => Self::Fade,
        }
    }
}

/// Track result
pub struct TrackResult {
    pub binding: String,
    pub track_type: TrackType,
    pub value: KeyValue,
}

/// Cinematic camera
pub struct CinematicCamera {
    pub position: Vec3,
    pub rotation: Quat,
    pub fov: f32,
    pub focus_distance: f32,
    pub aperture: f32,
    pub shake_intensity: f32,
}

impl Default for CinematicCamera {
    fn default() -> Self {
        Self { position: Vec3::ZERO, rotation: Quat::IDENTITY, fov: 50.0, focus_distance: 10.0, aperture: 2.8, shake_intensity: 0.0 }
    }
}

/// Director
pub struct Director {
    pub sequence: Option<Sequence>,
    pub camera: CinematicCamera,
    pub fade: f32,
}

impl Director {
    pub fn new() -> Self { Self { sequence: None, camera: CinematicCamera::default(), fade: 0.0 } }
    
    pub fn play(&mut self, seq: Sequence) { self.sequence = Some(seq); self.sequence.as_mut().unwrap().play(); }
    
    pub fn update(&mut self, dt: f32) {
        if let Some(seq) = &mut self.sequence {
            seq.update(dt);
            for result in seq.evaluate() {
                match (result.track_type, result.value) {
                    (TrackType::Camera, KeyValue::Float(fov)) => self.camera.fov = fov,
                    (TrackType::Transform, KeyValue::Vec3(pos)) => self.camera.position = pos,
                    (TrackType::Fade, KeyValue::Float(f)) => self.fade = f,
                    _ => {}
                }
            }
        }
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 { a + (b - a) * t }
