//! Debug Drawing System
//!
//! Runtime visualization for debugging physics, AI, and gameplay.

use glam::{Vec2, Vec3};
use lunaris_core::Color;

/// Debug line
#[derive(Debug, Clone)]
pub struct DebugLine {
    /// Start position
    pub start: Vec3,
    /// End position
    pub end: Vec3,
    /// Color
    pub color: Color,
    /// Duration (0 = one frame)
    pub duration: f32,
    /// Depth testing
    pub depth_test: bool,
}

/// Debug shape
#[derive(Debug, Clone)]
pub enum DebugShape {
    /// Line from A to B
    Line(Vec3, Vec3),
    /// Sphere at position with radius
    Sphere { center: Vec3, radius: f32 },
    /// Box with min/max corners
    Box { min: Vec3, max: Vec3 },
    /// Capsule
    Capsule { start: Vec3, end: Vec3, radius: f32 },
    /// Arrow
    Arrow { origin: Vec3, direction: Vec3 },
    /// Circle in 3D
    Circle { center: Vec3, normal: Vec3, radius: f32 },
    /// Frustum planes
    Frustum { planes: [Vec3; 6] },
    /// Path as connected lines
    Path(Vec<Vec3>),
    /// Text at 3D position
    Text { position: Vec3, text: String },
}

/// Debug draw command
#[derive(Debug, Clone)]
pub struct DebugDrawCommand {
    /// Shape to draw
    pub shape: DebugShape,
    /// Color
    pub color: Color,
    /// Duration (0 = one frame)
    pub duration: f32,
    /// Depth testing enabled
    pub depth_test: bool,
}

/// Debug draw system
pub struct DebugDraw {
    /// Pending commands
    commands: Vec<DebugDrawCommand>,
    /// Persistent commands (with duration > 0)
    persistent: Vec<(DebugDrawCommand, f32)>,
    /// Is enabled
    pub enabled: bool,
    /// Default color
    pub default_color: Color,
    /// Draw physics
    pub draw_physics: bool,
    /// Draw navigation
    pub draw_navigation: bool,
    /// Draw AI
    pub draw_ai: bool,
}

impl Default for DebugDraw {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugDraw {
    /// Create a new debug draw system
    #[must_use]
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            persistent: Vec::new(),
            enabled: true,
            default_color: Color::GREEN,
            draw_physics: true,
            draw_navigation: true,
            draw_ai: true,
        }
    }

    /// Draw a line
    pub fn line(&mut self, start: Vec3, end: Vec3, color: Color) {
        if !self.enabled { return; }
        self.add_command(DebugShape::Line(start, end), color, 0.0, true);
    }

    /// Draw a ray
    pub fn ray(&mut self, origin: Vec3, direction: Vec3, length: f32, color: Color) {
        if !self.enabled { return; }
        self.line(origin, origin + direction.normalize() * length, color);
    }

    /// Draw an arrow
    pub fn arrow(&mut self, origin: Vec3, direction: Vec3, color: Color) {
        if !self.enabled { return; }
        self.add_command(DebugShape::Arrow { origin, direction }, color, 0.0, true);
    }

    /// Draw a sphere
    pub fn sphere(&mut self, center: Vec3, radius: f32, color: Color) {
        if !self.enabled { return; }
        self.add_command(DebugShape::Sphere { center, radius }, color, 0.0, true);
    }

    /// Draw a box
    pub fn cube(&mut self, min: Vec3, max: Vec3, color: Color) {
        if !self.enabled { return; }
        self.add_command(DebugShape::Box { min, max }, color, 0.0, true);
    }

    /// Draw a wire box
    pub fn wire_box(&mut self, center: Vec3, size: Vec3, color: Color) {
        let half = size * 0.5;
        self.cube(center - half, center + half, color);
    }

    /// Draw a capsule
    pub fn capsule(&mut self, start: Vec3, end: Vec3, radius: f32, color: Color) {
        if !self.enabled { return; }
        self.add_command(DebugShape::Capsule { start, end, radius }, color, 0.0, true);
    }

    /// Draw a circle
    pub fn circle(&mut self, center: Vec3, normal: Vec3, radius: f32, color: Color) {
        if !self.enabled { return; }
        self.add_command(DebugShape::Circle { center, normal, radius }, color, 0.0, true);
    }

    /// Draw a path
    pub fn path(&mut self, points: &[Vec3], color: Color) {
        if !self.enabled || points.is_empty() { return; }
        self.add_command(DebugShape::Path(points.to_vec()), color, 0.0, true);
    }

    /// Draw text at 3D position
    pub fn text(&mut self, position: Vec3, text: &str, color: Color) {
        if !self.enabled { return; }
        self.add_command(
            DebugShape::Text { position, text: text.to_string() },
            color,
            0.0,
            false,
        );
    }

    /// Draw with duration (persists for N seconds)
    pub fn line_duration(&mut self, start: Vec3, end: Vec3, color: Color, duration: f32) {
        if !self.enabled { return; }
        self.add_command(DebugShape::Line(start, end), color, duration, true);
    }

    /// Draw a coordinate system (RGB = XYZ)
    pub fn axes(&mut self, origin: Vec3, scale: f32) {
        if !self.enabled { return; }
        self.arrow(origin, Vec3::X * scale, Color::RED);
        self.arrow(origin, Vec3::Y * scale, Color::GREEN);
        self.arrow(origin, Vec3::Z * scale, Color::BLUE);
    }

    /// Draw a grid on XZ plane
    pub fn grid(&mut self, center: Vec3, size: f32, divisions: u32, color: Color) {
        if !self.enabled { return; }
        
        let half = size / 2.0;
        let step = size / divisions as f32;

        for i in 0..=divisions {
            let offset = i as f32 * step - half;
            // X lines
            self.line(
                center + Vec3::new(-half, 0.0, offset),
                center + Vec3::new(half, 0.0, offset),
                color,
            );
            // Z lines
            self.line(
                center + Vec3::new(offset, 0.0, -half),
                center + Vec3::new(offset, 0.0, half),
                color,
            );
        }
    }

    /// Add a command
    fn add_command(&mut self, shape: DebugShape, color: Color, duration: f32, depth_test: bool) {
        let cmd = DebugDrawCommand {
            shape,
            color,
            duration,
            depth_test,
        };

        if duration > 0.0 {
            self.persistent.push((cmd, duration));
        } else {
            self.commands.push(cmd);
        }
    }

    /// Update persistent commands
    pub fn update(&mut self, delta_time: f32) {
        // Update persistent timers
        for (_, time) in &mut self.persistent {
            *time -= delta_time;
        }

        // Remove expired
        self.persistent.retain(|(_, time)| *time > 0.0);
    }

    /// Get all commands for this frame
    pub fn get_commands(&mut self) -> Vec<DebugDrawCommand> {
        let mut result = std::mem::take(&mut self.commands);
        
        // Add persistent commands
        for (cmd, _) in &self.persistent {
            result.push(cmd.clone());
        }

        result
    }

    /// Clear all commands
    pub fn clear(&mut self) {
        self.commands.clear();
    }

    /// Clear all including persistent
    pub fn clear_all(&mut self) {
        self.commands.clear();
        self.persistent.clear();
    }
}

/// 2D debug drawing
pub struct DebugDraw2D {
    /// Lines
    lines: Vec<(Vec2, Vec2, Color)>,
    /// Rectangles
    rects: Vec<(Vec2, Vec2, Color, bool)>,
    /// Circles
    circles: Vec<(Vec2, f32, Color)>,
    /// Text labels
    texts: Vec<(Vec2, String, Color)>,
    /// Is enabled
    pub enabled: bool,
}

impl Default for DebugDraw2D {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugDraw2D {
    /// Create a new 2D debug draw
    #[must_use]
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            rects: Vec::new(),
            circles: Vec::new(),
            texts: Vec::new(),
            enabled: true,
        }
    }

    /// Draw a line
    pub fn line(&mut self, start: Vec2, end: Vec2, color: Color) {
        if self.enabled {
            self.lines.push((start, end, color));
        }
    }

    /// Draw a rectangle
    pub fn rect(&mut self, min: Vec2, max: Vec2, color: Color, filled: bool) {
        if self.enabled {
            self.rects.push((min, max, color, filled));
        }
    }

    /// Draw a circle
    pub fn circle(&mut self, center: Vec2, radius: f32, color: Color) {
        if self.enabled {
            self.circles.push((center, radius, color));
        }
    }

    /// Draw text
    pub fn text(&mut self, position: Vec2, text: &str, color: Color) {
        if self.enabled {
            self.texts.push((position, text.to_string(), color));
        }
    }

    /// Clear all drawings
    pub fn clear(&mut self) {
        self.lines.clear();
        self.rects.clear();
        self.circles.clear();
        self.texts.clear();
    }

    /// Get line count
    #[must_use]
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}
