//! Visual Graph Editor
//!
//! Complete node-based editor for Blueprints, Materials, VFX, and AI.
//! Integrates with the dock system and retained UI.

use glam::Vec2;
use std::collections::HashMap;
use super::design_system::*;

// ==================== GRAPH TYPES ====================

/// Type of visual graph
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphType {
    /// Blueprint visual scripting
    Blueprint,
    /// Material shader graph
    Material,
    /// VFX/Particle graph
    Vfx,
    /// Animation state machine
    AnimationStateMachine,
    /// Animation blend tree
    AnimationBlendTree,
    /// AI behavior tree
    BehaviorTree,
    /// Audio/Sound graph
    Audio,
    /// Generic data flow
    DataFlow,
}

impl GraphType {
    /// Get node palette for this graph type
    pub fn node_palette(&self) -> Vec<NodeCategory> {
        match self {
            Self::Blueprint => vec![
                NodeCategory {
                    name: "Flow Control".to_string(),
                    nodes: vec![
                        NodeTemplate::new("Branch", "if_then"),
                        NodeTemplate::new("Sequence", "sequence"),
                        NodeTemplate::new("For Loop", "for_loop"),
                        NodeTemplate::new("While Loop", "while_loop"),
                        NodeTemplate::new("Switch", "switch"),
                        NodeTemplate::new("Gate", "gate"),
                        NodeTemplate::new("Delay", "delay"),
                    ],
                },
                NodeCategory {
                    name: "Events".to_string(),
                    nodes: vec![
                        NodeTemplate::new("Begin Play", "event_begin_play"),
                        NodeTemplate::new("Tick", "event_tick"),
                        NodeTemplate::new("On Overlap", "event_overlap"),
                        NodeTemplate::new("On Hit", "event_hit"),
                        NodeTemplate::new("Input Action", "event_input"),
                        NodeTemplate::new("Custom Event", "event_custom"),
                    ],
                },
                NodeCategory {
                    name: "Math".to_string(),
                    nodes: vec![
                        NodeTemplate::new("Add", "math_add"),
                        NodeTemplate::new("Subtract", "math_sub"),
                        NodeTemplate::new("Multiply", "math_mul"),
                        NodeTemplate::new("Divide", "math_div"),
                        NodeTemplate::new("Lerp", "math_lerp"),
                        NodeTemplate::new("Clamp", "math_clamp"),
                        NodeTemplate::new("Random", "math_random"),
                    ],
                },
                NodeCategory {
                    name: "Variables".to_string(),
                    nodes: vec![
                        NodeTemplate::new("Get Variable", "var_get"),
                        NodeTemplate::new("Set Variable", "var_set"),
                        NodeTemplate::new("Make Struct", "make_struct"),
                        NodeTemplate::new("Break Struct", "break_struct"),
                    ],
                },
            ],
            Self::Material => vec![
                NodeCategory {
                    name: "Texture".to_string(),
                    nodes: vec![
                        NodeTemplate::new("Texture Sample", "tex_sample"),
                        NodeTemplate::new("Texture Object", "tex_object"),
                        NodeTemplate::new("Texture Coords", "tex_coords"),
                        NodeTemplate::new("Panner", "tex_panner"),
                        NodeTemplate::new("Rotator", "tex_rotator"),
                    ],
                },
                NodeCategory {
                    name: "Math".to_string(),
                    nodes: vec![
                        NodeTemplate::new("Add", "math_add"),
                        NodeTemplate::new("Multiply", "math_mul"),
                        NodeTemplate::new("Lerp", "math_lerp"),
                        NodeTemplate::new("Clamp", "math_clamp"),
                        NodeTemplate::new("Dot", "math_dot"),
                        NodeTemplate::new("Cross", "math_cross"),
                        NodeTemplate::new("Normalize", "math_normalize"),
                        NodeTemplate::new("Power", "math_power"),
                    ],
                },
                NodeCategory {
                    name: "Utility".to_string(),
                    nodes: vec![
                        NodeTemplate::new("Fresnel", "util_fresnel"),
                        NodeTemplate::new("Time", "util_time"),
                        NodeTemplate::new("Camera Position", "util_cam_pos"),
                        NodeTemplate::new("Vertex Normal", "util_vertex_normal"),
                        NodeTemplate::new("World Position", "util_world_pos"),
                    ],
                },
                NodeCategory {
                    name: "Constants".to_string(),
                    nodes: vec![
                        NodeTemplate::new("Constant", "const_scalar"),
                        NodeTemplate::new("Constant2", "const_vec2"),
                        NodeTemplate::new("Constant3", "const_vec3"),
                        NodeTemplate::new("Constant4", "const_vec4"),
                        NodeTemplate::new("Color", "const_color"),
                    ],
                },
            ],
            Self::Vfx => vec![
                NodeCategory {
                    name: "Spawn".to_string(),
                    nodes: vec![
                        NodeTemplate::new("Spawn Rate", "spawn_rate"),
                        NodeTemplate::new("Spawn Burst", "spawn_burst"),
                        NodeTemplate::new("Spawn Per Unit", "spawn_per_unit"),
                    ],
                },
                NodeCategory {
                    name: "Initialize".to_string(),
                    nodes: vec![
                        NodeTemplate::new("Set Position", "init_position"),
                        NodeTemplate::new("Set Velocity", "init_velocity"),
                        NodeTemplate::new("Set Lifetime", "init_lifetime"),
                        NodeTemplate::new("Set Size", "init_size"),
                        NodeTemplate::new("Set Color", "init_color"),
                        NodeTemplate::new("Set Rotation", "init_rotation"),
                    ],
                },
                NodeCategory {
                    name: "Update".to_string(),
                    nodes: vec![
                        NodeTemplate::new("Gravity", "update_gravity"),
                        NodeTemplate::new("Drag", "update_drag"),
                        NodeTemplate::new("Curl Noise", "update_curl_noise"),
                        NodeTemplate::new("Orbit", "update_orbit"),
                        NodeTemplate::new("Collision", "update_collision"),
                    ],
                },
                NodeCategory {
                    name: "Output".to_string(),
                    nodes: vec![
                        NodeTemplate::new("Sprite Renderer", "render_sprite"),
                        NodeTemplate::new("Mesh Renderer", "render_mesh"),
                        NodeTemplate::new("Trail Renderer", "render_trail"),
                        NodeTemplate::new("Light Renderer", "render_light"),
                    ],
                },
            ],
            _ => vec![],
        }
    }

    /// Get output node for this graph type
    pub fn output_node(&self) -> Option<NodeTemplate> {
        match self {
            Self::Material => Some(NodeTemplate::new("Material Output", "material_output")),
            Self::Vfx => Some(NodeTemplate::new("Particle Output", "particle_output")),
            Self::Audio => Some(NodeTemplate::new("Audio Output", "audio_output")),
            _ => None,
        }
    }
}

/// Node category for palette
#[derive(Debug, Clone)]
pub struct NodeCategory {
    pub name: String,
    pub nodes: Vec<NodeTemplate>,
}

/// Node template
#[derive(Debug, Clone)]
pub struct NodeTemplate {
    pub display_name: String,
    pub type_id: String,
}

impl NodeTemplate {
    pub fn new(name: &str, type_id: &str) -> Self {
        Self {
            display_name: name.to_string(),
            type_id: type_id.to_string(),
        }
    }
}

// ==================== GRAPH EDITOR ====================

/// Visual graph editor widget
pub struct VisualGraphEditor {
    /// Graph type
    pub graph_type: GraphType,
    /// Graph name
    pub name: String,
    /// All nodes
    pub nodes: HashMap<u64, GraphNode>,
    /// All connections
    pub connections: Vec<Connection>,
    /// Next node ID
    next_id: u64,
    /// View transform
    pub view: ViewTransform,
    /// Selection state
    pub selection: SelectionState,
    /// Interaction state
    pub interaction: InteractionState,
    /// Grid settings
    pub grid: GridSettings,
    /// Minimap settings
    pub minimap: MinimapSettings,
    /// Undo stack
    undo_stack: Vec<GraphAction>,
    /// Redo stack
    redo_stack: Vec<GraphAction>,
    /// Is dirty (has unsaved changes)
    pub is_dirty: bool,
}

/// Graph node
#[derive(Debug, Clone)]
pub struct GraphNode {
    pub id: u64,
    pub type_id: String,
    pub display_name: String,
    pub position: Vec2,
    pub size: Vec2,
    pub inputs: Vec<Pin>,
    pub outputs: Vec<Pin>,
    pub color: Color,
    pub collapsed: bool,
    pub comment: Option<String>,
    pub custom_data: HashMap<String, PinValue>,
}

/// Pin (input/output connector)
#[derive(Debug, Clone)]
pub struct Pin {
    pub id: u64,
    pub name: String,
    pub pin_type: PinType,
    pub is_connected: bool,
    pub default_value: Option<PinValue>,
    pub hidden: bool,
}

/// Pin type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinType {
    Exec,
    Bool,
    Int,
    Float,
    Vector2,
    Vector3,
    Vector4,
    Color,
    Texture,
    Object,
    Struct(u64),
    Any,
}

impl PinType {
    /// Get color for this pin type
    pub fn color(&self) -> Color {
        match self {
            Self::Exec => Color::hex("#ffffff"),
            Self::Bool => Color::hex("#dc2626"),
            Self::Int => Color::hex("#22c55e"),
            Self::Float => Color::hex("#3b82f6"),
            Self::Vector2 => Color::hex("#f59e0b"),
            Self::Vector3 => Color::hex("#eab308"),
            Self::Vector4 => Color::hex("#a855f7"),
            Self::Color => Color::hex("#ec4899"),
            Self::Texture => Color::hex("#f97316"),
            Self::Object => Color::hex("#6366f1"),
            Self::Struct(_) => Color::hex("#06b6d4"),
            Self::Any => Color::hex("#6b7280"),
        }
    }

    /// Check if compatible with other type
    pub fn is_compatible(&self, other: &PinType) -> bool {
        if *self == PinType::Any || *other == PinType::Any {
            return true;
        }
        if *self == *other {
            return true;
        }
        // Implicit conversions
        match (self, other) {
            (Self::Int, Self::Float) | (Self::Float, Self::Int) => true,
            (Self::Vector3, Self::Vector4) | (Self::Vector4, Self::Vector3) => true,
            (Self::Vector3, Self::Color) | (Self::Color, Self::Vector3) => true,
            (Self::Vector4, Self::Color) | (Self::Color, Self::Vector4) => true,
            _ => false,
        }
    }
}

/// Pin value
#[derive(Debug, Clone)]
pub enum PinValue {
    Bool(bool),
    Int(i32),
    Float(f32),
    Vector2([f32; 2]),
    Vector3([f32; 3]),
    Vector4([f32; 4]),
    Color([f32; 4]),
    String(String),
    Object(Option<u64>),
}

/// Connection between pins
#[derive(Debug, Clone)]
pub struct Connection {
    pub id: u64,
    pub from_node: u64,
    pub from_pin: u64,
    pub to_node: u64,
    pub to_pin: u64,
}

/// View transform
#[derive(Debug, Clone)]
pub struct ViewTransform {
    pub offset: Vec2,
    pub zoom: f32,
    pub zoom_min: f32,
    pub zoom_max: f32,
}

impl Default for ViewTransform {
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
            zoom: 1.0,
            zoom_min: 0.1,
            zoom_max: 4.0,
        }
    }
}

impl ViewTransform {
    pub fn world_to_screen(&self, world: Vec2) -> Vec2 {
        (world + self.offset) * self.zoom
    }

    pub fn screen_to_world(&self, screen: Vec2) -> Vec2 {
        screen / self.zoom - self.offset
    }

    pub fn zoom_at(&mut self, center: Vec2, factor: f32) {
        let world_before = self.screen_to_world(center);
        self.zoom = (self.zoom * factor).clamp(self.zoom_min, self.zoom_max);
        let world_after = self.screen_to_world(center);
        self.offset += world_after - world_before;
    }

    pub fn pan(&mut self, delta: Vec2) {
        self.offset += delta / self.zoom;
    }
}

/// Selection state
#[derive(Debug, Clone, Default)]
pub struct SelectionState {
    pub selected_nodes: Vec<u64>,
    pub selected_connections: Vec<u64>,
    pub marquee: Option<MarqueeSelection>,
}

/// Marquee selection
#[derive(Debug, Clone)]
pub struct MarqueeSelection {
    pub start: Vec2,
    pub end: Vec2,
}

/// Interaction state
#[derive(Debug, Clone, Default)]
pub struct InteractionState {
    pub hovering_node: Option<u64>,
    pub hovering_pin: Option<(u64, u64, bool)>, // (node_id, pin_id, is_output)
    pub dragging_nodes: Option<DragState>,
    pub connecting: Option<ConnectionDrag>,
    pub panning: bool,
    pub context_menu: Option<ContextMenu>,
    pub search_box: Option<SearchBox>,
}

/// Drag state for nodes
#[derive(Debug, Clone)]
pub struct DragState {
    pub node_ids: Vec<u64>,
    pub start_positions: Vec<Vec2>,
    pub start_mouse: Vec2,
}

/// Connection drag state
#[derive(Debug, Clone)]
pub struct ConnectionDrag {
    pub from_node: u64,
    pub from_pin: u64,
    pub is_output: bool,
    pub mouse_pos: Vec2,
}

/// Context menu
#[derive(Debug, Clone)]
pub struct ContextMenu {
    pub position: Vec2,
    pub items: Vec<ContextMenuItem>,
    pub search_query: String,
    pub filtered_items: Vec<usize>,
}

/// Context menu item
#[derive(Debug, Clone)]
pub struct ContextMenuItem {
    pub label: String,
    pub action: String,
    pub icon: Option<String>,
    pub shortcut: Option<String>,
    pub submenu: Option<Vec<ContextMenuItem>>,
    pub enabled: bool,
}

/// Search box for node creation
#[derive(Debug, Clone)]
pub struct SearchBox {
    pub position: Vec2,
    pub query: String,
    pub results: Vec<NodeTemplate>,
    pub selected_index: usize,
}

/// Grid settings
#[derive(Debug, Clone)]
pub struct GridSettings {
    pub visible: bool,
    pub size: f32,
    pub subdivision: u32,
    pub snap_enabled: bool,
    pub snap_size: f32,
    pub major_color: Color,
    pub minor_color: Color,
}

impl Default for GridSettings {
    fn default() -> Self {
        Self {
            visible: true,
            size: 100.0,
            subdivision: 4,
            snap_enabled: true,
            snap_size: 16.0,
            major_color: Color::rgba(255, 255, 255, 20),
            minor_color: Color::rgba(255, 255, 255, 8),
        }
    }
}

/// Minimap settings
#[derive(Debug, Clone)]
pub struct MinimapSettings {
    pub visible: bool,
    pub position: MinimapPosition,
    pub size: Vec2,
    pub opacity: f32,
}

/// Minimap position
#[derive(Debug, Clone, Copy)]
pub enum MinimapPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Default for MinimapSettings {
    fn default() -> Self {
        Self {
            visible: true,
            position: MinimapPosition::BottomRight,
            size: Vec2::new(200.0, 150.0),
            opacity: 0.8,
        }
    }
}

/// Graph action for undo/redo
#[derive(Debug, Clone)]
pub enum GraphAction {
    AddNode(GraphNode),
    RemoveNode(u64, GraphNode),
    MoveNodes(Vec<(u64, Vec2, Vec2)>), // (id, old_pos, new_pos)
    AddConnection(Connection),
    RemoveConnection(u64, Connection),
    ChangeNodeProperty(u64, String, PinValue, PinValue), // (node_id, prop_name, old, new)
}

impl Default for VisualGraphEditor {
    fn default() -> Self {
        Self::new(GraphType::Blueprint, "Untitled")
    }
}

impl VisualGraphEditor {
    /// Create new graph editor
    pub fn new(graph_type: GraphType, name: &str) -> Self {
        let mut editor = Self {
            graph_type,
            name: name.to_string(),
            nodes: HashMap::new(),
            connections: Vec::new(),
            next_id: 1,
            view: ViewTransform::default(),
            selection: SelectionState::default(),
            interaction: InteractionState::default(),
            grid: GridSettings::default(),
            minimap: MinimapSettings::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            is_dirty: false,
        };

        // Add output node if applicable
        if let Some(template) = graph_type.output_node() {
            editor.create_node(&template.type_id, Vec2::new(400.0, 0.0));
        }

        editor
    }

    fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Create a node from type ID
    pub fn create_node(&mut self, type_id: &str, position: Vec2) -> u64 {
        let id = self.next_id();
        
        let position = if self.grid.snap_enabled {
            Vec2::new(
                (position.x / self.grid.snap_size).round() * self.grid.snap_size,
                (position.y / self.grid.snap_size).round() * self.grid.snap_size,
            )
        } else {
            position
        };

        let (display_name, inputs, outputs, color) = self.get_node_definition(type_id);

        let node = GraphNode {
            id,
            type_id: type_id.to_string(),
            display_name,
            position,
            size: Vec2::new(180.0, 100.0),
            inputs,
            outputs,
            color,
            collapsed: false,
            comment: None,
            custom_data: HashMap::new(),
        };

        self.nodes.insert(id, node.clone());
        self.undo_stack.push(GraphAction::AddNode(node));
        self.redo_stack.clear();
        self.is_dirty = true;

        id
    }

    fn get_node_definition(&mut self, type_id: &str) -> (String, Vec<Pin>, Vec<Pin>, Color) {
        let make_pin = |id: u64, name: &str, pin_type: PinType| Pin {
            id,
            name: name.to_string(),
            pin_type,
            is_connected: false,
            default_value: None,
            hidden: false,
        };

        match type_id {
            // Blueprint nodes
            "event_begin_play" => (
                "Event Begin Play".to_string(),
                vec![],
                vec![make_pin(self.next_id(), "", PinType::Exec)],
                Color::hex("#dc2626"),
            ),
            "event_tick" => (
                "Event Tick".to_string(),
                vec![],
                vec![
                    make_pin(self.next_id(), "", PinType::Exec),
                    make_pin(self.next_id(), "Delta Time", PinType::Float),
                ],
                Color::hex("#dc2626"),
            ),
            "if_then" => (
                "Branch".to_string(),
                vec![
                    make_pin(self.next_id(), "", PinType::Exec),
                    make_pin(self.next_id(), "Condition", PinType::Bool),
                ],
                vec![
                    make_pin(self.next_id(), "True", PinType::Exec),
                    make_pin(self.next_id(), "False", PinType::Exec),
                ],
                Color::hex("#6b7280"),
            ),
            "math_add" => (
                "Add".to_string(),
                vec![
                    make_pin(self.next_id(), "A", PinType::Float),
                    make_pin(self.next_id(), "B", PinType::Float),
                ],
                vec![make_pin(self.next_id(), "Result", PinType::Float)],
                Color::hex("#22c55e"),
            ),
            "math_mul" => (
                "Multiply".to_string(),
                vec![
                    make_pin(self.next_id(), "A", PinType::Float),
                    make_pin(self.next_id(), "B", PinType::Float),
                ],
                vec![make_pin(self.next_id(), "Result", PinType::Float)],
                Color::hex("#22c55e"),
            ),
            "math_lerp" => (
                "Lerp".to_string(),
                vec![
                    make_pin(self.next_id(), "A", PinType::Float),
                    make_pin(self.next_id(), "B", PinType::Float),
                    make_pin(self.next_id(), "Alpha", PinType::Float),
                ],
                vec![make_pin(self.next_id(), "Result", PinType::Float)],
                Color::hex("#22c55e"),
            ),
            // Material nodes
            "tex_sample" => (
                "Texture Sample".to_string(),
                vec![
                    make_pin(self.next_id(), "Texture", PinType::Texture),
                    make_pin(self.next_id(), "UVs", PinType::Vector2),
                ],
                vec![
                    make_pin(self.next_id(), "RGB", PinType::Vector3),
                    make_pin(self.next_id(), "R", PinType::Float),
                    make_pin(self.next_id(), "G", PinType::Float),
                    make_pin(self.next_id(), "B", PinType::Float),
                    make_pin(self.next_id(), "A", PinType::Float),
                ],
                Color::hex("#f97316"),
            ),
            "material_output" => (
                "Material Output".to_string(),
                vec![
                    make_pin(self.next_id(), "Base Color", PinType::Vector3),
                    make_pin(self.next_id(), "Metallic", PinType::Float),
                    make_pin(self.next_id(), "Roughness", PinType::Float),
                    make_pin(self.next_id(), "Normal", PinType::Vector3),
                    make_pin(self.next_id(), "Emissive", PinType::Vector3),
                    make_pin(self.next_id(), "Opacity", PinType::Float),
                ],
                vec![],
                Color::hex("#a855f7"),
            ),
            // Default
            _ => (
                type_id.to_string(),
                vec![make_pin(self.next_id(), "In", PinType::Any)],
                vec![make_pin(self.next_id(), "Out", PinType::Any)],
                Color::hex("#6b7280"),
            ),
        }
    }

    /// Delete selected nodes
    pub fn delete_selected(&mut self) {
        for &node_id in &self.selection.selected_nodes.clone() {
            self.delete_node(node_id);
        }
        self.selection.selected_nodes.clear();
        self.is_dirty = true;
    }

    /// Delete a node
    pub fn delete_node(&mut self, node_id: u64) {
        // Remove connections
        self.connections.retain(|c| c.from_node != node_id && c.to_node != node_id);
        
        // Remove node
        if let Some(node) = self.nodes.remove(&node_id) {
            self.undo_stack.push(GraphAction::RemoveNode(node_id, node));
        }
    }

    /// Connect two pins
    pub fn connect(&mut self, from_node: u64, from_pin: u64, to_node: u64, to_pin: u64) -> bool {
        // Validate connection
        let from = self.nodes.get(&from_node);
        let to = self.nodes.get(&to_node);

        if from.is_none() || to.is_none() {
            return false;
        }

        let from_type = from.unwrap().outputs.iter()
            .find(|p| p.id == from_pin)
            .map(|p| p.pin_type);
        let to_type = to.unwrap().inputs.iter()
            .find(|p| p.id == to_pin)
            .map(|p| p.pin_type);

        if let (Some(ft), Some(tt)) = (from_type, to_type) {
            if !ft.is_compatible(&tt) {
                return false;
            }
        } else {
            return false;
        }

        // Remove existing connection to this input (unless it's Exec)
        if to_type != Some(PinType::Exec) {
            self.connections.retain(|c| !(c.to_node == to_node && c.to_pin == to_pin));
        }

        let id = self.next_id();
        let connection = Connection {
            id,
            from_node,
            from_pin,
            to_node,
            to_pin,
        };

        self.connections.push(connection.clone());
        self.undo_stack.push(GraphAction::AddConnection(connection));
        self.redo_stack.clear();
        self.is_dirty = true;

        // Mark pins as connected
        if let Some(node) = self.nodes.get_mut(&from_node) {
            if let Some(pin) = node.outputs.iter_mut().find(|p| p.id == from_pin) {
                pin.is_connected = true;
            }
        }
        if let Some(node) = self.nodes.get_mut(&to_node) {
            if let Some(pin) = node.inputs.iter_mut().find(|p| p.id == to_pin) {
                pin.is_connected = true;
            }
        }

        true
    }

    /// Select node
    pub fn select(&mut self, node_id: u64, add_to_selection: bool) {
        if !add_to_selection {
            self.selection.selected_nodes.clear();
        }
        if !self.selection.selected_nodes.contains(&node_id) {
            self.selection.selected_nodes.push(node_id);
        }
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selection.selected_nodes.clear();
        self.selection.selected_connections.clear();
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
        self.view.offset = -center;
    }

    /// Frame selected nodes
    pub fn frame_selected(&mut self) {
        if self.selection.selected_nodes.is_empty() {
            self.frame_all();
            return;
        }

        let mut min = Vec2::splat(f32::MAX);
        let mut max = Vec2::splat(f32::MIN);

        for &id in &self.selection.selected_nodes {
            if let Some(node) = self.nodes.get(&id) {
                min = min.min(node.position);
                max = max.max(node.position + node.size);
            }
        }

        let center = (min + max) / 2.0;
        self.view.offset = -center;
    }

    /// Undo
    pub fn undo(&mut self) {
        if let Some(action) = self.undo_stack.pop() {
            match &action {
                GraphAction::AddNode(node) => {
                    self.nodes.remove(&node.id);
                }
                GraphAction::RemoveNode(id, node) => {
                    self.nodes.insert(*id, node.clone());
                }
                GraphAction::MoveNodes(moves) => {
                    for (id, old_pos, _) in moves {
                        if let Some(node) = self.nodes.get_mut(id) {
                            node.position = *old_pos;
                        }
                    }
                }
                GraphAction::AddConnection(conn) => {
                    self.connections.retain(|c| c.id != conn.id);
                }
                GraphAction::RemoveConnection(_, conn) => {
                    self.connections.push(conn.clone());
                }
                GraphAction::ChangeNodeProperty(id, name, old, _) => {
                    if let Some(node) = self.nodes.get_mut(id) {
                        node.custom_data.insert(name.clone(), old.clone());
                    }
                }
            }
            self.redo_stack.push(action);
            self.is_dirty = true;
        }
    }

    /// Redo
    pub fn redo(&mut self) {
        if let Some(action) = self.redo_stack.pop() {
            match &action {
                GraphAction::AddNode(node) => {
                    self.nodes.insert(node.id, node.clone());
                }
                GraphAction::RemoveNode(id, _) => {
                    self.nodes.remove(id);
                }
                GraphAction::MoveNodes(moves) => {
                    for (id, _, new_pos) in moves {
                        if let Some(node) = self.nodes.get_mut(id) {
                            node.position = *new_pos;
                        }
                    }
                }
                GraphAction::AddConnection(conn) => {
                    self.connections.push(conn.clone());
                }
                GraphAction::RemoveConnection(id, _) => {
                    self.connections.retain(|c| c.id != *id);
                }
                GraphAction::ChangeNodeProperty(id, name, _, new) => {
                    if let Some(node) = self.nodes.get_mut(id) {
                        node.custom_data.insert(name.clone(), new.clone());
                    }
                }
            }
            self.undo_stack.push(action);
            self.is_dirty = true;
        }
    }

    /// Copy selected nodes to clipboard
    pub fn copy(&self) -> Vec<GraphNode> {
        self.selection.selected_nodes.iter()
            .filter_map(|id| self.nodes.get(id).cloned())
            .collect()
    }

    /// Paste nodes from clipboard
    pub fn paste(&mut self, nodes: &[GraphNode], offset: Vec2) {
        let mut id_map: HashMap<u64, u64> = HashMap::new();

        for node in nodes {
            let new_id = self.next_id();
            id_map.insert(node.id, new_id);

            let mut new_node = node.clone();
            new_node.id = new_id;
            new_node.position += offset;

            // Update pin IDs
            for pin in &mut new_node.inputs {
                pin.id = self.next_id();
            }
            for pin in &mut new_node.outputs {
                pin.id = self.next_id();
            }

            self.nodes.insert(new_id, new_node);
        }

        // Select pasted nodes
        self.selection.selected_nodes = id_map.values().copied().collect();
        self.is_dirty = true;
    }

    /// Duplicate selected nodes
    pub fn duplicate(&mut self) {
        let copied = self.copy();
        self.paste(&copied, Vec2::new(50.0, 50.0));
    }

    /// Align selected nodes
    pub fn align(&mut self, alignment: Alignment) {
        if self.selection.selected_nodes.len() < 2 {
            return;
        }

        let positions: Vec<Vec2> = self.selection.selected_nodes.iter()
            .filter_map(|id| self.nodes.get(id).map(|n| n.position))
            .collect();

        let target = match alignment {
            Alignment::Left => positions.iter().map(|p| p.x).min_by(|a, b| a.partial_cmp(b).unwrap()),
            Alignment::Right => positions.iter().map(|p| p.x).max_by(|a, b| a.partial_cmp(b).unwrap()),
            Alignment::Top => positions.iter().map(|p| p.y).min_by(|a, b| a.partial_cmp(b).unwrap()),
            Alignment::Bottom => positions.iter().map(|p| p.y).max_by(|a, b| a.partial_cmp(b).unwrap()),
            Alignment::CenterH => {
                let sum: f32 = positions.iter().map(|p| p.x).sum();
                Some(sum / positions.len() as f32)
            }
            Alignment::CenterV => {
                let sum: f32 = positions.iter().map(|p| p.y).sum();
                Some(sum / positions.len() as f32)
            }
        };

        if let Some(target) = target {
            for &id in &self.selection.selected_nodes {
                if let Some(node) = self.nodes.get_mut(&id) {
                    match alignment {
                        Alignment::Left | Alignment::Right | Alignment::CenterH => {
                            node.position.x = target;
                        }
                        Alignment::Top | Alignment::Bottom | Alignment::CenterV => {
                            node.position.y = target;
                        }
                    }
                }
            }
        }

        self.is_dirty = true;
    }
}

/// Alignment mode
#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Left,
    Right,
    Top,
    Bottom,
    CenterH,
    CenterV,
}
