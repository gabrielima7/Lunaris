//! Time utilities for the engine

use std::time::{Duration, Instant};

/// Frame timing information
#[derive(Debug, Clone, Copy)]
pub struct Time {
    /// Time since engine started
    startup: Instant,
    /// Current frame start time
    frame_start: Instant,
    /// Delta time since last frame
    delta: Duration,
    /// Fixed timestep for physics (default 60Hz)
    fixed_delta: Duration,
    /// Total elapsed time
    elapsed: Duration,
    /// Frame count
    frame_count: u64,
}

impl Time {
    /// Create a new Time instance
    #[must_use]
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            startup: now,
            frame_start: now,
            delta: Duration::ZERO,
            fixed_delta: Duration::from_secs_f64(1.0 / 60.0),
            elapsed: Duration::ZERO,
            frame_count: 0,
        }
    }

    /// Update time for a new frame
    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.frame_start;
        self.frame_start = now;
        self.elapsed = now - self.startup;
        self.frame_count += 1;
    }

    /// Delta time in seconds
    #[must_use]
    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    /// Delta time as Duration
    #[must_use]
    pub const fn delta(&self) -> Duration {
        self.delta
    }

    /// Fixed delta time in seconds (for physics)
    #[must_use]
    pub fn fixed_delta_seconds(&self) -> f32 {
        self.fixed_delta.as_secs_f32()
    }

    /// Total elapsed time since engine start
    #[must_use]
    pub const fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// Total elapsed time in seconds
    #[must_use]
    pub fn elapsed_seconds(&self) -> f32 {
        self.elapsed.as_secs_f32()
    }

    /// Current frame count
    #[must_use]
    pub const fn frame_count(&self) -> u64 {
        self.frame_count
    }

    /// Approximate FPS based on delta time
    #[must_use]
    pub fn fps(&self) -> f32 {
        if self.delta.as_secs_f32() > 0.0 {
            1.0 / self.delta.as_secs_f32()
        } else {
            0.0
        }
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_updates() {
        let mut time = Time::new();
        std::thread::sleep(Duration::from_millis(10));
        time.update();
        assert!(time.delta_seconds() > 0.0);
        assert_eq!(time.frame_count(), 1);
    }
}
