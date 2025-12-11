//! Visual Scripting System
//!
//! Node-based visual programming for game logic.

use std::collections::HashMap;
use glam::{Vec2, Vec3, Vec4};
use serde::{Deserialize, Serialize};

/// Node ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u64);

/// Pin ID (node_id, pin_index, is_output)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PinId(pub u64, pub u32, pub bool);

/// Pin data type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PinType {
    /// Execution flow
    Exec,
    /// Boolean
    Bool,
    /// Integer
    Int,
    /// Float
    Float,
    /// String
    String,
    /// Vector2
    Vec2,
    /// Vector3
    Vec3,
    /// Vector4
    Vec4,
    /// Object reference
    Object,
    /// Any type (wildcard)
    Any,
}

/// Pin value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PinValue {
    /// No value (exec pin)
    None,
    /// Boolean
    Bool(bool),
    /// Integer
    Int(i64),
    /// Float
    Float(f64),
    /// String
    String(String),
    /// Vector2
    Vec2([f32; 2]),
    /// Vector3
    Vec3([f32; 3]),
    /// Vector4
    Vec4([f32; 4]),
    /// Object ID
    Object(u64),
}

impl Default for PinValue {
    fn default() -> Self {
        Self::None
    }
}

/// Node pin definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinDef {
    /// Pin name
    pub name: String,
    /// Pin type
    pub pin_type: PinType,
    /// Is output
    pub is_output: bool,
    /// Default value
    pub default: PinValue,
}

/// Node definition (blueprint)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDef {
    /// Node type name
    pub type_name: String,
    /// Category
    pub category: String,
    /// Display name
    pub display_name: String,
    /// Description
    pub description: String,
    /// Input pins
    pub inputs: Vec<PinDef>,
    /// Output pins
    pub outputs: Vec<PinDef>,
    /// Color
    pub color: [f32; 3],
    /// Is pure (no exec pins)
    pub is_pure: bool,
}

/// Node instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Node ID
    pub id: NodeId,
    /// Node type
    pub node_type: String,
    /// Position in editor
    pub position: Vec2,
    /// Input values
    pub input_values: Vec<PinValue>,
    /// Comment
    pub comment: String,
    /// Is breakpoint
    pub breakpoint: bool,
}

/// Connection between pins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    /// Source node
    pub from_node: NodeId,
    /// Source pin index
    pub from_pin: u32,
    /// Target node
    pub to_node: NodeId,
    /// Target pin index
    pub to_pin: u32,
}

/// Visual script graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualGraph {
    /// Graph name
    pub name: String,
    /// Nodes
    pub nodes: HashMap<NodeId, Node>,
    /// Connections
    pub connections: Vec<Connection>,
    /// Variables
    pub variables: HashMap<String, Variable>,
    /// Entry points
    pub entry_points: Vec<NodeId>,
    /// Next node ID
    next_id: u64,
}

/// Script variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    /// Name
    pub name: String,
    /// Type
    pub var_type: PinType,
    /// Value
    pub value: PinValue,
    /// Is exposed
    pub exposed: bool,
}

impl Default for VisualGraph {
    fn default() -> Self {
        Self::new("Untitled")
    }
}

impl VisualGraph {
    /// Create a new graph
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            nodes: HashMap::new(),
            connections: Vec::new(),
            variables: HashMap::new(),
            entry_points: Vec::new(),
            next_id: 1,
        }
    }

    /// Add a node
    pub fn add_node(&mut self, node_type: &str, position: Vec2) -> NodeId {
        let id = NodeId(self.next_id);
        self.next_id += 1;

        self.nodes.insert(id, Node {
            id,
            node_type: node_type.to_string(),
            position,
            input_values: Vec::new(),
            comment: String::new(),
            breakpoint: false,
        });

        id
    }

    /// Remove a node
    pub fn remove_node(&mut self, id: NodeId) {
        self.nodes.remove(&id);
        self.connections.retain(|c| c.from_node != id && c.to_node != id);
        self.entry_points.retain(|&e| e != id);
    }

    /// Connect two pins
    pub fn connect(&mut self, from_node: NodeId, from_pin: u32, to_node: NodeId, to_pin: u32) -> bool {
        // Check if already connected
        let exists = self.connections.iter().any(|c| 
            c.to_node == to_node && c.to_pin == to_pin
        );
        
        if exists {
            return false;
        }

        self.connections.push(Connection {
            from_node,
            from_pin,
            to_node,
            to_pin,
        });

        true
    }

    /// Disconnect a pin
    pub fn disconnect(&mut self, to_node: NodeId, to_pin: u32) {
        self.connections.retain(|c| !(c.to_node == to_node && c.to_pin == to_pin));
    }

    /// Add a variable
    pub fn add_variable(&mut self, name: &str, var_type: PinType, default: PinValue) {
        self.variables.insert(name.to_string(), Variable {
            name: name.to_string(),
            var_type,
            value: default,
            exposed: false,
        });
    }

    /// Set entry point
    pub fn set_entry_point(&mut self, node_id: NodeId) {
        if !self.entry_points.contains(&node_id) {
            self.entry_points.push(node_id);
        }
    }

    /// Serialize to JSON
    ///
    /// # Errors
    ///
    /// Returns error if serialization fails
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from JSON
    ///
    /// # Errors
    ///
    /// Returns error if deserialization fails
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// Node registry
pub struct NodeRegistry {
    /// Node definitions
    definitions: HashMap<String, NodeDef>,
    /// Categories
    categories: Vec<String>,
}

impl Default for NodeRegistry {
    fn default() -> Self {
        let mut registry = Self {
            definitions: HashMap::new(),
            categories: Vec::new(),
        };
        registry.register_builtin_nodes();
        registry
    }
}

impl NodeRegistry {
    fn register_builtin_nodes(&mut self) {
        // Event nodes
        self.register(NodeDef {
            type_name: "Event_BeginPlay".into(),
            category: "Events".into(),
            display_name: "Begin Play".into(),
            description: "Called when game starts".into(),
            inputs: vec![],
            outputs: vec![PinDef {
                name: "Exec".into(),
                pin_type: PinType::Exec,
                is_output: true,
                default: PinValue::None,
            }],
            color: [1.0, 0.0, 0.0],
            is_pure: false,
        });

        self.register(NodeDef {
            type_name: "Event_Tick".into(),
            category: "Events".into(),
            display_name: "Tick".into(),
            description: "Called every frame".into(),
            inputs: vec![],
            outputs: vec![
                PinDef { name: "Exec".into(), pin_type: PinType::Exec, is_output: true, default: PinValue::None },
                PinDef { name: "Delta Time".into(), pin_type: PinType::Float, is_output: true, default: PinValue::Float(0.0) },
            ],
            color: [1.0, 0.0, 0.0],
            is_pure: false,
        });

        // Flow control
        self.register(NodeDef {
            type_name: "Branch".into(),
            category: "Flow Control".into(),
            display_name: "Branch".into(),
            description: "If-else branch".into(),
            inputs: vec![
                PinDef { name: "Exec".into(), pin_type: PinType::Exec, is_output: false, default: PinValue::None },
                PinDef { name: "Condition".into(), pin_type: PinType::Bool, is_output: false, default: PinValue::Bool(false) },
            ],
            outputs: vec![
                PinDef { name: "True".into(), pin_type: PinType::Exec, is_output: true, default: PinValue::None },
                PinDef { name: "False".into(), pin_type: PinType::Exec, is_output: true, default: PinValue::None },
            ],
            color: [0.8, 0.8, 0.8],
            is_pure: false,
        });

        self.register(NodeDef {
            type_name: "ForLoop".into(),
            category: "Flow Control".into(),
            display_name: "For Loop".into(),
            description: "Loop from start to end".into(),
            inputs: vec![
                PinDef { name: "Exec".into(), pin_type: PinType::Exec, is_output: false, default: PinValue::None },
                PinDef { name: "Start".into(), pin_type: PinType::Int, is_output: false, default: PinValue::Int(0) },
                PinDef { name: "End".into(), pin_type: PinType::Int, is_output: false, default: PinValue::Int(10) },
            ],
            outputs: vec![
                PinDef { name: "Body".into(), pin_type: PinType::Exec, is_output: true, default: PinValue::None },
                PinDef { name: "Index".into(), pin_type: PinType::Int, is_output: true, default: PinValue::Int(0) },
                PinDef { name: "Completed".into(), pin_type: PinType::Exec, is_output: true, default: PinValue::None },
            ],
            color: [0.8, 0.8, 0.8],
            is_pure: false,
        });

        // Math nodes
        self.register(NodeDef {
            type_name: "Math_Add".into(),
            category: "Math".into(),
            display_name: "Add".into(),
            description: "A + B".into(),
            inputs: vec![
                PinDef { name: "A".into(), pin_type: PinType::Float, is_output: false, default: PinValue::Float(0.0) },
                PinDef { name: "B".into(), pin_type: PinType::Float, is_output: false, default: PinValue::Float(0.0) },
            ],
            outputs: vec![
                PinDef { name: "Result".into(), pin_type: PinType::Float, is_output: true, default: PinValue::Float(0.0) },
            ],
            color: [0.0, 0.8, 0.0],
            is_pure: true,
        });

        self.register(NodeDef {
            type_name: "Math_Multiply".into(),
            category: "Math".into(),
            display_name: "Multiply".into(),
            description: "A * B".into(),
            inputs: vec![
                PinDef { name: "A".into(), pin_type: PinType::Float, is_output: false, default: PinValue::Float(0.0) },
                PinDef { name: "B".into(), pin_type: PinType::Float, is_output: false, default: PinValue::Float(1.0) },
            ],
            outputs: vec![
                PinDef { name: "Result".into(), pin_type: PinType::Float, is_output: true, default: PinValue::Float(0.0) },
            ],
            color: [0.0, 0.8, 0.0],
            is_pure: true,
        });

        // Vector nodes
        self.register(NodeDef {
            type_name: "MakeVector".into(),
            category: "Vector".into(),
            display_name: "Make Vector".into(),
            description: "Create a vector from components".into(),
            inputs: vec![
                PinDef { name: "X".into(), pin_type: PinType::Float, is_output: false, default: PinValue::Float(0.0) },
                PinDef { name: "Y".into(), pin_type: PinType::Float, is_output: false, default: PinValue::Float(0.0) },
                PinDef { name: "Z".into(), pin_type: PinType::Float, is_output: false, default: PinValue::Float(0.0) },
            ],
            outputs: vec![
                PinDef { name: "Vector".into(), pin_type: PinType::Vec3, is_output: true, default: PinValue::Vec3([0.0, 0.0, 0.0]) },
            ],
            color: [1.0, 0.8, 0.0],
            is_pure: true,
        });

        // Print
        self.register(NodeDef {
            type_name: "PrintString".into(),
            category: "Debug".into(),
            display_name: "Print String".into(),
            description: "Print to console".into(),
            inputs: vec![
                PinDef { name: "Exec".into(), pin_type: PinType::Exec, is_output: false, default: PinValue::None },
                PinDef { name: "String".into(), pin_type: PinType::String, is_output: false, default: PinValue::String(String::new()) },
            ],
            outputs: vec![
                PinDef { name: "Exec".into(), pin_type: PinType::Exec, is_output: true, default: PinValue::None },
            ],
            color: [0.5, 0.5, 1.0],
            is_pure: false,
        });
    }

    /// Register a node type
    pub fn register(&mut self, def: NodeDef) {
        if !self.categories.contains(&def.category) {
            self.categories.push(def.category.clone());
        }
        self.definitions.insert(def.type_name.clone(), def);
    }

    /// Get node definition
    #[must_use]
    pub fn get(&self, type_name: &str) -> Option<&NodeDef> {
        self.definitions.get(type_name)
    }

    /// Get all nodes in category
    #[must_use]
    pub fn get_by_category(&self, category: &str) -> Vec<&NodeDef> {
        self.definitions.values()
            .filter(|d| d.category == category)
            .collect()
    }

    /// Get categories
    #[must_use]
    pub fn categories(&self) -> &[String] {
        &self.categories
    }

    /// Search nodes
    #[must_use]
    pub fn search(&self, query: &str) -> Vec<&NodeDef> {
        let query = query.to_lowercase();
        self.definitions.values()
            .filter(|d| d.display_name.to_lowercase().contains(&query) || 
                        d.description.to_lowercase().contains(&query))
            .collect()
    }
}

/// Visual script interpreter
pub struct ScriptInterpreter {
    /// Node registry
    registry: NodeRegistry,
    /// Current graph
    graph: Option<VisualGraph>,
    /// Variable values
    variables: HashMap<String, PinValue>,
    /// Execution stack
    exec_stack: Vec<(NodeId, u32)>,
    /// Is running
    running: bool,
    /// Current node (for debugging)
    current_node: Option<NodeId>,
}

impl Default for ScriptInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl ScriptInterpreter {
    /// Create a new interpreter
    #[must_use]
    pub fn new() -> Self {
        Self {
            registry: NodeRegistry::default(),
            graph: None,
            variables: HashMap::new(),
            exec_stack: Vec::new(),
            running: false,
            current_node: None,
        }
    }

    /// Load a graph
    pub fn load_graph(&mut self, graph: VisualGraph) {
        // Copy variables
        for (name, var) in &graph.variables {
            self.variables.insert(name.clone(), var.value.clone());
        }
        self.graph = Some(graph);
    }

    /// Execute from entry point
    pub fn execute(&mut self, entry: &str) {
        let Some(ref graph) = self.graph else { return };
        
        // Find entry node
        let entry_node = graph.nodes.values()
            .find(|n| n.node_type == entry)
            .map(|n| n.id);

        if let Some(node_id) = entry_node {
            self.running = true;
            self.exec_stack.push((node_id, 0));
            
            while !self.exec_stack.is_empty() && self.running {
                self.step();
            }
        }
    }

    /// Execute one step
    pub fn step(&mut self) {
        let Some((node_id, output_pin)) = self.exec_stack.pop() else {
            self.running = false;
            return;
        };

        self.current_node = Some(node_id);
        
        // Execute node (simplified)
        // In a real implementation, this would evaluate the node and push next exec pins
    }

    /// Stop execution
    pub fn stop(&mut self) {
        self.running = false;
        self.exec_stack.clear();
    }

    /// Get variable
    #[must_use]
    pub fn get_variable(&self, name: &str) -> Option<&PinValue> {
        self.variables.get(name)
    }

    /// Set variable
    pub fn set_variable(&mut self, name: &str, value: PinValue) {
        self.variables.insert(name.to_string(), value);
    }

    /// Is running
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Get registry
    #[must_use]
    pub fn registry(&self) -> &NodeRegistry {
        &self.registry
    }
}
