//! Inverse Dynamics
//!
//! Physics-based animation, force-driven movement, muscle simulation.

use glam::{Vec3, Quat, Mat3};

/// Inverse dynamics system  
pub struct InverseDynamics {
    pub bodies: Vec<RigidBodyDyn>,
    pub joints: Vec<JointDyn>,
    pub muscles: Vec<Muscle>,
    pub gravity: Vec3,
    pub settings: DynamicsSettings,
}

/// Rigid body for dynamics
pub struct RigidBodyDyn {
    pub id: u64,
    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
    pub angular_velocity: Vec3,
    pub mass: f32,
    pub inertia: Mat3,
    pub force: Vec3,
    pub torque: Vec3,
    pub parent: Option<usize>,
}

/// Joint for dynamics
pub struct JointDyn {
    pub body_a: usize,
    pub body_b: usize,
    pub anchor_a: Vec3,
    pub anchor_b: Vec3,
    pub joint_type: JointType,
    pub target_rotation: Option<Quat>,
    pub stiffness: f32,
    pub damping: f32,
    pub max_force: f32,
}

/// Joint type
pub enum JointType { Fixed, Hinge { axis: Vec3, limits: (f32, f32) }, Ball { swing_limit: f32, twist_limit: f32 }, Prismatic { axis: Vec3, limits: (f32, f32) } }

/// Muscle
pub struct Muscle {
    pub name: String,
    pub origin_body: usize,
    pub origin_point: Vec3,
    pub insertion_body: usize,
    pub insertion_point: Vec3,
    pub max_force: f32,
    pub activation: f32,
    pub rest_length: f32,
    pub max_velocity: f32,
}

/// Dynamics settings
pub struct DynamicsSettings {
    pub iterations: u32,
    pub substeps: u32,
    pub joint_erp: f32,
    pub joint_cfm: f32,
}

impl Default for DynamicsSettings {
    fn default() -> Self {
        Self { iterations: 10, substeps: 4, joint_erp: 0.8, joint_cfm: 0.0001 }
    }
}

impl InverseDynamics {
    pub fn new() -> Self {
        Self { bodies: Vec::new(), joints: Vec::new(), muscles: Vec::new(), gravity: Vec3::new(0.0, -9.81, 0.0), settings: DynamicsSettings::default() }
    }

    pub fn add_body(&mut self, mass: f32, position: Vec3) -> usize {
        let id = self.bodies.len();
        self.bodies.push(RigidBodyDyn {
            id: id as u64, position, rotation: Quat::IDENTITY, velocity: Vec3::ZERO, angular_velocity: Vec3::ZERO,
            mass, inertia: Mat3::from_diagonal(Vec3::splat(mass * 0.4)), force: Vec3::ZERO, torque: Vec3::ZERO, parent: None,
        });
        id
    }

    pub fn add_joint(&mut self, body_a: usize, body_b: usize, anchor_a: Vec3, anchor_b: Vec3, joint_type: JointType) {
        self.joints.push(JointDyn { body_a, body_b, anchor_a, anchor_b, joint_type, target_rotation: None, stiffness: 1000.0, damping: 50.0, max_force: 10000.0 });
    }

    pub fn add_muscle(&mut self, name: &str, origin: (usize, Vec3), insertion: (usize, Vec3), max_force: f32) {
        let rest_length = ((self.bodies[origin.0].position + origin.1) - (self.bodies[insertion.0].position + insertion.1)).length();
        self.muscles.push(Muscle { name: name.into(), origin_body: origin.0, origin_point: origin.1, insertion_body: insertion.0, insertion_point: insertion.1, max_force, activation: 0.0, rest_length, max_velocity: 5.0 });
    }

    pub fn set_muscle_activation(&mut self, name: &str, activation: f32) {
        if let Some(m) = self.muscles.iter_mut().find(|m| m.name == name) {
            m.activation = activation.clamp(0.0, 1.0);
        }
    }

    pub fn step(&mut self, dt: f32) {
        let sub_dt = dt / self.settings.substeps as f32;
        
        for _ in 0..self.settings.substeps {
            // Clear forces
            for body in &mut self.bodies { body.force = Vec3::ZERO; body.torque = Vec3::ZERO; }
            
            // Apply gravity
            for body in &mut self.bodies {
                body.force += self.gravity * body.mass;
            }
            
            // Apply muscle forces
            self.apply_muscle_forces();
            
            // Apply joint constraints
            for _ in 0..self.settings.iterations {
                self.solve_joints(sub_dt);
            }
            
            // Integrate
            for body in &mut self.bodies {
                let inv_mass = 1.0 / body.mass;
                body.velocity += body.force * inv_mass * sub_dt;
                body.position += body.velocity * sub_dt;
                
                let inv_inertia = body.inertia.inverse();
                body.angular_velocity += inv_inertia * body.torque * sub_dt;
                let dq = Quat::from_scaled_axis(body.angular_velocity * sub_dt * 0.5);
                body.rotation = (dq * body.rotation).normalize();
            }
        }
    }

    fn apply_muscle_forces(&mut self) {
        for muscle in &self.muscles {
            let origin_world = self.bodies[muscle.origin_body].position + self.bodies[muscle.origin_body].rotation * muscle.origin_point;
            let insertion_world = self.bodies[muscle.insertion_body].position + self.bodies[muscle.insertion_body].rotation * muscle.insertion_point;
            
            let diff = insertion_world - origin_world;
            let length = diff.length();
            let direction = diff / length.max(0.001);
            
            // Hill muscle model (simplified)
            let length_factor = gaussian(length / muscle.rest_length, 0.5);
            let force_magnitude = muscle.max_force * muscle.activation * length_factor;
            let force = direction * force_magnitude;
            
            self.bodies[muscle.origin_body].force += force;
            self.bodies[muscle.insertion_body].force -= force;
            
            // Torque
            self.bodies[muscle.origin_body].torque += muscle.origin_point.cross(force);
            self.bodies[muscle.insertion_body].torque -= muscle.insertion_point.cross(force);
        }
    }

    fn solve_joints(&mut self, dt: f32) {
        for joint in &self.joints {
            let anchor_a_world = self.bodies[joint.body_a].position + self.bodies[joint.body_a].rotation * joint.anchor_a;
            let anchor_b_world = self.bodies[joint.body_b].position + self.bodies[joint.body_b].rotation * joint.anchor_b;
            
            let error = anchor_b_world - anchor_a_world;
            let correction = error * joint.stiffness * dt;
            let damping_force = (self.bodies[joint.body_b].velocity - self.bodies[joint.body_a].velocity) * joint.damping;
            
            let total = (correction - damping_force * dt).clamp_length_max(joint.max_force * dt);
            
            self.bodies[joint.body_a].velocity += total / self.bodies[joint.body_a].mass;
            self.bodies[joint.body_b].velocity -= total / self.bodies[joint.body_b].mass;
            
            // Angular constraints for target rotation
            if let Some(target) = joint.target_rotation {
                let current = self.bodies[joint.body_b].rotation * self.bodies[joint.body_a].rotation.inverse();
                let error_rot = target * current.inverse();
                let (axis, angle) = error_rot.to_axis_angle();
                if angle.abs() > 0.001 {
                    let angular_correction = axis * angle * joint.stiffness * 0.1 * dt;
                    self.bodies[joint.body_b].angular_velocity += angular_correction;
                }
            }
        }
    }

    pub fn compute_torques_for_pose(&self, target_pose: &[(usize, Quat)]) -> Vec<(usize, Vec3)> {
        target_pose.iter().map(|(joint_idx, target_rot)| {
            let joint = &self.joints[*joint_idx];
            let current = self.bodies[joint.body_b].rotation;
            let error = *target_rot * current.inverse();
            let (axis, angle) = error.to_axis_angle();
            (*joint_idx, axis * angle * joint.stiffness)
        }).collect()
    }
}

fn gaussian(x: f32, sigma: f32) -> f32 {
    let a = (x - 1.0) / sigma;
    (-a * a * 0.5).exp()
}
