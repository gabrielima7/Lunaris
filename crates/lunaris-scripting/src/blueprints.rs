//! Blueprints-like Visual Programming
//!
//! Advanced visual scripting with full programming capabilities.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Blueprint ID
pub type BlueprintId = u64;

/// Node ID
pub type NodeId = u64;

/// Pin ID  
pub type PinId = u64;

/// Pin direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PinDirection {
    Input,
    Output,
}

/// Pin category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PinCategory {
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
    Vector2,
    /// Vector3
    Vector3,
    /// Rotator
    Rotator,
    /// Transform
    Transform,
    /// Object reference
    Object,
    /// Class reference
    Class,
    /// Struct
    Struct,
    /// Enum
    Enum,
    /// Array
    Array,
    /// Map
    Map,
    /// Delegate
    Delegate,
    /// Wildcard (any type)
    Wildcard,
}

/// Pin definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintPin {
    /// Pin ID
    pub id: PinId,
    /// Pin name
    pub name: String,
    /// Direction
    pub direction: PinDirection,
    /// Category
    pub category: PinCategory,
    /// Sub-category (for structs, enums)
    pub sub_category: Option<String>,
    /// Is array
    pub is_array: bool,
    /// Is reference
    pub is_reference: bool,
    /// Default value
    pub default_value: Option<String>,
    /// Is hidden
    pub hidden: bool,
    /// Linked pins
    pub links: Vec<PinId>,
}

/// Node category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeCategory {
    /// Event (entry point)
    Event,
    /// Function call
    Function,
    /// Flow control
    FlowControl,
    /// Variable access
    Variable,
    /// Math operation
    Math,
    /// Conversion
    Conversion,
    /// Macro
    Macro,
    /// Comment
    Comment,
    /// Reroute
    Reroute,
    /// Custom
    Custom,
}

/// Blueprint node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintNode {
    /// Node ID
    pub id: NodeId,
    /// Node type
    pub node_type: String,
    /// Category
    pub category: NodeCategory,
    /// Display name
    pub display_name: String,
    /// Position in graph
    pub position: (f32, f32),
    /// Input pins
    pub inputs: Vec<BlueprintPin>,
    /// Output pins
    pub outputs: Vec<BlueprintPin>,
    /// Is pure (no exec flow)
    pub pure: bool,
    /// Is latent (async)
    pub latent: bool,
    /// Is compact (small display)
    pub compact: bool,
    /// Comment/tooltip
    pub comment: Option<String>,
    /// Custom data
    pub custom_data: HashMap<String, String>,
}

/// Blueprint variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintVariable {
    /// Variable name
    pub name: String,
    /// Category
    pub category: PinCategory,
    /// Sub-category
    pub sub_category: Option<String>,
    /// Default value
    pub default_value: Option<String>,
    /// Is array
    pub is_array: bool,
    /// Is exposed to editor
    pub expose_on_spawn: bool,
    /// Is private
    pub private: bool,
    /// Replication
    pub replicated: bool,
    /// Tooltip
    pub tooltip: Option<String>,
}

/// Blueprint function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintFunction {
    /// Function name
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Input parameters
    pub inputs: Vec<BlueprintVariable>,
    /// Output parameters
    pub outputs: Vec<BlueprintVariable>,
    /// Nodes in function
    pub nodes: Vec<BlueprintNode>,
    /// Is pure
    pub pure: bool,
    /// Is static
    pub is_static: bool,
    /// Access level
    pub access: AccessLevel,
    /// Category for menu
    pub category: String,
}

/// Access level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessLevel {
    Public,
    Protected,
    Private,
}

/// Event dispatcher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDispatcher {
    /// Event name
    pub name: String,
    /// Parameters
    pub params: Vec<BlueprintVariable>,
}

/// Blueprint class
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blueprint {
    /// Blueprint ID
    pub id: BlueprintId,
    /// Blueprint name
    pub name: String,
    /// Parent class
    pub parent_class: String,
    /// Variables
    pub variables: Vec<BlueprintVariable>,
    /// Functions
    pub functions: Vec<BlueprintFunction>,
    /// Event graph nodes
    pub event_graph: Vec<BlueprintNode>,
    /// Event dispatchers
    pub dispatchers: Vec<EventDispatcher>,
    /// Components (for actor blueprints)
    pub components: Vec<BlueprintComponent>,
    /// Is abstract
    pub is_abstract: bool,
    /// Is const
    pub is_const: bool,
    /// Description
    pub description: Option<String>,
}

/// Blueprint component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintComponent {
    /// Component name
    pub name: String,
    /// Component class
    pub component_class: String,
    /// Parent component
    pub parent: Option<String>,
    /// Property overrides
    pub properties: HashMap<String, String>,
    /// Is scene component
    pub is_scene: bool,
    /// Transform (for scene components)
    pub transform: Option<ComponentTransform>,
}

/// Component transform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentTransform {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
}

impl Blueprint {
    /// Create new blueprint
    #[must_use]
    pub fn new(name: &str, parent_class: &str) -> Self {
        Self {
            id: 0,
            name: name.to_string(),
            parent_class: parent_class.to_string(),
            variables: Vec::new(),
            functions: Vec::new(),
            event_graph: Vec::new(),
            dispatchers: Vec::new(),
            components: Vec::new(),
            is_abstract: false,
            is_const: false,
            description: None,
        }
    }

    /// Add variable
    pub fn add_variable(&mut self, var: BlueprintVariable) {
        self.variables.push(var);
    }

    /// Add function
    pub fn add_function(&mut self, func: BlueprintFunction) {
        self.functions.push(func);
    }

    /// Add event node
    pub fn add_event_node(&mut self, node: BlueprintNode) {
        self.event_graph.push(node);
    }

    /// Find function by name
    #[must_use]
    pub fn find_function(&self, name: &str) -> Option<&BlueprintFunction> {
        self.functions.iter().find(|f| f.name == name)
    }

    /// Get all events
    #[must_use]
    pub fn events(&self) -> Vec<&BlueprintNode> {
        self.event_graph.iter()
            .filter(|n| n.category == NodeCategory::Event)
            .collect()
    }
}

/// Blueprint library (static functions)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintLibrary {
    /// Library name
    pub name: String,
    /// Functions
    pub functions: Vec<BlueprintFunction>,
    /// Category
    pub category: String,
}

/// Built-in node definitions
pub mod builtin {
    use super::*;

    /// Create BeginPlay event node
    #[must_use]
    pub fn begin_play() -> BlueprintNode {
        BlueprintNode {
            id: 0,
            node_type: "Event BeginPlay".to_string(),
            category: NodeCategory::Event,
            display_name: "Event Begin Play".to_string(),
            position: (0.0, 0.0),
            inputs: vec![],
            outputs: vec![
                BlueprintPin {
                    id: 0,
                    name: String::new(),
                    direction: PinDirection::Output,
                    category: PinCategory::Exec,
                    sub_category: None,
                    is_array: false,
                    is_reference: false,
                    default_value: None,
                    hidden: false,
                    links: vec![],
                },
            ],
            pure: false,
            latent: false,
            compact: false,
            comment: None,
            custom_data: HashMap::new(),
        }
    }

    /// Create Tick event node
    #[must_use]
    pub fn tick() -> BlueprintNode {
        BlueprintNode {
            id: 0,
            node_type: "Event Tick".to_string(),
            category: NodeCategory::Event,
            display_name: "Event Tick".to_string(),
            position: (0.0, 0.0),
            inputs: vec![],
            outputs: vec![
                BlueprintPin {
                    id: 0,
                    name: String::new(),
                    direction: PinDirection::Output,
                    category: PinCategory::Exec,
                    sub_category: None,
                    is_array: false,
                    is_reference: false,
                    default_value: None,
                    hidden: false,
                    links: vec![],
                },
                BlueprintPin {
                    id: 1,
                    name: "Delta Seconds".to_string(),
                    direction: PinDirection::Output,
                    category: PinCategory::Float,
                    sub_category: None,
                    is_array: false,
                    is_reference: false,
                    default_value: None,
                    hidden: false,
                    links: vec![],
                },
            ],
            pure: false,
            latent: false,
            compact: false,
            comment: None,
            custom_data: HashMap::new(),
        }
    }

    /// Create Branch node
    #[must_use]
    pub fn branch() -> BlueprintNode {
        BlueprintNode {
            id: 0,
            node_type: "Branch".to_string(),
            category: NodeCategory::FlowControl,
            display_name: "Branch".to_string(),
            position: (0.0, 0.0),
            inputs: vec![
                BlueprintPin {
                    id: 0,
                    name: String::new(),
                    direction: PinDirection::Input,
                    category: PinCategory::Exec,
                    sub_category: None,
                    is_array: false,
                    is_reference: false,
                    default_value: None,
                    hidden: false,
                    links: vec![],
                },
                BlueprintPin {
                    id: 1,
                    name: "Condition".to_string(),
                    direction: PinDirection::Input,
                    category: PinCategory::Bool,
                    sub_category: None,
                    is_array: false,
                    is_reference: false,
                    default_value: Some("false".to_string()),
                    hidden: false,
                    links: vec![],
                },
            ],
            outputs: vec![
                BlueprintPin {
                    id: 2,
                    name: "True".to_string(),
                    direction: PinDirection::Output,
                    category: PinCategory::Exec,
                    sub_category: None,
                    is_array: false,
                    is_reference: false,
                    default_value: None,
                    hidden: false,
                    links: vec![],
                },
                BlueprintPin {
                    id: 3,
                    name: "False".to_string(),
                    direction: PinDirection::Output,
                    category: PinCategory::Exec,
                    sub_category: None,
                    is_array: false,
                    is_reference: false,
                    default_value: None,
                    hidden: false,
                    links: vec![],
                },
            ],
            pure: false,
            latent: false,
            compact: true,
            comment: None,
            custom_data: HashMap::new(),
        }
    }

    /// Create ForEachLoop node
    #[must_use]
    pub fn for_each_loop() -> BlueprintNode {
        BlueprintNode {
            id: 0,
            node_type: "ForEachLoop".to_string(),
            category: NodeCategory::FlowControl,
            display_name: "For Each Loop".to_string(),
            position: (0.0, 0.0),
            inputs: vec![
                BlueprintPin {
                    id: 0,
                    name: String::new(),
                    direction: PinDirection::Input,
                    category: PinCategory::Exec,
                    sub_category: None,
                    is_array: false,
                    is_reference: false,
                    default_value: None,
                    hidden: false,
                    links: vec![],
                },
                BlueprintPin {
                    id: 1,
                    name: "Array".to_string(),
                    direction: PinDirection::Input,
                    category: PinCategory::Wildcard,
                    sub_category: None,
                    is_array: true,
                    is_reference: true,
                    default_value: None,
                    hidden: false,
                    links: vec![],
                },
            ],
            outputs: vec![
                BlueprintPin {
                    id: 2,
                    name: "Loop Body".to_string(),
                    direction: PinDirection::Output,
                    category: PinCategory::Exec,
                    sub_category: None,
                    is_array: false,
                    is_reference: false,
                    default_value: None,
                    hidden: false,
                    links: vec![],
                },
                BlueprintPin {
                    id: 3,
                    name: "Array Element".to_string(),
                    direction: PinDirection::Output,
                    category: PinCategory::Wildcard,
                    sub_category: None,
                    is_array: false,
                    is_reference: false,
                    default_value: None,
                    hidden: false,
                    links: vec![],
                },
                BlueprintPin {
                    id: 4,
                    name: "Array Index".to_string(),
                    direction: PinDirection::Output,
                    category: PinCategory::Int,
                    sub_category: None,
                    is_array: false,
                    is_reference: false,
                    default_value: None,
                    hidden: false,
                    links: vec![],
                },
                BlueprintPin {
                    id: 5,
                    name: "Completed".to_string(),
                    direction: PinDirection::Output,
                    category: PinCategory::Exec,
                    sub_category: None,
                    is_array: false,
                    is_reference: false,
                    default_value: None,
                    hidden: false,
                    links: vec![],
                },
            ],
            pure: false,
            latent: false,
            compact: false,
            comment: None,
            custom_data: HashMap::new(),
        }
    }

    /// Create Print String node
    #[must_use]
    pub fn print_string() -> BlueprintNode {
        BlueprintNode {
            id: 0,
            node_type: "PrintString".to_string(),
            category: NodeCategory::Function,
            display_name: "Print String".to_string(),
            position: (0.0, 0.0),
            inputs: vec![
                BlueprintPin {
                    id: 0,
                    name: String::new(),
                    direction: PinDirection::Input,
                    category: PinCategory::Exec,
                    sub_category: None,
                    is_array: false,
                    is_reference: false,
                    default_value: None,
                    hidden: false,
                    links: vec![],
                },
                BlueprintPin {
                    id: 1,
                    name: "In String".to_string(),
                    direction: PinDirection::Input,
                    category: PinCategory::String,
                    sub_category: None,
                    is_array: false,
                    is_reference: false,
                    default_value: Some("Hello".to_string()),
                    hidden: false,
                    links: vec![],
                },
            ],
            outputs: vec![
                BlueprintPin {
                    id: 2,
                    name: String::new(),
                    direction: PinDirection::Output,
                    category: PinCategory::Exec,
                    sub_category: None,
                    is_array: false,
                    is_reference: false,
                    default_value: None,
                    hidden: false,
                    links: vec![],
                },
            ],
            pure: false,
            latent: false,
            compact: false,
            comment: None,
            custom_data: HashMap::new(),
        }
    }
}

/// Blueprint interpreter
pub struct BlueprintVM {
    /// Blueprints
    blueprints: HashMap<BlueprintId, Blueprint>,
    /// Next ID
    next_id: BlueprintId,
}

impl Default for BlueprintVM {
    fn default() -> Self {
        Self::new()
    }
}

impl BlueprintVM {
    /// Create new VM
    #[must_use]
    pub fn new() -> Self {
        Self {
            blueprints: HashMap::new(),
            next_id: 1,
        }
    }

    /// Register blueprint
    pub fn register(&mut self, mut blueprint: Blueprint) -> BlueprintId {
        let id = self.next_id;
        self.next_id += 1;
        blueprint.id = id;
        self.blueprints.insert(id, blueprint);
        id
    }

    /// Get blueprint
    #[must_use]
    pub fn get(&self, id: BlueprintId) -> Option<&Blueprint> {
        self.blueprints.get(&id)
    }
}
