//! AI Navigation System
//!
//! Pathfinding and navigation mesh for AI agents.

use lunaris_core::math::Vec3;
use std::collections::{BinaryHeap, HashMap, HashSet};

/// Navigation mesh polygon
#[derive(Debug, Clone)]
pub struct NavMeshPolygon {
    /// Polygon ID
    pub id: u32,
    /// Vertices (indices into navmesh vertices)
    pub vertices: Vec<u32>,
    /// Center point
    pub center: Vec3,
    /// Neighbor polygon IDs
    pub neighbors: Vec<u32>,
    /// Area type
    pub area: NavArea,
    /// Is walkable
    pub walkable: bool,
}

/// Navigation area type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavArea {
    /// Default walkable area
    #[default]
    Walkable,
    /// Water (swimable)
    Water,
    /// Road (faster)
    Road,
    /// Grass
    Grass,
    /// Jump (requires ability)
    Jump,
    /// Off-mesh link
    OffMeshLink,
    /// Not walkable
    NotWalkable,
}

impl NavArea {
    /// Get movement cost multiplier
    #[must_use]
    pub fn cost(&self) -> f32 {
        match self {
            Self::Walkable => 1.0,
            Self::Water => 3.0,
            Self::Road => 0.5,
            Self::Grass => 1.2,
            Self::Jump => 2.0,
            Self::OffMeshLink => 1.0,
            Self::NotWalkable => f32::INFINITY,
        }
    }
}

/// Navigation mesh
#[derive(Debug, Clone)]
pub struct NavMesh {
    /// Vertices
    pub vertices: Vec<Vec3>,
    /// Polygons
    pub polygons: Vec<NavMeshPolygon>,
    /// Bounds min
    pub bounds_min: Vec3,
    /// Bounds max
    pub bounds_max: Vec3,
}

impl Default for NavMesh {
    fn default() -> Self {
        Self::new()
    }
}

impl NavMesh {
    /// Create an empty navmesh
    #[must_use]
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            polygons: Vec::new(),
            bounds_min: Vec3::ZERO,
            bounds_max: Vec3::ZERO,
        }
    }

    /// Create a simple grid navmesh
    #[must_use]
    pub fn grid(width: u32, height: u32, cell_size: f32) -> Self {
        let mut vertices = Vec::new();
        let mut polygons = Vec::new();

        // Create vertices
        for z in 0..=height {
            for x in 0..=width {
                vertices.push(Vec3::new(
                    x as f32 * cell_size,
                    0.0,
                    z as f32 * cell_size,
                ));
            }
        }

        // Create polygons (quads)
        let stride = width + 1;
        for z in 0..height {
            for x in 0..width {
                let base = z * stride + x;
                let poly_id = (z * width + x) as u32;

                // Calculate neighbors
                let mut neighbors = Vec::new();
                if x > 0 { neighbors.push(poly_id - 1); }
                if x < width - 1 { neighbors.push(poly_id + 1); }
                if z > 0 { neighbors.push(poly_id - width); }
                if z < height - 1 { neighbors.push(poly_id + width); }

                let verts = vec![base, base + 1, base + stride + 1, base + stride];
                let center = Vec3::new(
                    (x as f32 + 0.5) * cell_size,
                    0.0,
                    (z as f32 + 0.5) * cell_size,
                );

                polygons.push(NavMeshPolygon {
                    id: poly_id,
                    vertices: verts,
                    center,
                    neighbors,
                    area: NavArea::Walkable,
                    walkable: true,
                });
            }
        }

        Self {
            vertices,
            polygons,
            bounds_min: Vec3::ZERO,
            bounds_max: Vec3::new(width as f32 * cell_size, 0.0, height as f32 * cell_size),
        }
    }

    /// Find the polygon containing a point
    #[must_use]
    pub fn find_polygon(&self, point: Vec3) -> Option<u32> {
        // Simple brute force for now
        for poly in &self.polygons {
            if !poly.walkable {
                continue;
            }
            // Check if point is inside polygon (simplified)
            let dist = (poly.center - point).length();
            if dist < 2.0 {
                return Some(poly.id);
            }
        }
        None
    }

    /// Get polygon by ID
    #[must_use]
    pub fn polygon(&self, id: u32) -> Option<&NavMeshPolygon> {
        self.polygons.get(id as usize)
    }
}

/// A* pathfinding node
#[derive(Debug, Clone, Copy)]
struct PathNode {
    poly_id: u32,
    g_cost: f32,
    f_cost: f32,
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.poly_id == other.poly_id
    }
}

impl Eq for PathNode {}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_cost.partial_cmp(&self.f_cost).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Navigation path
#[derive(Debug, Clone, Default)]
pub struct NavPath {
    /// Path points
    pub points: Vec<Vec3>,
    /// Path length
    pub length: f32,
    /// Is complete
    pub complete: bool,
}

impl NavPath {
    /// Create an empty path
    #[must_use]
    pub fn empty() -> Self {
        Self {
            points: Vec::new(),
            length: 0.0,
            complete: false,
        }
    }

    /// Check if path is valid
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.points.is_empty()
    }
}

/// Navigation agent
#[derive(Debug, Clone)]
pub struct NavAgent {
    /// Current position
    pub position: Vec3,
    /// Target position
    pub target: Option<Vec3>,
    /// Current path
    pub path: NavPath,
    /// Current path index
    pub path_index: usize,
    /// Agent radius
    pub radius: f32,
    /// Agent height
    pub height: f32,
    /// Max speed
    pub max_speed: f32,
    /// Acceleration
    pub acceleration: f32,
    /// Angular speed
    pub angular_speed: f32,
    /// Stopping distance
    pub stopping_distance: f32,
    /// Current velocity
    pub velocity: Vec3,
    /// Is moving
    pub is_moving: bool,
}

impl Default for NavAgent {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            target: None,
            path: NavPath::empty(),
            path_index: 0,
            radius: 0.5,
            height: 2.0,
            max_speed: 5.0,
            acceleration: 10.0,
            angular_speed: 360.0_f32.to_radians(),
            stopping_distance: 0.5,
            velocity: Vec3::ZERO,
            is_moving: false,
        }
    }
}

impl NavAgent {
    /// Create a new nav agent
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set destination
    pub fn set_destination(&mut self, target: Vec3, navmesh: &NavMesh) {
        self.target = Some(target);
        self.path = find_path(navmesh, self.position, target);
        self.path_index = 0;
        self.is_moving = self.path.is_valid();
    }

    /// Stop movement
    pub fn stop(&mut self) {
        self.target = None;
        self.path = NavPath::empty();
        self.is_moving = false;
        self.velocity = Vec3::ZERO;
    }

    /// Update agent
    pub fn update(&mut self, delta_time: f32) {
        if !self.is_moving || self.path.points.is_empty() {
            return;
        }

        // Get current waypoint
        if self.path_index >= self.path.points.len() {
            self.is_moving = false;
            return;
        }

        let waypoint = self.path.points[self.path_index];
        let to_waypoint = waypoint - self.position;
        let distance = to_waypoint.length();

        // Check if reached waypoint
        if distance < self.stopping_distance {
            self.path_index += 1;
            if self.path_index >= self.path.points.len() {
                self.is_moving = false;
                self.velocity = Vec3::ZERO;
            }
            return;
        }

        // Move towards waypoint
        let direction = to_waypoint.normalize();
        let target_velocity = direction * self.max_speed;
        
        // Smooth acceleration
        self.velocity = self.velocity.lerp(target_velocity, self.acceleration * delta_time);
        self.position = self.position + self.velocity * delta_time;
    }

    /// Check if reached destination
    #[must_use]
    pub fn has_reached_destination(&self) -> bool {
        !self.is_moving && self.target.is_some()
    }

    /// Get remaining distance
    #[must_use]
    pub fn remaining_distance(&self) -> f32 {
        if !self.is_moving {
            return 0.0;
        }
        
        let mut distance = 0.0;
        let mut prev = self.position;
        
        for i in self.path_index..self.path.points.len() {
            distance += (self.path.points[i] - prev).length();
            prev = self.path.points[i];
        }
        
        distance
    }
}

/// Find path using A*
#[must_use]
pub fn find_path(navmesh: &NavMesh, start: Vec3, end: Vec3) -> NavPath {
    let start_poly = match navmesh.find_polygon(start) {
        Some(id) => id,
        None => return NavPath::empty(),
    };
    
    let end_poly = match navmesh.find_polygon(end) {
        Some(id) => id,
        None => return NavPath::empty(),
    };

    if start_poly == end_poly {
        return NavPath {
            points: vec![start, end],
            length: (end - start).length(),
            complete: true,
        };
    }

    // A* pathfinding
    let mut open = BinaryHeap::new();
    let mut came_from: HashMap<u32, u32> = HashMap::new();
    let mut g_score: HashMap<u32, f32> = HashMap::new();
    let mut closed: HashSet<u32> = HashSet::new();

    g_score.insert(start_poly, 0.0);
    open.push(PathNode {
        poly_id: start_poly,
        g_cost: 0.0,
        f_cost: heuristic(navmesh, start_poly, end_poly),
    });

    while let Some(current) = open.pop() {
        if current.poly_id == end_poly {
            // Reconstruct path
            let mut path_polys = vec![end_poly];
            let mut current_id = end_poly;
            while let Some(&prev) = came_from.get(&current_id) {
                path_polys.push(prev);
                current_id = prev;
            }
            path_polys.reverse();

            // Convert to points
            let mut points = vec![start];
            for poly_id in &path_polys[1..path_polys.len()-1] {
                if let Some(poly) = navmesh.polygon(*poly_id) {
                    points.push(poly.center);
                }
            }
            points.push(end);

            let mut length = 0.0;
            for i in 1..points.len() {
                length += (points[i] - points[i-1]).length();
            }

            return NavPath {
                points,
                length,
                complete: true,
            };
        }

        closed.insert(current.poly_id);

        let current_polygon = match navmesh.polygon(current.poly_id) {
            Some(p) => p,
            None => continue,
        };

        for &neighbor_id in &current_polygon.neighbors {
            if closed.contains(&neighbor_id) {
                continue;
            }

            let neighbor = match navmesh.polygon(neighbor_id) {
                Some(p) if p.walkable => p,
                _ => continue,
            };

            let move_cost = (neighbor.center - current_polygon.center).length() * neighbor.area.cost();
            let tentative_g = g_score.get(&current.poly_id).unwrap_or(&f32::INFINITY) + move_cost;

            if tentative_g < *g_score.get(&neighbor_id).unwrap_or(&f32::INFINITY) {
                came_from.insert(neighbor_id, current.poly_id);
                g_score.insert(neighbor_id, tentative_g);
                
                open.push(PathNode {
                    poly_id: neighbor_id,
                    g_cost: tentative_g,
                    f_cost: tentative_g + heuristic(navmesh, neighbor_id, end_poly),
                });
            }
        }
    }

    NavPath::empty()
}

fn heuristic(navmesh: &NavMesh, from: u32, to: u32) -> f32 {
    let from_poly = navmesh.polygon(from);
    let to_poly = navmesh.polygon(to);
    
    match (from_poly, to_poly) {
        (Some(f), Some(t)) => (t.center - f.center).length(),
        _ => f32::INFINITY,
    }
}

/// Behavior tree node result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BehaviorStatus {
    /// Still running
    Running,
    /// Succeeded
    Success,
    /// Failed
    Failure,
}

/// Behavior tree node
pub trait BehaviorNode: Send + Sync {
    /// Execute the node
    fn execute(&mut self, context: &mut BehaviorContext) -> BehaviorStatus;
    /// Reset the node
    fn reset(&mut self);
}

/// Behavior tree context
pub struct BehaviorContext {
    /// Blackboard (shared state)
    pub blackboard: HashMap<String, BlackboardValue>,
    /// Delta time
    pub delta_time: f32,
}

/// Blackboard value
#[derive(Debug, Clone)]
pub enum BlackboardValue {
    /// Boolean
    Bool(bool),
    /// Integer
    Int(i32),
    /// Float
    Float(f32),
    /// Vector3
    Vec3(Vec3),
    /// String
    String(String),
}

impl Default for BehaviorContext {
    fn default() -> Self {
        Self {
            blackboard: HashMap::new(),
            delta_time: 0.0,
        }
    }
}

impl BehaviorContext {
    /// Get a bool from blackboard
    #[must_use]
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.blackboard.get(key)? {
            BlackboardValue::Bool(v) => Some(*v),
            _ => None,
        }
    }

    /// Set a bool in blackboard
    pub fn set_bool(&mut self, key: &str, value: bool) {
        self.blackboard.insert(key.to_string(), BlackboardValue::Bool(value));
    }

    /// Get a float from blackboard
    #[must_use]
    pub fn get_float(&self, key: &str) -> Option<f32> {
        match self.blackboard.get(key)? {
            BlackboardValue::Float(v) => Some(*v),
            _ => None,
        }
    }

    /// Set a float in blackboard
    pub fn set_float(&mut self, key: &str, value: f32) {
        self.blackboard.insert(key.to_string(), BlackboardValue::Float(value));
    }

    /// Get a Vec3 from blackboard
    #[must_use]
    pub fn get_vec3(&self, key: &str) -> Option<Vec3> {
        match self.blackboard.get(key)? {
            BlackboardValue::Vec3(v) => Some(*v),
            _ => None,
        }
    }

    /// Set a Vec3 in blackboard
    pub fn set_vec3(&mut self, key: &str, value: Vec3) {
        self.blackboard.insert(key.to_string(), BlackboardValue::Vec3(value));
    }
}

/// Sequence node (runs children in order until one fails)
pub struct Sequence {
    children: Vec<Box<dyn BehaviorNode>>,
    current_child: usize,
}

impl Sequence {
    /// Create a new sequence
    #[must_use]
    pub fn new(children: Vec<Box<dyn BehaviorNode>>) -> Self {
        Self {
            children,
            current_child: 0,
        }
    }
}

impl BehaviorNode for Sequence {
    fn execute(&mut self, context: &mut BehaviorContext) -> BehaviorStatus {
        while self.current_child < self.children.len() {
            match self.children[self.current_child].execute(context) {
                BehaviorStatus::Running => return BehaviorStatus::Running,
                BehaviorStatus::Failure => {
                    self.reset();
                    return BehaviorStatus::Failure;
                }
                BehaviorStatus::Success => {
                    self.current_child += 1;
                }
            }
        }
        self.reset();
        BehaviorStatus::Success
    }

    fn reset(&mut self) {
        self.current_child = 0;
        for child in &mut self.children {
            child.reset();
        }
    }
}

/// Selector node (runs children until one succeeds)
pub struct Selector {
    children: Vec<Box<dyn BehaviorNode>>,
    current_child: usize,
}

impl Selector {
    /// Create a new selector
    #[must_use]
    pub fn new(children: Vec<Box<dyn BehaviorNode>>) -> Self {
        Self {
            children,
            current_child: 0,
        }
    }
}

impl BehaviorNode for Selector {
    fn execute(&mut self, context: &mut BehaviorContext) -> BehaviorStatus {
        while self.current_child < self.children.len() {
            match self.children[self.current_child].execute(context) {
                BehaviorStatus::Running => return BehaviorStatus::Running,
                BehaviorStatus::Success => {
                    self.reset();
                    return BehaviorStatus::Success;
                }
                BehaviorStatus::Failure => {
                    self.current_child += 1;
                }
            }
        }
        self.reset();
        BehaviorStatus::Failure
    }

    fn reset(&mut self) {
        self.current_child = 0;
        for child in &mut self.children {
            child.reset();
        }
    }
}
