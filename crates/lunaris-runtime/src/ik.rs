//! Inverse Kinematics
//!
//! Full-body IK, look-at, aim, and foot placement.

use glam::{Vec3, Quat, Mat4};

/// IK rig
pub struct IKRig {
    pub bones: Vec<IKBone>,
    pub chains: Vec<IKChain>,
    pub constraints: Vec<IKConstraint>,
    pub targets: Vec<IKTarget>,
}

/// IK bone
pub struct IKBone {
    pub id: u64,
    pub name: String,
    pub parent: Option<u64>,
    pub local_position: Vec3,
    pub local_rotation: Quat,
    pub world_position: Vec3,
    pub world_rotation: Quat,
    pub length: f32,
}

/// IK chain
pub struct IKChain {
    pub name: String,
    pub bones: Vec<u64>,
    pub solver: IKSolver,
    pub iterations: u32,
    pub tolerance: f32,
}

/// IK solver
pub enum IKSolver { FABRIK, CCD, TwoBone, Jacobian }

/// IK constraint
pub struct IKConstraint {
    pub bone_id: u64,
    pub constraint_type: ConstraintType,
}

/// Constraint type
pub enum ConstraintType {
    Hinge { axis: Vec3, min: f32, max: f32 },
    Ball { twist_min: f32, twist_max: f32, swing_limit: f32 },
    Fixed,
}

/// IK target
pub struct IKTarget {
    pub chain_name: String,
    pub position: Vec3,
    pub rotation: Option<Quat>,
    pub weight: f32,
}

impl IKRig {
    pub fn new() -> Self { Self { bones: Vec::new(), chains: Vec::new(), constraints: Vec::new(), targets: Vec::new() } }

    pub fn solve(&mut self) {
        for chain in &self.chains {
            if let Some(target) = self.targets.iter().find(|t| t.chain_name == chain.name) {
                match chain.solver {
                    IKSolver::FABRIK => self.solve_fabrik(chain, target),
                    IKSolver::TwoBone => self.solve_two_bone(chain, target),
                    _ => {}
                }
            }
        }
    }

    fn solve_fabrik(&mut self, chain: &IKChain, target: &IKTarget) {
        let bone_ids: Vec<u64> = chain.bones.clone();
        if bone_ids.len() < 2 { return; }

        for _ in 0..chain.iterations {
            // Backward pass
            self.set_bone_position(*bone_ids.last().unwrap(), target.position);
            for i in (1..bone_ids.len()).rev() {
                let current = self.get_bone_position(bone_ids[i]);
                let prev = self.get_bone_position(bone_ids[i - 1]);
                let length = self.get_bone_length(bone_ids[i - 1]);
                let dir = (prev - current).normalize();
                self.set_bone_position(bone_ids[i - 1], current + dir * length);
            }

            // Forward pass
            let root_pos = self.get_bone_position(bone_ids[0]);
            for i in 1..bone_ids.len() {
                let prev = self.get_bone_position(bone_ids[i - 1]);
                let current = self.get_bone_position(bone_ids[i]);
                let length = self.get_bone_length(bone_ids[i - 1]);
                let dir = (current - prev).normalize();
                self.set_bone_position(bone_ids[i], prev + dir * length);
            }
        }
    }

    fn solve_two_bone(&mut self, chain: &IKChain, target: &IKTarget) {
        if chain.bones.len() != 3 { return; }
        let (root, mid, end) = (chain.bones[0], chain.bones[1], chain.bones[2]);
        
        let root_pos = self.get_bone_position(root);
        let len_a = self.get_bone_length(root);
        let len_b = self.get_bone_length(mid);
        
        let target_vec = target.position - root_pos;
        let target_dist = target_vec.length().min(len_a + len_b - 0.001);
        
        // Law of cosines
        let cos_angle = ((len_a * len_a + target_dist * target_dist - len_b * len_b) / (2.0 * len_a * target_dist)).clamp(-1.0, 1.0);
        let _angle = cos_angle.acos();
        
        // Apply rotations
        let dir = target_vec.normalize();
        self.set_bone_position(mid, root_pos + dir * len_a);
        self.set_bone_position(end, target.position);
    }

    fn get_bone_position(&self, id: u64) -> Vec3 {
        self.bones.iter().find(|b| b.id == id).map(|b| b.world_position).unwrap_or(Vec3::ZERO)
    }

    fn set_bone_position(&mut self, id: u64, pos: Vec3) {
        if let Some(bone) = self.bones.iter_mut().find(|b| b.id == id) { bone.world_position = pos; }
    }

    fn get_bone_length(&self, id: u64) -> f32 {
        self.bones.iter().find(|b| b.id == id).map(|b| b.length).unwrap_or(1.0)
    }
}

/// Look-at IK
pub struct LookAtIK {
    pub head_bone: u64,
    pub target: Vec3,
    pub weight: f32,
    pub clamp_angle: f32,
}

impl LookAtIK {
    pub fn solve(&self, rig: &mut IKRig) {
        if let Some(bone) = rig.bones.iter_mut().find(|b| b.id == self.head_bone) {
            let dir = (self.target - bone.world_position).normalize();
            let target_rot = Quat::from_rotation_arc(Vec3::Z, dir);
            bone.world_rotation = bone.world_rotation.slerp(target_rot, self.weight);
        }
    }
}

/// Foot IK (ground alignment)
pub struct FootIK {
    pub left_foot: u64,
    pub right_foot: u64,
    pub raycast_height: f32,
    pub raycast_distance: f32,
    pub hip_adjustment: f32,
}

impl FootIK {
    pub fn solve(&self, rig: &mut IKRig, get_ground_height: impl Fn(Vec3) -> f32) {
        for foot_id in [self.left_foot, self.right_foot] {
            if let Some(bone) = rig.bones.iter_mut().find(|b| b.id == foot_id) {
                let ground = get_ground_height(bone.world_position);
                if bone.world_position.y < ground + 0.1 {
                    bone.world_position.y = ground;
                }
            }
        }
    }
}

/// Aim IK
pub struct AimIK {
    pub spine_bones: Vec<u64>,
    pub target: Vec3,
    pub weight: f32,
}

impl AimIK {
    pub fn solve(&self, rig: &mut IKRig) {
        let weight_per_bone = self.weight / self.spine_bones.len() as f32;
        for bone_id in &self.spine_bones {
            if let Some(bone) = rig.bones.iter_mut().find(|b| b.id == *bone_id) {
                let dir = (self.target - bone.world_position).normalize();
                let target_rot = Quat::from_rotation_arc(Vec3::Z, dir);
                bone.world_rotation = bone.world_rotation.slerp(target_rot, weight_per_bone);
            }
        }
    }
}
