//! Level of Detail (LOD) System
//!
//! Automatic mesh simplification based on distance.

use lunaris_core::math::Vec3;
use std::collections::HashMap;

/// LOD level configuration
#[derive(Debug, Clone)]
pub struct LodLevel {
    /// Distance threshold
    pub distance: f32,
    /// Screen size threshold (0-1)
    pub screen_size: f32,
    /// Mesh index for this level (in mesh array)
    pub mesh_index: usize,
}

/// LOD group configuration
#[derive(Debug, Clone)]
pub struct LodGroup {
    /// LOD levels (sorted by distance)
    pub levels: Vec<LodLevel>,
    /// Fade transition distance
    pub fade_distance: f32,
    /// Use screen size instead of distance
    pub use_screen_size: bool,
    /// Current LOD level
    current_level: usize,
    /// Fade factor (0-1)
    fade_factor: f32,
}

impl Default for LodGroup {
    fn default() -> Self {
        Self {
            levels: vec![
                LodLevel { distance: 10.0, screen_size: 0.6, mesh_index: 0 },
                LodLevel { distance: 25.0, screen_size: 0.3, mesh_index: 1 },
                LodLevel { distance: 50.0, screen_size: 0.15, mesh_index: 2 },
                LodLevel { distance: 100.0, screen_size: 0.05, mesh_index: 3 },
            ],
            fade_distance: 5.0,
            use_screen_size: false,
            current_level: 0,
            fade_factor: 1.0,
        }
    }
}

impl LodGroup {
    /// Create a new LOD group
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with custom levels
    #[must_use]
    pub fn with_levels(levels: Vec<LodLevel>) -> Self {
        Self {
            levels,
            ..Default::default()
        }
    }

    /// Update LOD based on camera
    pub fn update(&mut self, object_pos: Vec3, camera_pos: Vec3, _screen_height: f32) {
        let distance = (object_pos - camera_pos).length();
        
        let target_level = self.levels
            .iter()
            .enumerate()
            .rev()
            .find(|(_, level)| distance < level.distance)
            .map(|(i, _)| i)
            .unwrap_or(self.levels.len().saturating_sub(1));

        // Smooth transition
        if target_level != self.current_level {
            let current_dist = if self.current_level < self.levels.len() {
                self.levels[self.current_level].distance
            } else {
                f32::MAX
            };
            
            let target_dist = if target_level < self.levels.len() {
                self.levels[target_level].distance
            } else {
                f32::MAX
            };

            let transition_start = current_dist.min(target_dist);
            let transition_end = current_dist.max(target_dist);
            
            if distance > transition_start && distance < transition_end {
                self.fade_factor = (distance - transition_start) / (transition_end - transition_start);
            } else {
                self.current_level = target_level;
                self.fade_factor = 1.0;
            }
        }
    }

    /// Get current mesh index
    #[must_use]
    pub fn current_mesh_index(&self) -> usize {
        if self.current_level < self.levels.len() {
            self.levels[self.current_level].mesh_index
        } else {
            0
        }
    }

    /// Get current LOD level
    #[must_use]
    pub fn current_level(&self) -> usize {
        self.current_level
    }

    /// Get fade factor
    #[must_use]
    pub fn fade_factor(&self) -> f32 {
        self.fade_factor
    }

    /// Should cull (too far)
    #[must_use]
    pub fn should_cull(&self, distance: f32) -> bool {
        if let Some(last) = self.levels.last() {
            distance > last.distance * 1.5
        } else {
            false
        }
    }
}

/// Frustum for culling
#[derive(Debug, Clone)]
pub struct Frustum {
    /// Near plane distance
    pub near: f32,
    /// Far plane distance
    pub far: f32,
    /// Field of view (radians)
    pub fov: f32,
    /// Aspect ratio
    pub aspect: f32,
    /// Camera position
    pub position: Vec3,
    /// Camera forward
    pub forward: Vec3,
    /// Camera right
    pub right: Vec3,
    /// Camera up
    pub up: Vec3,
}

impl Default for Frustum {
    fn default() -> Self {
        Self {
            near: 0.1,
            far: 1000.0,
            fov: 60.0_f32.to_radians(),
            aspect: 16.0 / 9.0,
            position: Vec3::ZERO,
            forward: Vec3::new(0.0, 0.0, -1.0),
            right: Vec3::new(1.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
        }
    }
}

impl Frustum {
    /// Create a new frustum from camera parameters
    #[must_use]
    pub fn new(position: Vec3, forward: Vec3, up: Vec3, fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let right = forward.cross(up).normalize();
        let up = right.cross(forward).normalize();
        
        Self {
            near,
            far,
            fov,
            aspect,
            position,
            forward: forward.normalize(),
            right,
            up,
        }
    }

    /// Update from camera
    pub fn update(&mut self, position: Vec3, forward: Vec3, up: Vec3) {
        self.position = position;
        self.forward = forward.normalize();
        self.right = forward.cross(up).normalize();
        self.up = self.right.cross(self.forward).normalize();
    }

    /// Check if point is inside frustum
    #[must_use]
    pub fn contains_point(&self, point: Vec3) -> bool {
        let to_point = point - self.position;
        
        // Check distance along forward axis
        let z = to_point.dot(self.forward);
        if z < self.near || z > self.far {
            return false;
        }

        // Calculate frustum bounds at this distance
        let half_height = z * (self.fov / 2.0).tan();
        let half_width = half_height * self.aspect;

        // Check x and y bounds
        let x = to_point.dot(self.right);
        if x.abs() > half_width {
            return false;
        }

        let y = to_point.dot(self.up);
        if y.abs() > half_height {
            return false;
        }

        true
    }

    /// Check if sphere is inside frustum
    #[must_use]
    pub fn contains_sphere(&self, center: Vec3, radius: f32) -> bool {
        let to_center = center - self.position;
        
        // Check distance along forward axis
        let z = to_center.dot(self.forward);
        if z < self.near - radius || z > self.far + radius {
            return false;
        }

        // Calculate frustum bounds at this distance (with margin)
        let z_clamped = z.max(self.near);
        let half_height = z_clamped * (self.fov / 2.0).tan() + radius;
        let half_width = half_height * self.aspect;

        // Check x and y bounds
        let x = to_center.dot(self.right);
        if x.abs() > half_width {
            return false;
        }

        let y = to_center.dot(self.up);
        if y.abs() > half_height {
            return false;
        }

        true
    }

    /// Check if AABB is inside frustum
    #[must_use]
    pub fn contains_aabb(&self, min: Vec3, max: Vec3) -> bool {
        let center = (min + max) * 0.5;
        let half_extents = (max - min) * 0.5;
        let radius = half_extents.length();
        
        self.contains_sphere(center, radius)
    }
}

/// Culling system
pub struct CullingSystem {
    /// Current frustum
    pub frustum: Frustum,
    /// Visible object count (last frame)
    visible_count: usize,
    /// Total object count
    total_count: usize,
}

impl Default for CullingSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl CullingSystem {
    /// Create a new culling system
    #[must_use]
    pub fn new() -> Self {
        Self {
            frustum: Frustum::default(),
            visible_count: 0,
            total_count: 0,
        }
    }

    /// Update frustum from camera
    pub fn update_frustum(&mut self, position: Vec3, forward: Vec3, up: Vec3) {
        self.frustum.update(position, forward, up);
    }

    /// Cull a list of objects, returns visibility list
    pub fn cull(&mut self, objects: &[(Vec3, f32)]) -> Vec<bool> {
        self.total_count = objects.len();
        self.visible_count = 0;
        
        let visibility: Vec<bool> = objects
            .iter()
            .map(|(center, radius)| {
                let visible = self.frustum.contains_sphere(*center, *radius);
                if visible {
                    self.visible_count += 1;
                }
                visible
            })
            .collect();
        
        visibility
    }

    /// Get visible count
    #[must_use]
    pub fn visible_count(&self) -> usize {
        self.visible_count
    }

    /// Get culled count
    #[must_use]
    pub fn culled_count(&self) -> usize {
        self.total_count - self.visible_count
    }

    /// Get cull percentage
    #[must_use]
    pub fn cull_percentage(&self) -> f32 {
        if self.total_count == 0 {
            return 0.0;
        }
        self.culled_count() as f32 / self.total_count as f32 * 100.0
    }
}

/// Occlusion query result
#[derive(Debug, Clone, Copy, Default)]
pub struct OcclusionResult {
    /// Object ID
    pub object_id: u64,
    /// Is visible
    pub visible: bool,
    /// Visible pixel count
    pub visible_pixels: u32,
}

/// Occlusion culling system (software)
pub struct OcclusionCulling {
    /// Depth buffer
    depth_buffer: Vec<f32>,
    /// Buffer width
    width: u32,
    /// Buffer height
    height: u32,
}

impl OcclusionCulling {
    /// Create a new occlusion culling system
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            depth_buffer: vec![1.0; (width * height) as usize],
            width,
            height,
        }
    }

    /// Clear depth buffer
    pub fn clear(&mut self) {
        for d in &mut self.depth_buffer {
            *d = 1.0;
        }
    }

    /// Resize buffer
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.depth_buffer = vec![1.0; (width * height) as usize];
    }

    /// Check if object might be visible (conservative)
    #[must_use]
    pub fn is_potentially_visible(&self, screen_min: Vec2, screen_max: Vec2, depth: f32) -> bool {
        let x0 = (screen_min.x * self.width as f32) as i32;
        let y0 = (screen_min.y * self.height as f32) as i32;
        let x1 = (screen_max.x * self.width as f32) as i32;
        let y1 = (screen_max.y * self.height as f32) as i32;

        // Check samples
        for y in [y0, (y0 + y1) / 2, y1] {
            for x in [x0, (x0 + x1) / 2, x1] {
                if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                    let idx = (y as u32 * self.width + x as u32) as usize;
                    if depth <= self.depth_buffer[idx] {
                        return true;
                    }
                }
            }
        }

        false
    }
}

/// Use Vec2 from core
use lunaris_core::math::Vec2;
