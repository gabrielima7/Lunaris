//! Procedural Animation
//!
//! Motion matching, procedural locomotion, ragdoll blending.

use glam::{Vec3, Quat};
use std::collections::HashMap;

/// Procedural animation system
pub struct ProceduralAnimation {
    pub motion_matching: MotionMatching,
    pub procedural_ik: ProceduralIK,
    pub ragdoll_blend: RagdollBlend,
    pub spring_bones: Vec<SpringBone>,
}

/// Motion matching
pub struct MotionMatching {
    pub database: MotionDatabase,
    pub current_pose: Pose,
    pub trajectory: Trajectory,
    pub blend_time: f32,
    pub search_interval: f32,
    pub last_search: f32,
}

/// Motion database
pub struct MotionDatabase {
    pub clips: Vec<MotionClip>,
    pub features: Vec<MotionFeature>,
}

/// Motion clip
pub struct MotionClip {
    pub name: String,
    pub frames: Vec<Pose>,
    pub frame_rate: f32,
    pub loop_clip: bool,
}

/// Motion feature (for matching)
pub struct MotionFeature {
    pub clip_index: usize,
    pub frame: usize,
    pub trajectory: Trajectory,
    pub foot_positions: [Vec3; 2],
    pub velocity: Vec3,
}

/// Pose
#[derive(Clone)]
pub struct Pose {
    pub bones: Vec<BonePose>,
    pub root_position: Vec3,
    pub root_rotation: Quat,
}

/// Bone pose
#[derive(Clone)]
pub struct BonePose {
    pub rotation: Quat,
    pub position: Vec3,
}

/// Trajectory
#[derive(Clone)]
pub struct Trajectory {
    pub points: Vec<TrajectoryPoint>,
}

/// Trajectory point
#[derive(Clone)]
pub struct TrajectoryPoint {
    pub position: Vec3,
    pub direction: Vec3,
    pub time: f32,
}

impl MotionMatching {
    pub fn new() -> Self {
        Self {
            database: MotionDatabase { clips: Vec::new(), features: Vec::new() },
            current_pose: Pose { bones: Vec::new(), root_position: Vec3::ZERO, root_rotation: Quat::IDENTITY },
            trajectory: Trajectory { points: Vec::new() },
            blend_time: 0.2,
            search_interval: 0.1,
            last_search: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, input_trajectory: &Trajectory) -> &Pose {
        self.last_search += dt;
        if self.last_search >= self.search_interval {
            self.last_search = 0.0;
            self.trajectory = input_trajectory.clone();
            self.find_best_match();
        }
        &self.current_pose
    }

    fn find_best_match(&mut self) {
        let mut best_cost = f32::MAX;
        let mut best_feature: Option<&MotionFeature> = None;

        for feature in &self.database.features {
            let cost = self.compute_cost(feature);
            if cost < best_cost {
                best_cost = cost;
                best_feature = Some(feature);
            }
        }

        if let Some(feature) = best_feature {
            if let Some(clip) = self.database.clips.get(feature.clip_index) {
                if let Some(pose) = clip.frames.get(feature.frame) {
                    self.current_pose = pose.clone();
                }
            }
        }
    }

    fn compute_cost(&self, feature: &MotionFeature) -> f32 {
        let mut cost = 0.0;
        for (i, point) in self.trajectory.points.iter().enumerate() {
            if let Some(fpoint) = feature.trajectory.points.get(i) {
                cost += (point.position - fpoint.position).length_squared();
                cost += (1.0 - point.direction.dot(fpoint.direction)) * 10.0;
            }
        }
        cost
    }
}

/// Procedural IK
pub struct ProceduralIK {
    pub enabled: bool,
    pub foot_ik: FootIK,
    pub look_ik: LookIK,
    pub reach_ik: ReachIK,
}

/// Foot IK
pub struct FootIK {
    pub left_foot: Vec3,
    pub right_foot: Vec3,
    pub pelvis_offset: f32,
    pub foot_height: f32,
    pub raycast_distance: f32,
}

impl FootIK {
    pub fn solve(&mut self, ground_height_fn: impl Fn(Vec3) -> f32, skeleton: &mut Pose) {
        let left_pos = skeleton.root_position + Vec3::new(-0.15, 0.0, 0.0);
        let right_pos = skeleton.root_position + Vec3::new(0.15, 0.0, 0.0);
        
        let left_ground = ground_height_fn(left_pos);
        let right_ground = ground_height_fn(right_pos);
        
        self.left_foot.y = left_ground + self.foot_height;
        self.right_foot.y = right_ground + self.foot_height;
        
        let lowest = left_ground.min(right_ground);
        self.pelvis_offset = -(skeleton.root_position.y - lowest - 1.0).max(0.0);
    }
}

/// Look IK
pub struct LookIK {
    pub target: Vec3,
    pub weight: f32,
    pub clamp_angle: f32,
}

/// Reach IK
pub struct ReachIK {
    pub target: Vec3,
    pub hand: Hand,
    pub weight: f32,
}

/// Hand
pub enum Hand { Left, Right }

/// Ragdoll blend
pub struct RagdollBlend {
    pub blend: f32,
    pub physics_bones: Vec<PhysicsBone>,
    pub state: RagdollState,
    pub blend_time: f32,
}

/// Ragdoll state
pub enum RagdollState { Animated, BlendingToPhysics, Physics, BlendingToAnimated }

/// Physics bone
pub struct PhysicsBone {
    pub bone_index: usize,
    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
    pub angular_velocity: Vec3,
}

impl RagdollBlend {
    pub fn activate(&mut self) {
        self.state = RagdollState::BlendingToPhysics;
        self.blend = 0.0;
    }

    pub fn deactivate(&mut self) {
        self.state = RagdollState::BlendingToAnimated;
        self.blend = 1.0;
    }

    pub fn update(&mut self, dt: f32, animated_pose: &Pose) -> Pose {
        match self.state {
            RagdollState::BlendingToPhysics => {
                self.blend = (self.blend + dt / self.blend_time).min(1.0);
                if self.blend >= 1.0 { self.state = RagdollState::Physics; }
            }
            RagdollState::BlendingToAnimated => {
                self.blend = (self.blend - dt / self.blend_time).max(0.0);
                if self.blend <= 0.0 { self.state = RagdollState::Animated; }
            }
            _ => {}
        }
        self.blend_poses(animated_pose)
    }

    fn blend_poses(&self, animated: &Pose) -> Pose {
        let mut result = animated.clone();
        for (i, bone) in result.bones.iter_mut().enumerate() {
            if let Some(phys) = self.physics_bones.get(i) {
                bone.position = bone.position.lerp(phys.position, self.blend);
                bone.rotation = bone.rotation.slerp(phys.rotation, self.blend);
            }
        }
        result
    }
}

/// Spring bone (for hair, cloth, tails)
pub struct SpringBone {
    pub bone_index: usize,
    pub stiffness: f32,
    pub damping: f32,
    pub gravity: f32,
    pub position: Vec3,
    pub prev_position: Vec3,
    pub initial_local: Vec3,
}

impl SpringBone {
    pub fn update(&mut self, dt: f32, parent_transform: (Vec3, Quat)) {
        let (parent_pos, parent_rot) = parent_transform;
        let target = parent_pos + parent_rot * self.initial_local;
        
        let velocity = self.position - self.prev_position;
        self.prev_position = self.position;
        
        let spring_force = (target - self.position) * self.stiffness;
        let gravity_force = Vec3::new(0.0, -self.gravity, 0.0);
        let damping_force = -velocity * self.damping;
        
        self.position += velocity + (spring_force + gravity_force + damping_force) * dt * dt;
    }
}
