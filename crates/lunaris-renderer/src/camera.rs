//! Camera system for 2D and 3D rendering

use lunaris_core::math::{Vec2, Vec3};

/// 2D Camera for orthographic projection
#[derive(Debug, Clone)]
pub struct Camera2D {
    /// Camera position in world space
    pub position: Vec2,
    /// Rotation in radians
    pub rotation: f32,
    /// Zoom level (1.0 = normal)
    pub zoom: f32,
    /// Viewport size
    pub viewport: Vec2,
}

impl Default for Camera2D {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.0,
            zoom: 1.0,
            viewport: Vec2::new(1280.0, 720.0),
        }
    }
}

impl Camera2D {
    /// Create a new 2D camera
    #[must_use]
    pub fn new(viewport_width: f32, viewport_height: f32) -> Self {
        Self {
            viewport: Vec2::new(viewport_width, viewport_height),
            ..Default::default()
        }
    }

    /// Get the orthographic projection matrix
    #[must_use]
    pub fn projection_matrix(&self) -> [[f32; 4]; 4] {
        let half_width = (self.viewport.x / 2.0) / self.zoom;
        let half_height = (self.viewport.y / 2.0) / self.zoom;

        let left = self.position.x - half_width;
        let right = self.position.x + half_width;
        let bottom = self.position.y - half_height;
        let top = self.position.y + half_height;

        // Orthographic projection matrix
        [
            [2.0 / (right - left), 0.0, 0.0, 0.0],
            [0.0, 2.0 / (top - bottom), 0.0, 0.0],
            [0.0, 0.0, -1.0, 0.0],
            [
                -(right + left) / (right - left),
                -(top + bottom) / (top - bottom),
                0.0,
                1.0,
            ],
        ]
    }

    /// Convert screen coordinates to world coordinates
    #[must_use]
    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        let normalized = Vec2::new(
            (screen_pos.x / self.viewport.x) * 2.0 - 1.0,
            1.0 - (screen_pos.y / self.viewport.y) * 2.0,
        );

        let half_width = (self.viewport.x / 2.0) / self.zoom;
        let half_height = (self.viewport.y / 2.0) / self.zoom;

        Vec2::new(
            self.position.x + normalized.x * half_width,
            self.position.y + normalized.y * half_height,
        )
    }

    /// Convert world coordinates to screen coordinates
    #[must_use]
    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        let half_width = (self.viewport.x / 2.0) / self.zoom;
        let half_height = (self.viewport.y / 2.0) / self.zoom;

        let normalized = Vec2::new(
            (world_pos.x - self.position.x) / half_width,
            (world_pos.y - self.position.y) / half_height,
        );

        Vec2::new(
            (normalized.x + 1.0) * 0.5 * self.viewport.x,
            (1.0 - normalized.y) * 0.5 * self.viewport.y,
        )
    }
}

/// 3D Camera with perspective projection
#[derive(Debug, Clone)]
pub struct Camera3D {
    /// Camera position in world space
    pub position: Vec3,
    /// Look-at target
    pub target: Vec3,
    /// Up vector
    pub up: Vec3,
    /// Field of view in radians
    pub fov: f32,
    /// Near clipping plane
    pub near: f32,
    /// Far clipping plane
    pub far: f32,
    /// Aspect ratio (width / height)
    pub aspect: f32,
}

impl Default for Camera3D {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 5.0, 10.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
            fov: std::f32::consts::FRAC_PI_4, // 45 degrees
            near: 0.1,
            far: 1000.0,
            aspect: 16.0 / 9.0,
        }
    }
}

impl Camera3D {
    /// Create a new 3D camera
    #[must_use]
    pub fn new(position: Vec3, target: Vec3) -> Self {
        Self {
            position,
            target,
            ..Default::default()
        }
    }

    /// Get the view matrix (camera transform)
    #[must_use]
    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        let f = (self.target - self.position).normalize();
        let s = f.cross(self.up).normalize();
        let u = s.cross(f);

        [
            [s.x, u.x, -f.x, 0.0],
            [s.y, u.y, -f.y, 0.0],
            [s.z, u.z, -f.z, 0.0],
            [
                -s.dot(self.position),
                -u.dot(self.position),
                f.dot(self.position),
                1.0,
            ],
        ]
    }

    /// Get the perspective projection matrix
    #[must_use]
    pub fn projection_matrix(&self) -> [[f32; 4]; 4] {
        let f = 1.0 / (self.fov / 2.0).tan();

        [
            [f / self.aspect, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [
                0.0,
                0.0,
                (self.far + self.near) / (self.near - self.far),
                -1.0,
            ],
            [
                0.0,
                0.0,
                (2.0 * self.far * self.near) / (self.near - self.far),
                0.0,
            ],
        ]
    }

    /// Get the forward direction
    #[must_use]
    pub fn forward(&self) -> Vec3 {
        (self.target - self.position).normalize()
    }

    /// Get the right direction
    #[must_use]
    pub fn right(&self) -> Vec3 {
        self.forward().cross(self.up).normalize()
    }

    /// Move the camera forward/backward
    pub fn move_forward(&mut self, amount: f32) {
        let forward = self.forward();
        self.position = self.position + forward * amount;
        self.target = self.target + forward * amount;
    }

    /// Orbit around target
    pub fn orbit(&mut self, yaw: f32, pitch: f32) {
        let direction = self.position - self.target;
        let distance = direction.length();

        // Calculate current angles
        let current_yaw = direction.z.atan2(direction.x);
        let current_pitch = (direction.y / distance).asin();

        // Apply rotation
        let new_yaw = current_yaw + yaw;
        let new_pitch = (current_pitch + pitch).clamp(-1.5, 1.5);

        // Calculate new position
        self.position = Vec3::new(
            self.target.x + distance * new_pitch.cos() * new_yaw.cos(),
            self.target.y + distance * new_pitch.sin(),
            self.target.z + distance * new_pitch.cos() * new_yaw.sin(),
        );
    }
}

/// Camera uniform buffer data
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    /// View-Projection matrix
    pub view_proj: [[f32; 4]; 4],
    /// Camera position
    pub position: [f32; 4],
}

impl CameraUniform {
    /// Create from a 2D camera
    #[must_use]
    pub fn from_camera_2d(camera: &Camera2D) -> Self {
        Self {
            view_proj: camera.projection_matrix(),
            position: [camera.position.x, camera.position.y, 0.0, 1.0],
        }
    }

    /// Create from a 3D camera
    #[must_use]
    pub fn from_camera_3d(camera: &Camera3D) -> Self {
        let view = camera.view_matrix();
        let proj = camera.projection_matrix();

        // Multiply view * projection
        let mut view_proj = [[0.0f32; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    view_proj[i][j] += proj[i][k] * view[k][j];
                }
            }
        }

        Self {
            view_proj,
            position: [camera.position.x, camera.position.y, camera.position.z, 1.0],
        }
    }
}
