//! Inverse Kinematics System
//!
//! Full-body IK with multiple solvers for realistic character animation.

use glam::{Quat, Vec3, Mat4};
use std::collections::HashMap;

/// IK chain bone
#[derive(Debug, Clone)]
pub struct IKBone {
    /// Bone index
    pub index: u32,
    /// Local position
    pub local_position: Vec3,
    /// Local rotation
    pub local_rotation: Quat,
    /// Bone length
    pub length: f32,
    /// Parent index (None = root)
    pub parent: Option<u32>,
    /// Rotation constraints
    pub constraints: RotationConstraints,
}

/// Rotation constraints
#[derive(Debug, Clone, Copy)]
pub struct RotationConstraints {
    /// Min rotation (euler angles, radians)
    pub min: Vec3,
    /// Max rotation (euler angles, radians)
    pub max: Vec3,
    /// Stiffness (0-1)
    pub stiffness: f32,
}

impl Default for RotationConstraints {
    fn default() -> Self {
        Self {
            min: Vec3::splat(-std::f32::consts::PI),
            max: Vec3::splat(std::f32::consts::PI),
            stiffness: 0.0,
        }
    }
}

/// IK target
#[derive(Debug, Clone)]
pub struct IKTarget {
    /// Target position
    pub position: Vec3,
    /// Target rotation (optional)
    pub rotation: Option<Quat>,
    /// Weight (0-1)
    pub weight: f32,
    /// Blend speed
    pub blend_speed: f32,
}

impl Default for IKTarget {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: None,
            weight: 1.0,
            blend_speed: 10.0,
        }
    }
}

/// IK solver type
#[derive(Debug, Clone, Copy, Default)]
pub enum IKSolverType {
    /// Two-bone IK (arm/leg)
    #[default]
    TwoBone,
    /// FABRIK (Forward And Backward Reaching IK)
    FABRIK,
    /// CCD (Cyclic Coordinate Descent)
    CCD,
    /// Full body IK
    FullBody,
    /// Look-at IK
    LookAt,
}

/// Two-bone IK solver (for arms/legs)
pub struct TwoBoneIK {
    /// Upper bone index
    pub upper_bone: u32,
    /// Middle bone index (elbow/knee)
    pub middle_bone: u32,
    /// End bone index (hand/foot)
    pub end_bone: u32,
    /// Pole vector (elbow/knee direction hint)
    pub pole_target: Vec3,
    /// Pole weight
    pub pole_weight: f32,
    /// Softness
    pub softness: f32,
    /// Twist offset
    pub twist_offset: f32,
}

impl TwoBoneIK {
    /// Solve two-bone IK
    #[must_use]
    pub fn solve(
        &self,
        upper_pos: Vec3,
        upper_len: f32,
        lower_len: f32,
        target: Vec3,
    ) -> (Quat, Quat) {
        let total_len = upper_len + lower_len;
        let target_vec = target - upper_pos;
        let target_dist = target_vec.length().min(total_len * 0.9999);
        let target_dir = target_vec.normalize();

        // Law of cosines for elbow/knee angle
        let cos_elbow = ((upper_len * upper_len + lower_len * lower_len - target_dist * target_dist)
            / (2.0 * upper_len * lower_len))
            .clamp(-1.0, 1.0);
        let elbow_angle = cos_elbow.acos();

        // Shoulder/hip angle
        let cos_shoulder = ((upper_len * upper_len + target_dist * target_dist - lower_len * lower_len)
            / (2.0 * upper_len * target_dist))
            .clamp(-1.0, 1.0);
        let shoulder_angle = cos_shoulder.acos();

        // Calculate pole direction
        let pole_dir = (self.pole_target - upper_pos).normalize();
        let plane_normal = target_dir.cross(pole_dir).normalize();
        let bend_dir = plane_normal.cross(target_dir).normalize();

        // Calculate rotations
        let upper_rot = Quat::from_rotation_arc(Vec3::Y, target_dir)
            * Quat::from_axis_angle(plane_normal, -shoulder_angle);
        
        let lower_rot = Quat::from_axis_angle(Vec3::X, std::f32::consts::PI - elbow_angle);

        (upper_rot, lower_rot)
    }
}

/// FABRIK solver (multi-bone chain)
pub struct FABRIKSolver {
    /// Bone positions
    positions: Vec<Vec3>,
    /// Bone lengths
    lengths: Vec<f32>,
    /// Max iterations
    pub max_iterations: u32,
    /// Tolerance
    pub tolerance: f32,
}

impl FABRIKSolver {
    /// Create a new FABRIK solver
    #[must_use]
    pub fn new(max_iterations: u32, tolerance: f32) -> Self {
        Self {
            positions: Vec::new(),
            lengths: Vec::new(),
            max_iterations,
            tolerance,
        }
    }

    /// Initialize chain
    pub fn set_chain(&mut self, positions: Vec<Vec3>) {
        self.lengths.clear();
        for i in 0..(positions.len().saturating_sub(1)) {
            self.lengths.push((positions[i + 1] - positions[i]).length());
        }
        self.positions = positions;
    }

    /// Solve IK for target
    pub fn solve(&mut self, target: Vec3, base_fixed: bool) -> &[Vec3] {
        if self.positions.len() < 2 {
            return &self.positions;
        }

        let base = self.positions[0];
        let n = self.positions.len();

        for _ in 0..self.max_iterations {
            // Check if close enough
            let end_pos = self.positions[n - 1];
            if (end_pos - target).length() < self.tolerance {
                break;
            }

            // Forward reaching (from end to base)
            self.positions[n - 1] = target;
            for i in (1..n).rev() {
                let dir = (self.positions[i - 1] - self.positions[i]).normalize();
                self.positions[i - 1] = self.positions[i] + dir * self.lengths[i - 1];
            }

            // Backward reaching (from base to end)
            if base_fixed {
                self.positions[0] = base;
            }
            for i in 0..(n - 1) {
                let dir = (self.positions[i + 1] - self.positions[i]).normalize();
                self.positions[i + 1] = self.positions[i] + dir * self.lengths[i];
            }
        }

        &self.positions
    }

    /// Get positions
    #[must_use]
    pub fn positions(&self) -> &[Vec3] {
        &self.positions
    }
}

/// CCD (Cyclic Coordinate Descent) solver
pub struct CCDSolver {
    /// Max iterations
    pub max_iterations: u32,
    /// Tolerance
    pub tolerance: f32,
    /// Damping
    pub damping: f32,
}

impl Default for CCDSolver {
    fn default() -> Self {
        Self {
            max_iterations: 10,
            tolerance: 0.001,
            damping: 0.5,
        }
    }
}

impl CCDSolver {
    /// Solve CCD for a chain
    #[must_use]
    pub fn solve(&self, bones: &mut [IKBone], target: Vec3) -> bool {
        if bones.is_empty() {
            return false;
        }

        for _ in 0..self.max_iterations {
            let end_pos = self.calculate_end_position(bones);
            if (end_pos - target).length() < self.tolerance {
                return true;
            }

            // Iterate from end to root
            for i in (0..bones.len()).rev() {
                let bone_pos = self.calculate_bone_position(bones, i);
                let end_pos = self.calculate_end_position(bones);

                let to_end = (end_pos - bone_pos).normalize();
                let to_target = (target - bone_pos).normalize();

                let rotation = Quat::from_rotation_arc(to_end, to_target);
                let damped = Quat::IDENTITY.slerp(rotation, self.damping);
                
                bones[i].local_rotation = damped * bones[i].local_rotation;
                
                // Apply constraints
                self.apply_constraints(&mut bones[i]);
            }
        }

        false
    }

    fn calculate_end_position(&self, bones: &[IKBone]) -> Vec3 {
        self.calculate_bone_position(bones, bones.len())
    }

    fn calculate_bone_position(&self, bones: &[IKBone], index: usize) -> Vec3 {
        let mut pos = Vec3::ZERO;
        let mut rot = Quat::IDENTITY;

        for (i, bone) in bones.iter().enumerate() {
            if i >= index {
                break;
            }
            rot = rot * bone.local_rotation;
            pos += rot * Vec3::new(0.0, bone.length, 0.0);
        }

        pos
    }

    fn apply_constraints(&self, bone: &mut IKBone) {
        let (mut x, mut y, mut z) = bone.local_rotation.to_euler(glam::EulerRot::XYZ);
        
        x = x.clamp(bone.constraints.min.x, bone.constraints.max.x);
        y = y.clamp(bone.constraints.min.y, bone.constraints.max.y);
        z = z.clamp(bone.constraints.min.z, bone.constraints.max.z);
        
        bone.local_rotation = Quat::from_euler(glam::EulerRot::XYZ, x, y, z);
    }
}

/// Look-at IK (for head/eyes)
pub struct LookAtIK {
    /// Head bone index
    pub head_bone: u32,
    /// Neck bone index
    pub neck_bone: Option<u32>,
    /// Spine bones
    pub spine_bones: Vec<u32>,
    /// Look target
    pub target: Vec3,
    /// Weight
    pub weight: f32,
    /// Clamp angle (degrees)
    pub clamp_angle: f32,
    /// Spine contribution (0-1)
    pub spine_weight: f32,
}

impl Default for LookAtIK {
    fn default() -> Self {
        Self {
            head_bone: 0,
            neck_bone: None,
            spine_bones: Vec::new(),
            target: Vec3::Z,
            weight: 1.0,
            clamp_angle: 80.0,
            spine_weight: 0.3,
        }
    }
}

impl LookAtIK {
    /// Calculate look-at rotation
    #[must_use]
    pub fn calculate_rotation(&self, current_forward: Vec3, eye_position: Vec3) -> Quat {
        let to_target = (self.target - eye_position).normalize();
        
        // Clamp angle
        let angle = current_forward.dot(to_target).acos();
        let max_angle = self.clamp_angle.to_radians();
        
        let clamped_target = if angle > max_angle {
            let axis = current_forward.cross(to_target).normalize();
            Quat::from_axis_angle(axis, max_angle) * current_forward
        } else {
            to_target
        };

        let rotation = Quat::from_rotation_arc(current_forward, clamped_target);
        Quat::IDENTITY.slerp(rotation, self.weight)
    }
}

/// Full body IK rig
pub struct FullBodyIK {
    /// Spine chain
    pub spine: FABRIKSolver,
    /// Left arm
    pub left_arm: TwoBoneIK,
    /// Right arm
    pub right_arm: TwoBoneIK,
    /// Left leg
    pub left_leg: TwoBoneIK,
    /// Right leg
    pub right_leg: TwoBoneIK,
    /// Head look-at
    pub look_at: LookAtIK,
    /// Targets
    targets: HashMap<String, IKTarget>,
    /// Weight
    pub weight: f32,
}

impl FullBodyIK {
    /// Create a new full body IK
    #[must_use]
    pub fn new() -> Self {
        Self {
            spine: FABRIKSolver::new(10, 0.001),
            left_arm: TwoBoneIK {
                upper_bone: 0,
                middle_bone: 1,
                end_bone: 2,
                pole_target: Vec3::new(-1.0, 0.0, -1.0),
                pole_weight: 1.0,
                softness: 0.0,
                twist_offset: 0.0,
            },
            right_arm: TwoBoneIK {
                upper_bone: 0,
                middle_bone: 1,
                end_bone: 2,
                pole_target: Vec3::new(1.0, 0.0, -1.0),
                pole_weight: 1.0,
                softness: 0.0,
                twist_offset: 0.0,
            },
            left_leg: TwoBoneIK {
                upper_bone: 0,
                middle_bone: 1,
                end_bone: 2,
                pole_target: Vec3::new(0.0, 0.0, 1.0),
                pole_weight: 1.0,
                softness: 0.0,
                twist_offset: 0.0,
            },
            right_leg: TwoBoneIK {
                upper_bone: 0,
                middle_bone: 1,
                end_bone: 2,
                pole_target: Vec3::new(0.0, 0.0, 1.0),
                pole_weight: 1.0,
                softness: 0.0,
                twist_offset: 0.0,
            },
            look_at: LookAtIK::default(),
            targets: HashMap::new(),
            weight: 1.0,
        }
    }

    /// Set target
    pub fn set_target(&mut self, name: &str, target: IKTarget) {
        self.targets.insert(name.to_string(), target);
    }

    /// Get target
    #[must_use]
    pub fn get_target(&self, name: &str) -> Option<&IKTarget> {
        self.targets.get(name)
    }

    /// Clear all targets
    pub fn clear_targets(&mut self) {
        self.targets.clear();
    }
}

impl Default for FullBodyIK {
    fn default() -> Self {
        Self::new()
    }
}

/// Foot IK for ground adaptation
pub struct FootIK {
    /// Left foot target
    pub left_foot: IKTarget,
    /// Right foot target
    pub right_foot: IKTarget,
    /// Pelvis adjustment
    pub pelvis_offset: Vec3,
    /// Max step height
    pub max_step_height: f32,
    /// Foot height offset
    pub foot_height: f32,
    /// Raycast distance
    pub raycast_distance: f32,
    /// Blend speed
    pub blend_speed: f32,
}

impl Default for FootIK {
    fn default() -> Self {
        Self {
            left_foot: IKTarget::default(),
            right_foot: IKTarget::default(),
            pelvis_offset: Vec3::ZERO,
            max_step_height: 0.5,
            foot_height: 0.1,
            raycast_distance: 1.0,
            blend_speed: 10.0,
        }
    }
}

impl FootIK {
    /// Update foot positions from raycast results
    pub fn update(&mut self, left_ground: Vec3, right_ground: Vec3, delta_time: f32) {
        // Calculate target foot positions
        let left_target = left_ground + Vec3::Y * self.foot_height;
        let right_target = right_ground + Vec3::Y * self.foot_height;

        // Blend towards targets
        let blend = (self.blend_speed * delta_time).clamp(0.0, 1.0);
        self.left_foot.position = self.left_foot.position.lerp(left_target, blend);
        self.right_foot.position = self.right_foot.position.lerp(right_target, blend);

        // Adjust pelvis
        let height_diff = (left_ground.y - right_ground.y).abs();
        if height_diff <= self.max_step_height {
            let lower = left_ground.y.min(right_ground.y);
            self.pelvis_offset.y = (lower - self.pelvis_offset.y) * blend;
        }
    }
}
