//! MetaSounds-like Procedural Audio System
//!
//! Node-based audio synthesis and processing.

use std::collections::HashMap;

/// Audio value type
#[derive(Debug, Clone)]
pub enum AudioValue {
    Float(f32),
    Int(i32),
    Bool(bool),
    Buffer(Vec<f32>),
    Trigger,
}

/// Audio node type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioNodeType {
    // Sources
    Oscillator,
    Noise,
    SamplePlayer,
    Input,
    
    // Filters
    LowPass,
    HighPass,
    BandPass,
    Notch,
    Parametric,
    
    // Modulators
    LFO,
    Envelope,
    Random,
    
    // Effects
    Delay,
    Reverb,
    Chorus,
    Phaser,
    Flanger,
    Distortion,
    Compressor,
    Limiter,
    
    // Math
    Add,
    Multiply,
    Subtract,
    Divide,
    Clamp,
    Map,
    Abs,
    
    // Control
    Trigger,
    Gate,
    Switch,
    Crossfade,
    
    // Output
    Output,
}

/// Audio node ID
pub type AudioNodeId = u64;

/// Audio connection
#[derive(Debug, Clone)]
pub struct AudioConnection {
    /// Source node
    pub source_node: AudioNodeId,
    /// Source pin
    pub source_pin: String,
    /// Target node
    pub target_node: AudioNodeId,
    /// Target pin
    pub target_pin: String,
}

/// Audio node
#[derive(Debug, Clone)]
pub struct AudioNode {
    /// Unique ID
    pub id: AudioNodeId,
    /// Node type
    pub node_type: AudioNodeType,
    /// Parameters
    pub params: HashMap<String, AudioValue>,
    /// Position in graph
    pub position: (f32, f32),
}

impl AudioNode {
    /// Create new node
    #[must_use]
    pub fn new(id: AudioNodeId, node_type: AudioNodeType) -> Self {
        let params = default_params(node_type);
        Self {
            id,
            node_type,
            params,
            position: (0.0, 0.0),
        }
    }
}

fn default_params(node_type: AudioNodeType) -> HashMap<String, AudioValue> {
    let mut params = HashMap::new();
    
    match node_type {
        AudioNodeType::Oscillator => {
            params.insert("frequency".to_string(), AudioValue::Float(440.0));
            params.insert("amplitude".to_string(), AudioValue::Float(1.0));
            params.insert("waveform".to_string(), AudioValue::Int(0)); // Sine
        }
        AudioNodeType::Noise => {
            params.insert("amplitude".to_string(), AudioValue::Float(1.0));
            params.insert("type".to_string(), AudioValue::Int(0)); // White
        }
        AudioNodeType::LowPass | AudioNodeType::HighPass | AudioNodeType::BandPass => {
            params.insert("cutoff".to_string(), AudioValue::Float(1000.0));
            params.insert("resonance".to_string(), AudioValue::Float(0.7));
        }
        AudioNodeType::Envelope => {
            params.insert("attack".to_string(), AudioValue::Float(0.01));
            params.insert("decay".to_string(), AudioValue::Float(0.1));
            params.insert("sustain".to_string(), AudioValue::Float(0.7));
            params.insert("release".to_string(), AudioValue::Float(0.3));
        }
        AudioNodeType::Delay => {
            params.insert("time".to_string(), AudioValue::Float(0.5));
            params.insert("feedback".to_string(), AudioValue::Float(0.3));
            params.insert("mix".to_string(), AudioValue::Float(0.5));
        }
        AudioNodeType::Reverb => {
            params.insert("room_size".to_string(), AudioValue::Float(0.8));
            params.insert("damping".to_string(), AudioValue::Float(0.5));
            params.insert("mix".to_string(), AudioValue::Float(0.3));
        }
        AudioNodeType::Compressor => {
            params.insert("threshold".to_string(), AudioValue::Float(-20.0));
            params.insert("ratio".to_string(), AudioValue::Float(4.0));
            params.insert("attack".to_string(), AudioValue::Float(0.003));
            params.insert("release".to_string(), AudioValue::Float(0.1));
        }
        _ => {}
    }
    
    params
}

/// MetaSounds audio graph
pub struct MetaSoundsGraph {
    /// Unique ID
    pub id: u64,
    /// Name
    pub name: String,
    /// Nodes
    nodes: HashMap<AudioNodeId, AudioNode>,
    /// Connections
    connections: Vec<AudioConnection>,
    /// Next node ID
    next_id: AudioNodeId,
    /// Sample rate
    pub sample_rate: u32,
    /// Block size
    pub block_size: u32,
}

impl Default for MetaSoundsGraph {
    fn default() -> Self {
        Self::new("Untitled")
    }
}

impl MetaSoundsGraph {
    /// Create new graph
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            id: 0,
            name: name.to_string(),
            nodes: HashMap::new(),
            connections: Vec::new(),
            next_id: 1,
            sample_rate: 48000,
            block_size: 256,
        }
    }

    /// Add node
    pub fn add_node(&mut self, node_type: AudioNodeType) -> AudioNodeId {
        let id = self.next_id;
        self.next_id += 1;
        
        let node = AudioNode::new(id, node_type);
        self.nodes.insert(id, node);
        id
    }

    /// Remove node
    pub fn remove_node(&mut self, id: AudioNodeId) {
        self.nodes.remove(&id);
        self.connections.retain(|c| c.source_node != id && c.target_node != id);
    }

    /// Connect nodes
    pub fn connect(
        &mut self,
        source: AudioNodeId,
        source_pin: &str,
        target: AudioNodeId,
        target_pin: &str,
    ) {
        self.connections.push(AudioConnection {
            source_node: source,
            source_pin: source_pin.to_string(),
            target_node: target,
            target_pin: target_pin.to_string(),
        });
    }

    /// Set parameter
    pub fn set_param(&mut self, node_id: AudioNodeId, name: &str, value: AudioValue) {
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.params.insert(name.to_string(), value);
        }
    }

    /// Get nodes
    #[must_use]
    pub fn nodes(&self) -> &HashMap<AudioNodeId, AudioNode> {
        &self.nodes
    }

    /// Get connections
    #[must_use]
    pub fn connections(&self) -> &[AudioConnection] {
        &self.connections
    }

    /// Get topological order for processing
    #[must_use]
    pub fn processing_order(&self) -> Vec<AudioNodeId> {
        // Simple topological sort
        let mut order = Vec::new();
        let mut visited = std::collections::HashSet::new();
        
        for &id in self.nodes.keys() {
            self.visit_node(id, &mut visited, &mut order);
        }
        
        order
    }

    fn visit_node(
        &self,
        id: AudioNodeId,
        visited: &mut std::collections::HashSet<AudioNodeId>,
        order: &mut Vec<AudioNodeId>,
    ) {
        if visited.contains(&id) {
            return;
        }
        visited.insert(id);
        
        // Visit dependencies first
        for conn in &self.connections {
            if conn.target_node == id {
                self.visit_node(conn.source_node, visited, order);
            }
        }
        
        order.push(id);
    }
}

/// MetaSounds preset
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetaSoundsPreset {
    FootstepSurface,
    WeaponFire,
    ExplosionLayered,
    AmbientWind,
    Impact,
    VehicleEngine,
    UIHover,
    UIClick,
    MusicStinger,
}

impl MetaSoundsPreset {
    /// Create graph from preset
    #[must_use]
    pub fn create_graph(self) -> MetaSoundsGraph {
        let mut graph = MetaSoundsGraph::new(match self {
            Self::FootstepSurface => "Footstep Surface",
            Self::WeaponFire => "Weapon Fire",
            Self::ExplosionLayered => "Explosion Layered",
            Self::AmbientWind => "Ambient Wind",
            Self::Impact => "Impact",
            Self::VehicleEngine => "Vehicle Engine",
            Self::UIHover => "UI Hover",
            Self::UIClick => "UI Click",
            Self::MusicStinger => "Music Stinger",
        });

        match self {
            Self::FootstepSurface => {
                let input = graph.add_node(AudioNodeType::Input);
                let sample = graph.add_node(AudioNodeType::SamplePlayer);
                let filter = graph.add_node(AudioNodeType::LowPass);
                let env = graph.add_node(AudioNodeType::Envelope);
                let output = graph.add_node(AudioNodeType::Output);
                
                graph.connect(input, "trigger", env, "trigger");
                graph.connect(sample, "audio", filter, "input");
                graph.connect(filter, "output", output, "left");
                graph.connect(filter, "output", output, "right");
            }
            Self::WeaponFire => {
                let noise = graph.add_node(AudioNodeType::Noise);
                let _osc = graph.add_node(AudioNodeType::Oscillator);
                let env = graph.add_node(AudioNodeType::Envelope);
                let filter = graph.add_node(AudioNodeType::LowPass);
                let dist = graph.add_node(AudioNodeType::Distortion);
                let output = graph.add_node(AudioNodeType::Output);

                graph.set_param(env, "attack", AudioValue::Float(0.001));
                graph.set_param(env, "decay", AudioValue::Float(0.05));
                graph.set_param(env, "sustain", AudioValue::Float(0.0));
                
                graph.connect(noise, "output", filter, "input");
                graph.connect(filter, "output", dist, "input");
                graph.connect(dist, "output", output, "left");
            }
            Self::VehicleEngine => {
                let osc1 = graph.add_node(AudioNodeType::Oscillator);
                let osc2 = graph.add_node(AudioNodeType::Oscillator);
                let lfo = graph.add_node(AudioNodeType::LFO);
                let filter = graph.add_node(AudioNodeType::LowPass);
                let _dist = graph.add_node(AudioNodeType::Distortion);
                let _output = graph.add_node(AudioNodeType::Output);

                graph.set_param(osc1, "frequency", AudioValue::Float(60.0));
                graph.set_param(osc2, "frequency", AudioValue::Float(120.0));
                
                graph.connect(osc1, "output", filter, "input");
                graph.connect(lfo, "output", filter, "cutoff");
            }
            _ => {
                // Basic setup for other presets
                let input = graph.add_node(AudioNodeType::Input);
                let output = graph.add_node(AudioNodeType::Output);
                graph.connect(input, "audio", output, "left");
            }
        }

        graph
    }
}

/// MetaSounds instance
pub struct MetaSoundsInstance {
    /// Graph reference
    pub graph_id: u64,
    /// Is playing
    pub playing: bool,
    /// Current time
    pub time: f64,
    /// Parameter overrides
    pub overrides: HashMap<String, AudioValue>,
    /// Output buffer
    pub output_buffer: Vec<f32>,
}

impl MetaSoundsInstance {
    /// Create new instance
    #[must_use]
    pub fn new(graph_id: u64, block_size: usize) -> Self {
        Self {
            graph_id,
            playing: false,
            time: 0.0,
            overrides: HashMap::new(),
            output_buffer: vec![0.0; block_size * 2],
        }
    }

    /// Trigger playback
    pub fn trigger(&mut self) {
        self.playing = true;
        self.time = 0.0;
    }

    /// Stop playback
    pub fn stop(&mut self) {
        self.playing = false;
    }

    /// Set parameter override
    pub fn set_parameter(&mut self, name: &str, value: AudioValue) {
        self.overrides.insert(name.to_string(), value);
    }

    /// Process audio block
    pub fn process(&mut self, sample_rate: u32, _graph: &MetaSoundsGraph) {
        if !self.playing {
            self.output_buffer.fill(0.0);
            return;
        }

        let dt = 1.0 / sample_rate as f64;
        let samples = self.output_buffer.len() / 2;

        for i in 0..samples {
            // Simplified processing - would use actual graph
            let t = self.time + i as f64 * dt;
            let sample = (t * 440.0 * std::f64::consts::TAU).sin() as f32 * 0.5;
            
            self.output_buffer[i * 2] = sample;
            self.output_buffer[i * 2 + 1] = sample;
        }

        self.time += samples as f64 * dt;
    }
}
