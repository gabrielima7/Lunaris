//! Crowd Simulation
//!
//! Large-scale agent simulation with flocking and avoidance.

use glam::Vec3;
use std::collections::HashMap;

/// Crowd agent
#[derive(Debug, Clone)]
pub struct CrowdAgent {
    /// Agent ID
    pub id: u64,
    /// Position
    pub position: Vec3,
    /// Velocity
    pub velocity: Vec3,
    /// Desired velocity
    pub desired_velocity: Vec3,
    /// Radius
    pub radius: f32,
    /// Max speed
    pub max_speed: f32,
    /// Max force
    pub max_force: f32,
    /// Mass
    pub mass: f32,
    /// Group ID
    pub group: u32,
    /// Priority (higher = more important)
    pub priority: u32,
    /// Is active
    pub active: bool,
    /// Target position
    pub target: Option<Vec3>,
    /// Current state
    pub state: AgentState,
}

/// Agent state
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AgentState {
    /// Idle
    #[default]
    Idle,
    /// Walking
    Walking,
    /// Running
    Running,
    /// Avoiding
    Avoiding,
    /// Waiting
    Waiting,
}

impl CrowdAgent {
    /// Create a new agent
    #[must_use]
    pub fn new(id: u64, position: Vec3) -> Self {
        Self {
            id,
            position,
            velocity: Vec3::ZERO,
            desired_velocity: Vec3::ZERO,
            radius: 0.5,
            max_speed: 1.4,
            max_force: 5.0,
            mass: 80.0,
            group: 0,
            priority: 0,
            active: true,
            target: None,
            state: AgentState::Idle,
        }
    }

    /// Set target
    pub fn set_target(&mut self, target: Vec3) {
        self.target = Some(target);
        self.state = AgentState::Walking;
    }

    /// Clear target
    pub fn clear_target(&mut self) {
        self.target = None;
        self.state = AgentState::Idle;
    }

    /// Apply force
    pub fn apply_force(&mut self, force: Vec3) {
        let acceleration = force / self.mass;
        self.velocity += acceleration;
        
        // Clamp velocity
        let speed = self.velocity.length();
        if speed > self.max_speed {
            self.velocity = self.velocity.normalize() * self.max_speed;
        }
    }
}

/// Crowd configuration
#[derive(Debug, Clone)]
pub struct CrowdConfig {
    /// Separation weight
    pub separation_weight: f32,
    /// Alignment weight  
    pub alignment_weight: f32,
    /// Cohesion weight
    pub cohesion_weight: f32,
    /// Avoidance weight
    pub avoidance_weight: f32,
    /// Neighbor radius
    pub neighbor_radius: f32,
    /// Separation radius
    pub separation_radius: f32,
    /// Time horizon for avoidance
    pub time_horizon: f32,
    /// Max neighbors to consider
    pub max_neighbors: usize,
}

impl Default for CrowdConfig {
    fn default() -> Self {
        Self {
            separation_weight: 1.5,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
            avoidance_weight: 2.0,
            neighbor_radius: 5.0,
            separation_radius: 1.0,
            time_horizon: 2.0,
            max_neighbors: 10,
        }
    }
}

/// Spatial hash grid for fast neighbor queries
struct SpatialGrid {
    cell_size: f32,
    cells: HashMap<(i32, i32, i32), Vec<u64>>,
}

impl SpatialGrid {
    fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    fn clear(&mut self) {
        self.cells.clear();
    }

    fn cell(&self, pos: Vec3) -> (i32, i32, i32) {
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.y / self.cell_size).floor() as i32,
            (pos.z / self.cell_size).floor() as i32,
        )
    }

    fn insert(&mut self, id: u64, pos: Vec3) {
        let cell = self.cell(pos);
        self.cells.entry(cell).or_default().push(id);
    }

    fn query(&self, pos: Vec3, radius: f32) -> Vec<u64> {
        let mut result = Vec::new();
        let cells_radius = (radius / self.cell_size).ceil() as i32;
        let center = self.cell(pos);
        
        for dx in -cells_radius..=cells_radius {
            for dy in -cells_radius..=cells_radius {
                for dz in -cells_radius..=cells_radius {
                    let cell = (center.0 + dx, center.1 + dy, center.2 + dz);
                    if let Some(ids) = self.cells.get(&cell) {
                        result.extend(ids);
                    }
                }
            }
        }
        
        result
    }
}

/// Crowd manager
pub struct CrowdManager {
    /// Configuration
    pub config: CrowdConfig,
    /// All agents
    agents: HashMap<u64, CrowdAgent>,
    /// Spatial grid
    grid: SpatialGrid,
    /// Next agent ID
    next_id: u64,
    /// Obstacles
    obstacles: Vec<Obstacle>,
}

/// Static obstacle
#[derive(Debug, Clone)]
pub struct Obstacle {
    /// Position
    pub position: Vec3,
    /// Radius
    pub radius: f32,
    /// Height
    pub height: f32,
}

impl Default for CrowdManager {
    fn default() -> Self {
        Self::new(CrowdConfig::default())
    }
}

impl CrowdManager {
    /// Create a new crowd manager
    #[must_use]
    pub fn new(config: CrowdConfig) -> Self {
        Self {
            config,
            agents: HashMap::new(),
            grid: SpatialGrid::new(5.0),
            next_id: 1,
            obstacles: Vec::new(),
        }
    }

    /// Add an agent
    pub fn add_agent(&mut self, position: Vec3) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.agents.insert(id, CrowdAgent::new(id, position));
        id
    }

    /// Remove an agent
    pub fn remove_agent(&mut self, id: u64) {
        self.agents.remove(&id);
    }

    /// Get agent
    #[must_use]
    pub fn get(&self, id: u64) -> Option<&CrowdAgent> {
        self.agents.get(&id)
    }

    /// Get mutable agent
    pub fn get_mut(&mut self, id: u64) -> Option<&mut CrowdAgent> {
        self.agents.get_mut(&id)
    }

    /// Set agent target
    pub fn set_target(&mut self, id: u64, target: Vec3) {
        if let Some(agent) = self.agents.get_mut(&id) {
            agent.set_target(target);
        }
    }

    /// Add obstacle
    pub fn add_obstacle(&mut self, obstacle: Obstacle) {
        self.obstacles.push(obstacle);
    }

    /// Update crowd
    pub fn update(&mut self, delta_time: f32) {
        // Rebuild spatial grid
        self.grid.clear();
        for agent in self.agents.values() {
            if agent.active {
                self.grid.insert(agent.id, agent.position);
            }
        }

        // Calculate forces for each agent
        let agent_ids: Vec<u64> = self.agents.keys().copied().collect();
        let mut forces: HashMap<u64, Vec3> = HashMap::new();

        for &id in &agent_ids {
            let agent = &self.agents[&id];
            if !agent.active {
                continue;
            }

            let mut total_force = Vec3::ZERO;

            // Seek target
            if let Some(target) = agent.target {
                let desired = (target - agent.position).normalize() * agent.max_speed;
                let seek = desired - agent.velocity;
                total_force += seek;
            }

            // Get neighbors
            let neighbor_ids = self.grid.query(agent.position, self.config.neighbor_radius);
            let neighbors: Vec<&CrowdAgent> = neighbor_ids.iter()
                .filter(|&&nid| nid != id)
                .filter_map(|nid| self.agents.get(nid))
                .take(self.config.max_neighbors)
                .collect();

            // Flocking behaviors
            let separation = self.calculate_separation(agent, &neighbors);
            let alignment = self.calculate_alignment(agent, &neighbors);
            let cohesion = self.calculate_cohesion(agent, &neighbors);
            let avoidance = self.calculate_avoidance(agent, &neighbors);
            let obstacle_avoidance = self.calculate_obstacle_avoidance(agent);

            total_force += separation * self.config.separation_weight;
            total_force += alignment * self.config.alignment_weight;
            total_force += cohesion * self.config.cohesion_weight;
            total_force += avoidance * self.config.avoidance_weight;
            total_force += obstacle_avoidance * self.config.avoidance_weight;

            // Clamp force
            let force_mag = total_force.length();
            if force_mag > agent.max_force {
                total_force = total_force.normalize() * agent.max_force;
            }

            forces.insert(id, total_force);
        }

        // Apply forces and update positions
        for (&id, &force) in &forces {
            if let Some(agent) = self.agents.get_mut(&id) {
                agent.apply_force(force);
                agent.position += agent.velocity * delta_time;

                // Check if reached target
                if let Some(target) = agent.target {
                    if (target - agent.position).length() < 0.5 {
                        agent.clear_target();
                        agent.velocity = Vec3::ZERO;
                    }
                }

                // Apply friction when idle
                if agent.state == AgentState::Idle {
                    agent.velocity *= 0.9;
                }
            }
        }
    }

    fn calculate_separation(&self, agent: &CrowdAgent, neighbors: &[&CrowdAgent]) -> Vec3 {
        let mut steer = Vec3::ZERO;
        let mut count = 0;

        for neighbor in neighbors {
            let dist = (agent.position - neighbor.position).length();
            if dist > 0.0 && dist < self.config.separation_radius {
                let diff = (agent.position - neighbor.position).normalize() / dist;
                steer += diff;
                count += 1;
            }
        }

        if count > 0 {
            steer / count as f32
        } else {
            steer
        }
    }

    fn calculate_alignment(&self, agent: &CrowdAgent, neighbors: &[&CrowdAgent]) -> Vec3 {
        let mut avg_velocity = Vec3::ZERO;
        let mut count = 0;

        for neighbor in neighbors {
            if neighbor.group == agent.group {
                avg_velocity += neighbor.velocity;
                count += 1;
            }
        }

        if count > 0 {
            avg_velocity /= count as f32;
            avg_velocity.normalize_or_zero() * agent.max_speed - agent.velocity
        } else {
            Vec3::ZERO
        }
    }

    fn calculate_cohesion(&self, agent: &CrowdAgent, neighbors: &[&CrowdAgent]) -> Vec3 {
        let mut center = Vec3::ZERO;
        let mut count = 0;

        for neighbor in neighbors {
            if neighbor.group == agent.group {
                center += neighbor.position;
                count += 1;
            }
        }

        if count > 0 {
            center /= count as f32;
            let desired = (center - agent.position).normalize() * agent.max_speed;
            desired - agent.velocity
        } else {
            Vec3::ZERO
        }
    }

    fn calculate_avoidance(&self, agent: &CrowdAgent, neighbors: &[&CrowdAgent]) -> Vec3 {
        let mut avoidance = Vec3::ZERO;

        for neighbor in neighbors {
            let relative_pos = neighbor.position - agent.position;
            let relative_vel = agent.velocity - neighbor.velocity;
            
            let dist = relative_pos.length();
            let combined_radius = agent.radius + neighbor.radius;
            
            if dist < combined_radius * 2.0 {
                // Time to collision
                let ttc = -relative_pos.dot(relative_vel) / relative_vel.length_squared().max(0.001);
                
                if ttc > 0.0 && ttc < self.config.time_horizon {
                    let collision_point = relative_pos + relative_vel * ttc;
                    let avoid_dir = -collision_point.normalize();
                    avoidance += avoid_dir * (1.0 / ttc.max(0.1));
                }
            }
        }

        avoidance
    }

    fn calculate_obstacle_avoidance(&self, agent: &CrowdAgent) -> Vec3 {
        let mut avoidance = Vec3::ZERO;

        for obstacle in &self.obstacles {
            let to_obstacle = obstacle.position - agent.position;
            let dist = to_obstacle.length();
            let combined_radius = agent.radius + obstacle.radius;

            if dist < combined_radius * 2.0 {
                let away = -to_obstacle.normalize();
                let strength = 1.0 / (dist - combined_radius).max(0.1);
                avoidance += away * strength;
            }
        }

        avoidance
    }

    /// Get agent count
    #[must_use]
    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }

    /// Get all agent positions for rendering
    #[must_use]
    pub fn positions(&self) -> Vec<(u64, Vec3)> {
        self.agents.iter()
            .filter(|(_, a)| a.active)
            .map(|(&id, a)| (id, a.position))
            .collect()
    }
}
