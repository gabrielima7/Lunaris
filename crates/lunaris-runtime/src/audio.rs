//! Audio System
//!
//! Provides spatial audio, music, and sound effect management.

use glam::Vec3;
use std::collections::HashMap;

/// Audio listener (usually the camera/player)
#[derive(Debug, Clone)]
pub struct AudioListener {
    /// Position
    pub position: Vec3,
    /// Forward direction
    pub forward: Vec3,
    /// Up direction
    pub up: Vec3,
    /// Master volume
    pub master_volume: f32,
}

impl Default for AudioListener {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            forward: Vec3::NEG_Z,
            up: Vec3::Y,
            master_volume: 1.0,
        }
    }
}

/// Audio source
#[derive(Debug, Clone)]
pub struct AudioSource {
    /// Unique ID
    pub id: u64,
    /// Sound clip ID
    pub clip_id: SoundClipId,
    /// Position (None for 2D sound)
    pub position: Option<Vec3>,
    /// Volume (0-1)
    pub volume: f32,
    /// Pitch multiplier
    pub pitch: f32,
    /// Is looping
    pub loop_enabled: bool,
    /// Is playing
    pub is_playing: bool,
    /// Is paused
    pub is_paused: bool,
    /// Playback position (seconds)
    pub playback_position: f32,
    /// Spatial settings
    pub spatial: SpatialSettings,
    /// Priority (higher = more important)
    pub priority: i32,
}

/// Spatial audio settings
#[derive(Debug, Clone, Copy)]
pub struct SpatialSettings {
    /// Min distance (full volume)
    pub min_distance: f32,
    /// Max distance (silent)
    pub max_distance: f32,
    /// Rolloff mode
    pub rolloff: AudioRolloff,
    /// Doppler level
    pub doppler_level: f32,
    /// Spread angle (degrees)
    pub spread: f32,
}

impl Default for SpatialSettings {
    fn default() -> Self {
        Self {
            min_distance: 1.0,
            max_distance: 500.0,
            rolloff: AudioRolloff::Logarithmic,
            doppler_level: 1.0,
            spread: 0.0,
        }
    }
}

/// Audio rolloff mode
#[derive(Debug, Clone, Copy, Default)]
pub enum AudioRolloff {
    /// Linear falloff
    Linear,
    /// Logarithmic falloff (realistic)
    #[default]
    Logarithmic,
    /// Custom curve
    Custom,
}

/// Sound clip ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SoundClipId(pub u64);

/// Sound clip data
#[derive(Debug, Clone)]
pub struct SoundClip {
    /// Clip ID
    pub id: SoundClipId,
    /// Name
    pub name: String,
    /// Duration in seconds
    pub duration: f32,
    /// Sample rate
    pub sample_rate: u32,
    /// Number of channels
    pub channels: u8,
    /// Audio data (samples)
    pub samples: Vec<f32>,
}

/// Audio mixer channel
#[derive(Debug, Clone)]
pub struct MixerChannel {
    /// Channel name
    pub name: String,
    /// Volume
    pub volume: f32,
    /// Is muted
    pub muted: bool,
    /// Effects chain
    pub effects: Vec<AudioEffect>,
    /// Parent channel (for submixing)
    pub parent: Option<String>,
}

impl MixerChannel {
    /// Create a new channel
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            volume: 1.0,
            muted: false,
            effects: Vec::new(),
            parent: None,
        }
    }
}

/// Audio effect
#[derive(Debug, Clone)]
pub enum AudioEffect {
    /// Low-pass filter
    LowPass { cutoff: f32, resonance: f32 },
    /// High-pass filter
    HighPass { cutoff: f32, resonance: f32 },
    /// Reverb
    Reverb {
        room_size: f32,
        damping: f32,
        wet: f32,
        dry: f32,
    },
    /// Delay/Echo
    Delay {
        time: f32,
        feedback: f32,
        wet: f32,
    },
    /// Distortion
    Distortion { amount: f32 },
    /// Chorus
    Chorus {
        rate: f32,
        depth: f32,
        wet: f32,
    },
    /// Compressor
    Compressor {
        threshold: f32,
        ratio: f32,
        attack: f32,
        release: f32,
    },
    /// EQ band
    Equalizer { bands: Vec<EqBand> },
}

/// EQ band
#[derive(Debug, Clone, Copy)]
pub struct EqBand {
    /// Center frequency
    pub frequency: f32,
    /// Gain in dB
    pub gain: f32,
    /// Q factor
    pub q: f32,
}

/// Music track
#[derive(Debug, Clone)]
pub struct MusicTrack {
    /// Track name
    pub name: String,
    /// Clip ID
    pub clip_id: SoundClipId,
    /// Volume
    pub volume: f32,
    /// BPM for beat matching
    pub bpm: Option<f32>,
    /// Loop points
    pub loop_start: f32,
    pub loop_end: f32,
}

/// Audio system
pub struct AudioSystem {
    /// Audio listener
    pub listener: AudioListener,
    /// Sound clips
    clips: HashMap<SoundClipId, SoundClip>,
    /// Active sources
    sources: Vec<AudioSource>,
    /// Mixer channels
    channels: HashMap<String, MixerChannel>,
    /// Next source ID
    next_source_id: u64,
    /// Next clip ID
    next_clip_id: u64,
    /// Current music
    current_music: Option<u64>,
    /// Music crossfade time
    pub crossfade_time: f32,
    /// Max simultaneous sounds
    pub max_sounds: usize,
}

impl Default for AudioSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioSystem {
    /// Create a new audio system
    #[must_use]
    pub fn new() -> Self {
        let mut channels = HashMap::new();
        channels.insert("Master".to_string(), MixerChannel::new("Master"));
        channels.insert("Music".to_string(), MixerChannel {
            parent: Some("Master".to_string()),
            ..MixerChannel::new("Music")
        });
        channels.insert("SFX".to_string(), MixerChannel {
            parent: Some("Master".to_string()),
            ..MixerChannel::new("SFX")
        });
        channels.insert("Voice".to_string(), MixerChannel {
            parent: Some("Master".to_string()),
            ..MixerChannel::new("Voice")
        });
        channels.insert("Ambient".to_string(), MixerChannel {
            parent: Some("Master".to_string()),
            ..MixerChannel::new("Ambient")
        });

        Self {
            listener: AudioListener::default(),
            clips: HashMap::new(),
            sources: Vec::new(),
            channels,
            next_source_id: 1,
            next_clip_id: 1,
            current_music: None,
            crossfade_time: 1.0,
            max_sounds: 32,
        }
    }

    /// Register a sound clip
    pub fn register_clip(&mut self, name: &str, samples: Vec<f32>, sample_rate: u32, channels: u8) -> SoundClipId {
        let id = SoundClipId(self.next_clip_id);
        self.next_clip_id += 1;

        let duration = samples.len() as f32 / (sample_rate as f32 * channels as f32);

        self.clips.insert(id, SoundClip {
            id,
            name: name.to_string(),
            duration,
            sample_rate,
            channels,
            samples,
        });

        id
    }

    /// Play a sound
    pub fn play(&mut self, clip_id: SoundClipId) -> Option<u64> {
        self.play_at(clip_id, None, 1.0, 1.0, false)
    }

    /// Play a sound with options
    pub fn play_at(
        &mut self,
        clip_id: SoundClipId,
        position: Option<Vec3>,
        volume: f32,
        pitch: f32,
        loop_enabled: bool,
    ) -> Option<u64> {
        if !self.clips.contains_key(&clip_id) {
            return None;
        }

        // Check sound limit
        if self.sources.len() >= self.max_sounds {
            // Remove lowest priority non-playing sound
            if let Some(idx) = self.sources
                .iter()
                .enumerate()
                .filter(|(_, s)| !s.is_playing)
                .min_by_key(|(_, s)| s.priority)
                .map(|(i, _)| i)
            {
                self.sources.remove(idx);
            } else {
                return None;
            }
        }

        let id = self.next_source_id;
        self.next_source_id += 1;

        self.sources.push(AudioSource {
            id,
            clip_id,
            position,
            volume,
            pitch,
            loop_enabled,
            is_playing: true,
            is_paused: false,
            playback_position: 0.0,
            spatial: SpatialSettings::default(),
            priority: 0,
        });

        Some(id)
    }

    /// Stop a sound
    pub fn stop(&mut self, source_id: u64) {
        if let Some(source) = self.sources.iter_mut().find(|s| s.id == source_id) {
            source.is_playing = false;
        }
    }

    /// Pause a sound
    pub fn pause(&mut self, source_id: u64) {
        if let Some(source) = self.sources.iter_mut().find(|s| s.id == source_id) {
            source.is_paused = true;
        }
    }

    /// Resume a sound
    pub fn resume(&mut self, source_id: u64) {
        if let Some(source) = self.sources.iter_mut().find(|s| s.id == source_id) {
            source.is_paused = false;
        }
    }

    /// Stop all sounds
    pub fn stop_all(&mut self) {
        for source in &mut self.sources {
            source.is_playing = false;
        }
    }

    /// Play music with crossfade
    pub fn play_music(&mut self, clip_id: SoundClipId, fade_in: bool) -> Option<u64> {
        // Stop current music with fadeout
        if let Some(current) = self.current_music {
            self.stop(current);
        }

        let id = self.play_at(clip_id, None, if fade_in { 0.0 } else { 1.0 }, 1.0, true)?;
        self.current_music = Some(id);
        Some(id)
    }

    /// Set channel volume
    pub fn set_channel_volume(&mut self, channel: &str, volume: f32) {
        if let Some(ch) = self.channels.get_mut(channel) {
            ch.volume = volume.clamp(0.0, 1.0);
        }
    }

    /// Mute channel
    pub fn mute_channel(&mut self, channel: &str, muted: bool) {
        if let Some(ch) = self.channels.get_mut(channel) {
            ch.muted = muted;
        }
    }

    /// Update audio system
    pub fn update(&mut self, delta_time: f32) {
        // Update playback positions
        for source in &mut self.sources {
            if source.is_playing && !source.is_paused {
                source.playback_position += delta_time * source.pitch;

                // Check if finished
                if let Some(clip) = self.clips.get(&source.clip_id) {
                    if source.playback_position >= clip.duration {
                        if source.loop_enabled {
                            source.playback_position = 0.0;
                        } else {
                            source.is_playing = false;
                        }
                    }
                }
            }
        }

        // Remove finished sounds
        self.sources.retain(|s| s.is_playing || s.is_paused);
    }

    /// Calculate volume for spatial sound
    #[must_use]
    pub fn calculate_spatial_volume(&self, source: &AudioSource) -> f32 {
        let Some(pos) = source.position else {
            return source.volume;
        };

        let distance = (pos - self.listener.position).length();
        let spatial = &source.spatial;

        let attenuation = match spatial.rolloff {
            AudioRolloff::Linear => {
                1.0 - ((distance - spatial.min_distance) / (spatial.max_distance - spatial.min_distance)).clamp(0.0, 1.0)
            }
            AudioRolloff::Logarithmic => {
                if distance <= spatial.min_distance {
                    1.0
                } else {
                    spatial.min_distance / (spatial.min_distance + (distance - spatial.min_distance))
                }
            }
            AudioRolloff::Custom => 1.0, // Would use a curve
        };

        source.volume * attenuation * self.listener.master_volume
    }

    /// Get active source count
    #[must_use]
    pub fn active_source_count(&self) -> usize {
        self.sources.iter().filter(|s| s.is_playing && !s.is_paused).count()
    }
}

/// Audio event for triggering sounds
#[derive(Debug, Clone)]
pub struct AudioEvent {
    /// Event name
    pub name: String,
    /// Possible clips (random selection)
    pub clips: Vec<SoundClipId>,
    /// Volume range
    pub volume_min: f32,
    pub volume_max: f32,
    /// Pitch range
    pub pitch_min: f32,
    pub pitch_max: f32,
    /// Cooldown between plays
    pub cooldown: f32,
}

impl AudioEvent {
    /// Create a new audio event
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            clips: Vec::new(),
            volume_min: 1.0,
            volume_max: 1.0,
            pitch_min: 1.0,
            pitch_max: 1.0,
            cooldown: 0.0,
        }
    }
}
