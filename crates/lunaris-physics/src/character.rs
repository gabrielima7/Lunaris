//! Character Controller
//!
//! Physics-based character movement for games.

use lunaris_core::math::{Vec2, Vec3};

/// 2D Character controller configuration
#[derive(Debug, Clone)]
pub struct CharacterController2DConfig {
    /// Movement speed
    pub move_speed: f32,
    /// Jump force
    pub jump_force: f32,
    /// Gravity
    pub gravity: f32,
    /// Max fall speed
    pub max_fall_speed: f32,
    /// Ground friction
    pub ground_friction: f32,
    /// Air friction
    pub air_friction: f32,
    /// Acceleration
    pub acceleration: f32,
    /// Ground check distance
    pub ground_check_dist: f32,
    /// Coyote time (grace period after leaving ground)
    pub coyote_time: f32,
    /// Jump buffer (input buffer for jump)
    pub jump_buffer: f32,
}

impl Default for CharacterController2DConfig {
    fn default() -> Self {
        Self {
            move_speed: 200.0,
            jump_force: 400.0,
            gravity: 980.0,
            max_fall_speed: 600.0,
            ground_friction: 0.9,
            air_friction: 0.98,
            acceleration: 2000.0,
            ground_check_dist: 5.0,
            coyote_time: 0.1,
            jump_buffer: 0.1,
        }
    }
}

/// 2D Character controller state
#[derive(Debug, Clone)]
pub struct CharacterController2D {
    /// Configuration
    pub config: CharacterController2DConfig,
    /// Velocity
    pub velocity: Vec2,
    /// Is grounded
    pub is_grounded: bool,
    /// Was grounded last frame
    pub was_grounded: bool,
    /// Time since leaving ground
    pub time_since_grounded: f32,
    /// Time since jump pressed
    pub time_since_jump_press: f32,
    /// Can jump
    pub can_jump: bool,
    /// Is jumping
    pub is_jumping: bool,
    /// Facing right
    pub facing_right: bool,
}

impl Default for CharacterController2D {
    fn default() -> Self {
        Self::new(CharacterController2DConfig::default())
    }
}

impl CharacterController2D {
    /// Create a new controller
    #[must_use]
    pub fn new(config: CharacterController2DConfig) -> Self {
        Self {
            config,
            velocity: Vec2::ZERO,
            is_grounded: true,
            was_grounded: true,
            time_since_grounded: 0.0,
            time_since_jump_press: 1.0,
            can_jump: true,
            is_jumping: false,
            facing_right: true,
        }
    }

    /// Update controller
    pub fn update(&mut self, dt: f32, input_x: f32, input_jump: bool) -> Vec2 {
        self.was_grounded = self.is_grounded;

        // Update timers
        if !self.is_grounded {
            self.time_since_grounded += dt;
        } else {
            self.time_since_grounded = 0.0;
            self.can_jump = true;
            self.is_jumping = false;
        }

        // Jump buffer
        if input_jump {
            self.time_since_jump_press = 0.0;
        } else {
            self.time_since_jump_press += dt;
        }

        // Horizontal movement
        let target_speed = input_x * self.config.move_speed;
        let speed_diff = target_speed - self.velocity.x;
        let accel = if self.is_grounded {
            self.config.acceleration
        } else {
            self.config.acceleration * 0.5
        };
        self.velocity.x += speed_diff.clamp(-accel * dt, accel * dt);

        // Update facing
        if input_x > 0.01 {
            self.facing_right = true;
        } else if input_x < -0.01 {
            self.facing_right = false;
        }

        // Apply friction
        let friction = if self.is_grounded {
            self.config.ground_friction
        } else {
            self.config.air_friction
        };
        if input_x.abs() < 0.01 {
            self.velocity.x *= friction;
        }

        // Jump (with coyote time and jump buffer)
        let can_coyote = self.time_since_grounded < self.config.coyote_time;
        let has_jump_buffer = self.time_since_jump_press < self.config.jump_buffer;
        
        if has_jump_buffer && (self.is_grounded || can_coyote) && self.can_jump {
            self.velocity.y = -self.config.jump_force;
            self.is_jumping = true;
            self.can_jump = false;
            self.time_since_jump_press = 1.0; // Consume buffer
        }

        // Variable jump height (release to fall faster)
        if self.is_jumping && !input_jump && self.velocity.y < 0.0 {
            self.velocity.y *= 0.5;
            self.is_jumping = false;
        }

        // Gravity
        if !self.is_grounded {
            self.velocity.y += self.config.gravity * dt;
            self.velocity.y = self.velocity.y.min(self.config.max_fall_speed);
        }

        self.velocity
    }

    /// Set grounded state (call after physics)
    pub fn set_grounded(&mut self, grounded: bool) {
        self.is_grounded = grounded;
        if grounded && self.velocity.y > 0.0 {
            self.velocity.y = 0.0;
        }
    }

    /// Apply delta movement from physics
    pub fn apply_movement(&mut self, delta: Vec2) {
        // Could be used for slope handling etc
        if delta.y.abs() < 0.001 && self.velocity.y > 0.0 {
            self.is_grounded = true;
        }
    }
}

/// 3D Character controller configuration
#[derive(Debug, Clone)]
pub struct CharacterController3DConfig {
    /// Walk speed
    pub walk_speed: f32,
    /// Run speed
    pub run_speed: f32,
    /// Jump force
    pub jump_force: f32,
    /// Gravity
    pub gravity: f32,
    /// Max fall speed
    pub max_fall_speed: f32,
    /// Ground friction
    pub ground_friction: f32,
    /// Air control
    pub air_control: f32,
    /// Capsule height
    pub height: f32,
    /// Capsule radius
    pub radius: f32,
    /// Step height
    pub step_height: f32,
    /// Slope limit (degrees)
    pub slope_limit: f32,
}

impl Default for CharacterController3DConfig {
    fn default() -> Self {
        Self {
            walk_speed: 5.0,
            run_speed: 10.0,
            jump_force: 8.0,
            gravity: 20.0,
            max_fall_speed: 50.0,
            ground_friction: 0.9,
            air_control: 0.3,
            height: 1.8,
            radius: 0.3,
            step_height: 0.3,
            slope_limit: 45.0,
        }
    }
}

/// 3D Character controller state
#[derive(Debug, Clone)]
pub struct CharacterController3D {
    /// Configuration
    pub config: CharacterController3DConfig,
    /// Velocity
    pub velocity: Vec3,
    /// Is grounded
    pub is_grounded: bool,
    /// Is running
    pub is_running: bool,
    /// Ground normal
    pub ground_normal: Vec3,
}

impl Default for CharacterController3D {
    fn default() -> Self {
        Self::new(CharacterController3DConfig::default())
    }
}

impl CharacterController3D {
    /// Create a new controller
    #[must_use]
    pub fn new(config: CharacterController3DConfig) -> Self {
        Self {
            config,
            velocity: Vec3::ZERO,
            is_grounded: true,
            is_running: false,
            ground_normal: Vec3::new(0.0, 1.0, 0.0),
        }
    }

    /// Update controller
    pub fn update(&mut self, dt: f32, input: Vec2, jump: bool, run: bool) -> Vec3 {
        self.is_running = run;
        
        let speed = if run { self.config.run_speed } else { self.config.walk_speed };
        
        // Horizontal movement
        let control = if self.is_grounded { 1.0 } else { self.config.air_control };
        let target_vel = Vec3::new(input.x * speed, 0.0, input.y * speed);
        
        self.velocity.x += (target_vel.x - self.velocity.x) * control * 10.0 * dt;
        self.velocity.z += (target_vel.z - self.velocity.z) * control * 10.0 * dt;

        // Apply friction
        if self.is_grounded && input.length_squared() < 0.01 {
            self.velocity.x *= self.config.ground_friction;
            self.velocity.z *= self.config.ground_friction;
        }

        // Jump
        if jump && self.is_grounded {
            self.velocity.y = self.config.jump_force;
            self.is_grounded = false;
        }

        // Gravity
        if !self.is_grounded {
            self.velocity.y -= self.config.gravity * dt;
            self.velocity.y = self.velocity.y.max(-self.config.max_fall_speed);
        }

        self.velocity
    }

    /// Set grounded state
    pub fn set_grounded(&mut self, grounded: bool, normal: Vec3) {
        self.is_grounded = grounded;
        self.ground_normal = normal;
        if grounded && self.velocity.y < 0.0 {
            self.velocity.y = 0.0;
        }
    }
}
