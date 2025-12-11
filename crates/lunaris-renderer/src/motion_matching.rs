//! Motion Matching System
//!
//! Data-driven animation system for smooth, responsive character movement.

use glam::{Quat, Vec3};
use std::collections::HashMap;

/// Motion clip (animation segment)
#[derive(Debug, Clone)]
pub struct MotionClip {
    /// Unique ID
    pub id: u32,
    /// Name
    pub name: String,
    /// Duration in seconds
    pub duration: f32,
    /// Frames
    pub frames: Vec<MotionFrame>,
    /// Loop mode
    pub looping: bool,
    /// Tags
    pub tags: Vec<String>,
}

/// Motion frame data
#[derive(Debug, Clone)]
pub struct MotionFrame {
    /// Time in clip
    pub time: f32,
    /// Root position
    pub root_position: Vec3,
    /// Root rotation
    pub root_rotation: Quat,
    /// Root velocity
    pub root_velocity: Vec3,
    /// Root angular velocity
    pub root_angular_velocity: Vec3,
    /// Bone poses
    pub bone_poses: Vec<BonePose>,
    /// Feature vector for matching
    pub features: MotionFeatures,
}

/// Bone pose
#[derive(Debug, Clone, Copy)]
pub struct BonePose {
    /// Bone index
    pub bone_index: u32,
    /// Local position
    pub position: Vec3,
    /// Local rotation
    pub rotation: Quat,
    /// Velocity
    pub velocity: Vec3,
}

/// Features for motion matching
#[derive(Debug, Clone, Default)]
pub struct MotionFeatures {
    /// Future trajectory positions (relative)
    pub future_trajectory: Vec<Vec3>,
    /// Future trajectory directions
    pub future_directions: Vec<Vec3>,
    /// Left foot position
    pub left_foot_pos: Vec3,
    /// Right foot position
    pub right_foot_pos: Vec3,
    /// Left foot velocity
    pub left_foot_vel: Vec3,
    /// Right foot velocity
    pub right_foot_vel: Vec3,
    /// Hip velocity
    pub hip_velocity: Vec3,
}

impl MotionFeatures {
    /// Calculate distance to another feature set
    #[must_use]
    pub fn distance(&self, other: &MotionFeatures, weights: &FeatureWeights) -> f32 {
        let mut dist = 0.0;

        // Trajectory
        for (a, b) in self.future_trajectory.iter().zip(&other.future_trajectory) {
            dist += (a - b).length_squared() * weights.trajectory;
        }

        // Directions
        for (a, b) in self.future_directions.iter().zip(&other.future_directions) {
            dist += (a - b).length_squared() * weights.direction;
        }

        // Feet
        dist += (self.left_foot_pos - other.left_foot_pos).length_squared() * weights.feet_position;
        dist += (self.right_foot_pos - other.right_foot_pos).length_squared() * weights.feet_position;
        dist += (self.left_foot_vel - other.left_foot_vel).length_squared() * weights.feet_velocity;
        dist += (self.right_foot_vel - other.right_foot_vel).length_squared() * weights.feet_velocity;

        // Hip
        dist += (self.hip_velocity - other.hip_velocity).length_squared() * weights.hip_velocity;

        dist
    }
}

/// Feature weights for matching
#[derive(Debug, Clone)]
pub struct FeatureWeights {
    /// Trajectory position weight
    pub trajectory: f32,
    /// Direction weight
    pub direction: f32,
    /// Feet position weight
    pub feet_position: f32,
    /// Feet velocity weight
    pub feet_velocity: f32,
    /// Hip velocity weight
    pub hip_velocity: f32,
}

impl Default for FeatureWeights {
    fn default() -> Self {
        Self {
            trajectory: 1.0,
            direction: 1.5,
            feet_position: 0.75,
            feet_velocity: 1.0,
            hip_velocity: 1.0,
        }
    }
}

/// Motion database
pub struct MotionDatabase {
    /// All clips
    clips: Vec<MotionClip>,
    /// Clips by tag
    tag_index: HashMap<String, Vec<u32>>,
    /// Pre-computed KD-tree or similar for fast search
    feature_index: Vec<(u32, usize, MotionFeatures)>, // (clip_id, frame_idx, features)
}

impl Default for MotionDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl MotionDatabase {
    /// Create a new motion database
    #[must_use]
    pub fn new() -> Self {
        Self {
            clips: Vec::new(),
            tag_index: HashMap::new(),
            feature_index: Vec::new(),
        }
    }

    /// Add a clip
    pub fn add_clip(&mut self, clip: MotionClip) {
        let clip_id = clip.id;
        
        // Index by tags
        for tag in &clip.tags {
            self.tag_index.entry(tag.clone()).or_default().push(clip_id);
        }

        // Index features
        for (frame_idx, frame) in clip.frames.iter().enumerate() {
            self.feature_index.push((clip_id, frame_idx, frame.features.clone()));
        }

        self.clips.push(clip);
    }

    /// Find best matching frame
    #[must_use]
    pub fn find_best_match(
        &self,
        query: &MotionFeatures,
        weights: &FeatureWeights,
        tags: Option<&[String]>,
    ) -> Option<MotionMatch> {
        let mut best_match: Option<MotionMatch> = None;
        let mut best_cost = f32::MAX;

        // Filter by tags if specified
        let valid_clips: Option<Vec<u32>> = tags.map(|t| {
            t.iter()
                .filter_map(|tag| self.tag_index.get(tag))
                .flatten()
                .copied()
                .collect()
        });

        for (clip_id, frame_idx, features) in &self.feature_index {
            // Check tag filter
            if let Some(ref valid) = valid_clips {
                if !valid.contains(clip_id) {
                    continue;
                }
            }

            let cost = query.distance(features, weights);
            
            if cost < best_cost {
                best_cost = cost;
                best_match = Some(MotionMatch {
                    clip_id: *clip_id,
                    frame_index: *frame_idx,
                    cost,
                });
            }
        }

        best_match
    }

    /// Get clip by ID
    #[must_use]
    pub fn get_clip(&self, id: u32) -> Option<&MotionClip> {
        self.clips.iter().find(|c| c.id == id)
    }

    /// Get clip count
    #[must_use]
    pub fn clip_count(&self) -> usize {
        self.clips.len()
    }

    /// Get total frame count
    #[must_use]
    pub fn frame_count(&self) -> usize {
        self.feature_index.len()
    }
}

/// Motion match result
#[derive(Debug, Clone)]
pub struct MotionMatch {
    /// Matched clip ID
    pub clip_id: u32,
    /// Frame index
    pub frame_index: usize,
    /// Match cost (lower = better)
    pub cost: f32,
}

/// Motion matcher controller
pub struct MotionMatcher {
    /// Motion database
    database: MotionDatabase,
    /// Feature weights
    weights: FeatureWeights,
    /// Current clip
    current_clip: u32,
    /// Current time
    current_time: f32,
    /// Blend time
    blend_time: f32,
    /// Previous pose (for blending)
    previous_pose: Option<Vec<BonePose>>,
    /// Blend progress
    blend_progress: f32,
    /// Search interval
    pub search_interval: f32,
    /// Time since last search
    search_timer: f32,
    /// Min improvement for switch
    pub min_improvement: f32,
    /// Responsiveness (0 = smooth, 1 = responsive)
    pub responsiveness: f32,
}

impl MotionMatcher {
    /// Create a new motion matcher
    #[must_use]
    pub fn new(database: MotionDatabase) -> Self {
        Self {
            database,
            weights: FeatureWeights::default(),
            current_clip: 0,
            current_time: 0.0,
            blend_time: 0.2,
            previous_pose: None,
            blend_progress: 1.0,
            search_interval: 0.1,
            search_timer: 0.0,
            min_improvement: 0.5,
            responsiveness: 0.5,
        }
    }

    /// Update motion matcher
    pub fn update(
        &mut self,
        desired_trajectory: &[Vec3],
        desired_directions: &[Vec3],
        delta_time: f32,
    ) -> Option<Vec<BonePose>> {
        // Advance time
        self.current_time += delta_time;
        self.search_timer += delta_time;

        // Update blend
        if self.blend_progress < 1.0 {
            self.blend_progress = (self.blend_progress + delta_time / self.blend_time).min(1.0);
        }

        // Search for better match periodically
        if self.search_timer >= self.search_interval {
            self.search_timer = 0.0;
            
            // Build query features
            let query = MotionFeatures {
                future_trajectory: desired_trajectory.to_vec(),
                future_directions: desired_directions.to_vec(),
                ..Default::default()
            };

            if let Some(new_match) = self.database.find_best_match(&query, &self.weights, None) {
                // Check if significantly better
                if new_match.cost < self.min_improvement {
                    // Start transition
                    self.previous_pose = self.get_current_pose();
                    self.current_clip = new_match.clip_id;
                    self.current_time = self.database.get_clip(new_match.clip_id)
                        .map(|c| c.frames.get(new_match.frame_index).map(|f| f.time).unwrap_or(0.0))
                        .unwrap_or(0.0);
                    self.blend_progress = 0.0;
                }
            }
        }

        self.get_current_pose()
    }

    fn get_current_pose(&self) -> Option<Vec<BonePose>> {
        let clip = self.database.get_clip(self.current_clip)?;
        
        // Find frames for interpolation
        let mut frame_a = &clip.frames[0];
        let mut frame_b = &clip.frames[0];
        let mut t = 0.0;

        for i in 0..(clip.frames.len().saturating_sub(1)) {
            if clip.frames[i].time <= self.current_time && clip.frames[i + 1].time > self.current_time {
                frame_a = &clip.frames[i];
                frame_b = &clip.frames[i + 1];
                t = (self.current_time - frame_a.time) / (frame_b.time - frame_a.time);
                break;
            }
        }

        // Interpolate pose
        let mut pose = Vec::new();
        for (a, b) in frame_a.bone_poses.iter().zip(&frame_b.bone_poses) {
            pose.push(BonePose {
                bone_index: a.bone_index,
                position: a.position.lerp(b.position, t),
                rotation: a.rotation.slerp(b.rotation, t),
                velocity: a.velocity.lerp(b.velocity, t),
            });
        }

        // Blend with previous pose
        if self.blend_progress < 1.0 {
            if let Some(ref prev) = self.previous_pose {
                for (curr, prev_bone) in pose.iter_mut().zip(prev) {
                    curr.position = prev_bone.position.lerp(curr.position, self.blend_progress);
                    curr.rotation = prev_bone.rotation.slerp(curr.rotation, self.blend_progress);
                }
            }
        }

        Some(pose)
    }

    /// Set weights
    pub fn set_weights(&mut self, weights: FeatureWeights) {
        self.weights = weights;
    }

    /// Set blend time
    pub fn set_blend_time(&mut self, time: f32) {
        self.blend_time = time.max(0.01);
    }

    /// Get current clip ID
    #[must_use]
    pub fn current_clip_id(&self) -> u32 {
        self.current_clip
    }

    /// Get current time
    #[must_use]
    pub fn current_time(&self) -> f32 {
        self.current_time
    }

    /// Is blending
    #[must_use]
    pub fn is_blending(&self) -> bool {
        self.blend_progress < 1.0
    }
}

/// Trajectory prediction for motion matching
pub struct TrajectoryPredictor {
    /// Prediction horizon (seconds)
    pub horizon: f32,
    /// Sample count
    pub samples: u32,
    /// Smoothing factor
    pub smoothing: f32,
    /// History of positions
    history: Vec<Vec3>,
    /// History of times
    history_times: Vec<f32>,
}

impl Default for TrajectoryPredictor {
    fn default() -> Self {
        Self {
            horizon: 1.0,
            samples: 4,
            smoothing: 0.9,
            history: Vec::new(),
            history_times: Vec::new(),
        }
    }
}

impl TrajectoryPredictor {
    /// Update with current position
    pub fn update(&mut self, position: Vec3, time: f32) {
        self.history.push(position);
        self.history_times.push(time);

        // Keep limited history
        while self.history.len() > 60 {
            self.history.remove(0);
            self.history_times.remove(0);
        }
    }

    /// Predict future trajectory
    #[must_use]
    pub fn predict(&self, current_velocity: Vec3, desired_velocity: Vec3) -> Vec<Vec3> {
        let mut trajectory = Vec::new();
        let dt = self.horizon / self.samples as f32;
        
        let mut pos = *self.history.last().unwrap_or(&Vec3::ZERO);
        let mut vel = current_velocity;

        for i in 1..=self.samples {
            // Blend towards desired velocity
            let t = i as f32 / self.samples as f32;
            vel = vel.lerp(desired_velocity, t * (1.0 - self.smoothing));
            pos += vel * dt;
            trajectory.push(pos);
        }

        trajectory
    }
}
