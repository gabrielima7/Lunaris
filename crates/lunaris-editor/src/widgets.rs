//! Advanced UI Widgets
//!
//! Complex widgets for animation curves, node graphs, and more.

use glam::Vec2;
use std::collections::HashMap;

// ==================== ANIMATION CURVE EDITOR ====================

/// Curve interpolation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurveInterpolation {
    /// Linear interpolation
    Linear,
    /// Constant (step)
    Constant,
    /// Cubic bezier
    Bezier,
    /// Hermite spline
    Hermite,
}

/// Curve tangent mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TangentMode {
    /// Free tangents
    Free,
    /// Aligned tangents (same direction)
    Aligned,
    /// Flat tangent
    Flat,
    /// Linear tangent
    Linear,
    /// Auto smooth
    Auto,
}

/// Keyframe in a curve
#[derive(Debug, Clone)]
pub struct Keyframe {
    /// Time position
    pub time: f32,
    /// Value
    pub value: f32,
    /// Interpolation to next key
    pub interpolation: CurveInterpolation,
    /// In tangent (for Bezier/Hermite)
    pub in_tangent: Vec2,
    /// Out tangent
    pub out_tangent: Vec2,
    /// Tangent mode
    pub tangent_mode: TangentMode,
    /// Is selected
    pub selected: bool,
}

impl Keyframe {
    /// Create linear keyframe
    #[must_use]
    pub fn linear(time: f32, value: f32) -> Self {
        Self {
            time,
            value,
            interpolation: CurveInterpolation::Linear,
            in_tangent: Vec2::new(-1.0, 0.0),
            out_tangent: Vec2::new(1.0, 0.0),
            tangent_mode: TangentMode::Linear,
            selected: false,
        }
    }

    /// Create bezier keyframe
    #[must_use]
    pub fn bezier(time: f32, value: f32, in_tan: Vec2, out_tan: Vec2) -> Self {
        Self {
            time,
            value,
            interpolation: CurveInterpolation::Bezier,
            in_tangent: in_tan,
            out_tangent: out_tan,
            tangent_mode: TangentMode::Free,
            selected: false,
        }
    }
}

/// Animation curve
#[derive(Debug, Clone)]
pub struct AnimationCurve {
    /// Curve name
    pub name: String,
    /// Keyframes (sorted by time)
    pub keys: Vec<Keyframe>,
    /// Curve color
    pub color: [f32; 4],
    /// Is visible
    pub visible: bool,
    /// Is locked
    pub locked: bool,
    /// Pre-infinity mode
    pub pre_infinity: InfinityMode,
    /// Post-infinity mode
    pub post_infinity: InfinityMode,
}

/// Infinity extrapolation mode
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
    /// Create new curve
    #[must_use]
    pub fn new(name: &str, color: [f32; 4]) -> Self {
        Self {
            name: name.to_string(),
            keys: Vec::new(),
            color,
            visible: true,
            locked: false,
            pre_infinity: InfinityMode::Constant,
            post_infinity: InfinityMode::Constant,
        }
    }

    /// Add keyframe
    pub fn add_key(&mut self, key: Keyframe) {
        // Insert sorted by time
        let idx = self.keys.partition_point(|k| k.time < key.time);
        self.keys.insert(idx, key);
    }

    /// Remove keyframe at index
    pub fn remove_key(&mut self, index: usize) {
        if index < self.keys.len() {
            self.keys.remove(index);
        }
    }

    /// Evaluate curve at time
    #[must_use]
    pub fn evaluate(&self, time: f32) -> f32 {
        if self.keys.is_empty() {
            return 0.0;
        }

        if self.keys.len() == 1 {
            return self.keys[0].value;
        }

        // Handle pre-infinity
        if time < self.keys[0].time {
            return match self.pre_infinity {
                InfinityMode::Constant => self.keys[0].value,
                InfinityMode::Linear => {
                    let slope = (self.keys[1].value - self.keys[0].value) 
                        / (self.keys[1].time - self.keys[0].time);
                    self.keys[0].value + slope * (time - self.keys[0].time)
                }
                _ => self.keys[0].value,
            };
        }

        // Handle post-infinity
        let last = self.keys.last().unwrap();
        if time >= last.time {
            return match self.post_infinity {
                InfinityMode::Constant => last.value,
                InfinityMode::Linear => {
                    let prev = &self.keys[self.keys.len() - 2];
                    let slope = (last.value - prev.value) / (last.time - prev.time);
                    last.value + slope * (time - last.time)
                }
                _ => last.value,
            };
        }

        // Find segment
        let idx = self.keys.partition_point(|k| k.time <= time);
        let idx = idx.saturating_sub(1);
        
        let k0 = &self.keys[idx];
        let k1 = &self.keys[idx + 1];
        let t = (time - k0.time) / (k1.time - k0.time);

        match k0.interpolation {
            CurveInterpolation::Constant => k0.value,
            CurveInterpolation::Linear => k0.value + (k1.value - k0.value) * t,
            CurveInterpolation::Bezier => {
                // Cubic bezier interpolation
                let p0 = k0.value;
                let p1 = k0.value + k0.out_tangent.y;
                let p2 = k1.value + k1.in_tangent.y;
                let p3 = k1.value;
                
                let t2 = t * t;
                let t3 = t2 * t;
                let mt = 1.0 - t;
                let mt2 = mt * mt;
                let mt3 = mt2 * mt;
                
                mt3 * p0 + 3.0 * mt2 * t * p1 + 3.0 * mt * t2 * p2 + t3 * p3
            }
            CurveInterpolation::Hermite => {
                // Hermite interpolation
                let p0 = k0.value;
                let p1 = k1.value;
                let m0 = k0.out_tangent.y * (k1.time - k0.time);
                let m1 = k1.in_tangent.y * (k1.time - k0.time);

                let t2 = t * t;
                let t3 = t2 * t;

                (2.0 * t3 - 3.0 * t2 + 1.0) * p0 +
                (t3 - 2.0 * t2 + t) * m0 +
                (-2.0 * t3 + 3.0 * t2) * p1 +
                (t3 - t2) * m1
            }
        }
    }

    /// Auto-calculate tangents
    pub fn auto_tangents(&mut self) {
        for i in 0..self.keys.len() {
            let prev = if i > 0 { Some(&self.keys[i - 1]) } else { None };
            let next = if i < self.keys.len() - 1 { Some(&self.keys[i + 1]) } else { None };

            let key = &mut self.keys[i];
            if key.tangent_mode != TangentMode::Auto {
                continue;
            }

            match (prev, next) {
                (Some(p), Some(n)) => {
                    let slope = (n.value - p.value) / (n.time - p.time);
                    key.in_tangent = Vec2::new(-0.3, -slope * 0.3);
                    key.out_tangent = Vec2::new(0.3, slope * 0.3);
                }
                (Some(p), None) => {
                    let slope = (key.value - p.value) / (key.time - p.time);
                    key.in_tangent = Vec2::new(-0.3, -slope * 0.3);
                    key.out_tangent = Vec2::new(0.3, slope * 0.3);
                }
                (None, Some(n)) => {
                    let slope = (n.value - key.value) / (n.time - key.time);
                    key.in_tangent = Vec2::new(-0.3, -slope * 0.3);
                    key.out_tangent = Vec2::new(0.3, slope * 0.3);
                }
                (None, None) => {
                    key.in_tangent = Vec2::new(-0.3, 0.0);
                    key.out_tangent = Vec2::new(0.3, 0.0);
                }
            }
        }
    }
}

/// Curve editor widget
pub struct CurveEditor {
    /// Curves being edited
    pub curves: Vec<AnimationCurve>,
    /// View bounds (time_min, time_max, value_min, value_max)
    pub view_bounds: (f32, f32, f32, f32),
    /// Widget bounds
    pub widget_bounds: Rect,
    /// Grid settings
    pub grid: CurveGridSettings,
    /// Selection
    pub selection: Vec<(usize, usize)>, // (curve_idx, key_idx)
    /// Snapping
    pub snap_to_grid: bool,
    /// Show tangent handles
    pub show_tangents: bool,
}

/// Curve grid settings
#[derive(Debug, Clone)]
pub struct CurveGridSettings {
    pub major_lines: u32,
    pub minor_subdivisions: u32,
    pub show_grid: bool,
    pub show_values: bool,
    pub time_snap: f32,
    pub value_snap: f32,
}

impl Default for CurveGridSettings {
    fn default() -> Self {
        Self {
            major_lines: 10,
            minor_subdivisions: 5,
            show_grid: true,
            show_values: true,
            time_snap: 0.1,
            value_snap: 0.1,
        }
    }
}

/// Rectangle for widgets
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for CurveEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl CurveEditor {
    /// Create new curve editor
    #[must_use]
    pub fn new() -> Self {
        Self {
            curves: Vec::new(),
            view_bounds: (0.0, 10.0, -1.0, 1.0),
            widget_bounds: Rect { x: 0.0, y: 0.0, width: 800.0, height: 400.0 },
            grid: CurveGridSettings::default(),
            selection: Vec::new(),
            snap_to_grid: true,
            show_tangents: true,
        }
    }

    /// Convert time to screen X
    #[must_use]
    pub fn time_to_screen(&self, time: f32) -> f32 {
        let (t_min, t_max, _, _) = self.view_bounds;
        let t = (time - t_min) / (t_max - t_min);
        self.widget_bounds.x + t * self.widget_bounds.width
    }

    /// Convert value to screen Y
    #[must_use]
    pub fn value_to_screen(&self, value: f32) -> f32 {
        let (_, _, v_min, v_max) = self.view_bounds;
        let t = (value - v_min) / (v_max - v_min);
        self.widget_bounds.y + self.widget_bounds.height - t * self.widget_bounds.height
    }

    /// Convert screen X to time
    #[must_use]
    pub fn screen_to_time(&self, x: f32) -> f32 {
        let (t_min, t_max, _, _) = self.view_bounds;
        let t = (x - self.widget_bounds.x) / self.widget_bounds.width;
        t_min + t * (t_max - t_min)
    }

    /// Convert screen Y to value
    #[must_use]
    pub fn screen_to_value(&self, y: f32) -> f32 {
        let (_, _, v_min, v_max) = self.view_bounds;
        let t = 1.0 - (y - self.widget_bounds.y) / self.widget_bounds.height;
        v_min + t * (v_max - v_min)
    }

    /// Add key at screen position
    pub fn add_key_at(&mut self, curve_idx: usize, screen_pos: Vec2) {
        if let Some(curve) = self.curves.get_mut(curve_idx) {
            let time = self.screen_to_time(screen_pos.x);
            let value = self.screen_to_value(screen_pos.y);
            
            let time = if self.snap_to_grid {
                (time / self.grid.time_snap).round() * self.grid.time_snap
            } else {
                time
            };
            
            curve.add_key(Keyframe::linear(time, value));
        }
    }

    /// Zoom view
    pub fn zoom(&mut self, factor: f32, center: Vec2) {
        let time = self.screen_to_time(center.x);
        let value = self.screen_to_value(center.y);

        let (t_min, t_max, v_min, v_max) = self.view_bounds;
        let t_range = (t_max - t_min) * factor;
        let v_range = (v_max - v_min) * factor;

        let t_ratio = (time - t_min) / (t_max - t_min);
        let v_ratio = (value - v_min) / (v_max - v_min);

        self.view_bounds = (
            time - t_range * t_ratio,
            time + t_range * (1.0 - t_ratio),
            value - v_range * v_ratio,
            value + v_range * (1.0 - v_ratio),
        );
    }

    /// Pan view
    pub fn pan(&mut self, delta: Vec2) {
        let (t_min, t_max, v_min, v_max) = self.view_bounds;
        let t_per_pixel = (t_max - t_min) / self.widget_bounds.width;
        let v_per_pixel = (v_max - v_min) / self.widget_bounds.height;

        self.view_bounds = (
            t_min - delta.x * t_per_pixel,
            t_max - delta.x * t_per_pixel,
            v_min + delta.y * v_per_pixel,
            v_max + delta.y * v_per_pixel,
        );
    }

    /// Frame all curves
    pub fn frame_all(&mut self) {
        if self.curves.is_empty() {
            return;
        }

        let mut t_min = f32::MAX;
        let mut t_max = f32::MIN;
        let mut v_min = f32::MAX;
        let mut v_max = f32::MIN;

        for curve in &self.curves {
            for key in &curve.keys {
                t_min = t_min.min(key.time);
                t_max = t_max.max(key.time);
                v_min = v_min.min(key.value);
                v_max = v_max.max(key.value);
            }
        }

        // Add padding
        let t_pad = (t_max - t_min) * 0.1;
        let v_pad = (v_max - v_min) * 0.1;

        self.view_bounds = (
            t_min - t_pad,
            t_max + t_pad,
            v_min - v_pad,
            v_max + v_pad,
        );
    }
}

// ==================== NODE GRAPH EDITOR ====================

/// Node in a visual graph
#[derive(Debug, Clone)]
pub struct GraphNode {
    /// Unique ID
    pub id: u64,
    /// Node type
    pub node_type: String,
    /// Display name
    pub name: String,
    /// Position
    pub position: Vec2,
    /// Size
    pub size: Vec2,
    /// Input pins
    pub inputs: Vec<Pin>,
    /// Output pins
    pub outputs: Vec<Pin>,
    /// Is selected
    pub selected: bool,
    /// Is collapsed
    pub collapsed: bool,
    /// Color
    pub color: [f32; 4],
    /// Custom data
    pub data: HashMap<String, PinValue>,
}

/// Pin on a node
#[derive(Debug, Clone)]
pub struct Pin {
    /// Pin ID
    pub id: u64,
    /// Pin name
    pub name: String,
    /// Pin type
    pub pin_type: PinType,
    /// Is connected
    pub connected: bool,
    /// Default value
    pub default: Option<PinValue>,
}

/// Pin type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinType {
    /// Execution flow
    Exec,
    /// Boolean
    Bool,
    /// Integer
    Int,
    /// Float
    Float,
    /// Vector2
    Vec2,
    /// Vector3
    Vec3,
    /// Vector4/Color
    Vec4,
    /// Texture
    Texture,
    /// Object reference
    Object,
    /// Any type
    Any,
}

impl PinType {
    /// Get color for pin type
    #[must_use]
    pub fn color(&self) -> [f32; 4] {
        match self {
            Self::Exec => [1.0, 1.0, 1.0, 1.0],
            Self::Bool => [0.9, 0.2, 0.2, 1.0],
            Self::Int => [0.2, 0.9, 0.9, 1.0],
            Self::Float => [0.2, 0.9, 0.2, 1.0],
            Self::Vec2 => [0.9, 0.7, 0.2, 1.0],
            Self::Vec3 => [0.9, 0.9, 0.2, 1.0],
            Self::Vec4 => [0.9, 0.2, 0.9, 1.0],
            Self::Texture => [0.9, 0.4, 0.2, 1.0],
            Self::Object => [0.2, 0.4, 0.9, 1.0],
            Self::Any => [0.7, 0.7, 0.7, 1.0],
        }
    }

    /// Check if types are compatible
    #[must_use]
    pub fn is_compatible(&self, other: &PinType) -> bool {
        if *self == PinType::Any || *other == PinType::Any {
            return true;
        }
        // Allow numeric conversions
        let is_numeric = |t: &PinType| matches!(t, PinType::Bool | PinType::Int | PinType::Float);
        if is_numeric(self) && is_numeric(other) {
            return true;
        }
        // Allow vector conversions
        let is_vector = |t: &PinType| matches!(t, PinType::Vec2 | PinType::Vec3 | PinType::Vec4);
        if is_vector(self) && is_vector(other) {
            return true;
        }
        *self == *other
    }
}

/// Pin value
#[derive(Debug, Clone)]
pub enum PinValue {
    Bool(bool),
    Int(i32),
    Float(f32),
    Vec2(Vec2),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    String(String),
    Object(u64),
}

/// Connection between pins
#[derive(Debug, Clone)]
pub struct Connection {
    /// Connection ID
    pub id: u64,
    /// Source node ID
    pub from_node: u64,
    /// Source pin ID
    pub from_pin: u64,
    /// Target node ID
    pub to_node: u64,
    /// Target pin ID
    pub to_pin: u64,
}

/// Visual node graph
pub struct NodeGraph {
    /// Graph name
    pub name: String,
    /// Nodes
    pub nodes: HashMap<u64, GraphNode>,
    /// Connections
    pub connections: Vec<Connection>,
    /// Next ID
    next_id: u64,
    /// View offset
    pub view_offset: Vec2,
    /// View zoom
    pub view_zoom: f32,
    /// Selected nodes
    pub selection: Vec<u64>,
    /// Dragging state
    dragging: Option<DragState>,
    /// Connecting state
    connecting: Option<ConnectState>,
}

/// Drag state
#[derive(Debug, Clone)]
struct DragState {
    node_ids: Vec<u64>,
    start_positions: Vec<Vec2>,
}

/// Connect state
#[derive(Debug, Clone)]
struct ConnectState {
    from_node: u64,
    from_pin: u64,
    is_output: bool,
    mouse_pos: Vec2,
}

impl Default for NodeGraph {
    fn default() -> Self {
        Self::new("Untitled")
    }
}

impl NodeGraph {
    /// Create new graph
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            nodes: HashMap::new(),
            connections: Vec::new(),
            next_id: 1,
            view_offset: Vec2::ZERO,
            view_zoom: 1.0,
            selection: Vec::new(),
            dragging: None,
            connecting: None,
        }
    }

    /// Get next ID
    fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Create node
    pub fn create_node(&mut self, node_type: &str, name: &str, position: Vec2) -> u64 {
        let id = self.next_id();
        
        let node = GraphNode {
            id,
            node_type: node_type.to_string(),
            name: name.to_string(),
            position,
            size: Vec2::new(200.0, 100.0),
            inputs: Vec::new(),
            outputs: Vec::new(),
            selected: false,
            collapsed: false,
            color: [0.2, 0.2, 0.2, 1.0],
            data: HashMap::new(),
        };

        self.nodes.insert(id, node);
        id
    }

    /// Add input pin to node
    pub fn add_input(&mut self, node_id: u64, name: &str, pin_type: PinType) -> Option<u64> {
        let pin_id = self.next_id();
        
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.inputs.push(Pin {
                id: pin_id,
                name: name.to_string(),
                pin_type,
                connected: false,
                default: None,
            });
            Some(pin_id)
        } else {
            None
        }
    }

    /// Add output pin to node
    pub fn add_output(&mut self, node_id: u64, name: &str, pin_type: PinType) -> Option<u64> {
        let pin_id = self.next_id();
        
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.outputs.push(Pin {
                id: pin_id,
                name: name.to_string(),
                pin_type,
                connected: false,
                default: None,
            });
            Some(pin_id)
        } else {
            None
        }
    }

    /// Connect pins
    pub fn connect(&mut self, from_node: u64, from_pin: u64, to_node: u64, to_pin: u64) -> Option<u64> {
        // Validate connection
        let from = self.nodes.get(&from_node)?;
        let to = self.nodes.get(&to_node)?;

        let from_type = from.outputs.iter().find(|p| p.id == from_pin)?.pin_type;
        let to_type = to.inputs.iter().find(|p| p.id == to_pin)?.pin_type;

        if !from_type.is_compatible(&to_type) {
            return None;
        }

        // Check for existing connection to same input
        self.connections.retain(|c| c.to_node != to_node || c.to_pin != to_pin);

        let id = self.next_id();
        self.connections.push(Connection {
            id,
            from_node,
            from_pin,
            to_node,
            to_pin,
        });

        // Mark pins as connected
        if let Some(node) = self.nodes.get_mut(&from_node) {
            if let Some(pin) = node.outputs.iter_mut().find(|p| p.id == from_pin) {
                pin.connected = true;
            }
        }
        if let Some(node) = self.nodes.get_mut(&to_node) {
            if let Some(pin) = node.inputs.iter_mut().find(|p| p.id == to_pin) {
                pin.connected = true;
            }
        }

        Some(id)
    }

    /// Disconnect
    pub fn disconnect(&mut self, connection_id: u64) {
        if let Some(idx) = self.connections.iter().position(|c| c.id == connection_id) {
            let conn = self.connections.remove(idx);
            
            // Check if pins still have connections
            let from_still_connected = self.connections.iter()
                .any(|c| c.from_node == conn.from_node && c.from_pin == conn.from_pin);
            let to_still_connected = self.connections.iter()
                .any(|c| c.to_node == conn.to_node && c.to_pin == conn.to_pin);

            if !from_still_connected {
                if let Some(node) = self.nodes.get_mut(&conn.from_node) {
                    if let Some(pin) = node.outputs.iter_mut().find(|p| p.id == conn.from_pin) {
                        pin.connected = false;
                    }
                }
            }
            if !to_still_connected {
                if let Some(node) = self.nodes.get_mut(&conn.to_node) {
                    if let Some(pin) = node.inputs.iter_mut().find(|p| p.id == conn.to_pin) {
                        pin.connected = false;
                    }
                }
            }
        }
    }

    /// Delete node
    pub fn delete_node(&mut self, node_id: u64) {
        // Remove all connections
        self.connections.retain(|c| c.from_node != node_id && c.to_node != node_id);
        
        // Remove node
        self.nodes.remove(&node_id);
        self.selection.retain(|&id| id != node_id);
    }

    /// Select node
    pub fn select(&mut self, node_id: u64, add_to_selection: bool) {
        if !add_to_selection {
            for node in self.nodes.values_mut() {
                node.selected = false;
            }
            self.selection.clear();
        }

        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.selected = true;
            if !self.selection.contains(&node_id) {
                self.selection.push(node_id);
            }
        }
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        for node in self.nodes.values_mut() {
            node.selected = false;
        }
        self.selection.clear();
    }

    /// Start drag
    pub fn start_drag(&mut self, node_ids: Vec<u64>) {
        let start_positions: Vec<Vec2> = node_ids.iter()
            .filter_map(|id| self.nodes.get(id).map(|n| n.position))
            .collect();

        self.dragging = Some(DragState { node_ids, start_positions });
    }

    /// Update drag
    pub fn update_drag(&mut self, delta: Vec2) {
        if let Some(ref drag) = self.dragging {
            for (i, &node_id) in drag.node_ids.iter().enumerate() {
                if let Some(node) = self.nodes.get_mut(&node_id) {
                    node.position = drag.start_positions[i] + delta / self.view_zoom;
                }
            }
        }
    }

    /// End drag
    pub fn end_drag(&mut self) {
        self.dragging = None;
    }

    /// World to screen position
    #[must_use]
    pub fn world_to_screen(&self, world: Vec2) -> Vec2 {
        (world + self.view_offset) * self.view_zoom
    }

    /// Screen to world position
    #[must_use]
    pub fn screen_to_world(&self, screen: Vec2) -> Vec2 {
        screen / self.view_zoom - self.view_offset
    }

    /// Zoom
    pub fn zoom(&mut self, factor: f32, center: Vec2) {
        let world_before = self.screen_to_world(center);
        self.view_zoom = (self.view_zoom * factor).clamp(0.1, 4.0);
        let world_after = self.screen_to_world(center);
        self.view_offset += world_after - world_before;
    }

    /// Pan
    pub fn pan(&mut self, delta: Vec2) {
        self.view_offset += delta / self.view_zoom;
    }

    /// Frame all nodes
    pub fn frame_all(&mut self) {
        if self.nodes.is_empty() {
            return;
        }

        let mut min = Vec2::splat(f32::MAX);
        let mut max = Vec2::splat(f32::MIN);

        for node in self.nodes.values() {
            min = min.min(node.position);
            max = max.max(node.position + node.size);
        }

        let center = (min + max) / 2.0;
        self.view_offset = -center;
    }
}

/// Create common VFX node
pub fn create_vfx_node(graph: &mut NodeGraph, node_type: &str, position: Vec2) -> u64 {
    let id = match node_type {
        "Spawn" => {
            let id = graph.create_node("Spawn", "Spawn Rate", position);
            graph.add_output(id, "Particles", PinType::Object);
            graph.add_input(id, "Rate", PinType::Float);
            graph.add_input(id, "Burst", PinType::Int);
            id
        }
        "Velocity" => {
            let id = graph.create_node("Velocity", "Initial Velocity", position);
            graph.add_input(id, "Particles", PinType::Object);
            graph.add_output(id, "Particles", PinType::Object);
            graph.add_input(id, "Direction", PinType::Vec3);
            graph.add_input(id, "Speed", PinType::Float);
            graph.add_input(id, "Randomness", PinType::Float);
            id
        }
        "Color" => {
            let id = graph.create_node("Color", "Color Over Life", position);
            graph.add_input(id, "Particles", PinType::Object);
            graph.add_output(id, "Particles", PinType::Object);
            graph.add_input(id, "Start Color", PinType::Vec4);
            graph.add_input(id, "End Color", PinType::Vec4);
            id
        }
        "Size" => {
            let id = graph.create_node("Size", "Size Over Life", position);
            graph.add_input(id, "Particles", PinType::Object);
            graph.add_output(id, "Particles", PinType::Object);
            graph.add_input(id, "Start Size", PinType::Float);
            graph.add_input(id, "End Size", PinType::Float);
            id
        }
        "Render" => {
            let id = graph.create_node("Render", "Sprite Renderer", position);
            graph.add_input(id, "Particles", PinType::Object);
            graph.add_input(id, "Texture", PinType::Texture);
            graph.add_input(id, "Blend Mode", PinType::Int);
            id
        }
        _ => graph.create_node(node_type, node_type, position),
    };

    id
}
