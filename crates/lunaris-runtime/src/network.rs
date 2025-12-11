//! Networking System
//!
//! Client/Server architecture for multiplayer games.

use lunaris_core::{id::Id, Result};
use std::collections::HashMap;
use std::net::SocketAddr;

/// Network configuration
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// Max connections (server only)
    pub max_connections: usize,
    /// Tick rate (updates per second)
    pub tick_rate: u32,
    /// Connection timeout in seconds
    pub timeout: f32,
    /// Enable compression
    pub compression: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            max_connections: 64,
            tick_rate: 60,
            timeout: 10.0,
            compression: true,
        }
    }
}

/// Network role
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkRole {
    /// No networking
    None,
    /// Server
    Server,
    /// Client
    Client,
    /// Listen server (server + local client)
    ListenServer,
}

/// Connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    /// Disconnected
    Disconnected,
    /// Connecting
    Connecting,
    /// Connected
    Connected,
    /// Disconnecting
    Disconnecting,
}

/// Client ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientId(pub u32);

/// Network peer (client or server connection)
#[derive(Debug, Clone)]
pub struct NetworkPeer {
    /// Peer ID
    pub id: ClientId,
    /// Address
    pub address: Option<SocketAddr>,
    /// Connection state
    pub state: ConnectionState,
    /// Latency in milliseconds
    pub latency: f32,
    /// Packet loss percentage
    pub packet_loss: f32,
}

/// Network message
#[derive(Debug, Clone)]
pub struct NetworkMessage {
    /// Message type
    pub msg_type: MessageType,
    /// Channel
    pub channel: Channel,
    /// Payload
    pub payload: Vec<u8>,
}

/// Message type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    /// Connection request
    Connect,
    /// Disconnect
    Disconnect,
    /// Ping
    Ping,
    /// Pong
    Pong,
    /// Game state update
    StateUpdate,
    /// Player input
    Input,
    /// Entity spawn
    Spawn,
    /// Entity despawn
    Despawn,
    /// RPC call
    Rpc,
    /// Custom message
    Custom(u16),
}

/// Network channel
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    /// Unreliable (UDP-like)
    Unreliable,
    /// Reliable ordered
    ReliableOrdered,
    /// Reliable unordered
    ReliableUnordered,
}

/// Network entity (replicated across network)
#[derive(Debug, Clone)]
pub struct NetworkEntity {
    /// Network ID
    pub net_id: u32,
    /// Entity ID (local ECS)
    pub entity_id: u64,
    /// Owner client
    pub owner: ClientId,
    /// Is local authority
    pub is_authority: bool,
}

/// Network server
pub struct NetworkServer {
    /// Configuration
    config: NetworkConfig,
    /// Is running
    running: bool,
    /// Connected clients
    clients: HashMap<ClientId, NetworkPeer>,
    /// Network entities
    entities: HashMap<u32, NetworkEntity>,
    /// Next entity ID
    next_entity_id: u32,
    /// Next client ID
    next_client_id: u32,
    /// Outgoing messages
    outgoing: Vec<(Option<ClientId>, NetworkMessage)>,
}

impl Default for NetworkServer {
    fn default() -> Self {
        Self::new(NetworkConfig::default())
    }
}

impl NetworkServer {
    /// Create a new server
    #[must_use]
    pub fn new(config: NetworkConfig) -> Self {
        Self {
            config,
            running: false,
            clients: HashMap::new(),
            entities: HashMap::new(),
            next_entity_id: 1,
            next_client_id: 1,
            outgoing: Vec::new(),
        }
    }

    /// Start the server
    ///
    /// # Errors
    ///
    /// Returns error if server fails to start
    pub fn start(&mut self, _port: u16) -> Result<()> {
        self.running = true;
        tracing::info!("Network server started");
        Ok(())
    }

    /// Stop the server
    pub fn stop(&mut self) {
        self.running = false;
        self.clients.clear();
        tracing::info!("Network server stopped");
    }

    /// Check if running
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Get client count
    #[must_use]
    pub fn client_count(&self) -> usize {
        self.clients.len()
    }

    /// Spawn a network entity
    pub fn spawn_entity(&mut self, owner: ClientId) -> u32 {
        let net_id = self.next_entity_id;
        self.next_entity_id += 1;

        self.entities.insert(net_id, NetworkEntity {
            net_id,
            entity_id: 0, // Would be set by ECS
            owner,
            is_authority: true,
        });

        // Broadcast spawn to all clients
        self.broadcast(NetworkMessage {
            msg_type: MessageType::Spawn,
            channel: Channel::ReliableOrdered,
            payload: net_id.to_le_bytes().to_vec(),
        });

        net_id
    }

    /// Despawn a network entity
    pub fn despawn_entity(&mut self, net_id: u32) {
        self.entities.remove(&net_id);
        self.broadcast(NetworkMessage {
            msg_type: MessageType::Despawn,
            channel: Channel::ReliableOrdered,
            payload: net_id.to_le_bytes().to_vec(),
        });
    }

    /// Send to a specific client
    pub fn send(&mut self, client: ClientId, message: NetworkMessage) {
        self.outgoing.push((Some(client), message));
    }

    /// Broadcast to all clients
    pub fn broadcast(&mut self, message: NetworkMessage) {
        self.outgoing.push((None, message));
    }

    /// Update server
    pub fn update(&mut self, _delta_time: f32) {
        if !self.running {
            return;
        }

        // Process incoming messages (simulated)
        // In real implementation, would poll socket

        // Send outgoing messages
        self.outgoing.clear();
    }

    /// Get all connected clients
    #[must_use]
    pub fn clients(&self) -> impl Iterator<Item = &NetworkPeer> {
        self.clients.values()
    }
}

/// Network client
pub struct NetworkClient {
    /// Configuration
    config: NetworkConfig,
    /// Connection state
    state: ConnectionState,
    /// Server address
    server_address: Option<SocketAddr>,
    /// Local client ID (assigned by server)
    client_id: Option<ClientId>,
    /// Network entities
    entities: HashMap<u32, NetworkEntity>,
    /// Outgoing messages
    outgoing: Vec<NetworkMessage>,
    /// Latency
    latency: f32,
}

impl Default for NetworkClient {
    fn default() -> Self {
        Self::new(NetworkConfig::default())
    }
}

impl NetworkClient {
    /// Create a new client
    #[must_use]
    pub fn new(config: NetworkConfig) -> Self {
        Self {
            config,
            state: ConnectionState::Disconnected,
            server_address: None,
            client_id: None,
            entities: HashMap::new(),
            outgoing: Vec::new(),
            latency: 0.0,
        }
    }

    /// Connect to a server
    ///
    /// # Errors
    ///
    /// Returns error if connection fails
    pub fn connect(&mut self, address: &str) -> Result<()> {
        let addr: SocketAddr = address.parse()
            .map_err(|e| lunaris_core::Error::Internal(format!("Invalid address: {}", e)))?;
        
        self.server_address = Some(addr);
        self.state = ConnectionState::Connecting;
        
        self.send(NetworkMessage {
            msg_type: MessageType::Connect,
            channel: Channel::ReliableOrdered,
            payload: Vec::new(),
        });

        tracing::info!("Connecting to {}", address);
        Ok(())
    }

    /// Disconnect from server
    pub fn disconnect(&mut self) {
        self.send(NetworkMessage {
            msg_type: MessageType::Disconnect,
            channel: Channel::ReliableOrdered,
            payload: Vec::new(),
        });
        self.state = ConnectionState::Disconnecting;
    }

    /// Get connection state
    #[must_use]
    pub fn state(&self) -> ConnectionState {
        self.state
    }

    /// Check if connected
    #[must_use]
    pub fn is_connected(&self) -> bool {
        self.state == ConnectionState::Connected
    }

    /// Get latency
    #[must_use]
    pub fn latency(&self) -> f32 {
        self.latency
    }

    /// Send a message
    pub fn send(&mut self, message: NetworkMessage) {
        self.outgoing.push(message);
    }

    /// Update client
    pub fn update(&mut self, _delta_time: f32) {
        // Simulate connection
        if self.state == ConnectionState::Connecting {
            self.state = ConnectionState::Connected;
            self.client_id = Some(ClientId(1));
        }

        // Send outgoing
        self.outgoing.clear();
    }
}

/// Input prediction for client-side prediction
#[derive(Debug, Clone)]
pub struct InputPrediction {
    /// History of inputs
    input_history: Vec<PredictedInput>,
    /// Max history size
    max_history: usize,
    /// Current sequence number
    sequence: u32,
}

/// Predicted input
#[derive(Debug, Clone)]
pub struct PredictedInput {
    /// Sequence number
    pub sequence: u32,
    /// Input data
    pub input: Vec<u8>,
    /// Predicted state after input
    pub predicted_state: Vec<u8>,
    /// Timestamp
    pub timestamp: f32,
}

impl Default for InputPrediction {
    fn default() -> Self {
        Self::new()
    }
}

impl InputPrediction {
    /// Create new prediction system
    #[must_use]
    pub fn new() -> Self {
        Self {
            input_history: Vec::new(),
            max_history: 128,
            sequence: 0,
        }
    }

    /// Record an input
    pub fn record_input(&mut self, input: Vec<u8>, predicted_state: Vec<u8>, timestamp: f32) -> u32 {
        self.sequence += 1;
        
        self.input_history.push(PredictedInput {
            sequence: self.sequence,
            input,
            predicted_state,
            timestamp,
        });

        // Trim history
        while self.input_history.len() > self.max_history {
            self.input_history.remove(0);
        }

        self.sequence
    }

    /// Reconcile with server state
    pub fn reconcile(&mut self, server_sequence: u32, _server_state: &[u8]) -> bool {
        // Remove acknowledged inputs
        self.input_history.retain(|i| i.sequence > server_sequence);
        
        // In real implementation, would compare states and replay inputs if mismatch
        !self.input_history.is_empty()
    }

    /// Get pending inputs for replay
    #[must_use]
    pub fn pending_inputs(&self) -> &[PredictedInput] {
        &self.input_history
    }
}

/// Initialize networking
///
/// # Errors
///
/// Returns error if initialization fails
pub fn init() -> Result<()> {
    tracing::info!("Networking subsystem initialized");
    Ok(())
}
