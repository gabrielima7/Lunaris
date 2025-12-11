//! Physics Constraints
//!
//! Joint and constraint systems for physics simulation.

use glam::{Quat, Vec3};

/// Constraint type
#[derive(Debug, Clone, Copy)]
pub enum ConstraintType {
    /// Fixed constraint (no relative movement)
    Fixed,
    /// Ball socket (free rotation)
    BallSocket,
    /// Hinge (rotation around one axis)
    Hinge { axis: Vec3 },
    /// Slider (translation along one axis)
    Slider { axis: Vec3 },
    /// Distance (maintain distance)
    Distance { min: f32, max: f32 },
    /// Cone twist (limited rotation)
    ConeTwist { swing_span: f32, twist_span: f32 },
    /// Universal (two axes of rotation)
    Universal { axis1: Vec3, axis2: Vec3 },
}

/// Physics constraint between two bodies
#[derive(Debug, Clone)]
pub struct Constraint {
    /// Constraint ID
    pub id: u64,
    /// First body ID
    pub body_a: u64,
    /// Second body ID (None for world attachment)
    pub body_b: Option<u64>,
    /// Constraint type
    pub constraint_type: ConstraintType,
    /// Anchor point on body A (local space)
    pub anchor_a: Vec3,
    /// Anchor point on body B (local space)
    pub anchor_b: Vec3,
    /// Reference rotation A
    pub rotation_a: Quat,
    /// Reference rotation B
    pub rotation_b: Quat,
    /// Is enabled
    pub enabled: bool,
    /// Break force (None = unbreakable)
    pub break_force: Option<f32>,
    /// Break torque (None = unbreakable)
    pub break_torque: Option<f32>,
    /// Is broken
    pub is_broken: bool,
}

impl Constraint {
    /// Create a new constraint
    #[must_use]
    pub fn new(body_a: u64, body_b: Option<u64>, constraint_type: ConstraintType) -> Self {
        Self {
            id: 0,
            body_a,
            body_b,
            constraint_type,
            anchor_a: Vec3::ZERO,
            anchor_b: Vec3::ZERO,
            rotation_a: Quat::IDENTITY,
            rotation_b: Quat::IDENTITY,
            enabled: true,
            break_force: None,
            break_torque: None,
            is_broken: false,
        }
    }

    /// Set anchor points
    #[must_use]
    pub fn with_anchors(mut self, anchor_a: Vec3, anchor_b: Vec3) -> Self {
        self.anchor_a = anchor_a;
        self.anchor_b = anchor_b;
        self
    }

    /// Set break thresholds
    #[must_use]
    pub fn with_break_thresholds(mut self, force: f32, torque: f32) -> Self {
        self.break_force = Some(force);
        self.break_torque = Some(torque);
        self
    }
}

/// Motor for powered constraints
#[derive(Debug, Clone, Copy)]
pub struct Motor {
    /// Target velocity
    pub target_velocity: f32,
    /// Max force
    pub max_force: f32,
    /// Is enabled
    pub enabled: bool,
}

impl Default for Motor {
    fn default() -> Self {
        Self {
            target_velocity: 0.0,
            max_force: 1000.0,
            enabled: false,
        }
    }
}

/// Spring constraint
#[derive(Debug, Clone, Copy)]
pub struct Spring {
    /// Spring stiffness
    pub stiffness: f32,
    /// Damping ratio
    pub damping: f32,
    /// Rest length
    pub rest_length: f32,
}

impl Spring {
    /// Calculate spring force
    #[must_use]
    pub fn calculate_force(&self, current_length: f32, velocity: f32) -> f32 {
        let displacement = current_length - self.rest_length;
        -self.stiffness * displacement - self.damping * velocity
    }
}

/// Rope/Chain constraint
#[derive(Debug, Clone)]
pub struct RopeConstraint {
    /// Constraint ID
    pub id: u64,
    /// Particle positions
    pub particles: Vec<Vec3>,
    /// Segment length
    pub segment_length: f32,
    /// Stiffness
    pub stiffness: f32,
    /// Iterations for solver
    pub iterations: u32,
    /// Is first end fixed
    pub fixed_start: bool,
    /// Is last end fixed
    pub fixed_end: bool,
}

impl RopeConstraint {
    /// Create a new rope between two points
    #[must_use]
    pub fn new(start: Vec3, end: Vec3, segments: u32) -> Self {
        let total_length = (end - start).length();
        let segment_length = total_length / segments as f32;
        let direction = (end - start).normalize();

        let particles: Vec<Vec3> = (0..=segments)
            .map(|i| start + direction * (i as f32 * segment_length))
            .collect();

        Self {
            id: 0,
            particles,
            segment_length,
            stiffness: 1.0,
            iterations: 10,
            fixed_start: true,
            fixed_end: false,
        }
    }

    /// Simulate one step
    pub fn simulate(&mut self, gravity: Vec3, delta_time: f32) {
        // Apply gravity to particles (except fixed ones)
        let start_idx = if self.fixed_start { 1 } else { 0 };
        let end_idx = if self.fixed_end { self.particles.len() - 1 } else { self.particles.len() };

        for i in start_idx..end_idx {
            self.particles[i] += gravity * delta_time * delta_time;
        }

        // Constraint solving
        for _ in 0..self.iterations {
            for i in 0..(self.particles.len() - 1) {
                let p1 = self.particles[i];
                let p2 = self.particles[i + 1];
                let delta = p2 - p1;
                let distance = delta.length();
                
                if distance > 0.0 {
                    let correction = delta * ((distance - self.segment_length) / distance) * 0.5 * self.stiffness;
                    
                    let is_first_fixed = i == 0 && self.fixed_start;
                    let is_second_fixed = i + 1 == self.particles.len() - 1 && self.fixed_end;
                    
                    if !is_first_fixed && !is_second_fixed {
                        self.particles[i] += correction;
                        self.particles[i + 1] -= correction;
                    } else if !is_first_fixed {
                        self.particles[i] += correction * 2.0;
                    } else if !is_second_fixed {
                        self.particles[i + 1] -= correction * 2.0;
                    }
                }
            }
        }
    }
}

/// Constraint manager
pub struct ConstraintManager {
    /// All constraints
    constraints: Vec<Constraint>,
    /// Rope constraints
    ropes: Vec<RopeConstraint>,
    /// Next ID
    next_id: u64,
}

impl Default for ConstraintManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ConstraintManager {
    /// Create a new constraint manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            ropes: Vec::new(),
            next_id: 1,
        }
    }

    /// Add a constraint
    pub fn add_constraint(&mut self, mut constraint: Constraint) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        constraint.id = id;
        self.constraints.push(constraint);
        id
    }

    /// Add a rope constraint
    pub fn add_rope(&mut self, mut rope: RopeConstraint) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        rope.id = id;
        self.ropes.push(rope);
        id
    }

    /// Remove a constraint
    pub fn remove_constraint(&mut self, id: u64) {
        self.constraints.retain(|c| c.id != id);
    }

    /// Get constraint by ID
    #[must_use]
    pub fn get_constraint(&self, id: u64) -> Option<&Constraint> {
        self.constraints.iter().find(|c| c.id == id)
    }

    /// Get mutable constraint by ID
    pub fn get_constraint_mut(&mut self, id: u64) -> Option<&mut Constraint> {
        self.constraints.iter_mut().find(|c| c.id == id)
    }

    /// Update all constraints
    pub fn update(&mut self, delta_time: f32) {
        // Update rope simulations
        let gravity = Vec3::new(0.0, -9.81, 0.0);
        for rope in &mut self.ropes {
            rope.simulate(gravity, delta_time);
        }
    }

    /// Get broken constraints
    #[must_use]
    pub fn broken_constraints(&self) -> Vec<u64> {
        self.constraints.iter()
            .filter(|c| c.is_broken)
            .map(|c| c.id)
            .collect()
    }

    /// Clear all broken constraints
    pub fn clear_broken(&mut self) {
        self.constraints.retain(|c| !c.is_broken);
    }
}
