//! Audio mixing and channels

use crate::{
    listener::AudioListener,
    source::{AudioClip, AudioClipId, AudioSource, PlaybackState},
};
use lunaris_core::id::Id;
use std::collections::HashMap;

/// Audio channel for grouping sounds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioChannel {
    /// Master channel (affects all audio)
    Master,
    /// Music channel
    Music,
    /// Sound effects channel
    SFX,
    /// Voice/dialogue channel
    Voice,
    /// Ambient sounds channel
    Ambient,
    /// UI sounds channel
    UI,
    /// Custom channel
    Custom(u8),
}

/// Audio mixer managing all audio playback
pub struct AudioMixer {
    /// Audio clips (loaded audio data)
    clips: HashMap<AudioClipId, AudioClip>,
    /// Active audio sources
    sources: HashMap<Id, AudioSource>,
    /// Channel volumes
    channel_volumes: HashMap<AudioChannel, f32>,
    /// Audio listener
    listener: AudioListener,
    /// Master volume
    master_volume: f32,
    /// Is audio enabled
    enabled: bool,
}

impl Default for AudioMixer {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioMixer {
    /// Create a new audio mixer
    #[must_use]
    pub fn new() -> Self {
        let mut channel_volumes = HashMap::new();
        channel_volumes.insert(AudioChannel::Master, 1.0);
        channel_volumes.insert(AudioChannel::Music, 0.8);
        channel_volumes.insert(AudioChannel::SFX, 1.0);
        channel_volumes.insert(AudioChannel::Voice, 1.0);
        channel_volumes.insert(AudioChannel::Ambient, 0.5);
        channel_volumes.insert(AudioChannel::UI, 1.0);

        Self {
            clips: HashMap::new(),
            sources: HashMap::new(),
            channel_volumes,
            listener: AudioListener::default(),
            master_volume: 1.0,
            enabled: true,
        }
    }

    /// Load an audio clip
    pub fn load_clip(&mut self, clip: AudioClip) -> AudioClipId {
        let id = clip.id;
        self.clips.insert(id, clip);
        id
    }

    /// Unload an audio clip
    pub fn unload_clip(&mut self, id: AudioClipId) {
        self.clips.remove(&id);
        // Stop any sources using this clip
        self.sources.retain(|_, s| s.clip != id);
    }

    /// Play a sound effect (fire and forget)
    pub fn play_sfx(&mut self, clip: AudioClipId, volume: f32) -> Id {
        let mut source = AudioSource::new(clip);
        source.volume = volume;
        source.play();
        
        let id = source.id;
        self.sources.insert(id, source);
        id
    }

    /// Play a sound at a 3D position
    pub fn play_3d(&mut self, clip: AudioClipId, position: lunaris_core::math::Vec3, volume: f32) -> Id {
        let mut source = AudioSource::new(clip)
            .with_volume(volume)
            .with_position(position);
        source.play();
        
        let id = source.id;
        self.sources.insert(id, source);
        id
    }

    /// Play a looping sound
    pub fn play_loop(&mut self, clip: AudioClipId, volume: f32) -> Id {
        let mut source = AudioSource::new(clip)
            .with_volume(volume)
            .with_looping(true);
        source.play();
        
        let id = source.id;
        self.sources.insert(id, source);
        id
    }

    /// Stop a specific source
    pub fn stop(&mut self, id: Id) {
        if let Some(source) = self.sources.get_mut(&id) {
            source.stop();
        }
    }

    /// Pause a specific source
    pub fn pause(&mut self, id: Id) {
        if let Some(source) = self.sources.get_mut(&id) {
            source.pause();
        }
    }

    /// Resume a specific source
    pub fn resume(&mut self, id: Id) {
        if let Some(source) = self.sources.get_mut(&id) {
            source.resume();
        }
    }

    /// Stop all audio on a channel
    pub fn stop_channel(&mut self, _channel: AudioChannel) {
        // In full implementation, would filter by channel
        for source in self.sources.values_mut() {
            source.stop();
        }
    }

    /// Stop all audio
    pub fn stop_all(&mut self) {
        for source in self.sources.values_mut() {
            source.stop();
        }
    }

    /// Set channel volume
    pub fn set_channel_volume(&mut self, channel: AudioChannel, volume: f32) {
        self.channel_volumes.insert(channel, volume.clamp(0.0, 1.0));
    }

    /// Get channel volume
    #[must_use]
    pub fn get_channel_volume(&self, channel: AudioChannel) -> f32 {
        self.channel_volumes.get(&channel).copied().unwrap_or(1.0)
    }

    /// Set master volume
    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }

    /// Get master volume
    #[must_use]
    pub fn master_volume(&self) -> f32 {
        self.master_volume
    }

    /// Set audio listener
    pub fn set_listener(&mut self, listener: AudioListener) {
        self.listener = listener;
    }

    /// Get audio listener
    #[must_use]
    pub fn listener(&self) -> &AudioListener {
        &self.listener
    }

    /// Update the mixer (call each frame)
    pub fn update(&mut self, _delta_time: f32) {
        // Remove finished non-looping sources
        self.sources.retain(|_, source| {
            if source.state == PlaybackState::Stopped && !source.looping {
                false
            } else {
                true
            }
        });

        // Update spatial audio
        for source in self.sources.values_mut() {
            if let Some(pos) = source.spatial_position {
                let _attenuation = self.listener.calculate_attenuation(
                    pos,
                    source.min_distance,
                    source.max_distance,
                );
                let _pan = self.listener.calculate_pan(pos);
                // Would apply to actual audio output
            }
        }
    }

    /// Get number of active sources
    #[must_use]
    pub fn active_source_count(&self) -> usize {
        self.sources.values().filter(|s| s.is_playing()).count()
    }

    /// Enable/disable audio
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.stop_all();
        }
    }

    /// Check if audio is enabled
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn mixer_creation() {
        let mixer = AudioMixer::new();
        assert_eq!(mixer.active_source_count(), 0);
    }

    #[test]
    fn play_clip() {
        let mut mixer = AudioMixer::new();
        let clip = AudioClip::generate_sine(440.0, Duration::from_secs(1), 44100);
        let clip_id = mixer.load_clip(clip);
        
        let source_id = mixer.play_sfx(clip_id, 1.0);
        assert_eq!(mixer.active_source_count(), 1);
        
        mixer.stop(source_id);
        mixer.update(0.016);
        assert_eq!(mixer.active_source_count(), 0);
    }
}
