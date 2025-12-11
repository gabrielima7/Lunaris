//! Chaos-like Advanced Physics
//!
//! Advanced physics features matching Unreal's Chaos physics.

use glam::{Vec3, Quat, Mat4};
use std::collections::HashMap;

/// Physics solver type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicsSolver {
    /// Sequential impulse (default)
    SequentialImpulse,
    /// Position-based dynamics
    PBD,
    /// Extended position-based dynamics
    XPBD,
    /// Temporal Gauss-Seidel
    TGS,
}

/// Advanced physics config
#[derive(Debug, Clone)]
pub struct ChaosConfig {
    /// Solver type
    pub solver: PhysicsSolver,
    /// Solver iterations
    pub iterations: u32,
    /// Position iterations
    pub position_iterations: u32,
    /// Velocity iterations
    pub velocity_iterations: u32,
    /// Substeps per frame
    pub substeps: u32,
    /// Enable CCD (continuous collision detection)
    pub ccd: bool,
    /// Gravity
    pub gravity: Vec3,
    /// Sleep threshold
    pub sleep_threshold: f32,
    /// Linear damping
    pub linear_damping: f32,
    /// Angular damping
    pub angular_damping: f32,
    /// Max depenetration velocity
    pub max_depenetration: f32,
}

impl Default for ChaosConfig {
    fn default() -> Self {
        Self {
            solver: PhysicsSolver::TGS,
            iterations: 8,
            position_iterations: 2,
            velocity_iterations: 1,
            substeps: 2,
            ccd: true,
            gravity: Vec3::new(0.0, -9.81, 0.0),
            sleep_threshold: 0.05,
            linear_damping: 0.01,
            angular_damping: 0.05,
            max_depenetration: 10.0,
        }
    }
}

/// Geometry collection (destructible)
#[derive(Debug, Clone)]
pub struct GeometryCollection {
    /// ID
    pub id: u64,
    /// Geometry pieces
    pub pieces: Vec<GeometryPiece>,
    /// Cluster connections
    pub connections: Vec<ClusterConnection>,
    /// Is simulating
    pub simulating: bool,
    /// Root index
    pub root: usize,
}

/// Geometry piece
#[derive(Debug, Clone)]
pub struct GeometryPiece {
    /// Local transform
    pub transform: Mat4,
    /// Mass
    pub mass: f32,
    /// Volume
    pub volume: f32,
    /// Is kinematic
    pub kinematic: bool,
    /// Parent index
    pub parent: Option<usize>,
    /// Children indices
    pub children: Vec<usize>,
    /// Damage threshold
    pub damage_threshold: f32,
    /// Current damage
    pub damage: f32,
    /// Is broken
    pub broken: bool,
}

/// Cluster connection
#[derive(Debug, Clone)]
pub struct ClusterConnection {
    /// Piece A
    pub piece_a: usize,
    /// Piece B
    pub piece_b: usize,
    /// Break strain
    pub strain_threshold: f32,
    /// Current strain
    pub current_strain: f32,
    /// Is broken
    pub broken: bool,
    /// Connection type
    pub connection_type: ConnectionType,
}

/// Connection type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    /// Rigid connection
    Rigid,
    /// Soft connection (spring-like)
    Soft,
    /// Contact only
    Contact,
}

/// Field system (forces)
#[derive(Debug, Clone)]
pub struct PhysicsField {
    /// Field ID
    pub id: u64,
    /// Field type
    pub field_type: FieldType,
    /// Position
    pub position: Vec3,
    /// Radius
    pub radius: f32,
    /// Strength
    pub strength: f32,
    /// Falloff
    pub falloff: FieldFalloff,
    /// Is enabled
    pub enabled: bool,
}

/// Field type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    /// Radial force (explosion)
    Radial,
    /// Directional force (wind)
    Directional,
    /// Point attractor
    Attractor,
    /// Vortex
    Vortex,
    /// Noise
    Noise,
    /// Kill field
    Kill,
    /// Sleep field
    Sleep,
    /// Disable field
    Disable,
}

/// Field falloff
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldFalloff {
    None,
    Linear,
    Squared,
    Inverse,
    InverseSquared,
}

/// Constraint types
#[derive(Debug, Clone)]
pub enum AdvancedConstraint {
    /// Distance constraint
    Distance {
        body_a: u64,
        body_b: u64,
        anchor_a: Vec3,
        anchor_b: Vec3,
        min_distance: f32,
        max_distance: f32,
        stiffness: f32,
    },
    /// Ball socket (point)
    BallSocket {
        body_a: u64,
        body_b: u64,
        anchor: Vec3,
        swing_limit: Option<f32>,
        twist_limit: Option<f32>,
    },
    /// Hinge
    Hinge {
        body_a: u64,
        body_b: u64,
        anchor: Vec3,
        axis: Vec3,
        limits: Option<(f32, f32)>,
        motor: Option<HingeMotor>,
    },
    /// Slider (prismatic)
    Slider {
        body_a: u64,
        body_b: u64,
        axis: Vec3,
        limits: Option<(f32, f32)>,
        motor: Option<LinearMotor>,
    },
    /// Cone twist
    ConeTwist {
        body_a: u64,
        body_b: u64,
        anchor: Vec3,
        swing_span1: f32,
        swing_span2: f32,
        twist_span: f32,
        softness: f32,
    },
    /// 6-DOF
    SixDOF {
        body_a: u64,
        body_b: u64,
        anchor: Vec3,
        linear_limits: [(f32, f32); 3],
        angular_limits: [(f32, f32); 3],
        stiffness: [f32; 6],
        damping: [f32; 6],
    },
}

/// Hinge motor
#[derive(Debug, Clone)]
pub struct HingeMotor {
    /// Target velocity
    pub velocity: f32,
    /// Max impulse
    pub max_impulse: f32,
    /// Is enabled
    pub enabled: bool,
}

/// Linear motor
#[derive(Debug, Clone)]
pub struct LinearMotor {
    /// Target velocity
    pub velocity: f32,
    /// Max force
    pub max_force: f32,
    /// Is enabled
    pub enabled: bool,
}

/// Anchor system
#[derive(Debug, Clone)]
pub struct Anchor {
    /// Anchor ID
    pub id: u64,
    /// World position
    pub position: Vec3,
    /// Affected bodies
    pub bodies: Vec<u64>,
    /// Break threshold
    pub break_threshold: f32,
    /// Is broken
    pub broken: bool,
}

/// Chaos physics manager
pub struct ChaosPhysics {
    /// Configuration
    pub config: ChaosConfig,
    /// Geometry collections
    collections: HashMap<u64, GeometryCollection>,
    /// Fields
    fields: HashMap<u64, PhysicsField>,
    /// Constraints
    constraints: Vec<AdvancedConstraint>,
    /// Anchors
    anchors: HashMap<u64, Anchor>,
    /// Next ID
    next_id: u64,
    /// Frame count
    frame: u64,
}

impl Default for ChaosPhysics {
    fn default() -> Self {
        Self::new()
    }
}

impl ChaosPhysics {
    /// Create new Chaos physics
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: ChaosConfig::default(),
            collections: HashMap::new(),
            fields: HashMap::new(),
            constraints: Vec::new(),
            anchors: HashMap::new(),
            next_id: 1,
            frame: 0,
        }
    }

    /// Create geometry collection
    pub fn create_collection(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let collection = GeometryCollection {
            id,
            pieces: Vec::new(),
            connections: Vec::new(),
            simulating: false,
            root: 0,
        };

        self.collections.insert(id, collection);
        id
    }

    /// Add piece to collection
    pub fn add_piece(&mut self, collection_id: u64, piece: GeometryPiece) -> Option<usize> {
        if let Some(collection) = self.collections.get_mut(&collection_id) {
            let idx = collection.pieces.len();
            collection.pieces.push(piece);
            Some(idx)
        } else {
            None
        }
    }

    /// Add connection
    pub fn add_connection(&mut self, collection_id: u64, connection: ClusterConnection) {
        if let Some(collection) = self.collections.get_mut(&collection_id) {
            collection.connections.push(connection);
        }
    }

    /// Apply damage
    pub fn apply_damage(&mut self, collection_id: u64, piece_idx: usize, damage: f32) {
        if let Some(collection) = self.collections.get_mut(&collection_id) {
            if let Some(piece) = collection.pieces.get_mut(piece_idx) {
                piece.damage += damage;
                if piece.damage >= piece.damage_threshold {
                    piece.broken = true;
                }
            }
        }
    }

    /// Create field
    pub fn create_field(&mut self, field: PhysicsField) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        
        let mut field = field;
        field.id = id;
        self.fields.insert(id, field);
        id
    }

    /// Apply explosion
    pub fn apply_explosion(&mut self, position: Vec3, radius: f32, strength: f32) {
        let field = PhysicsField {
            id: 0,
            field_type: FieldType::Radial,
            position,
            radius,
            strength,
            falloff: FieldFalloff::InverseSquared,
            enabled: true,
        };
        
        self.create_field(field);
    }

    /// Add constraint
    pub fn add_constraint(&mut self, constraint: AdvancedConstraint) {
        self.constraints.push(constraint);
    }

    /// Create anchor
    pub fn create_anchor(&mut self, position: Vec3, bodies: Vec<u64>, break_threshold: f32) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let anchor = Anchor {
            id,
            position,
            bodies,
            break_threshold,
            broken: false,
        };

        self.anchors.insert(id, anchor);
        id
    }

    /// Step simulation
    pub fn step(&mut self, dt: f32) {
        self.frame += 1;
        let substep_dt = dt / self.config.substeps as f32;

        for _ in 0..self.config.substeps {
            self.substep(substep_dt);
        }
    }

    fn substep(&mut self, dt: f32) {
        // Apply fields to collections
        for field in self.fields.values() {
            if !field.enabled {
                continue;
            }
            // Would apply forces to nearby bodies
        }

        // Check connection strains
        for collection in self.collections.values_mut() {
            for connection in &mut collection.connections {
                if connection.current_strain >= connection.strain_threshold {
                    connection.broken = true;
                }
            }
        }

        // Check anchors
        for anchor in self.anchors.values_mut() {
            // Would check strain on anchor
        }
    }

    /// Get statistics
    #[must_use]
    pub fn stats(&self) -> ChaosStats {
        let total_pieces: usize = self.collections.values()
            .map(|c| c.pieces.len())
            .sum();
        
        let broken_pieces: usize = self.collections.values()
            .flat_map(|c| &c.pieces)
            .filter(|p| p.broken)
            .count();

        ChaosStats {
            collections: self.collections.len(),
            total_pieces,
            broken_pieces,
            fields: self.fields.len(),
            constraints: self.constraints.len(),
            anchors: self.anchors.len(),
        }
    }
}

/// Chaos physics statistics
#[derive(Debug, Clone)]
pub struct ChaosStats {
    pub collections: usize,
    pub total_pieces: usize,
    pub broken_pieces: usize,
    pub fields: usize,
    pub constraints: usize,
    pub anchors: usize,
}
