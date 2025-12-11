//! Audio source and playback

use lunaris_core::id::Id;
use std::time::Duration;

/// Handle to an audio clip
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AudioClipId(pub Id);

/// Audio clip data
#[derive(Debug, Clone)]
pub struct AudioClip {
    /// Clip ID
    pub id: AudioClipId,
    /// Clip name
    pub name: String,
    /// Duration
    pub duration: Duration,
    /// Sample rate
    pub sample_rate: u32,
    /// Number of channels
    pub channels: u16,
    /// Sample data (interleaved if stereo)
    pub samples: Vec<f32>,
}

impl AudioClip {
    /// Create a new audio clip
    #[must_use]
    pub fn new(name: impl Into<String>, sample_rate: u32, channels: u16, samples: Vec<f32>) -> Self {
        let duration_secs = samples.len() as f32 / (sample_rate as f32 * channels as f32);
        Self {
            id: AudioClipId(Id::new()),
            name: name.into(),
            duration: Duration::from_secs_f32(duration_secs),
            sample_rate,
            channels,
            samples,
        }
    }

    /// Generate a sine wave for testing
    #[must_use]
    pub fn generate_sine(frequency: f32, duration: Duration, sample_rate: u32) -> Self {
        let sample_count = (duration.as_secs_f32() * sample_rate as f32) as usize;
        let mut samples = Vec::with_capacity(sample_count);

        for i in 0..sample_count {
            let t = i as f32 / sample_rate as f32;
            let sample = (2.0 * std::f32::consts::PI * frequency * t).sin();
            samples.push(sample);
        }

        Self::new(format!("sine_{}hz", frequency as u32), sample_rate, 1, samples)
    }
}

/// Playback state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlaybackState {
    /// Not playing
    #[default]
    Stopped,
    /// Currently playing
    Playing,
    /// Paused
    Paused,
}

/// Audio source instance (playing audio)
#[derive(Debug)]
pub struct AudioSource {
    /// Source ID
    pub id: Id,
    /// The audio clip being played
    pub clip: AudioClipId,
    /// Current playback state
    pub state: PlaybackState,
    /// Current playback position (in samples)
    pub position: usize,
    /// Volume (0.0 - 1.0)
    pub volume: f32,
    /// Pitch multiplier (1.0 = normal)
    pub pitch: f32,
    /// Whether to loop
    pub looping: bool,
    /// Spatial position (None for 2D audio)
    pub spatial_position: Option<lunaris_core::math::Vec3>,
    /// Minimum distance for spatial audio
    pub min_distance: f32,
    /// Maximum distance for spatial audio
    pub max_distance: f32,
}

impl AudioSource {
    /// Create a new audio source
    #[must_use]
    pub fn new(clip: AudioClipId) -> Self {
        Self {
            id: Id::new(),
            clip,
            state: PlaybackState::Stopped,
            position: 0,
            volume: 1.0,
            pitch: 1.0,
            looping: false,
            spatial_position: None,
            min_distance: 1.0,
            max_distance: 100.0,
        }
    }

    /// Set volume
    #[must_use]
    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume.clamp(0.0, 1.0);
        self
    }

    /// Set looping
    #[must_use]
    pub const fn with_looping(mut self, looping: bool) -> Self {
        self.looping = looping;
        self
    }

    /// Set spatial position
    #[must_use]
    pub fn with_position(mut self, position: lunaris_core::math::Vec3) -> Self {
        self.spatial_position = Some(position);
        self
    }

    /// Play the audio
    pub fn play(&mut self) {
        self.state = PlaybackState::Playing;
    }

    /// Pause the audio
    pub fn pause(&mut self) {
        if self.state == PlaybackState::Playing {
            self.state = PlaybackState::Paused;
        }
    }

    /// Resume paused audio
    pub fn resume(&mut self) {
        if self.state == PlaybackState::Paused {
            self.state = PlaybackState::Playing;
        }
    }

    /// Stop the audio and reset position
    pub fn stop(&mut self) {
        self.state = PlaybackState::Stopped;
        self.position = 0;
    }

    /// Check if currently playing
    #[must_use]
    pub fn is_playing(&self) -> bool {
        self.state == PlaybackState::Playing
    }
}

/// Audio source builder for fluent API
pub struct AudioSourceBuilder {
    source: AudioSource,
}

impl AudioSourceBuilder {
    /// Start building a new audio source
    #[must_use]
    pub fn new(clip: AudioClipId) -> Self {
        Self {
            source: AudioSource::new(clip),
        }
    }

    /// Set volume
    #[must_use]
    pub fn volume(mut self, volume: f32) -> Self {
        self.source.volume = volume.clamp(0.0, 1.0);
        self
    }

    /// Set pitch
    #[must_use]
    pub fn pitch(mut self, pitch: f32) -> Self {
        self.source.pitch = pitch.max(0.01);
        self
    }

    /// Enable looping
    #[must_use]
    pub const fn looping(mut self, looping: bool) -> Self {
        self.source.looping = looping;
        self
    }

    /// Set spatial position
    #[must_use]
    pub fn spatial(mut self, position: lunaris_core::math::Vec3) -> Self {
        self.source.spatial_position = Some(position);
        self
    }

    /// Set distance attenuation range
    #[must_use]
    pub fn distance_range(mut self, min: f32, max: f32) -> Self {
        self.source.min_distance = min;
        self.source.max_distance = max;
        self
    }

    /// Build the audio source
    #[must_use]
    pub fn build(self) -> AudioSource {
        self.source
    }

    /// Build and immediately start playing
    #[must_use]
    pub fn play(mut self) -> AudioSource {
        self.source.play();
        self.source
    }
}
