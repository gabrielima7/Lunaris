//! Skeletal Animation System
//!
//! Provides bone-based animation for 3D models.

use lunaris_core::{
    id::{Id, TypedId},
    math::{Vec3, Vec2},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Skeleton identifier
pub type SkeletonId = TypedId<Skeleton>;

/// Animation clip identifier
pub type AnimationClipId = TypedId<AnimationClip>;

/// A bone in a skeleton
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bone {
    /// Bone name
    pub name: String,
    /// Parent bone index (-1 for root)
    pub parent: i32,
    /// Local position
    pub local_position: Vec3,
    /// Local rotation (euler angles)
    pub local_rotation: Vec3,
    /// Local scale
    pub local_scale: Vec3,
    /// Inverse bind matrix (flattened 4x4)
    pub inverse_bind: [f32; 16],
}

impl Default for Bone {
    fn default() -> Self {
        Self {
            name: String::new(),
            parent: -1,
            local_position: Vec3::ZERO,
            local_rotation: Vec3::ZERO,
            local_scale: Vec3::ONE,
            inverse_bind: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
}

/// A skeleton (bone hierarchy)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skeleton {
    /// Skeleton ID
    #[serde(skip)]
    pub id: SkeletonId,
    /// Skeleton name
    pub name: String,
    /// Bones
    pub bones: Vec<Bone>,
    /// Bone name to index mapping
    bone_map: HashMap<String, usize>,
}

impl Skeleton {
    /// Create a new skeleton
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: SkeletonId::new(),
            name: name.into(),
            bones: Vec::new(),
            bone_map: HashMap::new(),
        }
    }

    /// Add a bone
    pub fn add_bone(&mut self, bone: Bone) -> usize {
        let index = self.bones.len();
        self.bone_map.insert(bone.name.clone(), index);
        self.bones.push(bone);
        index
    }

    /// Get bone index by name
    #[must_use]
    pub fn bone_index(&self, name: &str) -> Option<usize> {
        self.bone_map.get(name).copied()
    }

    /// Get bone by index
    #[must_use]
    pub fn bone(&self, index: usize) -> Option<&Bone> {
        self.bones.get(index)
    }

    /// Get bone count
    #[must_use]
    pub fn bone_count(&self) -> usize {
        self.bones.len()
    }
}

/// Animation keyframe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe {
    /// Time in seconds
    pub time: f32,
    /// Position
    pub position: Option<Vec3>,
    /// Rotation (euler angles)
    pub rotation: Option<Vec3>,
    /// Scale
    pub scale: Option<Vec3>,
}

/// Animation channel (keyframes for one bone)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationChannel {
    /// Target bone name
    pub bone_name: String,
    /// Keyframes
    pub keyframes: Vec<Keyframe>,
}

impl AnimationChannel {
    /// Create a new channel
    #[must_use]
    pub fn new(bone_name: impl Into<String>) -> Self {
        Self {
            bone_name: bone_name.into(),
            keyframes: Vec::new(),
        }
    }

    /// Add a keyframe
    pub fn add_keyframe(&mut self, keyframe: Keyframe) {
        self.keyframes.push(keyframe);
        // Keep sorted by time
        self.keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap_or(std::cmp::Ordering::Equal));
    }

    /// Sample the channel at a time
    #[must_use]
    pub fn sample(&self, time: f32) -> (Option<Vec3>, Option<Vec3>, Option<Vec3>) {
        if self.keyframes.is_empty() {
            return (None, None, None);
        }

        // Find keyframes to interpolate
        let mut prev_idx = 0;
        let mut next_idx = 0;
        for (i, kf) in self.keyframes.iter().enumerate() {
            if kf.time <= time {
                prev_idx = i;
            }
            if kf.time >= time {
                next_idx = i;
                break;
            }
            next_idx = i;
        }

        let prev = &self.keyframes[prev_idx];
        let next = &self.keyframes[next_idx];

        if prev_idx == next_idx || prev.time == next.time {
            return (prev.position, prev.rotation, prev.scale);
        }

        // Interpolation factor
        let t = (time - prev.time) / (next.time - prev.time);

        // Lerp values
        let position = match (prev.position, next.position) {
            (Some(p1), Some(p2)) => Some(p1.lerp(p2, t)),
            (Some(p), None) | (None, Some(p)) => Some(p),
            (None, None) => None,
        };

        let rotation = match (prev.rotation, next.rotation) {
            (Some(r1), Some(r2)) => Some(r1.lerp(r2, t)),
            (Some(r), None) | (None, Some(r)) => Some(r),
            (None, None) => None,
        };

        let scale = match (prev.scale, next.scale) {
            (Some(s1), Some(s2)) => Some(s1.lerp(s2, t)),
            (Some(s), None) | (None, Some(s)) => Some(s),
            (None, None) => None,
        };

        (position, rotation, scale)
    }
}

/// Animation clip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationClip {
    /// Clip ID
    #[serde(skip)]
    pub id: AnimationClipId,
    /// Clip name
    pub name: String,
    /// Duration in seconds
    pub duration: f32,
    /// Channels
    pub channels: Vec<AnimationChannel>,
    /// Is looping
    pub looping: bool,
}

impl AnimationClip {
    /// Create a new animation clip
    #[must_use]
    pub fn new(name: impl Into<String>, duration: f32) -> Self {
        Self {
            id: AnimationClipId::new(),
            name: name.into(),
            duration,
            channels: Vec::new(),
            looping: true,
        }
    }

    /// Add a channel
    pub fn add_channel(&mut self, channel: AnimationChannel) {
        self.channels.push(channel);
    }

    /// Get channel for bone
    #[must_use]
    pub fn channel(&self, bone_name: &str) -> Option<&AnimationChannel> {
        self.channels.iter().find(|c| c.bone_name == bone_name)
    }
}

/// Animation playback state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationState {
    /// Stopped
    #[default]
    Stopped,
    /// Playing
    Playing,
    /// Paused
    Paused,
}

/// Animation player for a skeleton
#[derive(Debug)]
pub struct SkeletalAnimator {
    /// Skeleton
    pub skeleton_id: Option<SkeletonId>,
    /// Current animation
    pub current_clip: Option<AnimationClipId>,
    /// Playback state
    pub state: AnimationState,
    /// Current time
    pub time: f32,
    /// Playback speed
    pub speed: f32,
    /// Blend weight
    pub weight: f32,
    /// Current bone transforms (local)
    pub bone_locals: Vec<BoneTransform>,
    /// Current bone transforms (world)
    pub bone_worlds: Vec<[f32; 16]>,
}

/// Bone transform
#[derive(Debug, Clone, Copy, Default)]
pub struct BoneTransform {
    /// Position
    pub position: Vec3,
    /// Rotation (euler)
    pub rotation: Vec3,
    /// Scale
    pub scale: Vec3,
}

impl Default for SkeletalAnimator {
    fn default() -> Self {
        Self::new()
    }
}

impl SkeletalAnimator {
    /// Create a new animator
    #[must_use]
    pub fn new() -> Self {
        Self {
            skeleton_id: None,
            current_clip: None,
            state: AnimationState::Stopped,
            time: 0.0,
            speed: 1.0,
            weight: 1.0,
            bone_locals: Vec::new(),
            bone_worlds: Vec::new(),
        }
    }

    /// Set the skeleton
    pub fn set_skeleton(&mut self, skeleton: &Skeleton) {
        self.skeleton_id = Some(skeleton.id);
        self.bone_locals = skeleton.bones.iter().map(|b| BoneTransform {
            position: b.local_position,
            rotation: b.local_rotation,
            scale: b.local_scale,
        }).collect();
        self.bone_worlds = vec![[
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]; skeleton.bone_count()];
    }

    /// Play an animation
    pub fn play(&mut self, clip_id: AnimationClipId) {
        self.current_clip = Some(clip_id);
        self.state = AnimationState::Playing;
        self.time = 0.0;
    }

    /// Stop playback
    pub fn stop(&mut self) {
        self.state = AnimationState::Stopped;
        self.time = 0.0;
    }

    /// Pause playback
    pub fn pause(&mut self) {
        if self.state == AnimationState::Playing {
            self.state = AnimationState::Paused;
        }
    }

    /// Resume playback
    pub fn resume(&mut self) {
        if self.state == AnimationState::Paused {
            self.state = AnimationState::Playing;
        }
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32, clip: &AnimationClip, skeleton: &Skeleton) {
        if self.state != AnimationState::Playing {
            return;
        }

        self.time += delta_time * self.speed;

        // Handle looping
        if self.time >= clip.duration {
            if clip.looping {
                self.time %= clip.duration;
            } else {
                self.time = clip.duration;
                self.state = AnimationState::Stopped;
            }
        }

        // Sample channels
        for channel in &clip.channels {
            if let Some(bone_idx) = skeleton.bone_index(&channel.bone_name) {
                let (pos, rot, scale) = channel.sample(self.time);
                let transform = &mut self.bone_locals[bone_idx];

                if let Some(p) = pos {
                    transform.position = transform.position.lerp(p, self.weight);
                }
                if let Some(r) = rot {
                    transform.rotation = transform.rotation.lerp(r, self.weight);
                }
                if let Some(s) = scale {
                    transform.scale = transform.scale.lerp(s, self.weight);
                }
            }
        }

        // Calculate world transforms
        self.calculate_world_transforms(skeleton);
    }

    fn calculate_world_transforms(&mut self, skeleton: &Skeleton) {
        for i in 0..skeleton.bones.len() {
            let local = &self.bone_locals[i];
            let bone = &skeleton.bones[i];

            // Create local matrix (simplified)
            let local_matrix = create_transform_matrix(local.position, local.rotation, local.scale);

            if bone.parent < 0 {
                self.bone_worlds[i] = local_matrix;
            } else {
                let parent_world = self.bone_worlds[bone.parent as usize];
                self.bone_worlds[i] = multiply_matrices(parent_world, local_matrix);
            }
        }
    }

    /// Get final bone matrices for skinning
    #[must_use]
    pub fn skinning_matrices(&self, skeleton: &Skeleton) -> Vec<[f32; 16]> {
        self.bone_worlds.iter()
            .zip(&skeleton.bones)
            .map(|(world, bone)| multiply_matrices(*world, bone.inverse_bind))
            .collect()
    }
}

// Matrix helpers
fn create_transform_matrix(pos: Vec3, rot: Vec3, scale: Vec3) -> [f32; 16] {
    let (sx, cx) = rot.x.sin_cos();
    let (sy, cy) = rot.y.sin_cos();
    let (sz, cz) = rot.z.sin_cos();

    [
        cy * cz * scale.x, cy * sz * scale.x, -sy * scale.x, 0.0,
        (sx * sy * cz - cx * sz) * scale.y, (sx * sy * sz + cx * cz) * scale.y, sx * cy * scale.y, 0.0,
        (cx * sy * cz + sx * sz) * scale.z, (cx * sy * sz - sx * cz) * scale.z, cx * cy * scale.z, 0.0,
        pos.x, pos.y, pos.z, 1.0,
    ]
}

fn multiply_matrices(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    let mut result = [0.0; 16];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result[i * 4 + j] += a[i * 4 + k] * b[k * 4 + j];
            }
        }
    }
    result
}

/// Animation state machine
pub struct AnimationStateMachine {
    /// States
    states: HashMap<String, AnimState>,
    /// Current state
    current_state: Option<String>,
    /// Transitions
    transitions: Vec<AnimTransition>,
    /// Parameters
    parameters: HashMap<String, AnimParam>,
}

/// Animation state
#[derive(Debug, Clone)]
pub struct AnimState {
    /// State name
    pub name: String,
    /// Animation clip ID
    pub clip_id: AnimationClipId,
    /// Speed multiplier
    pub speed: f32,
}

/// Animation transition
#[derive(Debug, Clone)]
pub struct AnimTransition {
    /// From state
    pub from: String,
    /// To state
    pub to: String,
    /// Condition
    pub condition: TransitionCondition,
    /// Blend duration
    pub blend_duration: f32,
}

/// Transition condition
#[derive(Debug, Clone)]
pub enum TransitionCondition {
    /// Always transition
    Always,
    /// Parameter equals value
    ParamEquals(String, bool),
    /// Parameter greater than
    ParamGreater(String, f32),
    /// Parameter less than
    ParamLess(String, f32),
    /// Animation finished
    AnimationEnd,
}

/// Animation parameter
#[derive(Debug, Clone)]
pub enum AnimParam {
    /// Boolean
    Bool(bool),
    /// Float
    Float(f32),
    /// Trigger (resets after check)
    Trigger(bool),
}

impl Default for AnimationStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationStateMachine {
    /// Create a new state machine
    #[must_use]
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            current_state: None,
            transitions: Vec::new(),
            parameters: HashMap::new(),
        }
    }

    /// Add a state
    pub fn add_state(&mut self, name: impl Into<String>, clip_id: AnimationClipId) {
        let name = name.into();
        self.states.insert(name.clone(), AnimState {
            name,
            clip_id,
            speed: 1.0,
        });
    }

    /// Add a transition
    pub fn add_transition(&mut self, from: impl Into<String>, to: impl Into<String>, condition: TransitionCondition, blend: f32) {
        self.transitions.push(AnimTransition {
            from: from.into(),
            to: to.into(),
            condition,
            blend_duration: blend,
        });
    }

    /// Set parameter
    pub fn set_bool(&mut self, name: &str, value: bool) {
        self.parameters.insert(name.to_string(), AnimParam::Bool(value));
    }

    /// Set float parameter
    pub fn set_float(&mut self, name: &str, value: f32) {
        self.parameters.insert(name.to_string(), AnimParam::Float(value));
    }

    /// Trigger parameter
    pub fn trigger(&mut self, name: &str) {
        self.parameters.insert(name.to_string(), AnimParam::Trigger(true));
    }

    /// Set initial state
    pub fn set_state(&mut self, name: &str) {
        if self.states.contains_key(name) {
            self.current_state = Some(name.to_string());
        }
    }

    /// Get current state
    #[must_use]
    pub fn current_state(&self) -> Option<&AnimState> {
        self.current_state.as_ref().and_then(|n| self.states.get(n))
    }

    /// Update and check transitions
    pub fn update(&mut self, _delta_time: f32) -> Option<AnimationClipId> {
        let current = self.current_state.as_ref()?;

        // Check transitions
        for transition in &self.transitions {
            if &transition.from != current {
                continue;
            }

            let should_transition = match &transition.condition {
                TransitionCondition::Always => true,
                TransitionCondition::ParamEquals(name, value) => {
                    matches!(self.parameters.get(name), Some(AnimParam::Bool(v)) if v == value)
                }
                TransitionCondition::ParamGreater(name, threshold) => {
                    matches!(self.parameters.get(name), Some(AnimParam::Float(v)) if v > threshold)
                }
                TransitionCondition::ParamLess(name, threshold) => {
                    matches!(self.parameters.get(name), Some(AnimParam::Float(v)) if v < threshold)
                }
                TransitionCondition::AnimationEnd => false, // Would need animation time info
            };

            if should_transition {
                self.current_state = Some(transition.to.clone());
                return self.states.get(&transition.to).map(|s| s.clip_id);
            }
        }

        None
    }
}
