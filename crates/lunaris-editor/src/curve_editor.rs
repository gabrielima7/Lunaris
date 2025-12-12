//! Animation Curve Editor
//!
//! Professional timeline and curve editing for animation.

use glam::Vec2;
use std::collections::HashMap;
use super::design_system::*;

// ==================== CURVE TYPES ====================

/// Interpolation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurveInterpolation {
    /// Constant value (step)
    Constant,
    /// Linear interpolation
    Linear,
    /// Cubic bezier
    Bezier,
    /// Hermite spline
    Hermite,
    /// TCB (Tension-Continuity-Bias)
    TCB,
}

/// Tangent mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TangentMode {
    /// Free tangents (independent)
    Free,
    /// Aligned (same direction, different lengths)
    Aligned,
    /// Flat tangent
    Flat,
    /// Auto-calculated smooth
    Auto,
    /// Linear to next
    Linear,
    /// Stepped (hold)
    Stepped,
    /// Weighted tangent
    Weighted,
}

/// Keyframe
#[derive(Debug, Clone)]
pub struct Keyframe {
    pub id: u64,
    pub time: f32,
    pub value: f32,
    pub interpolation: CurveInterpolation,
    pub in_tangent: TangentData,
    pub out_tangent: TangentData,
    pub selected: bool,
}

/// Tangent data
#[derive(Debug, Clone)]
pub struct TangentData {
    pub mode: TangentMode,
    pub angle: f32,
    pub weight: f32,
    pub broken: bool,
}

impl Default for TangentData {
    fn default() -> Self {
        Self {
            mode: TangentMode::Auto,
            angle: 0.0,
            weight: 1.0,
            broken: false,
        }
    }
}

impl Keyframe {
    pub fn new(id: u64, time: f32, value: f32) -> Self {
        Self {
            id,
            time,
            value,
            interpolation: CurveInterpolation::Bezier,
            in_tangent: TangentData::default(),
            out_tangent: TangentData::default(),
            selected: false,
        }
    }

    /// Get tangent vector for in tangent
    pub fn in_tangent_vec(&self) -> Vec2 {
        let angle = self.in_tangent.angle;
        let len = self.in_tangent.weight * 0.3; // scale factor
        Vec2::new(-angle.cos() * len, -angle.sin() * len)
    }

    /// Get tangent vector for out tangent
    pub fn out_tangent_vec(&self) -> Vec2 {
        let angle = self.out_tangent.angle;
        let len = self.out_tangent.weight * 0.3;
        Vec2::new(angle.cos() * len, angle.sin() * len)
    }
}

// ==================== ANIMATION CURVE ====================

/// Animation curve
#[derive(Debug, Clone)]
pub struct AnimationCurve {
    pub id: u64,
    pub name: String,
    pub keyframes: Vec<Keyframe>,
    pub color: Color,
    pub visible: bool,
    pub locked: bool,
    pub pre_infinity: InfinityMode,
    pub post_infinity: InfinityMode,
    next_id: u64,
}

/// Infinity mode (behavior before/after curve)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfinityMode {
    /// Constant (hold first/last value)
    Constant,
    /// Linear extrapolation
    Linear,
    /// Cycle (repeat)
    Cycle,
    /// Cycle with offset
    CycleOffset,
    /// Ping-pong (oscillate)
    PingPong,
}

impl AnimationCurve {
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            keyframes: Vec::new(),
            color: Color::hex("#6366f1"),
            visible: true,
            locked: false,
            pre_infinity: InfinityMode::Constant,
            post_infinity: InfinityMode::Constant,
            next_id: 1,
        }
    }

    /// Add keyframe
    pub fn add_keyframe(&mut self, time: f32, value: f32) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let key = Keyframe::new(id, time, value);
        
        // Insert in sorted order
        let pos = self.keyframes.iter()
            .position(|k| k.time > time)
            .unwrap_or(self.keyframes.len());
        
        self.keyframes.insert(pos, key);
        self.auto_tangents();
        
        id
    }

    /// Remove keyframe
    pub fn remove_keyframe(&mut self, id: u64) {
        self.keyframes.retain(|k| k.id != id);
        self.auto_tangents();
    }

    /// Move keyframe
    pub fn move_keyframe(&mut self, id: u64, new_time: f32, new_value: f32) {
        if let Some(key) = self.keyframes.iter_mut().find(|k| k.id == id) {
            key.time = new_time;
            key.value = new_value;
        }
        // Re-sort
        self.keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        self.auto_tangents();
    }

    /// Evaluate curve at time
    pub fn evaluate(&self, time: f32) -> f32 {
        if self.keyframes.is_empty() {
            return 0.0;
        }

        if self.keyframes.len() == 1 {
            return self.keyframes[0].value;
        }

        // Handle infinity
        let first = &self.keyframes[0];
        let last = self.keyframes.last().unwrap();

        if time <= first.time {
            return match self.pre_infinity {
                InfinityMode::Constant => first.value,
                InfinityMode::Linear => {
                    if self.keyframes.len() >= 2 {
                        let second = &self.keyframes[1];
                        let slope = (second.value - first.value) / (second.time - first.time);
                        first.value + slope * (time - first.time)
                    } else {
                        first.value
                    }
                }
                _ => first.value,
            };
        }

        if time >= last.time {
            return match self.post_infinity {
                InfinityMode::Constant => last.value,
                InfinityMode::Linear => {
                    if self.keyframes.len() >= 2 {
                        let prev = &self.keyframes[self.keyframes.len() - 2];
                        let slope = (last.value - prev.value) / (last.time - prev.time);
                        last.value + slope * (time - last.time)
                    } else {
                        last.value
                    }
                }
                InfinityMode::Cycle => {
                    let duration = last.time - first.time;
                    let t = first.time + ((time - first.time) % duration);
                    self.evaluate(t)
                }
                _ => last.value,
            };
        }

        // Find segment
        let i = self.keyframes.iter()
            .position(|k| k.time > time)
            .unwrap_or(self.keyframes.len()) - 1;

        let k0 = &self.keyframes[i];
        let k1 = &self.keyframes[i + 1];

        let t = (time - k0.time) / (k1.time - k0.time);

        match k0.interpolation {
            CurveInterpolation::Constant => k0.value,
            CurveInterpolation::Linear => k0.value + (k1.value - k0.value) * t,
            CurveInterpolation::Bezier => {
                self.eval_bezier(k0, k1, t)
            }
            CurveInterpolation::Hermite => {
                self.eval_hermite(k0, k1, t)
            }
            CurveInterpolation::TCB => {
                self.eval_hermite(k0, k1, t) // Simplified
            }
        }
    }

    fn eval_bezier(&self, k0: &Keyframe, k1: &Keyframe, t: f32) -> f32 {
        let p0 = k0.value;
        let p3 = k1.value;
        
        let dt = k1.time - k0.time;
        let tan0 = k0.out_tangent.angle.tan() * k0.out_tangent.weight * dt / 3.0;
        let tan1 = k1.in_tangent.angle.tan() * k1.in_tangent.weight * dt / 3.0;
        
        let p1 = p0 + tan0;
        let p2 = p3 - tan1;
        
        // Cubic bezier
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        
        mt3 * p0 + 3.0 * mt2 * t * p1 + 3.0 * mt * t2 * p2 + t3 * p3
    }

    fn eval_hermite(&self, k0: &Keyframe, k1: &Keyframe, t: f32) -> f32 {
        let p0 = k0.value;
        let p1 = k1.value;
        let m0 = k0.out_tangent.angle.tan() * (k1.time - k0.time);
        let m1 = k1.in_tangent.angle.tan() * (k1.time - k0.time);
        
        let t2 = t * t;
        let t3 = t2 * t;
        
        (2.0 * t3 - 3.0 * t2 + 1.0) * p0 +
        (t3 - 2.0 * t2 + t) * m0 +
        (-2.0 * t3 + 3.0 * t2) * p1 +
        (t3 - t2) * m1
    }

    /// Auto-calculate tangents for smooth curve
    pub fn auto_tangents(&mut self) {
        let len = self.keyframes.len();
        if len < 2 {
            return;
        }

        // First pass: collect values we need
        let values: Vec<f32> = self.keyframes.iter().map(|k| k.value).collect();

        // Second pass: update tangents
        for i in 0..len {
            let needs_in_auto = self.keyframes[i].in_tangent.mode == TangentMode::Auto;
            let needs_out_auto = self.keyframes[i].out_tangent.mode == TangentMode::Auto;
            
            if needs_in_auto || needs_out_auto {
                let prev_val = if i > 0 { values[i - 1] } else { values[i] };
                let next_val = if i < len - 1 { values[i + 1] } else { values[i] };
                
                // Average slope
                let slope = (next_val - prev_val) / 2.0;
                let angle = slope.atan();
                
                let key = &mut self.keyframes[i];
                if needs_in_auto {
                    key.in_tangent.angle = angle;
                }
                if needs_out_auto {
                    key.out_tangent.angle = angle;
                }
            }
        }
    }

    /// Select all keyframes
    pub fn select_all(&mut self) {
        for key in &mut self.keyframes {
            key.selected = true;
        }
    }

    /// Deselect all keyframes
    pub fn deselect_all(&mut self) {
        for key in &mut self.keyframes {
            key.selected = false;
        }
    }

    /// Get selected keyframes
    pub fn selected(&self) -> Vec<&Keyframe> {
        self.keyframes.iter().filter(|k| k.selected).collect()
    }

    /// Delete selected keyframes
    pub fn delete_selected(&mut self) {
        self.keyframes.retain(|k| !k.selected);
        self.auto_tangents();
    }

    /// Get time range
    pub fn time_range(&self) -> (f32, f32) {
        if self.keyframes.is_empty() {
            return (0.0, 1.0);
        }
        (
            self.keyframes.first().unwrap().time,
            self.keyframes.last().unwrap().time,
        )
    }

    /// Get value range
    pub fn value_range(&self) -> (f32, f32) {
        if self.keyframes.is_empty() {
            return (0.0, 1.0);
        }
        let min = self.keyframes.iter().map(|k| k.value).fold(f32::MAX, f32::min);
        let max = self.keyframes.iter().map(|k| k.value).fold(f32::MIN, f32::max);
        (min, max)
    }
}

// ==================== CURVE EDITOR ====================

/// Curve editor widget
pub struct CurveEditorWidget {
    /// All curves
    pub curves: Vec<AnimationCurve>,
    /// View state
    pub view: CurveViewState,
    /// Interaction state
    pub interaction: CurveInteractionState,
    /// Grid settings
    pub grid: CurveGridSettings,
    /// Selection box
    pub selection_box: Option<SelectionBox>,
    /// Active curve index
    pub active_curve: Option<usize>,
    /// Show curve list panel
    pub show_curve_list: bool,
    /// Show properties panel
    pub show_properties: bool,
    /// Current time (playhead)
    pub current_time: f32,
    /// Time range
    pub time_range: (f32, f32),
    /// Value range
    pub value_range: (f32, f32),
    /// Auto-fit view
    pub auto_fit: bool,
    /// Show tangent handles
    pub show_tangents: bool,
    /// Snap to grid
    pub snap_to_grid: bool,
    /// Snap time increment
    pub time_snap: f32,
    /// Snap value increment
    pub value_snap: f32,
    /// Undo stack
    undo_stack: Vec<CurveAction>,
    /// Redo stack
    redo_stack: Vec<CurveAction>,
    next_id: u64,
}

/// Curve view state
#[derive(Debug, Clone)]
pub struct CurveViewState {
    pub offset: Vec2,
    pub scale: Vec2,
    pub size: Vec2,
}

impl Default for CurveViewState {
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
            scale: Vec2::new(100.0, 100.0), // pixels per unit
            size: Vec2::new(800.0, 400.0),
        }
    }
}

impl CurveViewState {
    /// Time/value to screen position
    pub fn to_screen(&self, time: f32, value: f32) -> Vec2 {
        Vec2::new(
            (time - self.offset.x) * self.scale.x,
            self.size.y - (value - self.offset.y) * self.scale.y,
        )
    }

    /// Screen position to time/value
    pub fn from_screen(&self, pos: Vec2) -> (f32, f32) {
        (
            pos.x / self.scale.x + self.offset.x,
            (self.size.y - pos.y) / self.scale.y + self.offset.y,
        )
    }

    /// Zoom at point
    pub fn zoom(&mut self, center: Vec2, factor: Vec2) {
        let (time, value) = self.from_screen(center);
        self.scale *= factor;
        self.scale = self.scale.clamp(Vec2::splat(10.0), Vec2::splat(10000.0));
        let (new_time, new_value) = self.from_screen(center);
        self.offset.x += time - new_time;
        self.offset.y += value - new_value;
    }

    /// Pan view
    pub fn pan(&mut self, delta: Vec2) {
        self.offset.x -= delta.x / self.scale.x;
        self.offset.y += delta.y / self.scale.y;
    }

    /// Frame all curves
    pub fn frame_all(&mut self, curves: &[AnimationCurve]) {
        if curves.is_empty() {
            return;
        }

        let mut time_min = f32::MAX;
        let mut time_max = f32::MIN;
        let mut val_min = f32::MAX;
        let mut val_max = f32::MIN;

        for curve in curves {
            if curve.visible && !curve.keyframes.is_empty() {
                let (t0, t1) = curve.time_range();
                let (v0, v1) = curve.value_range();
                time_min = time_min.min(t0);
                time_max = time_max.max(t1);
                val_min = val_min.min(v0);
                val_max = val_max.max(v1);
            }
        }

        if time_min == f32::MAX {
            return;
        }

        // Add padding
        let time_pad = (time_max - time_min).max(0.1) * 0.1;
        let val_pad = (val_max - val_min).max(0.1) * 0.1;

        time_min -= time_pad;
        time_max += time_pad;
        val_min -= val_pad;
        val_max += val_pad;

        self.offset = Vec2::new(time_min, val_min);
        self.scale = Vec2::new(
            self.size.x / (time_max - time_min),
            self.size.y / (val_max - val_min),
        );
    }
}

/// Curve interaction state
#[derive(Debug, Clone, Default)]
pub struct CurveInteractionState {
    pub hovering_key: Option<(usize, u64)>, // (curve_idx, key_id)
    pub hovering_tangent: Option<(usize, u64, bool)>, // (curve_idx, key_id, is_out)
    pub dragging_keys: bool,
    pub dragging_tangent: Option<(usize, u64, bool)>,
    pub panning: bool,
    pub box_selecting: bool,
    pub drag_start: Vec2,
}

/// Curve grid settings
#[derive(Debug, Clone)]
pub struct CurveGridSettings {
    pub show_grid: bool,
    pub show_numbers: bool,
    pub major_color: Color,
    pub minor_color: Color,
    pub time_subdivisions: u32,
    pub value_subdivisions: u32,
}

impl Default for CurveGridSettings {
    fn default() -> Self {
        Self {
            show_grid: true,
            show_numbers: true,
            major_color: Color::rgba(255, 255, 255, 0.12),
            minor_color: Color::rgba(255, 255, 255, 0.04),
            time_subdivisions: 4,
            value_subdivisions: 4,
        }
    }
}

/// Selection box
#[derive(Debug, Clone)]
pub struct SelectionBox {
    pub start: Vec2,
    pub end: Vec2,
}

/// Curve action for undo
#[derive(Debug, Clone)]
pub enum CurveAction {
    AddKeyframe(usize, Keyframe),
    RemoveKeyframe(usize, u64, Keyframe),
    MoveKeyframes(Vec<(usize, u64, f32, f32, f32, f32)>), // (curve, key, old_t, old_v, new_t, new_v)
    ChangeTangent(usize, u64, bool, TangentData, TangentData), // (curve, key, is_out, old, new)
}

impl Default for CurveEditorWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl CurveEditorWidget {
    /// Create new curve editor
    pub fn new() -> Self {
        Self {
            curves: Vec::new(),
            view: CurveViewState::default(),
            interaction: CurveInteractionState::default(),
            grid: CurveGridSettings::default(),
            selection_box: None,
            active_curve: None,
            show_curve_list: true,
            show_properties: true,
            current_time: 0.0,
            time_range: (0.0, 5.0),
            value_range: (0.0, 1.0),
            auto_fit: true,
            show_tangents: true,
            snap_to_grid: false,
            time_snap: 0.1,
            value_snap: 0.1,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            next_id: 1,
        }
    }

    /// Add a new curve
    pub fn add_curve(&mut self, name: &str) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        let colors = [
            Color::hex("#ef4444"), // red
            Color::hex("#22c55e"), // green
            Color::hex("#3b82f6"), // blue
            Color::hex("#f59e0b"), // amber
            Color::hex("#a855f7"), // purple
            Color::hex("#ec4899"), // pink
        ];

        let curve = AnimationCurve {
            id,
            name: name.to_string(),
            keyframes: Vec::new(),
            color: colors[self.curves.len() % colors.len()],
            visible: true,
            locked: false,
            pre_infinity: InfinityMode::Constant,
            post_infinity: InfinityMode::Constant,
            next_id: 1,
        };

        self.curves.push(curve);
        self.active_curve = Some(self.curves.len() - 1);
        self.curves.len() - 1
    }

    /// Remove curve
    pub fn remove_curve(&mut self, index: usize) {
        if index < self.curves.len() {
            self.curves.remove(index);
            if let Some(active) = self.active_curve {
                if active >= self.curves.len() {
                    self.active_curve = if self.curves.is_empty() { None } else { Some(self.curves.len() - 1) };
                }
            }
        }
    }

    /// Add keyframe to active curve
    pub fn add_keyframe(&mut self, time: f32, value: f32) -> Option<u64> {
        if let Some(idx) = self.active_curve {
            let time = if self.snap_to_grid {
                (time / self.time_snap).round() * self.time_snap
            } else {
                time
            };
            let value = if self.snap_to_grid {
                (value / self.value_snap).round() * self.value_snap
            } else {
                value
            };

            let id = self.curves[idx].add_keyframe(time, value);
            
            // Record for undo
            if let Some(key) = self.curves[idx].keyframes.iter().find(|k| k.id == id).cloned() {
                self.undo_stack.push(CurveAction::AddKeyframe(idx, key));
                self.redo_stack.clear();
            }

            Some(id)
        } else {
            None
        }
    }

    /// Delete selected keyframes
    pub fn delete_selected(&mut self) {
        for curve in &mut self.curves {
            curve.delete_selected();
        }
    }

    /// Select all keyframes
    pub fn select_all(&mut self) {
        for curve in &mut self.curves {
            if curve.visible {
                curve.select_all();
            }
        }
    }

    /// Deselect all
    pub fn deselect_all(&mut self) {
        for curve in &mut self.curves {
            curve.deselect_all();
        }
    }

    /// Frame view to fit all curves
    pub fn frame_all(&mut self) {
        self.view.frame_all(&self.curves);
    }

    /// Evaluate all curves at current time
    pub fn evaluate_all(&self) -> HashMap<String, f32> {
        let mut values = HashMap::new();
        for curve in &self.curves {
            if curve.visible {
                values.insert(curve.name.clone(), curve.evaluate(self.current_time));
            }
        }
        values
    }

    /// Set playhead time
    pub fn set_time(&mut self, time: f32) {
        self.current_time = time.max(0.0);
    }

    /// Undo
    pub fn undo(&mut self) {
        // Implementation similar to graph editor
    }

    /// Redo
    pub fn redo(&mut self) {
        // Implementation similar to graph editor
    }

    /// Set interpolation for selected keyframes
    pub fn set_interpolation(&mut self, interp: CurveInterpolation) {
        for curve in &mut self.curves {
            for key in &mut curve.keyframes {
                if key.selected {
                    key.interpolation = interp;
                }
            }
            curve.auto_tangents();
        }
    }

    /// Set tangent mode for selected keyframes
    pub fn set_tangent_mode(&mut self, mode: TangentMode) {
        for curve in &mut self.curves {
            for key in &mut curve.keyframes {
                if key.selected {
                    key.in_tangent.mode = mode;
                    key.out_tangent.mode = mode;
                }
            }
            curve.auto_tangents();
        }
    }

    /// Flatten selected keyframes
    pub fn flatten_tangents(&mut self) {
        for curve in &mut self.curves {
            for key in &mut curve.keyframes {
                if key.selected {
                    key.in_tangent.angle = 0.0;
                    key.out_tangent.angle = 0.0;
                }
            }
        }
    }
}
