//! Vehicle Physics System
//!
//! Realistic vehicle simulation with wheels, suspension, and drivetrain.

use glam::{Quat, Vec3};

/// Wheel info
#[derive(Debug, Clone)]
pub struct Wheel {
    /// Local attachment point
    pub attachment: Vec3,
    /// Wheel radius
    pub radius: f32,
    /// Suspension rest length
    pub suspension_rest: f32,
    /// Suspension stiffness
    pub suspension_stiffness: f32,
    /// Suspension damping
    pub suspension_damping: f32,
    /// Max suspension travel
    pub suspension_travel: f32,
    /// Is steering wheel
    pub steering: bool,
    /// Is drive wheel
    pub drive: bool,
    /// Has brake
    pub brake: bool,
    /// Has handbrake
    pub handbrake: bool,
    /// Friction coefficient
    pub friction: f32,
    /// Current steering angle (radians)
    pub steer_angle: f32,
    /// Current rotation (radians)
    pub rotation: f32,
    /// Current suspension compression (0-1)
    pub compression: f32,
    /// Is on ground
    pub grounded: bool,
    /// Ground hit normal
    pub ground_normal: Vec3,
    /// Slip ratio
    pub slip_ratio: f32,
    /// Slip angle
    pub slip_angle: f32,
}

impl Default for Wheel {
    fn default() -> Self {
        Self {
            attachment: Vec3::ZERO,
            radius: 0.4,
            suspension_rest: 0.3,
            suspension_stiffness: 35000.0,
            suspension_damping: 4500.0,
            suspension_travel: 0.2,
            steering: false,
            drive: false,
            brake: true,
            handbrake: false,
            friction: 1.0,
            steer_angle: 0.0,
            rotation: 0.0,
            compression: 0.0,
            grounded: false,
            ground_normal: Vec3::Y,
            slip_ratio: 0.0,
            slip_angle: 0.0,
        }
    }
}

/// Drivetrain type
#[derive(Debug, Clone, Copy, Default)]
pub enum DrivetrainType {
    /// Rear wheel drive
    #[default]
    RWD,
    /// Front wheel drive
    FWD,
    /// All wheel drive
    AWD,
    /// Four wheel drive (with transfer case)
    FourWD,
}

/// Engine configuration
#[derive(Debug, Clone)]
pub struct Engine {
    /// Max RPM
    pub max_rpm: f32,
    /// Idle RPM
    pub idle_rpm: f32,
    /// Current RPM
    pub rpm: f32,
    /// Torque curve (rpm -> torque Nm)
    pub torque_curve: Vec<(f32, f32)>,
    /// Inertia
    pub inertia: f32,
    /// Friction
    pub friction: f32,
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            max_rpm: 8000.0,
            idle_rpm: 800.0,
            rpm: 800.0,
            torque_curve: vec![
                (0.0, 100.0),
                (1000.0, 200.0),
                (3000.0, 350.0),
                (5000.0, 400.0),
                (7000.0, 350.0),
                (8000.0, 250.0),
            ],
            inertia: 0.15,
            friction: 0.02,
        }
    }
}

impl Engine {
    /// Get torque at current RPM
    #[must_use]
    pub fn torque(&self) -> f32 {
        let rpm = self.rpm;
        
        for i in 0..(self.torque_curve.len() - 1) {
            let (rpm_a, torque_a) = self.torque_curve[i];
            let (rpm_b, torque_b) = self.torque_curve[i + 1];
            
            if rpm >= rpm_a && rpm <= rpm_b {
                let t = (rpm - rpm_a) / (rpm_b - rpm_a);
                return torque_a + (torque_b - torque_a) * t;
            }
        }
        
        0.0
    }
}

/// Gearbox configuration
#[derive(Debug, Clone)]
pub struct Gearbox {
    /// Gear ratios (including reverse)
    pub ratios: Vec<f32>,
    /// Final drive ratio
    pub final_ratio: f32,
    /// Current gear (0 = neutral, -1 = reverse)
    pub current_gear: i32,
    /// Shift time
    pub shift_time: f32,
    /// Is automatic
    pub automatic: bool,
    /// Upshift RPM
    pub upshift_rpm: f32,
    /// Downshift RPM
    pub downshift_rpm: f32,
    /// Shifting timer
    shifting_timer: f32,
}

impl Default for Gearbox {
    fn default() -> Self {
        Self {
            ratios: vec![-3.5, 0.0, 3.5, 2.2, 1.5, 1.0, 0.8],
            final_ratio: 3.7,
            current_gear: 1,
            shift_time: 0.2,
            automatic: true,
            upshift_rpm: 6500.0,
            downshift_rpm: 2500.0,
            shifting_timer: 0.0,
        }
    }
}

impl Gearbox {
    /// Get current ratio
    #[must_use]
    pub fn ratio(&self) -> f32 {
        let idx = (self.current_gear + 1) as usize;
        if idx < self.ratios.len() {
            self.ratios[idx] * self.final_ratio
        } else {
            0.0
        }
    }

    /// Shift up
    pub fn shift_up(&mut self) {
        let max = self.ratios.len() as i32 - 2;
        if self.current_gear < max && self.shifting_timer <= 0.0 {
            self.current_gear += 1;
            self.shifting_timer = self.shift_time;
        }
    }

    /// Shift down
    pub fn shift_down(&mut self) {
        if self.current_gear > -1 && self.shifting_timer <= 0.0 {
            self.current_gear -= 1;
            self.shifting_timer = self.shift_time;
        }
    }

    /// Update
    pub fn update(&mut self, engine_rpm: f32, delta_time: f32) {
        self.shifting_timer = (self.shifting_timer - delta_time).max(0.0);

        if self.automatic && self.shifting_timer <= 0.0 {
            if engine_rpm >= self.upshift_rpm {
                self.shift_up();
            } else if engine_rpm <= self.downshift_rpm && self.current_gear > 1 {
                self.shift_down();
            }
        }
    }

    /// Is currently shifting
    #[must_use]
    pub fn is_shifting(&self) -> bool {
        self.shifting_timer > 0.0
    }
}

/// Vehicle input
#[derive(Debug, Clone, Copy, Default)]
pub struct VehicleInput {
    /// Throttle (0-1)
    pub throttle: f32,
    /// Brake (0-1)
    pub brake: f32,
    /// Steering (-1 to 1)
    pub steering: f32,
    /// Handbrake (0-1)
    pub handbrake: f32,
    /// Clutch (0-1)
    pub clutch: f32,
    /// Nitro
    pub nitro: bool,
}

/// Vehicle configuration
#[derive(Debug, Clone)]
pub struct VehicleConfig {
    /// Mass (kg)
    pub mass: f32,
    /// Center of mass offset
    pub center_of_mass: Vec3,
    /// Drag coefficient
    pub drag: f32,
    /// Downforce coefficient
    pub downforce: f32,
    /// Max steering angle (degrees)
    pub max_steer_angle: f32,
    /// Steering speed
    pub steer_speed: f32,
    /// ABS enabled
    pub abs: bool,
    /// Traction control enabled
    pub traction_control: bool,
    /// Stability control enabled
    pub stability_control: bool,
}

impl Default for VehicleConfig {
    fn default() -> Self {
        Self {
            mass: 1500.0,
            center_of_mass: Vec3::new(0.0, 0.3, 0.0),
            drag: 0.3,
            downforce: 0.05,
            max_steer_angle: 35.0,
            steer_speed: 5.0,
            abs: true,
            traction_control: true,
            stability_control: true,
        }
    }
}

/// Vehicle physics controller
pub struct Vehicle {
    /// Configuration
    pub config: VehicleConfig,
    /// Wheels
    pub wheels: Vec<Wheel>,
    /// Engine
    pub engine: Engine,
    /// Gearbox
    pub gearbox: Gearbox,
    /// Drivetrain type
    pub drivetrain: DrivetrainType,
    /// Position
    pub position: Vec3,
    /// Rotation
    pub rotation: Quat,
    /// Linear velocity
    pub velocity: Vec3,
    /// Angular velocity
    pub angular_velocity: Vec3,
    /// Current speed (km/h)
    pub speed_kmh: f32,
    /// Current input
    input: VehicleInput,
}

impl Vehicle {
    /// Create a new vehicle with default sedan setup
    #[must_use]
    pub fn sedan() -> Self {
        let mut vehicle = Self {
            config: VehicleConfig::default(),
            wheels: Vec::new(),
            engine: Engine::default(),
            gearbox: Gearbox::default(),
            drivetrain: DrivetrainType::RWD,
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            velocity: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
            speed_kmh: 0.0,
            input: VehicleInput::default(),
        };

        // Add wheels (FL, FR, RL, RR)
        vehicle.wheels.push(Wheel {
            attachment: Vec3::new(-0.8, 0.0, 1.4),
            steering: true,
            drive: false,
            ..Default::default()
        });
        vehicle.wheels.push(Wheel {
            attachment: Vec3::new(0.8, 0.0, 1.4),
            steering: true,
            drive: false,
            ..Default::default()
        });
        vehicle.wheels.push(Wheel {
            attachment: Vec3::new(-0.8, 0.0, -1.4),
            steering: false,
            drive: true,
            handbrake: true,
            ..Default::default()
        });
        vehicle.wheels.push(Wheel {
            attachment: Vec3::new(0.8, 0.0, -1.4),
            steering: false,
            drive: true,
            handbrake: true,
            ..Default::default()
        });

        vehicle
    }

    /// Set input
    pub fn set_input(&mut self, input: VehicleInput) {
        self.input = input;
    }

    /// Update physics
    pub fn update(&mut self, delta_time: f32, gravity: Vec3) {
        // Update steering
        let target_steer = self.input.steering * self.config.max_steer_angle.to_radians();
        for wheel in &mut self.wheels {
            if wheel.steering {
                let diff = target_steer - wheel.steer_angle;
                wheel.steer_angle += diff * self.config.steer_speed * delta_time;
            }
        }

        // Update gearbox
        self.gearbox.update(self.engine.rpm, delta_time);

        // Engine simulation
        let throttle = if self.gearbox.is_shifting() { 0.0 } else { self.input.throttle };
        let target_rpm = self.engine.idle_rpm + (self.engine.max_rpm - self.engine.idle_rpm) * throttle;
        self.engine.rpm = self.engine.rpm + (target_rpm - self.engine.rpm) * delta_time * 5.0;
        self.engine.rpm = self.engine.rpm.clamp(self.engine.idle_rpm, self.engine.max_rpm);

        // Calculate drive force
        let engine_torque = self.engine.torque() * throttle;
        let wheel_torque = engine_torque * self.gearbox.ratio();

        // Apply forces to drive wheels
        let drive_wheel_count = self.wheels.iter().filter(|w| w.drive && w.grounded).count();
        if drive_wheel_count > 0 {
            let force_per_wheel = wheel_torque / self.wheels[0].radius / drive_wheel_count as f32;
            let forward = self.rotation * Vec3::Z;
            
            for wheel in &mut self.wheels {
                if wheel.drive && wheel.grounded {
                    self.velocity += forward * (force_per_wheel / self.config.mass) * delta_time;
                    wheel.rotation += self.velocity.length() / wheel.radius * delta_time;
                }
            }
        }

        // Braking
        let brake_force = self.input.brake * 20000.0;
        if brake_force > 0.0 && self.velocity.length() > 0.1 {
            let brake_decel = (brake_force / self.config.mass) * delta_time;
            let speed = self.velocity.length();
            if brake_decel < speed {
                self.velocity -= self.velocity.normalize() * brake_decel;
            } else {
                self.velocity = Vec3::ZERO;
            }
        }

        // Drag
        let drag_force = self.velocity.length_squared() * self.config.drag;
        if self.velocity.length() > 0.1 {
            self.velocity -= self.velocity.normalize() * (drag_force / self.config.mass) * delta_time;
        }

        // Apply gravity
        self.velocity += gravity * delta_time;

        // Update position
        self.position += self.velocity * delta_time;

        // Calculate speed
        let forward = self.rotation * Vec3::Z;
        self.speed_kmh = self.velocity.dot(forward) * 3.6;
    }

    /// Get wheel world position
    #[must_use]
    pub fn wheel_world_position(&self, index: usize) -> Vec3 {
        if let Some(wheel) = self.wheels.get(index) {
            self.position + self.rotation * wheel.attachment
        } else {
            Vec3::ZERO
        }
    }

    /// Get speed in km/h
    #[must_use]
    pub fn speed(&self) -> f32 {
        self.speed_kmh
    }

    /// Get current gear
    #[must_use]
    pub fn gear(&self) -> i32 {
        self.gearbox.current_gear
    }

    /// Get RPM
    #[must_use]
    pub fn rpm(&self) -> f32 {
        self.engine.rpm
    }
}

/// Motorcycle (simplified)
pub struct Motorcycle {
    /// Base vehicle
    pub vehicle: Vehicle,
    /// Lean angle
    pub lean_angle: f32,
    /// Max lean angle
    pub max_lean: f32,
    /// Lean speed
    pub lean_speed: f32,
}

impl Motorcycle {
    /// Create a new motorcycle
    #[must_use]
    pub fn new() -> Self {
        let mut vehicle = Vehicle::sedan();
        vehicle.wheels.clear();
        
        // Front wheel
        vehicle.wheels.push(Wheel {
            attachment: Vec3::new(0.0, 0.0, 1.0),
            radius: 0.35,
            steering: true,
            drive: false,
            ..Default::default()
        });
        
        // Rear wheel
        vehicle.wheels.push(Wheel {
            attachment: Vec3::new(0.0, 0.0, -0.8),
            radius: 0.35,
            steering: false,
            drive: true,
            ..Default::default()
        });

        vehicle.config.mass = 250.0;
        
        Self {
            vehicle,
            lean_angle: 0.0,
            max_lean: 45.0,
            lean_speed: 3.0,
        }
    }

    /// Update with lean
    pub fn update(&mut self, delta_time: f32, gravity: Vec3) {
        // Calculate target lean from steering
        let speed_factor = (self.vehicle.speed().abs() / 100.0).clamp(0.0, 1.0);
        let target_lean = -self.vehicle.input.steering * self.max_lean * speed_factor;
        
        self.lean_angle += (target_lean - self.lean_angle) * self.lean_speed * delta_time;
        
        // Apply lean to rotation
        let lean_quat = Quat::from_rotation_z(self.lean_angle.to_radians());
        self.vehicle.rotation = self.vehicle.rotation * lean_quat;
        
        self.vehicle.update(delta_time, gravity);
    }
}

impl Default for Motorcycle {
    fn default() -> Self {
        Self::new()
    }
}
