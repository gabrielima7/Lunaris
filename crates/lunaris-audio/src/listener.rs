//! Audio listener for spatial audio

use lunaris_core::math::Vec3;

/// Audio listener (usually attached to the camera)
#[derive(Debug, Clone)]
pub struct AudioListener {
    /// Position in world space
    pub position: Vec3,
    /// Forward direction
    pub forward: Vec3,
    /// Up direction
    pub up: Vec3,
    /// Velocity (for doppler effect)
    pub velocity: Vec3,
}

impl Default for AudioListener {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            forward: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::Y,
            velocity: Vec3::ZERO,
        }
    }
}

impl AudioListener {
    /// Create a new listener at a position
    #[must_use]
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }

    /// Set the listener orientation
    pub fn set_orientation(&mut self, forward: Vec3, up: Vec3) {
        self.forward = forward.normalize();
        self.up = up.normalize();
    }

    /// Get the right direction
    #[must_use]
    pub fn right(&self) -> Vec3 {
        self.forward.cross(self.up).normalize()
    }

    /// Calculate distance attenuation for a source
    #[must_use]
    pub fn calculate_attenuation(
        &self,
        source_pos: Vec3,
        min_distance: f32,
        max_distance: f32,
    ) -> f32 {
        let distance = self.position.distance(source_pos);

        if distance <= min_distance {
            1.0
        } else if distance >= max_distance {
            0.0
        } else {
            let range = max_distance - min_distance;
            1.0 - (distance - min_distance) / range
        }
    }

    /// Calculate stereo panning for a source (-1 = left, 1 = right)
    #[must_use]
    pub fn calculate_pan(&self, source_pos: Vec3) -> f32 {
        let to_source = (source_pos - self.position).normalize();
        let right = self.right();
        
        // Dot product with right vector gives pan
        to_source.dot(right).clamp(-1.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attenuation() {
        let listener = AudioListener::new(Vec3::ZERO);
        
        // At min distance, full volume
        let atten = listener.calculate_attenuation(Vec3::new(1.0, 0.0, 0.0), 1.0, 100.0);
        assert!((atten - 1.0).abs() < 0.01);

        // At max distance, no volume
        let atten = listener.calculate_attenuation(Vec3::new(100.0, 0.0, 0.0), 1.0, 100.0);
        assert!((atten).abs() < 0.01);
    }

    #[test]
    fn panning() {
        let listener = AudioListener::default();
        
        // Source on the right
        let pan = listener.calculate_pan(Vec3::new(10.0, 0.0, 0.0));
        assert!(pan > 0.5);

        // Source on the left
        let pan = listener.calculate_pan(Vec3::new(-10.0, 0.0, 0.0));
        assert!(pan < -0.5);
    }
}
