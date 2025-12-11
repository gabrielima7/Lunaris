//! Root Motion System
//!
//! Extract and apply root bone motion to character movement.

use glam::{Quat, Vec3};

/// Root motion extraction mode
#[derive(Debug, Clone, Copy, Default)]
pub enum RootMotionMode {
    /// No root motion applied
    #[default]
    None,
    /// Extract and apply XZ motion only (ground plane)
    XZ,
    /// Extract all axes
    Full,
    /// Extract rotation only
    RotationOnly,
    /// Custom extraction
    Custom,
}

/// Root motion data for a single frame
#[derive(Debug, Clone, Copy, Default)]
pub struct RootMotionDelta {
    /// Position delta
    pub translation: Vec3,
    /// Rotation delta
    pub rotation: Quat,
    /// Velocity (for prediction)
    pub velocity: Vec3,
    /// Angular velocity
    pub angular_velocity: Vec3,
}

impl RootMotionDelta {
    /// Create a new delta
    #[must_use]
    pub fn new(translation: Vec3, rotation: Quat) -> Self {
        Self {
            translation,
            rotation,
            velocity: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
        }
    }

    /// Apply to transform
    #[must_use]
    pub fn apply(&self, position: Vec3, rotation: Quat) -> (Vec3, Quat) {
        let new_translation = position + rotation * self.translation;
        let new_rotation = rotation * self.rotation;
        (new_translation, new_rotation)
    }

    /// Blend with another delta
    #[must_use]
    pub fn blend(&self, other: &RootMotionDelta, weight: f32) -> RootMotionDelta {
        RootMotionDelta {
            translation: self.translation.lerp(other.translation, weight),
            rotation: self.rotation.slerp(other.rotation, weight),
            velocity: self.velocity.lerp(other.velocity, weight),
            angular_velocity: self.angular_velocity.lerp(other.angular_velocity, weight),
        }
    }
}

/// Root motion extractor
pub struct RootMotionExtractor {
    /// Extraction mode
    pub mode: RootMotionMode,
    /// Root bone index
    pub root_bone: u32,
    /// Lock Y position (for ground-locked animations)
    pub lock_y: bool,
    /// Previous root position
    prev_position: Vec3,
    /// Previous root rotation
    prev_rotation: Quat,
    /// Accumulated motion
    accumulated: RootMotionDelta,
    /// Is first frame
    first_frame: bool,
}

impl Default for RootMotionExtractor {
    fn default() -> Self {
        Self::new(0)
    }
}

impl RootMotionExtractor {
    /// Create a new extractor
    #[must_use]
    pub fn new(root_bone: u32) -> Self {
        Self {
            mode: RootMotionMode::Full,
            root_bone,
            lock_y: false,
            prev_position: Vec3::ZERO,
            prev_rotation: Quat::IDENTITY,
            accumulated: RootMotionDelta::default(),
            first_frame: true,
        }
    }

    /// Extract root motion from current pose
    pub fn extract(&mut self, root_position: Vec3, root_rotation: Quat, delta_time: f32) -> RootMotionDelta {
        if self.first_frame {
            self.prev_position = root_position;
            self.prev_rotation = root_rotation;
            self.first_frame = false;
            return RootMotionDelta::default();
        }

        // Calculate delta
        let mut translation = root_position - self.prev_position;
        let rotation = self.prev_rotation.inverse() * root_rotation;

        // Apply mode
        match self.mode {
            RootMotionMode::None => {
                translation = Vec3::ZERO;
            }
            RootMotionMode::XZ => {
                translation.y = 0.0;
            }
            RootMotionMode::RotationOnly => {
                translation = Vec3::ZERO;
            }
            RootMotionMode::Full | RootMotionMode::Custom => {}
        }

        // Lock Y if needed
        if self.lock_y {
            translation.y = 0.0;
        }

        // Calculate velocity
        let velocity = if delta_time > 0.0 {
            translation / delta_time
        } else {
            Vec3::ZERO
        };

        // Store for next frame
        self.prev_position = root_position;
        self.prev_rotation = root_rotation;

        // Accumulate
        self.accumulated.translation += translation;
        self.accumulated.rotation = self.accumulated.rotation * rotation;

        RootMotionDelta {
            translation,
            rotation,
            velocity,
            angular_velocity: Vec3::ZERO,
        }
    }

    /// Consume accumulated motion (resets accumulator)
    pub fn consume(&mut self) -> RootMotionDelta {
        let result = self.accumulated.clone();
        self.accumulated = RootMotionDelta::default();
        result
    }

    /// Reset extractor
    pub fn reset(&mut self) {
        self.first_frame = true;
        self.accumulated = RootMotionDelta::default();
    }

    /// Get accumulated motion without consuming
    #[must_use]
    pub fn accumulated(&self) -> &RootMotionDelta {
        &self.accumulated
    }
}

/// Root motion animator that manages extraction and application
pub struct RootMotionAnimator {
    /// Extractor
    pub extractor: RootMotionExtractor,
    /// Apply to physics
    pub apply_to_physics: bool,
    /// Blend weight
    pub weight: f32,
    /// Current delta
    current_delta: RootMotionDelta,
}

impl Default for RootMotionAnimator {
    fn default() -> Self {
        Self {
            extractor: RootMotionExtractor::default(),
            apply_to_physics: true,
            weight: 1.0,
            current_delta: RootMotionDelta::default(),
        }
    }
}

impl RootMotionAnimator {
    /// Update with new pose
    pub fn update(&mut self, root_position: Vec3, root_rotation: Quat, delta_time: f32) {
        self.current_delta = self.extractor.extract(root_position, root_rotation, delta_time);
    }

    /// Get weighted delta
    #[must_use]
    pub fn get_delta(&self) -> RootMotionDelta {
        RootMotionDelta {
            translation: self.current_delta.translation * self.weight,
            rotation: Quat::IDENTITY.slerp(self.current_delta.rotation, self.weight),
            velocity: self.current_delta.velocity * self.weight,
            angular_velocity: self.current_delta.angular_velocity * self.weight,
        }
    }

    /// Get raw unweighted delta
    #[must_use]
    pub fn raw_delta(&self) -> &RootMotionDelta {
        &self.current_delta
    }
}
