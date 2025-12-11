//! # Lunaris Audio
//!
//! Audio playback, mixing, and spatial audio for the Lunaris Game Engine.
//!
//! ## Features
//!
//! - Audio source playback (WAV, OGG, MP3)
//! - 3D spatial audio with distance attenuation
//! - Audio mixing with channels
//! - Effects (reverb, low-pass filter)
//! - Music streaming

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod listener;
pub mod mixer;
pub mod source;

pub use listener::AudioListener;
pub use mixer::{AudioChannel, AudioMixer};
pub use source::{AudioClip, AudioSource, PlaybackState};

use lunaris_core::Result;

/// Audio configuration
#[derive(Debug, Clone)]
pub struct AudioConfig {
    /// Sample rate (default: 44100)
    pub sample_rate: u32,
    /// Number of audio channels (1 = mono, 2 = stereo)
    pub channels: u16,
    /// Buffer size for streaming
    pub buffer_size: usize,
    /// Master volume (0.0 - 1.0)
    pub master_volume: f32,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            channels: 2,
            buffer_size: 4096,
            master_volume: 1.0,
        }
    }
}

/// Initialize the audio subsystem
///
/// # Errors
///
/// Returns an error if audio initialization fails
pub fn init() -> Result<()> {
    tracing::info!("Audio subsystem initialized");
    Ok(())
}
