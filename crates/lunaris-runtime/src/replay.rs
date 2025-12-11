//! Replay System
//!
//! Gameplay recording, playback, and killcam.

use glam::{Vec3, Quat};
use std::collections::HashMap;

/// Replay system
pub struct ReplaySystem {
    pub buffer: ReplayBuffer,
    pub state: ReplayState,
    pub settings: ReplaySettings,
}

/// Replay buffer
pub struct ReplayBuffer {
    pub frames: Vec<ReplayFrame>,
    pub max_duration: f32,
    pub current_duration: f32,
    pub entities: HashMap<u64, EntityTrack>,
}

/// Replay frame
pub struct ReplayFrame {
    pub time: f32,
    pub entities: Vec<EntitySnapshot>,
    pub events: Vec<ReplayEvent>,
    pub input: InputSnapshot,
}

/// Entity snapshot
#[derive(Clone)]
pub struct EntitySnapshot {
    pub entity_id: u64,
    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
    pub animation_state: u32,
    pub custom_data: Vec<u8>,
}

/// Entity track
pub struct EntityTrack {
    pub snapshots: Vec<(f32, EntitySnapshot)>,
}

/// Replay event
pub struct ReplayEvent {
    pub event_type: ReplayEventType,
    pub entity_id: Option<u64>,
    pub data: HashMap<String, String>,
}

/// Event type
pub enum ReplayEventType { Kill, Death, Spawn, Despawn, Damage, Pickup, Objective }

/// Input snapshot
#[derive(Clone, Default)]
pub struct InputSnapshot {
    pub movement: Vec3,
    pub look: Vec3,
    pub buttons: u64,
}

/// Replay state
pub enum ReplayState { Idle, Recording, Playing { time: f32, speed: f32 }, Paused { time: f32 } }

/// Replay settings
pub struct ReplaySettings {
    pub record_framerate: f32,
    pub max_duration_seconds: f32,
    pub interpolate: bool,
    pub compress: bool,
}

impl Default for ReplaySettings {
    fn default() -> Self {
        Self { record_framerate: 30.0, max_duration_seconds: 300.0, interpolate: true, compress: true }
    }
}

impl ReplaySystem {
    pub fn new() -> Self {
        Self { buffer: ReplayBuffer::new(300.0), state: ReplayState::Idle, settings: ReplaySettings::default() }
    }

    pub fn start_recording(&mut self) {
        self.buffer.clear();
        self.state = ReplayState::Recording;
    }

    pub fn stop_recording(&mut self) {
        self.state = ReplayState::Idle;
    }

    pub fn record_frame(&mut self, time: f32, entities: Vec<EntitySnapshot>, input: InputSnapshot) {
        if !matches!(self.state, ReplayState::Recording) { return; }
        self.buffer.frames.push(ReplayFrame { time, entities, events: Vec::new(), input });
        self.buffer.current_duration = time;
        
        // Trim old frames if over max duration
        while self.buffer.current_duration - self.buffer.frames.first().map(|f| f.time).unwrap_or(0.0) > self.buffer.max_duration {
            self.buffer.frames.remove(0);
        }
    }

    pub fn record_event(&mut self, event: ReplayEvent) {
        if let Some(frame) = self.buffer.frames.last_mut() {
            frame.events.push(event);
        }
    }

    pub fn play(&mut self, speed: f32) {
        if self.buffer.frames.is_empty() { return; }
        self.state = ReplayState::Playing { time: self.buffer.frames[0].time, speed };
    }

    pub fn pause(&mut self) {
        if let ReplayState::Playing { time, .. } = self.state {
            self.state = ReplayState::Paused { time };
        }
    }

    pub fn resume(&mut self, speed: f32) {
        if let ReplayState::Paused { time } = self.state {
            self.state = ReplayState::Playing { time, speed };
        }
    }

    pub fn seek(&mut self, time: f32) {
        match &mut self.state {
            ReplayState::Playing { time: t, .. } | ReplayState::Paused { time: t } => *t = time,
            _ => {}
        }
    }

    pub fn update(&mut self, dt: f32) -> Option<&ReplayFrame> {
        if let ReplayState::Playing { time, speed } = &mut self.state {
            *time += dt * *speed;
            let t = *time;
            
            // Find frame
            for (i, frame) in self.buffer.frames.iter().enumerate() {
                if frame.time >= t { return Some(frame); }
            }
            self.state = ReplayState::Idle;
        }
        None
    }

    pub fn get_interpolated(&self, entity_id: u64, time: f32) -> Option<EntitySnapshot> {
        let frames: Vec<_> = self.buffer.frames.iter().filter_map(|f| {
            f.entities.iter().find(|e| e.entity_id == entity_id).map(|e| (f.time, e.clone()))
        }).collect();
        
        for i in 0..frames.len().saturating_sub(1) {
            if frames[i].0 <= time && time < frames[i + 1].0 {
                let t = (time - frames[i].0) / (frames[i + 1].0 - frames[i].0);
                let a = &frames[i].1;
                let b = &frames[i + 1].1;
                return Some(EntitySnapshot {
                    entity_id,
                    position: a.position.lerp(b.position, t),
                    rotation: a.rotation.slerp(b.rotation, t),
                    velocity: a.velocity.lerp(b.velocity, t),
                    animation_state: a.animation_state,
                    custom_data: a.custom_data.clone(),
                });
            }
        }
        None
    }
}

impl ReplayBuffer {
    pub fn new(max_duration: f32) -> Self {
        Self { frames: Vec::new(), max_duration, current_duration: 0.0, entities: HashMap::new() }
    }
    
    pub fn clear(&mut self) { self.frames.clear(); self.current_duration = 0.0; }
    pub fn duration(&self) -> f32 { self.current_duration }
    pub fn frame_count(&self) -> usize { self.frames.len() }
}

/// Killcam
pub struct Killcam {
    pub killer_id: u64,
    pub victim_id: u64,
    pub start_time: f32,
    pub duration: f32,
    pub camera_offset: Vec3,
}

impl Killcam {
    pub fn create(replay: &ReplaySystem, killer: u64, victim: u64, duration: f32) -> Option<Self> {
        Some(Self { killer_id: killer, victim_id: victim, start_time: replay.buffer.current_duration - duration, duration, camera_offset: Vec3::new(0.0, 2.0, -5.0) })
    }

    pub fn play(&self, replay: &mut ReplaySystem) {
        replay.seek(self.start_time);
        replay.play(0.5); // Slow-mo
    }
}
