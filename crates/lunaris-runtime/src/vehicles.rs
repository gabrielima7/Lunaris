//! Vehicle Physics
//!
//! Cars, boats, aircraft with realistic simulation.

use glam::{Vec3, Quat};

/// Vehicle
pub struct Vehicle {
    pub id: u64,
    pub vehicle_type: VehicleType,
    pub transform: VehicleTransform,
    pub physics: VehiclePhysics,
    pub input: VehicleInput,
    pub engine: Engine,
    pub wheels: Vec<Wheel>,
}

/// Vehicle type
pub enum VehicleType { Car, Motorcycle, Tank, Boat, Airplane, Helicopter }

/// Vehicle transform
pub struct VehicleTransform {
    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
    pub angular_velocity: Vec3,
}

/// Vehicle physics
pub struct VehiclePhysics {
    pub mass: f32,
    pub drag: f32,
    pub angular_drag: f32,
    pub center_of_mass: Vec3,
    pub inertia: Vec3,
}

/// Vehicle input
#[derive(Default)]
pub struct VehicleInput {
    pub throttle: f32,
    pub brake: f32,
    pub steering: f32,
    pub handbrake: bool,
    pub clutch: f32,
    pub gear_up: bool,
    pub gear_down: bool,
}

/// Engine
pub struct Engine {
    pub rpm: f32,
    pub max_rpm: f32,
    pub idle_rpm: f32,
    pub torque_curve: Vec<(f32, f32)>,
    pub current_gear: i32,
    pub gear_ratios: Vec<f32>,
    pub final_drive: f32,
}

impl Engine {
    pub fn get_torque(&self) -> f32 {
        let normalized = self.rpm / self.max_rpm;
        let base_torque = 500.0 * (1.0 - (normalized - 0.5).abs() * 2.0);
        base_torque * self.gear_ratios.get(self.current_gear as usize).unwrap_or(&1.0) * self.final_drive
    }

    pub fn shift_up(&mut self) {
        if (self.current_gear as usize) < self.gear_ratios.len() - 1 { self.current_gear += 1; self.rpm *= 0.7; }
    }

    pub fn shift_down(&mut self) {
        if self.current_gear > 0 { self.current_gear -= 1; self.rpm = (self.rpm * 1.3).min(self.max_rpm); }
    }
}

/// Wheel
pub struct Wheel {
    pub position: Vec3,
    pub radius: f32,
    pub suspension: Suspension,
    pub is_driven: bool,
    pub is_steered: bool,
    pub rotation: f32,
    pub slip: f32,
    pub grip: f32,
}

/// Suspension
pub struct Suspension {
    pub rest_length: f32,
    pub travel: f32,
    pub stiffness: f32,
    pub damping: f32,
    pub compression: f32,
}

impl Vehicle {
    pub fn car(id: u64) -> Self {
        Self {
            id,
            vehicle_type: VehicleType::Car,
            transform: VehicleTransform { position: Vec3::ZERO, rotation: Quat::IDENTITY, velocity: Vec3::ZERO, angular_velocity: Vec3::ZERO },
            physics: VehiclePhysics { mass: 1500.0, drag: 0.3, angular_drag: 0.5, center_of_mass: Vec3::new(0.0, 0.3, 0.0), inertia: Vec3::splat(1000.0) },
            input: VehicleInput::default(),
            engine: Engine { rpm: 1000.0, max_rpm: 7000.0, idle_rpm: 800.0, torque_curve: Vec::new(), current_gear: 1, gear_ratios: vec![-3.5, 0.0, 3.5, 2.5, 1.8, 1.4, 1.1, 0.9], final_drive: 3.7 },
            wheels: (0..4).map(|i| Wheel {
                position: Vec3::new(if i % 2 == 0 { -0.8 } else { 0.8 }, 0.0, if i < 2 { 1.3 } else { -1.3 }),
                radius: 0.35, suspension: Suspension { rest_length: 0.3, travel: 0.2, stiffness: 35000.0, damping: 4000.0, compression: 0.0 },
                is_driven: i >= 2, is_steered: i < 2, rotation: 0.0, slip: 0.0, grip: 1.0,
            }).collect(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Engine
        let target_rpm = self.engine.idle_rpm + (self.engine.max_rpm - self.engine.idle_rpm) * self.input.throttle;
        self.engine.rpm += (target_rpm - self.engine.rpm) * dt * 5.0;
        if self.input.gear_up { self.engine.shift_up(); }
        if self.input.gear_down { self.engine.shift_down(); }

        // Drive force
        let torque = self.engine.get_torque() * self.input.throttle;
        let force_magnitude = torque / self.wheels[0].radius;
        let forward = self.transform.rotation * Vec3::Z;
        let drive_force = forward * force_magnitude;

        // Steering
        for wheel in &mut self.wheels {
            if wheel.is_steered { wheel.rotation = self.input.steering * 0.5; }
        }

        // Apply forces
        self.transform.velocity += drive_force / self.physics.mass * dt;
        self.transform.velocity -= self.transform.velocity * self.physics.drag * dt;
        self.transform.position += self.transform.velocity * dt;

        // Turning
        let steer_force = self.input.steering * self.transform.velocity.length() * 0.5;
        self.transform.rotation = self.transform.rotation * Quat::from_rotation_y(steer_force * dt);

        // Braking
        if self.input.brake > 0.0 || self.input.handbrake {
            self.transform.velocity *= 1.0 - (self.input.brake * 5.0 + if self.input.handbrake { 3.0 } else { 0.0 }) * dt;
        }
    }

    pub fn speed_kmh(&self) -> f32 { self.transform.velocity.length() * 3.6 }
}

/// Boat physics
pub struct Boat {
    pub position: Vec3,
    pub velocity: Vec3,
    pub buoyancy: f32,
    pub water_drag: f32,
    pub throttle: f32,
}

impl Boat {
    pub fn update(&mut self, dt: f32, water_height: f32) {
        let submerged = (water_height - self.position.y).max(0.0);
        let buoyancy_force = Vec3::Y * self.buoyancy * submerged;
        self.velocity += buoyancy_force * dt;
        self.velocity += Vec3::new(0.0, 0.0, self.throttle * 10.0) * dt;
        self.velocity *= 1.0 - self.water_drag * dt;
        self.position += self.velocity * dt;
    }
}

/// Aircraft
pub struct Aircraft {
    pub position: Vec3,
    pub velocity: Vec3,
    pub rotation: Quat,
    pub throttle: f32,
    pub lift_coefficient: f32,
    pub drag_coefficient: f32,
}

impl Aircraft {
    pub fn update(&mut self, dt: f32) {
        let speed = self.velocity.length();
        let forward = self.rotation * Vec3::Z;
        let up = self.rotation * Vec3::Y;
        
        let lift = up * self.lift_coefficient * speed * speed;
        let drag = -self.velocity.normalize_or_zero() * self.drag_coefficient * speed * speed;
        let thrust = forward * self.throttle * 50000.0;
        let gravity = Vec3::new(0.0, -9.81 * 1000.0, 0.0);

        self.velocity += (thrust + lift + drag + gravity) / 5000.0 * dt;
        self.position += self.velocity * dt;
    }
}
