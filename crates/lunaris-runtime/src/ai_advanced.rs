//! Advanced AI System
//!
//! NavMesh generation, behavior trees, and crowd simulation.

use glam::{Vec2, Vec3};
use std::collections::HashMap;

/// NavMesh
pub struct NavMesh {
    pub polygons: Vec<NavPolygon>,
    pub connections: Vec<NavConnection>,
    pub agents: Vec<NavAgent>,
    pub settings: NavMeshSettings,
}

/// Nav polygon
pub struct NavPolygon {
    pub id: u64,
    pub vertices: Vec<Vec3>,
    pub center: Vec3,
    pub area_type: AreaType,
    pub cost: f32,
}

/// Area type
pub enum AreaType { Walkable, Grass, Road, Water, Obstacle }

/// Nav connection
pub struct NavConnection {
    pub from: u64,
    pub to: u64,
    pub edge_start: Vec3,
    pub edge_end: Vec3,
    pub cost: f32,
}

/// Nav agent
pub struct NavAgent {
    pub id: u64,
    pub position: Vec3,
    pub target: Option<Vec3>,
    pub path: Vec<Vec3>,
    pub speed: f32,
    pub radius: f32,
    pub height: f32,
    pub state: AgentState,
}

/// Agent state
pub enum AgentState { Idle, Moving, Stuck, Arrived }

/// NavMesh settings
pub struct NavMeshSettings {
    pub cell_size: f32,
    pub cell_height: f32,
    pub agent_radius: f32,
    pub agent_height: f32,
    pub max_slope: f32,
    pub max_step: f32,
}

impl Default for NavMeshSettings {
    fn default() -> Self {
        Self { cell_size: 0.3, cell_height: 0.2, agent_radius: 0.5, agent_height: 2.0, max_slope: 45.0, max_step: 0.4 }
    }
}

impl NavMesh {
    pub fn new() -> Self {
        Self { polygons: Vec::new(), connections: Vec::new(), agents: Vec::new(), settings: NavMeshSettings::default() }
    }

    pub fn find_path(&self, start: Vec3, end: Vec3) -> Option<Vec<Vec3>> {
        // A* pathfinding
        let start_poly = self.find_polygon(start)?;
        let end_poly = self.find_polygon(end)?;
        
        // Simplified path
        Some(vec![start, end])
    }

    fn find_polygon(&self, pos: Vec3) -> Option<u64> {
        self.polygons.iter().find(|p| self.point_in_polygon(pos, p)).map(|p| p.id)
    }

    fn point_in_polygon(&self, pos: Vec3, poly: &NavPolygon) -> bool {
        let dist = (Vec2::new(pos.x, pos.z) - Vec2::new(poly.center.x, poly.center.z)).length();
        dist < 5.0 // Simplified
    }

    pub fn update_agents(&mut self, dt: f32) {
        for agent in &mut self.agents {
            if let AgentState::Moving = agent.state {
                if let Some(next) = agent.path.first() {
                    let dir = (*next - agent.position).normalize();
                    agent.position += dir * agent.speed * dt;
                    if (agent.position - *next).length() < 0.5 {
                        agent.path.remove(0);
                        if agent.path.is_empty() { agent.state = AgentState::Arrived; }
                    }
                }
            }
        }
    }

    pub fn add_agent(&mut self, position: Vec3) -> u64 {
        let id = self.agents.len() as u64;
        self.agents.push(NavAgent { id, position, target: None, path: Vec::new(), speed: 3.5, radius: 0.5, height: 2.0, state: AgentState::Idle });
        id
    }

    pub fn set_destination(&mut self, agent_id: u64, target: Vec3) {
        if let Some(agent) = self.agents.iter_mut().find(|a| a.id == agent_id) {
            agent.target = Some(target);
            if let Some(path) = self.find_path(agent.position, target) {
                agent.path = path;
                agent.state = AgentState::Moving;
            }
        }
    }
}

/// Behavior Tree
pub struct BehaviorTree {
    pub root: BehaviorNode,
    pub blackboard: Blackboard,
}

/// Behavior node
pub enum BehaviorNode {
    Selector(Vec<BehaviorNode>),
    Sequence(Vec<BehaviorNode>),
    Parallel(Vec<BehaviorNode>, u32), // min success count
    Inverter(Box<BehaviorNode>),
    Repeater(Box<BehaviorNode>, u32),
    Condition(String),
    Action(String),
}

/// Blackboard
pub struct Blackboard {
    pub data: HashMap<String, BlackboardValue>,
}

/// Blackboard value
pub enum BlackboardValue { Bool(bool), Int(i32), Float(f32), Vec3(Vec3), Entity(u64), String(String) }

/// Behavior status
#[derive(Clone, Copy, PartialEq)]
pub enum BehaviorStatus { Running, Success, Failure }

impl BehaviorTree {
    pub fn new(root: BehaviorNode) -> Self {
        Self { root, blackboard: Blackboard { data: HashMap::new() } }
    }

    pub fn tick(&mut self) -> BehaviorStatus {
        Self::tick_node(&self.root, &mut self.blackboard)
    }

    fn tick_node(node: &BehaviorNode, bb: &mut Blackboard) -> BehaviorStatus {
        match node {
            BehaviorNode::Selector(children) => {
                for child in children {
                    match Self::tick_node(child, bb) {
                        BehaviorStatus::Success => return BehaviorStatus::Success,
                        BehaviorStatus::Running => return BehaviorStatus::Running,
                        _ => {}
                    }
                }
                BehaviorStatus::Failure
            }
            BehaviorNode::Sequence(children) => {
                for child in children {
                    match Self::tick_node(child, bb) {
                        BehaviorStatus::Failure => return BehaviorStatus::Failure,
                        BehaviorStatus::Running => return BehaviorStatus::Running,
                        _ => {}
                    }
                }
                BehaviorStatus::Success
            }
            BehaviorNode::Condition(key) => {
                if let Some(BlackboardValue::Bool(v)) = bb.data.get(key) { if *v { BehaviorStatus::Success } else { BehaviorStatus::Failure } }
                else { BehaviorStatus::Failure }
            }
            BehaviorNode::Action(name) => {
                // Would execute action
                BehaviorStatus::Success
            }
            BehaviorNode::Inverter(child) => {
                match Self::tick_node(child, bb) {
                    BehaviorStatus::Success => BehaviorStatus::Failure,
                    BehaviorStatus::Failure => BehaviorStatus::Success,
                    s => s,
                }
            }
            _ => BehaviorStatus::Success,
        }
    }
}

/// Crowd simulation
pub struct CrowdSimulation {
    pub agents: Vec<CrowdAgent>,
    pub settings: CrowdSettings,
}

/// Crowd agent
pub struct CrowdAgent {
    pub position: Vec2,
    pub velocity: Vec2,
    pub target: Vec2,
    pub radius: f32,
    pub max_speed: f32,
}

/// Crowd settings
pub struct CrowdSettings {
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub target_weight: f32,
}

impl Default for CrowdSettings {
    fn default() -> Self {
        Self { separation_weight: 1.5, alignment_weight: 1.0, cohesion_weight: 1.0, target_weight: 2.0 }
    }
}

impl CrowdSimulation {
    pub fn new() -> Self { Self { agents: Vec::new(), settings: CrowdSettings::default() } }

    pub fn add_agent(&mut self, position: Vec2, target: Vec2) {
        self.agents.push(CrowdAgent { position, velocity: Vec2::ZERO, target, radius: 0.5, max_speed: 3.0 });
    }

    pub fn update(&mut self, dt: f32) {
        let positions: Vec<Vec2> = self.agents.iter().map(|a| a.position).collect();
        
        for (i, agent) in self.agents.iter_mut().enumerate() {
            let mut separation = Vec2::ZERO;
            let mut avg_pos = Vec2::ZERO;
            let mut count = 0;

            for (j, &other_pos) in positions.iter().enumerate() {
                if i != j {
                    let diff = agent.position - other_pos;
                    let dist = diff.length();
                    if dist < 3.0 && dist > 0.01 {
                        separation += diff.normalize() / dist;
                        avg_pos += other_pos;
                        count += 1;
                    }
                }
            }

            let mut force = Vec2::ZERO;
            force += separation * self.settings.separation_weight;
            if count > 0 { 
                avg_pos /= count as f32;
                force += (avg_pos - agent.position).normalize_or_zero() * self.settings.cohesion_weight;
            }
            force += (agent.target - agent.position).normalize_or_zero() * self.settings.target_weight;

            agent.velocity = (agent.velocity + force * dt).clamp_length_max(agent.max_speed);
            agent.position += agent.velocity * dt;
        }
    }
}
