//! Interactive Viewport
//!
//! 3D/2D scene viewport with camera controls and gizmo integration.

use glam::{Vec2, Vec3, Mat4, Quat};
use std::collections::HashMap;
use super::design_system::*;
use super::gizmo::{Gizmo, GizmoType, GizmoAxis};

// ==================== VIEWPORT ====================

/// Viewport widget
pub struct ViewportWidget {
    /// Unique ID
    pub id: u64,
    /// Viewport name
    pub name: String,
    /// Camera
    pub camera: ViewportCamera,
    /// Render mode
    pub render_mode: RenderMode,
    /// View type
    pub view_type: ViewType,
    /// Gizmo state
    pub gizmo: GizmoState,
    /// Grid settings
    pub grid: ViewportGrid,
    /// Selection
    pub selection: Vec<u64>,
    /// Interaction state
    pub interaction: ViewportInteraction,
    /// Overlays
    pub overlays: ViewportOverlays,
    /// Stats
    pub stats: ViewportStats,
    /// Size
    pub size: Vec2,
    /// Is focused
    pub focused: bool,
}

/// Viewport camera
#[derive(Debug, Clone)]
pub struct ViewportCamera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub orthographic: bool,
    pub ortho_size: f32,
    /// Orbit state
    pub orbit_distance: f32,
    pub orbit_yaw: f32,
    pub orbit_pitch: f32,
    /// Movement speed
    pub move_speed: f32,
    pub rotation_speed: f32,
    pub zoom_speed: f32,
}

impl Default for ViewportCamera {
    fn default() -> Self {
        Self {
            position: Vec3::new(5.0, 5.0, 5.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
            fov: 60.0,
            near: 0.1,
            far: 10000.0,
            orthographic: false,
            ortho_size: 10.0,
            orbit_distance: 10.0,
            orbit_yaw: -45.0_f32.to_radians(),
            orbit_pitch: 30.0_f32.to_radians(),
            move_speed: 10.0,
            rotation_speed: 0.3,
            zoom_speed: 0.5,
        }
    }
}

impl ViewportCamera {
    /// Get view matrix
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    /// Get projection matrix
    pub fn projection_matrix(&self, aspect: f32) -> Mat4 {
        if self.orthographic {
            Mat4::orthographic_rh(
                -self.ortho_size * aspect,
                self.ortho_size * aspect,
                -self.ortho_size,
                self.ortho_size,
                self.near,
                self.far,
            )
        } else {
            Mat4::perspective_rh(
                self.fov.to_radians(),
                aspect,
                self.near,
                self.far,
            )
        }
    }

    /// Orbit around target
    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.orbit_yaw += delta_yaw * self.rotation_speed;
        self.orbit_pitch = (self.orbit_pitch + delta_pitch * self.rotation_speed)
            .clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());

        self.update_from_orbit();
    }

    /// Update position from orbit parameters
    fn update_from_orbit(&mut self) {
        let x = self.orbit_distance * self.orbit_pitch.cos() * self.orbit_yaw.sin();
        let y = self.orbit_distance * self.orbit_pitch.sin();
        let z = self.orbit_distance * self.orbit_pitch.cos() * self.orbit_yaw.cos();

        self.position = self.target + Vec3::new(x, y, z);
    }

    /// Pan camera
    pub fn pan(&mut self, delta: Vec2) {
        let forward = (self.target - self.position).normalize();
        let right = forward.cross(self.up).normalize();
        let up = right.cross(forward);

        let move_amount = delta * self.move_speed * 0.01;
        let offset = right * -move_amount.x + up * move_amount.y;

        self.position += offset;
        self.target += offset;
    }

    /// Zoom (dolly)
    pub fn zoom(&mut self, delta: f32) {
        if self.orthographic {
            self.ortho_size = (self.ortho_size - delta * self.zoom_speed).max(0.1);
        } else {
            self.orbit_distance = (self.orbit_distance - delta * self.zoom_speed).max(0.1);
            self.update_from_orbit();
        }
    }

    /// WASD fly movement
    pub fn fly(&mut self, forward: f32, right: f32, up: f32, dt: f32) {
        let fwd = (self.target - self.position).normalize();
        let rgt = fwd.cross(self.up).normalize();

        let move_delta = fwd * forward + rgt * right + Vec3::Y * up;
        let offset = move_delta * self.move_speed * dt;

        self.position += offset;
        self.target += offset;
    }

    /// Look around (FPS style)
    pub fn look(&mut self, delta: Vec2) {
        let forward = (self.target - self.position).normalize();
        let distance = (self.target - self.position).length();

        // Apply rotation
        let yaw_rot = Quat::from_rotation_y(-delta.x * 0.01);
        let right = forward.cross(self.up).normalize();
        let pitch_rot = Quat::from_axis_angle(right, -delta.y * 0.01);

        let new_forward = pitch_rot * yaw_rot * forward;
        self.target = self.position + new_forward * distance;
    }

    /// Frame on bounds
    pub fn frame_bounds(&mut self, center: Vec3, radius: f32) {
        self.target = center;
        self.orbit_distance = radius * 2.5;
        self.update_from_orbit();
    }

    /// Set to orthographic view
    pub fn set_orthographic_view(&mut self, view: OrthographicView) {
        self.orthographic = true;
        
        match view {
            OrthographicView::Top => {
                self.position = self.target + Vec3::new(0.0, self.orbit_distance, 0.0);
                self.up = Vec3::NEG_Z;
            }
            OrthographicView::Bottom => {
                self.position = self.target + Vec3::new(0.0, -self.orbit_distance, 0.0);
                self.up = Vec3::Z;
            }
            OrthographicView::Front => {
                self.position = self.target + Vec3::new(0.0, 0.0, self.orbit_distance);
                self.up = Vec3::Y;
            }
            OrthographicView::Back => {
                self.position = self.target + Vec3::new(0.0, 0.0, -self.orbit_distance);
                self.up = Vec3::Y;
            }
            OrthographicView::Left => {
                self.position = self.target + Vec3::new(-self.orbit_distance, 0.0, 0.0);
                self.up = Vec3::Y;
            }
            OrthographicView::Right => {
                self.position = self.target + Vec3::new(self.orbit_distance, 0.0, 0.0);
                self.up = Vec3::Y;
            }
        }
    }

    /// Set to perspective
    pub fn set_perspective(&mut self) {
        self.orthographic = false;
        self.up = Vec3::Y;
        self.update_from_orbit();
    }
}

/// Orthographic view preset
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrthographicView {
    Top,
    Bottom,
    Front,
    Back,
    Left,
    Right,
}

/// Render mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    Lit,
    Unlit,
    Wireframe,
    WireframeOnShaded,
    Normals,
    UVs,
    Albedo,
    Metallic,
    Roughness,
    AO,
    Depth,
    LightingOnly,
    VertexColors,
    Overdraw,
}

impl RenderMode {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Lit => "Lit",
            Self::Unlit => "Unlit",
            Self::Wireframe => "Wireframe",
            Self::WireframeOnShaded => "Wireframe on Shaded",
            Self::Normals => "Normals",
            Self::UVs => "UVs",
            Self::Albedo => "Albedo",
            Self::Metallic => "Metallic",
            Self::Roughness => "Roughness",
            Self::AO => "Ambient Occlusion",
            Self::Depth => "Depth",
            Self::LightingOnly => "Lighting Only",
            Self::VertexColors => "Vertex Colors",
            Self::Overdraw => "Overdraw",
        }
    }
}

/// View type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewType {
    Perspective,
    Orthographic(OrthographicView),
}

/// Gizmo state for viewport
#[derive(Debug, Clone)]
pub struct GizmoState {
    pub gizmo_type: GizmoType,
    pub active_axis: Option<GizmoAxis>,
    pub world_space: bool,
    pub snap_enabled: bool,
    pub snap_translate: f32,
    pub snap_rotate: f32,
    pub snap_scale: f32,
    pub pivot: GizmoPivot,
    pub visible: bool,
    pub size: f32,
}

/// Gizmo pivot mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GizmoPivot {
    /// Center of selection bounds
    Center,
    /// Individual object pivots
    Individual,
    /// Active object pivot
    Active,
    /// 3D cursor
    Cursor,
}

impl Default for GizmoState {
    fn default() -> Self {
        Self {
            gizmo_type: GizmoType::Translate,
            active_axis: None,
            world_space: true,
            snap_enabled: false,
            snap_translate: 1.0,
            snap_rotate: 15.0,
            snap_scale: 0.1,
            pivot: GizmoPivot::Center,
            visible: true,
            size: 1.0,
        }
    }
}

/// Viewport grid
#[derive(Debug, Clone)]
pub struct ViewportGrid {
    pub visible: bool,
    pub size: f32,
    pub subdivisions: u32,
    pub color_major: Color,
    pub color_minor: Color,
    pub color_x: Color,
    pub color_z: Color,
    pub fade_distance: f32,
}

impl Default for ViewportGrid {
    fn default() -> Self {
        Self {
            visible: true,
            size: 100.0,
            subdivisions: 10,
            color_major: Color::rgba(100, 100, 100, 100),
            color_minor: Color::rgba(50, 50, 50, 50),
            color_x: Color::rgba(255, 80, 80, 200),
            color_z: Color::rgba(80, 80, 255, 200),
            fade_distance: 50.0,
        }
    }
}

/// Viewport interaction state
#[derive(Debug, Clone, Default)]
pub struct ViewportInteraction {
    pub mode: InteractionMode,
    pub mouse_pos: Vec2,
    pub mouse_delta: Vec2,
    pub is_orbiting: bool,
    pub is_panning: bool,
    pub is_flying: bool,
    pub is_dragging_gizmo: bool,
    pub drag_start: Option<Vec2>,
    pub hover_entity: Option<u64>,
    pub marquee: Option<MarqueeSelect>,
}

/// Interaction mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InteractionMode {
    #[default]
    Select,
    Orbit,
    Pan,
    Fly,
    Zoom,
}

/// Marquee selection
#[derive(Debug, Clone)]
pub struct MarqueeSelect {
    pub start: Vec2,
    pub end: Vec2,
    pub additive: bool,
}

/// Viewport overlays
#[derive(Debug, Clone)]
pub struct ViewportOverlays {
    pub show_grid: bool,
    pub show_gizmo: bool,
    pub show_selection_outline: bool,
    pub show_bone_names: bool,
    pub show_physics: bool,
    pub show_colliders: bool,
    pub show_bounds: bool,
    pub show_lights: bool,
    pub show_cameras: bool,
    pub show_navmesh: bool,
    pub show_audio_sources: bool,
    pub show_stats: bool,
    pub show_fps: bool,
}

impl Default for ViewportOverlays {
    fn default() -> Self {
        Self {
            show_grid: true,
            show_gizmo: true,
            show_selection_outline: true,
            show_bone_names: false,
            show_physics: false,
            show_colliders: false,
            show_bounds: false,
            show_lights: true,
            show_cameras: true,
            show_navmesh: false,
            show_audio_sources: false,
            show_stats: true,
            show_fps: true,
        }
    }
}

/// Viewport stats
#[derive(Debug, Clone, Default)]
pub struct ViewportStats {
    pub fps: f32,
    pub frame_time_ms: f32,
    pub draw_calls: u32,
    pub triangles: u64,
    pub vertices: u64,
    pub objects_visible: u32,
    pub objects_culled: u32,
    pub lights_active: u32,
    pub memory_gpu_mb: f32,
}

impl Default for ViewportWidget {
    fn default() -> Self {
        Self::new(0, "Viewport")
    }
}

impl ViewportWidget {
    /// Create new viewport
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            camera: ViewportCamera::default(),
            render_mode: RenderMode::Lit,
            view_type: ViewType::Perspective,
            gizmo: GizmoState::default(),
            grid: ViewportGrid::default(),
            selection: Vec::new(),
            interaction: ViewportInteraction::default(),
            overlays: ViewportOverlays::default(),
            stats: ViewportStats::default(),
            size: Vec2::new(800.0, 600.0),
            focused: false,
        }
    }

    /// Handle mouse input
    pub fn on_mouse_move(&mut self, pos: Vec2) {
        let delta = pos - self.interaction.mouse_pos;
        self.interaction.mouse_delta = delta;
        self.interaction.mouse_pos = pos;

        if self.interaction.is_orbiting {
            self.camera.orbit(delta.x, delta.y);
        } else if self.interaction.is_panning {
            self.camera.pan(delta);
        } else if self.interaction.is_flying {
            self.camera.look(delta);
        }

        // Update marquee
        if let Some(ref mut marquee) = self.interaction.marquee {
            marquee.end = pos;
        }
    }

    /// Handle mouse button
    pub fn on_mouse_button(&mut self, button: MouseButton, pressed: bool, modifiers: Modifiers) {
        match button {
            MouseButton::Middle => {
                if pressed {
                    if modifiers.shift {
                        self.interaction.is_panning = true;
                    } else {
                        self.interaction.is_orbiting = true;
                    }
                } else {
                    self.interaction.is_orbiting = false;
                    self.interaction.is_panning = false;
                }
            }
            MouseButton::Right => {
                if pressed {
                    self.interaction.is_flying = true;
                } else {
                    self.interaction.is_flying = false;
                }
            }
            MouseButton::Left => {
                if pressed {
                    // Start marquee or click select
                    if modifiers.ctrl || modifiers.shift {
                        self.interaction.marquee = Some(MarqueeSelect {
                            start: self.interaction.mouse_pos,
                            end: self.interaction.mouse_pos,
                            additive: modifiers.shift,
                        });
                    }
                } else {
                    // End marquee or complete click
                    if let Some(marquee) = self.interaction.marquee.take() {
                        self.select_in_marquee(&marquee);
                    }
                }
            }
        }
    }

    fn select_in_marquee(&mut self, _marquee: &MarqueeSelect) {
        // Would raycast into scene to find entities in rectangle
    }

    /// Handle scroll
    pub fn on_scroll(&mut self, delta: f32) {
        self.camera.zoom(delta);
    }

    /// Handle keyboard for fly mode
    pub fn on_fly_input(&mut self, forward: f32, right: f32, up: f32, dt: f32) {
        if self.interaction.is_flying {
            self.camera.fly(forward, right, up, dt);
        }
    }

    /// Set gizmo type
    pub fn set_gizmo(&mut self, gizmo_type: GizmoType) {
        self.gizmo.gizmo_type = gizmo_type;
    }

    /// Toggle world/local space
    pub fn toggle_space(&mut self) {
        self.gizmo.world_space = !self.gizmo.world_space;
    }

    /// Toggle orthographic/perspective
    pub fn toggle_projection(&mut self) {
        if self.camera.orthographic {
            self.camera.set_perspective();
            self.view_type = ViewType::Perspective;
        } else {
            self.camera.orthographic = true;
        }
    }

    /// Set orthographic view
    pub fn set_view(&mut self, view: OrthographicView) {
        self.camera.set_orthographic_view(view);
        self.view_type = ViewType::Orthographic(view);
    }

    /// Frame selected objects
    pub fn frame_selected(&mut self) {
        if self.selection.is_empty() {
            self.camera.frame_bounds(Vec3::ZERO, 5.0);
        } else {
            // Would calculate bounds of selected objects
            self.camera.frame_bounds(Vec3::ZERO, 5.0);
        }
    }

    /// Select entity
    pub fn select(&mut self, entity_id: u64, additive: bool) {
        if !additive {
            self.selection.clear();
        }
        if !self.selection.contains(&entity_id) {
            self.selection.push(entity_id);
        }
    }

    /// Deselect all
    pub fn deselect_all(&mut self) {
        self.selection.clear();
    }

    /// Get computed view-projection matrix
    pub fn view_projection(&self) -> Mat4 {
        let aspect = self.size.x / self.size.y;
        self.camera.projection_matrix(aspect) * self.camera.view_matrix()
    }

    /// Screen to world ray
    pub fn screen_to_ray(&self, screen_pos: Vec2) -> (Vec3, Vec3) {
        let vp = self.view_projection();
        let vp_inv = vp.inverse();

        let ndc = Vec3::new(
            (screen_pos.x / self.size.x) * 2.0 - 1.0,
            1.0 - (screen_pos.y / self.size.y) * 2.0,
            -1.0,
        );

        let near = vp_inv.project_point3(ndc);
        let ndc_far = Vec3::new(ndc.x, ndc.y, 1.0);
        let far = vp_inv.project_point3(ndc_far);

        let direction = (far - near).normalize();
        (near, direction)
    }

    /// World to screen
    pub fn world_to_screen(&self, world: Vec3) -> Option<Vec2> {
        let vp = self.view_projection();
        let clip = vp * world.extend(1.0);
        
        if clip.w <= 0.0 {
            return None; // Behind camera
        }

        let ndc = clip.truncate() / clip.w;
        
        Some(Vec2::new(
            (ndc.x * 0.5 + 0.5) * self.size.x,
            (0.5 - ndc.y * 0.5) * self.size.y,
        ))
    }
}

/// Mouse button
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

/// Keyboard modifiers (imported from shortcuts but simplified here)
#[derive(Debug, Clone, Copy, Default)]
pub struct Modifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
}
