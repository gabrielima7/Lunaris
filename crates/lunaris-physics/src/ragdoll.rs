//! Ragdoll Physics
//!
//! Physics-based ragdoll simulation for characters.

use lunaris_core::math::Vec3;
use super::rigidbody::RigidbodyHandle;
use std::collections::HashMap;

/// Joint type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointType {
    /// Fixed joint (no movement)
    Fixed,
    /// Ball socket (3 DOF rotation)
    BallSocket,
    /// Hinge (1 DOF rotation)
    Hinge,
    /// Slider (1 DOF translation)
    Slider,
    /// Universal (2 DOF rotation)
    Universal,
    /// Ragdoll (ball socket with limits)
    Ragdoll,
}

/// Joint limits
#[derive(Debug, Clone, Copy)]
pub struct JointLimits {
    /// Lower angle limit (radians)
    pub lower_angle: Vec3,
    /// Upper angle limit (radians)
    pub upper_angle: Vec3,
    /// Lower translation limit
    pub lower_translation: f32,
    /// Upper translation limit
    pub upper_translation: f32,
    /// Motor enabled
    pub motor_enabled: bool,
    /// Motor target velocity
    pub motor_velocity: f32,
    /// Motor max force
    pub motor_max_force: f32,
}

impl Default for JointLimits {
    fn default() -> Self {
        Self {
            lower_angle: Vec3::new(-std::f32::consts::PI, -std::f32::consts::PI, -std::f32::consts::PI),
            upper_angle: Vec3::new(std::f32::consts::PI, std::f32::consts::PI, std::f32::consts::PI),
            lower_translation: -1.0,
            upper_translation: 1.0,
            motor_enabled: false,
            motor_velocity: 0.0,
            motor_max_force: 1000.0,
        }
    }
}

/// Joint configuration
#[derive(Debug, Clone)]
pub struct JointConfig {
    /// Joint type
    pub joint_type: JointType,
    /// Anchor point on body A (local)
    pub anchor_a: Vec3,
    /// Anchor point on body B (local)
    pub anchor_b: Vec3,
    /// Axis for hinge/slider (local to A)
    pub axis: Vec3,
    /// Joint limits
    pub limits: JointLimits,
    /// Break force (0 = unbreakable)
    pub break_force: f32,
    /// Break torque (0 = unbreakable)
    pub break_torque: f32,
}

impl Default for JointConfig {
    fn default() -> Self {
        Self {
            joint_type: JointType::BallSocket,
            anchor_a: Vec3::ZERO,
            anchor_b: Vec3::ZERO,
            axis: Vec3::new(0.0, 1.0, 0.0),
            limits: JointLimits::default(),
            break_force: 0.0,
            break_torque: 0.0,
        }
    }
}

/// A physics joint
#[derive(Debug, Clone)]
pub struct Joint {
    /// Joint configuration
    pub config: JointConfig,
    /// Body A handle
    pub body_a: RigidbodyHandle,
    /// Body B handle
    pub body_b: RigidbodyHandle,
    /// Is broken
    pub broken: bool,
}

/// Ragdoll bone definition
#[derive(Debug, Clone)]
pub struct RagdollBone {
    /// Bone name
    pub name: String,
    /// Rigidbody
    pub rigidbody: RigidbodyHandle,
    /// Collider shape dimensions
    pub dimensions: Vec3,
    /// Mass
    pub mass: f32,
}

/// Ragdoll joint definition
#[derive(Debug, Clone)]
pub struct RagdollJoint {
    /// Parent bone name
    pub parent: String,
    /// Child bone name
    pub child: String,
    /// Joint configuration
    pub config: JointConfig,
}

/// Ragdoll configuration for a humanoid
#[derive(Debug, Clone)]
pub struct RagdollConfig {
    /// Bones
    pub bones: Vec<RagdollBone>,
    /// Joints
    pub joints: Vec<RagdollJoint>,
    /// Total mass
    pub total_mass: f32,
    /// Collision group
    pub collision_group: u32,
}

impl Default for RagdollConfig {
    fn default() -> Self {
        Self::humanoid()
    }
}

impl RagdollConfig {
    /// Create a basic humanoid ragdoll configuration
    #[must_use]
    pub fn humanoid() -> Self {
        let bones = vec![
            RagdollBone {
                name: "Pelvis".to_string(),
                rigidbody: RigidbodyHandle(0),
                dimensions: Vec3::new(0.25, 0.1, 0.15),
                mass: 15.0,
            },
            RagdollBone {
                name: "Spine".to_string(),
                rigidbody: RigidbodyHandle(1),
                dimensions: Vec3::new(0.2, 0.2, 0.12),
                mass: 10.0,
            },
            RagdollBone {
                name: "Chest".to_string(),
                rigidbody: RigidbodyHandle(2),
                dimensions: Vec3::new(0.25, 0.2, 0.15),
                mass: 12.0,
            },
            RagdollBone {
                name: "Head".to_string(),
                rigidbody: RigidbodyHandle(3),
                dimensions: Vec3::new(0.12, 0.15, 0.12),
                mass: 5.0,
            },
            RagdollBone {
                name: "UpperArmL".to_string(),
                rigidbody: RigidbodyHandle(4),
                dimensions: Vec3::new(0.06, 0.25, 0.06),
                mass: 3.0,
            },
            RagdollBone {
                name: "UpperArmR".to_string(),
                rigidbody: RigidbodyHandle(5),
                dimensions: Vec3::new(0.06, 0.25, 0.06),
                mass: 3.0,
            },
            RagdollBone {
                name: "LowerArmL".to_string(),
                rigidbody: RigidbodyHandle(6),
                dimensions: Vec3::new(0.05, 0.22, 0.05),
                mass: 2.0,
            },
            RagdollBone {
                name: "LowerArmR".to_string(),
                rigidbody: RigidbodyHandle(7),
                dimensions: Vec3::new(0.05, 0.22, 0.05),
                mass: 2.0,
            },
            RagdollBone {
                name: "UpperLegL".to_string(),
                rigidbody: RigidbodyHandle(8),
                dimensions: Vec3::new(0.08, 0.35, 0.08),
                mass: 8.0,
            },
            RagdollBone {
                name: "UpperLegR".to_string(),
                rigidbody: RigidbodyHandle(9),
                dimensions: Vec3::new(0.08, 0.35, 0.08),
                mass: 8.0,
            },
            RagdollBone {
                name: "LowerLegL".to_string(),
                rigidbody: RigidbodyHandle(10),
                dimensions: Vec3::new(0.06, 0.35, 0.06),
                mass: 5.0,
            },
            RagdollBone {
                name: "LowerLegR".to_string(),
                rigidbody: RigidbodyHandle(11),
                dimensions: Vec3::new(0.06, 0.35, 0.06),
                mass: 5.0,
            },
        ];

        let joints = vec![
            // Spine chain
            RagdollJoint {
                parent: "Pelvis".to_string(),
                child: "Spine".to_string(),
                config: JointConfig {
                    joint_type: JointType::Ragdoll,
                    limits: JointLimits {
                        lower_angle: Vec3::new(-0.3, -0.5, -0.3),
                        upper_angle: Vec3::new(0.5, 0.5, 0.3),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            },
            RagdollJoint {
                parent: "Spine".to_string(),
                child: "Chest".to_string(),
                config: JointConfig {
                    joint_type: JointType::Ragdoll,
                    limits: JointLimits {
                        lower_angle: Vec3::new(-0.2, -0.3, -0.2),
                        upper_angle: Vec3::new(0.3, 0.3, 0.2),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            },
            RagdollJoint {
                parent: "Chest".to_string(),
                child: "Head".to_string(),
                config: JointConfig {
                    joint_type: JointType::Ragdoll,
                    limits: JointLimits {
                        lower_angle: Vec3::new(-0.5, -0.8, -0.3),
                        upper_angle: Vec3::new(0.5, 0.8, 0.3),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            },
            // Arms
            RagdollJoint {
                parent: "Chest".to_string(),
                child: "UpperArmL".to_string(),
                config: JointConfig {
                    joint_type: JointType::Ragdoll,
                    ..Default::default()
                },
            },
            RagdollJoint {
                parent: "Chest".to_string(),
                child: "UpperArmR".to_string(),
                config: JointConfig {
                    joint_type: JointType::Ragdoll,
                    ..Default::default()
                },
            },
            RagdollJoint {
                parent: "UpperArmL".to_string(),
                child: "LowerArmL".to_string(),
                config: JointConfig {
                    joint_type: JointType::Hinge,
                    limits: JointLimits {
                        lower_angle: Vec3::new(0.0, 0.0, 0.0),
                        upper_angle: Vec3::new(2.5, 0.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            },
            RagdollJoint {
                parent: "UpperArmR".to_string(),
                child: "LowerArmR".to_string(),
                config: JointConfig {
                    joint_type: JointType::Hinge,
                    limits: JointLimits {
                        lower_angle: Vec3::new(0.0, 0.0, 0.0),
                        upper_angle: Vec3::new(2.5, 0.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            },
            // Legs
            RagdollJoint {
                parent: "Pelvis".to_string(),
                child: "UpperLegL".to_string(),
                config: JointConfig {
                    joint_type: JointType::Ragdoll,
                    limits: JointLimits {
                        lower_angle: Vec3::new(-1.5, -0.3, -0.3),
                        upper_angle: Vec3::new(0.5, 0.3, 0.8),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            },
            RagdollJoint {
                parent: "Pelvis".to_string(),
                child: "UpperLegR".to_string(),
                config: JointConfig {
                    joint_type: JointType::Ragdoll,
                    limits: JointLimits {
                        lower_angle: Vec3::new(-1.5, -0.3, -0.8),
                        upper_angle: Vec3::new(0.5, 0.3, 0.3),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            },
            RagdollJoint {
                parent: "UpperLegL".to_string(),
                child: "LowerLegL".to_string(),
                config: JointConfig {
                    joint_type: JointType::Hinge,
                    limits: JointLimits {
                        lower_angle: Vec3::new(-2.5, 0.0, 0.0),
                        upper_angle: Vec3::new(0.0, 0.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            },
            RagdollJoint {
                parent: "UpperLegR".to_string(),
                child: "LowerLegR".to_string(),
                config: JointConfig {
                    joint_type: JointType::Hinge,
                    limits: JointLimits {
                        lower_angle: Vec3::new(-2.5, 0.0, 0.0),
                        upper_angle: Vec3::new(0.0, 0.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            },
        ];

        Self {
            bones,
            joints,
            total_mass: 80.0,
            collision_group: 2,
        }
    }
}

/// Ragdoll state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RagdollState {
    /// Ragdoll is inactive (animated)
    #[default]
    Inactive,
    /// Ragdoll is active (physics driven)
    Active,
    /// Blending from animation to ragdoll
    BlendingIn,
    /// Blending from ragdoll to animation
    BlendingOut,
}

/// Ragdoll controller
pub struct RagdollController {
    /// Configuration
    pub config: RagdollConfig,
    /// Current state
    pub state: RagdollState,
    /// Blend weight (0 = animation, 1 = ragdoll)
    pub blend_weight: f32,
    /// Blend time
    pub blend_time: f32,
    /// Current blend duration
    blend_duration: f32,
}

impl RagdollController {
    /// Create a new ragdoll controller
    #[must_use]
    pub fn new(config: RagdollConfig) -> Self {
        Self {
            config,
            state: RagdollState::Inactive,
            blend_weight: 0.0,
            blend_time: 0.0,
            blend_duration: 0.3,
        }
    }

    /// Activate ragdoll (instant)
    pub fn activate(&mut self) {
        self.state = RagdollState::Active;
        self.blend_weight = 1.0;
    }

    /// Deactivate ragdoll (instant)
    pub fn deactivate(&mut self) {
        self.state = RagdollState::Inactive;
        self.blend_weight = 0.0;
    }

    /// Blend to ragdoll
    pub fn blend_in(&mut self, duration: f32) {
        self.state = RagdollState::BlendingIn;
        self.blend_duration = duration;
        self.blend_time = 0.0;
    }

    /// Blend out of ragdoll
    pub fn blend_out(&mut self, duration: f32) {
        self.state = RagdollState::BlendingOut;
        self.blend_duration = duration;
        self.blend_time = 0.0;
    }

    /// Update blend
    pub fn update(&mut self, delta_time: f32) {
        match self.state {
            RagdollState::BlendingIn => {
                self.blend_time += delta_time;
                self.blend_weight = (self.blend_time / self.blend_duration).min(1.0);
                if self.blend_weight >= 1.0 {
                    self.state = RagdollState::Active;
                }
            }
            RagdollState::BlendingOut => {
                self.blend_time += delta_time;
                self.blend_weight = 1.0 - (self.blend_time / self.blend_duration).min(1.0);
                if self.blend_weight <= 0.0 {
                    self.state = RagdollState::Inactive;
                }
            }
            _ => {}
        }
    }

    /// Is ragdoll active or blending?
    #[must_use]
    pub fn is_physics_active(&self) -> bool {
        matches!(self.state, RagdollState::Active | RagdollState::BlendingIn | RagdollState::BlendingOut)
    }

    /// Apply an impulse to a bone
    pub fn apply_impulse(&mut self, _bone_name: &str, _impulse: Vec3) {
        // Would apply to rigidbody
    }
}
