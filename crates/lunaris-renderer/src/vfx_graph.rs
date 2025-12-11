//! VFX Graph System
//!
//! Node-based visual effects editor and runtime.

use glam::Vec3;
use std::collections::HashMap;

/// VFX property type
#[derive(Debug, Clone)]
pub enum VFXValue {
    /// Float
    Float(f32),
    /// Integer
    Int(i32),
    /// Vector3
    Vec3(Vec3),
    /// Color (RGBA)
    Color([f32; 4]),
    /// Curve
    Curve(Vec<(f32, f32)>),
    /// Gradient
    Gradient(Vec<(f32, [f32; 4])>),
    /// Bool
    Bool(bool),
    /// Texture reference
    Texture(u64),
    /// Mesh reference
    Mesh(u64),
}

/// VFX node types
#[derive(Debug, Clone)]
pub enum VFXNodeType {
    // Spawn
    SpawnRate { rate: f32 },
    SpawnBurst { count: u32, interval: f32 },
    SpawnOnEvent { event: String },
    
    // Initialize
    SetPosition { value: VFXValue },
    SetVelocity { value: VFXValue },
    SetLifetime { value: VFXValue },
    SetSize { value: VFXValue },
    SetColor { value: VFXValue },
    SetRotation { value: VFXValue },
    
    // Update
    AddForce { value: VFXValue },
    ApplyGravity { strength: f32 },
    ApplyDrag { coefficient: f32 },
    AddNoise { amplitude: f32, frequency: f32 },
    AddTurbulence { amplitude: f32, scale: f32 },
    OrbitAround { center: Vec3, speed: f32 },
    FollowPath { path_id: u64, speed: f32 },
    
    // Size over lifetime
    SizeOverLifetime { curve: Vec<(f32, f32)> },
    ColorOverLifetime { gradient: Vec<(f32, [f32; 4])> },
    RotationOverLifetime { speed: f32 },
    VelocityOverLifetime { curve: Vec<(f32, Vec3)> },
    
    // Collision
    CollideWithPlane { normal: Vec3, offset: f32, bounce: f32 },
    CollideWithSphere { center: Vec3, radius: f32, bounce: f32 },
    KillOnCollision,
    
    // Output
    RenderSprites { texture_id: u64, blend_mode: u32 },
    RenderMeshes { mesh_id: u64 },
    RenderTrails { width: f32, lifetime: f32 },
    RenderRibbons { width: f32 },
    
    // Math
    Add { a: String, b: String },
    Multiply { a: String, b: String },
    Lerp { a: String, b: String, t: String },
    Random { min: f32, max: f32 },
    RandomVec3 { min: Vec3, max: Vec3 },
    
    // Sample
    SampleCurve { curve_id: String, time: String },
    SampleGradient { gradient_id: String, time: String },
    SampleTexture { texture_id: u64, uv: String },
    
    // Custom
    Custom { shader_code: String },
}

/// VFX node
#[derive(Debug, Clone)]
pub struct VFXNode {
    /// Node ID
    pub id: u64,
    /// Node type
    pub node_type: VFXNodeType,
    /// Enabled
    pub enabled: bool,
    /// Position in editor
    pub editor_pos: [f32; 2],
}

/// VFX connection
#[derive(Debug, Clone)]
pub struct VFXConnection {
    /// Source node
    pub from_node: u64,
    /// Source output name
    pub from_output: String,
    /// Target node
    pub to_node: u64,
    /// Target input name
    pub to_input: String,
}

/// VFX graph definition
#[derive(Debug, Clone)]
pub struct VFXGraph {
    /// Graph name
    pub name: String,
    /// Nodes
    pub nodes: HashMap<u64, VFXNode>,
    /// Connections
    pub connections: Vec<VFXConnection>,
    /// Exposed parameters
    pub parameters: HashMap<String, VFXValue>,
    /// Max particles
    pub capacity: u32,
    /// Bounds mode
    pub bounds_mode: BoundsMode,
    /// Custom bounds
    pub custom_bounds: Option<(Vec3, Vec3)>,
    /// Next node ID
    next_id: u64,
}

/// Bounds calculation mode
#[derive(Debug, Clone, Copy, Default)]
pub enum BoundsMode {
    /// Automatic based on particles
    #[default]
    Automatic,
    /// Fixed custom bounds
    Fixed,
    /// Manual (updated by user)
    Manual,
}

impl Default for VFXGraph {
    fn default() -> Self {
        Self::new("Untitled VFX")
    }
}

impl VFXGraph {
    /// Create a new VFX graph
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            nodes: HashMap::new(),
            connections: Vec::new(),
            parameters: HashMap::new(),
            capacity: 10000,
            bounds_mode: BoundsMode::Automatic,
            custom_bounds: None,
            next_id: 1,
        }
    }

    /// Add a node
    pub fn add_node(&mut self, node_type: VFXNodeType) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        
        self.nodes.insert(id, VFXNode {
            id,
            node_type,
            enabled: true,
            editor_pos: [0.0, 0.0],
        });
        
        id
    }

    /// Connect nodes
    pub fn connect(&mut self, from: u64, from_output: &str, to: u64, to_input: &str) {
        self.connections.push(VFXConnection {
            from_node: from,
            from_output: from_output.to_string(),
            to_node: to,
            to_input: to_input.to_string(),
        });
    }

    /// Set parameter
    pub fn set_parameter(&mut self, name: &str, value: VFXValue) {
        self.parameters.insert(name.to_string(), value);
    }

    /// Get spawn nodes
    #[must_use]
    pub fn spawn_nodes(&self) -> Vec<&VFXNode> {
        self.nodes.values()
            .filter(|n| matches!(n.node_type, 
                VFXNodeType::SpawnRate { .. } | 
                VFXNodeType::SpawnBurst { .. } |
                VFXNodeType::SpawnOnEvent { .. }
            ))
            .collect()
    }

    /// Get update nodes
    #[must_use]
    pub fn update_nodes(&self) -> Vec<&VFXNode> {
        self.nodes.values()
            .filter(|n| matches!(n.node_type,
                VFXNodeType::AddForce { .. } |
                VFXNodeType::ApplyGravity { .. } |
                VFXNodeType::ApplyDrag { .. } |
                VFXNodeType::AddNoise { .. } |
                VFXNodeType::AddTurbulence { .. } |
                VFXNodeType::OrbitAround { .. } |
                VFXNodeType::SizeOverLifetime { .. } |
                VFXNodeType::ColorOverLifetime { .. }
            ))
            .collect()
    }
}

/// VFX presets
pub struct VFXPresets;

impl VFXPresets {
    /// Fire effect
    #[must_use]
    pub fn fire() -> VFXGraph {
        let mut graph = VFXGraph::new("Fire");
        
        let spawn = graph.add_node(VFXNodeType::SpawnRate { rate: 50.0 });
        let _pos = graph.add_node(VFXNodeType::SetPosition { 
            value: VFXValue::Vec3(Vec3::ZERO) 
        });
        let _vel = graph.add_node(VFXNodeType::SetVelocity { 
            value: VFXValue::Vec3(Vec3::new(0.0, 2.0, 0.0)) 
        });
        let _life = graph.add_node(VFXNodeType::SetLifetime { 
            value: VFXValue::Float(1.5) 
        });
        let _color = graph.add_node(VFXNodeType::ColorOverLifetime {
            gradient: vec![
                (0.0, [1.0, 1.0, 0.0, 1.0]),
                (0.3, [1.0, 0.5, 0.0, 0.8]),
                (0.7, [1.0, 0.2, 0.0, 0.5]),
                (1.0, [0.2, 0.0, 0.0, 0.0]),
            ]
        });
        let _size = graph.add_node(VFXNodeType::SizeOverLifetime {
            curve: vec![(0.0, 0.2), (0.3, 0.5), (1.0, 0.0)]
        });
        let _turb = graph.add_node(VFXNodeType::AddTurbulence { 
            amplitude: 0.5, 
            scale: 2.0 
        });
        
        graph.capacity = 1000;
        graph
    }

    /// Smoke effect
    #[must_use]
    pub fn smoke() -> VFXGraph {
        let mut graph = VFXGraph::new("Smoke");
        
        graph.add_node(VFXNodeType::SpawnRate { rate: 20.0 });
        graph.add_node(VFXNodeType::SetVelocity { 
            value: VFXValue::Vec3(Vec3::new(0.0, 1.0, 0.0)) 
        });
        graph.add_node(VFXNodeType::SetLifetime { 
            value: VFXValue::Float(4.0) 
        });
        graph.add_node(VFXNodeType::ColorOverLifetime {
            gradient: vec![
                (0.0, [0.3, 0.3, 0.3, 0.8]),
                (0.5, [0.5, 0.5, 0.5, 0.5]),
                (1.0, [0.7, 0.7, 0.7, 0.0]),
            ]
        });
        graph.add_node(VFXNodeType::SizeOverLifetime {
            curve: vec![(0.0, 0.5), (1.0, 2.0)]
        });
        graph.add_node(VFXNodeType::AddNoise { 
            amplitude: 0.3, 
            frequency: 1.0 
        });
        
        graph.capacity = 500;
        graph
    }

    /// Sparks effect
    #[must_use]
    pub fn sparks() -> VFXGraph {
        let mut graph = VFXGraph::new("Sparks");
        
        graph.add_node(VFXNodeType::SpawnBurst { count: 30, interval: 0.0 });
        graph.add_node(VFXNodeType::SetVelocity { 
            value: VFXValue::Vec3(Vec3::new(0.0, 5.0, 0.0)) 
        });
        graph.add_node(VFXNodeType::ApplyGravity { strength: 9.8 });
        graph.add_node(VFXNodeType::SetLifetime { 
            value: VFXValue::Float(1.0) 
        });
        graph.add_node(VFXNodeType::ColorOverLifetime {
            gradient: vec![
                (0.0, [1.0, 0.9, 0.5, 1.0]),
                (0.5, [1.0, 0.5, 0.2, 1.0]),
                (1.0, [0.5, 0.2, 0.0, 0.0]),
            ]
        });
        
        graph.capacity = 200;
        graph
    }

    /// Explosion effect
    #[must_use]
    pub fn explosion() -> VFXGraph {
        let mut graph = VFXGraph::new("Explosion");
        
        graph.add_node(VFXNodeType::SpawnBurst { count: 100, interval: 0.0 });
        graph.add_node(VFXNodeType::SetLifetime { 
            value: VFXValue::Float(0.8) 
        });
        graph.add_node(VFXNodeType::ColorOverLifetime {
            gradient: vec![
                (0.0, [1.0, 1.0, 0.8, 1.0]),
                (0.1, [1.0, 0.7, 0.2, 1.0]),
                (0.3, [1.0, 0.3, 0.0, 0.8]),
                (0.6, [0.3, 0.1, 0.0, 0.5]),
                (1.0, [0.1, 0.1, 0.1, 0.0]),
            ]
        });
        graph.add_node(VFXNodeType::SizeOverLifetime {
            curve: vec![(0.0, 0.1), (0.2, 2.0), (1.0, 3.0)]
        });
        
        graph.capacity = 200;
        graph
    }

    /// Rain effect
    #[must_use]
    pub fn rain() -> VFXGraph {
        let mut graph = VFXGraph::new("Rain");
        
        graph.add_node(VFXNodeType::SpawnRate { rate: 500.0 });
        graph.add_node(VFXNodeType::SetVelocity { 
            value: VFXValue::Vec3(Vec3::new(-1.0, -15.0, 0.0)) 
        });
        graph.add_node(VFXNodeType::SetLifetime { 
            value: VFXValue::Float(1.0) 
        });
        graph.add_node(VFXNodeType::SetColor { 
            value: VFXValue::Color([0.7, 0.8, 0.9, 0.5]) 
        });
        graph.add_node(VFXNodeType::CollideWithPlane { 
            normal: Vec3::Y, 
            offset: 0.0,
            bounce: 0.0 
        });
        graph.add_node(VFXNodeType::KillOnCollision);
        
        graph.capacity = 5000;
        graph
    }

    /// Snow effect
    #[must_use]
    pub fn snow() -> VFXGraph {
        let mut graph = VFXGraph::new("Snow");
        
        graph.add_node(VFXNodeType::SpawnRate { rate: 100.0 });
        graph.add_node(VFXNodeType::SetVelocity { 
            value: VFXValue::Vec3(Vec3::new(0.0, -1.0, 0.0)) 
        });
        graph.add_node(VFXNodeType::SetLifetime { 
            value: VFXValue::Float(5.0) 
        });
        graph.add_node(VFXNodeType::SetColor { 
            value: VFXValue::Color([1.0, 1.0, 1.0, 0.9]) 
        });
        graph.add_node(VFXNodeType::AddNoise { 
            amplitude: 0.5, 
            frequency: 0.5 
        });
        
        graph.capacity = 2000;
        graph
    }
}
