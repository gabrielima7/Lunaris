//! Editor gizmos for visual manipulation

use lunaris_core::math::{Color, Vec2, Vec3};

/// Gizmo type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GizmoType {
    /// Translation gizmo (arrows)
    #[default]
    Translate,
    /// Rotation gizmo (circles)
    Rotate,
    /// Scale gizmo (boxes)
    Scale,
    /// Combined gizmo
    Universal,
}

/// Gizmo axis being manipulated
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GizmoAxis {
    /// No axis
    None,
    /// X axis
    X,
    /// Y axis
    Y,
    /// Z axis
    Z,
    /// XY plane
    XY,
    /// XZ plane
    XZ,
    /// YZ plane
    YZ,
    /// All axes
    XYZ,
}

/// Gizmo colors
pub struct GizmoColors {
    /// X axis color (red)
    pub x: Color,
    /// Y axis color (green)
    pub y: Color,
    /// Z axis color (blue)
    pub z: Color,
    /// Highlight color (yellow)
    pub highlight: Color,
}

impl Default for GizmoColors {
    fn default() -> Self {
        Self {
            x: Color::new(1.0, 0.2, 0.2, 1.0),
            y: Color::new(0.2, 1.0, 0.2, 1.0),
            z: Color::new(0.2, 0.2, 1.0, 1.0),
            highlight: Color::new(1.0, 1.0, 0.0, 1.0),
        }
    }
}

/// Gizmo state and logic
pub struct Gizmo {
    /// Current gizmo type
    pub gizmo_type: GizmoType,
    /// Selected axis
    pub selected_axis: GizmoAxis,
    /// Is being dragged
    pub is_dragging: bool,
    /// Drag start position
    pub drag_start: Vec3,
    /// Drag start value
    pub start_value: Vec3,
    /// Gizmo size (screen space)
    pub size: f32,
    /// Colors
    pub colors: GizmoColors,
    /// Is visible
    pub visible: bool,
    /// Local or world space
    pub local_space: bool,
}

impl Default for Gizmo {
    fn default() -> Self {
        Self {
            gizmo_type: GizmoType::Translate,
            selected_axis: GizmoAxis::None,
            is_dragging: false,
            drag_start: Vec3::ZERO,
            start_value: Vec3::ZERO,
            size: 100.0,
            colors: GizmoColors::default(),
            visible: true,
            local_space: false,
        }
    }
}

impl Gizmo {
    /// Create a new gizmo
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set gizmo type
    pub fn set_type(&mut self, gizmo_type: GizmoType) {
        self.gizmo_type = gizmo_type;
        self.selected_axis = GizmoAxis::None;
    }

    /// Start dragging
    pub fn start_drag(&mut self, axis: GizmoAxis, mouse_pos: Vec3, current_value: Vec3) {
        self.is_dragging = true;
        self.selected_axis = axis;
        self.drag_start = mouse_pos;
        self.start_value = current_value;
    }

    /// Update drag
    pub fn update_drag(&mut self, mouse_pos: Vec3, current_value: &mut Vec3) {
        if !self.is_dragging {
            return;
        }

        let delta = mouse_pos - self.drag_start;

        match self.gizmo_type {
            GizmoType::Translate => {
                self.apply_translate(delta, current_value);
            }
            GizmoType::Rotate => {
                self.apply_rotate(delta, current_value);
            }
            GizmoType::Scale => {
                self.apply_scale(delta, current_value);
            }
            GizmoType::Universal => {
                self.apply_translate(delta, current_value);
            }
        }
    }

    fn apply_translate(&self, delta: Vec3, value: &mut Vec3) {
        match self.selected_axis {
            GizmoAxis::X => {
                value.x = self.start_value.x + delta.x;
            }
            GizmoAxis::Y => {
                value.y = self.start_value.y + delta.y;
            }
            GizmoAxis::Z => {
                value.z = self.start_value.z + delta.z;
            }
            GizmoAxis::XY => {
                value.x = self.start_value.x + delta.x;
                value.y = self.start_value.y + delta.y;
            }
            GizmoAxis::XZ => {
                value.x = self.start_value.x + delta.x;
                value.z = self.start_value.z + delta.z;
            }
            GizmoAxis::YZ => {
                value.y = self.start_value.y + delta.y;
                value.z = self.start_value.z + delta.z;
            }
            GizmoAxis::XYZ | GizmoAxis::None => {
                *value = self.start_value + delta;
            }
        }
    }

    fn apply_rotate(&self, delta: Vec3, value: &mut Vec3) {
        let sensitivity = 0.01;
        match self.selected_axis {
            GizmoAxis::X => {
                value.x = self.start_value.x + delta.y * sensitivity;
            }
            GizmoAxis::Y => {
                value.y = self.start_value.y + delta.x * sensitivity;
            }
            GizmoAxis::Z => {
                value.z = self.start_value.z + delta.x * sensitivity;
            }
            _ => {}
        }
    }

    fn apply_scale(&self, delta: Vec3, value: &mut Vec3) {
        let sensitivity = 0.01;
        let scale_delta = delta.length() * sensitivity;
        
        match self.selected_axis {
            GizmoAxis::X => {
                value.x = (self.start_value.x + delta.x * sensitivity).max(0.01);
            }
            GizmoAxis::Y => {
                value.y = (self.start_value.y + delta.y * sensitivity).max(0.01);
            }
            GizmoAxis::Z => {
                value.z = (self.start_value.z + delta.z * sensitivity).max(0.01);
            }
            GizmoAxis::XYZ => {
                let uniform = self.start_value.x + scale_delta;
                value.x = uniform.max(0.01);
                value.y = uniform.max(0.01);
                value.z = uniform.max(0.01);
            }
            _ => {}
        }
    }

    /// End dragging
    pub fn end_drag(&mut self) {
        self.is_dragging = false;
    }

    /// Cancel dragging
    pub fn cancel_drag(&mut self, value: &mut Vec3) {
        if self.is_dragging {
            *value = self.start_value;
            self.is_dragging = false;
        }
    }

    /// Get color for an axis
    #[must_use]
    pub fn axis_color(&self, axis: GizmoAxis) -> Color {
        if self.selected_axis == axis {
            self.colors.highlight
        } else {
            match axis {
                GizmoAxis::X => self.colors.x,
                GizmoAxis::Y => self.colors.y,
                GizmoAxis::Z => self.colors.z,
                _ => Color::WHITE,
            }
        }
    }
}

/// Gizmo draw commands for 2D
pub struct GizmoDrawer2D {
    /// Lines to draw
    pub lines: Vec<GizmoLine>,
    /// Circles to draw
    pub circles: Vec<GizmoCircle>,
}

/// A line in the gizmo
#[derive(Debug, Clone)]
pub struct GizmoLine {
    /// Start point
    pub start: Vec2,
    /// End point
    pub end: Vec2,
    /// Color
    pub color: Color,
    /// Width
    pub width: f32,
}

/// A circle in the gizmo
#[derive(Debug, Clone)]
pub struct GizmoCircle {
    /// Center
    pub center: Vec2,
    /// Radius
    pub radius: f32,
    /// Color
    pub color: Color,
    /// Width
    pub width: f32,
}

impl Default for GizmoDrawer2D {
    fn default() -> Self {
        Self::new()
    }
}

impl GizmoDrawer2D {
    /// Create a new gizmo drawer
    #[must_use]
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            circles: Vec::new(),
        }
    }

    /// Clear all draw commands
    pub fn clear(&mut self) {
        self.lines.clear();
        self.circles.clear();
    }

    /// Draw a 2D translation gizmo
    pub fn draw_translate_2d(&mut self, position: Vec2, gizmo: &Gizmo) {
        let arrow_length = gizmo.size;
        let arrow_head = 15.0;

        // X axis arrow
        self.lines.push(GizmoLine {
            start: position,
            end: Vec2::new(position.x + arrow_length, position.y),
            color: gizmo.axis_color(GizmoAxis::X),
            width: 2.0,
        });
        // Arrowhead
        self.lines.push(GizmoLine {
            start: Vec2::new(position.x + arrow_length, position.y),
            end: Vec2::new(position.x + arrow_length - arrow_head, position.y - arrow_head * 0.5),
            color: gizmo.axis_color(GizmoAxis::X),
            width: 2.0,
        });
        self.lines.push(GizmoLine {
            start: Vec2::new(position.x + arrow_length, position.y),
            end: Vec2::new(position.x + arrow_length - arrow_head, position.y + arrow_head * 0.5),
            color: gizmo.axis_color(GizmoAxis::X),
            width: 2.0,
        });

        // Y axis arrow
        self.lines.push(GizmoLine {
            start: position,
            end: Vec2::new(position.x, position.y - arrow_length),
            color: gizmo.axis_color(GizmoAxis::Y),
            width: 2.0,
        });
        // Arrowhead
        self.lines.push(GizmoLine {
            start: Vec2::new(position.x, position.y - arrow_length),
            end: Vec2::new(position.x - arrow_head * 0.5, position.y - arrow_length + arrow_head),
            color: gizmo.axis_color(GizmoAxis::Y),
            width: 2.0,
        });
        self.lines.push(GizmoLine {
            start: Vec2::new(position.x, position.y - arrow_length),
            end: Vec2::new(position.x + arrow_head * 0.5, position.y - arrow_length + arrow_head),
            color: gizmo.axis_color(GizmoAxis::Y),
            width: 2.0,
        });
    }

    /// Draw a 2D rotation gizmo
    pub fn draw_rotate_2d(&mut self, position: Vec2, gizmo: &Gizmo) {
        self.circles.push(GizmoCircle {
            center: position,
            radius: gizmo.size * 0.8,
            color: gizmo.axis_color(GizmoAxis::Z),
            width: 2.0,
        });
    }

    /// Draw a 2D scale gizmo
    pub fn draw_scale_2d(&mut self, position: Vec2, gizmo: &Gizmo) {
        let scale_length = gizmo.size * 0.8;
        let box_size = 10.0;

        // X axis line + box
        self.lines.push(GizmoLine {
            start: position,
            end: Vec2::new(position.x + scale_length, position.y),
            color: gizmo.axis_color(GizmoAxis::X),
            width: 2.0,
        });

        // Y axis line + box
        self.lines.push(GizmoLine {
            start: position,
            end: Vec2::new(position.x, position.y - scale_length),
            color: gizmo.axis_color(GizmoAxis::Y),
            width: 2.0,
        });
    }
}
