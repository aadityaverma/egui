
use matchbox_socket::{WebRtcSocket, PeerState, ChannelConfig};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use std::time::{Duration, Instant};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    Chat(String),
    GameState(GameState),
    Ping,
    Pong,
    Heartbeat,
    ReconnectRequest,
    ReconnectResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub player_id: String,
    pub position: (f32, f32),
    pub timestamp: u64,
    pub sequence: u32,
}

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub room_url: String,
    pub heartbeat_interval: Duration,
    pub reconnect_attempts: u32,
    pub reconnect_delay: Duration,
    pub max_message_size: usize,
    pub enable_reliable_channel: bool,
    pub enable_unreliable_channel: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            room_url: "ws://localhost:3536/game_room".to_string(),
            heartbeat_interval: Duration::from_secs(5),
            reconnect_attempts: 5,
            reconnect_delay: Duration::from_secs(2),
            max_message_size: 1024 * 64, // 64KB
            enable_reliable_channel: true,
            enable_unreliable_channel: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkState {
    pub peers: Vec<String>,
    pub messages: Vec<(String, String, u64)>, // (peer_id, message, timestamp)
    pub connection_status: ConnectionStatus,
    pub last_heartbeat: Instant,
    pub reconnect_attempts: u32,
    pub network_stats: NetworkStats,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Error(String),
}

#[derive(Debug, Clone, Default)]
pub struct NetworkStats {
    pub messages_sent: u64,
    pub messages_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub last_ping: Option<Duration>,
    pub packet_loss: f32,
}

pub struct EnhancedNetworkManager {
    socket: Option<WebRtcSocket>,
    config: NetworkConfig,
    state: Arc<Mutex<NetworkState>>,
    command_sender: mpsc::UnboundedSender<NetworkCommand>,
    message_sender: mpsc::UnboundedSender<NetworkMessage>,
    heartbeat_handle: Option<tokio::task::JoinHandle<()>>,
    reconnect_handle: Option<tokio::task::JoinHandle<()>>,
}

#[derive(Debug)]
pub enum NetworkCommand {
    Connect(String),
    SendMessage(NetworkMessage),
    Disconnect,
    Reconnect,
    UpdateConfig(NetworkConfig),
}

impl EnhancedNetworkManager {
    pub fn new(config: NetworkConfig) -> (Self, mpsc::UnboundedReceiver<NetworkMessage>) {
        let (command_sender, command_receiver) = mpsc::unbounded_channel();
        let (message_sender, message_receiver) = mpsc::unbounded_channel();
        
        let state = Arc::new(Mutex::new(NetworkState {
            peers: Vec::new(),
            messages: Vec::new(),
            connection_status: ConnectionStatus::Disconnected,
            last_heartbeat: Instant::now(),
            reconnect_attempts: 0,
            network_stats: NetworkStats::default(),
        }));
        
        let manager = Self {
            socket: None,
            config,
            state: state.clone(),
            command_sender,
            message_sender,
            heartbeat_handle: None,
            reconnect_handle: None,
        };
        
        (manager, message_receiver)
    }
    
    pub async fn connect(&mut self, room_url: String) -> Result<(), String> {
        let mut config = self.config.clone();
        config.room_url = room_url;
        
        self.update_config(config).await
    }
    
    pub async fn update_config(&mut self, config: NetworkConfig) -> Result<(), String> {
        self.config = config;
        self.update_status(ConnectionStatus::Connecting);
        
        // Build socket with multiple channels
        let mut socket_builder = matchbox_socket::WebRtcSocketBuilder::new(&self.config.room_url);
        
        if self.config.enable_reliable_channel {
            socket_builder = socket_builder.add_reliable_channel();
        }
        
        if self.config.enable_unreliable_channel {
            socket_builder = socket_builder.add_unreliable_channel();
        }
        
        let (socket, message_loop) = socket_builder.build();
        
        // Start message loop
        let handle = tokio::spawn(message_loop);
        
        self.socket = Some(socket);
        self.start_heartbeat();
        self.start_reconnect_monitor();
        
        self.update_status(ConnectionStatus::Connected);
        Ok(())
    }
    
    pub fn disconnect(&mut self) {
        self.stop_heartbeat();
        self.stop_reconnect_monitor();
        
        if let Some(mut manager) = self.socket.take() {
            // Clean shutdown
        }
        
        self.update_status(ConnectionStatus::Disconnected);
    }
    
    pub fn send_message(&self, message: NetworkMessage) -> Result<(), String> {
        if let Some(socket) = &self.socket {
            let packet = bincode::serialize(&message)
                .map_err(|e| format!("Serialization error: {}", e))?;
            
            // Use appropriate channel based on message type
            let channel = match message {
                NetworkMessage::Chat(_) => 0, // Reliable channel
                NetworkMessage::GameState(_) => 1, // Unreliable channel
                _ => 0,
            };
            
            if channel < socket.channels() {
                socket.channel_mut(channel).send_to_all(packet.into_boxed_slice());
                
                if let Ok(mut state) = self.state.lock() {
                    state.network_stats.messages_sent += 1;
                    state.network_stats.bytes_sent += packet.len() as u64;
                }
            }
            
            Ok(())
        } else {
            Err("Not connected".to_string())
        }
    }
    
    pub fn get_state(&self) -> NetworkState {
        self.state.lock().unwrap().clone()
    }
    
    pub fn get_stats(&self) -> NetworkStats {
        self.state.lock().unwrap().network_stats.clone()
    }
    
    fn start_heartbeat(&mut self) {
        let state = self.state.clone();
        let config = self.config.clone();
        let message_sender = self.message_sender.clone();
        
        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(config.heartbeat_interval);
            
            loop {
                interval.tick().await;
                
                if let Ok(mut state) = state.lock() {
                    if state.connection_status == ConnectionStatus::Connected {
                        state.last_heartbeat = Instant::now();
                        
                        let _ = message_sender.send(NetworkMessage::Heartbeat);
                    }
                }
            }
        });
        
        self.heartbeat_handle = Some(handle);
    }
    
    fn start_reconnect_monitor(&mut self) {
        let state = self.state.clone();
        let config = self.config.clone();
        let command_sender = self.command_sender.clone();
        
        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            
            loop {
                interval.tick().await;
                
                if let Ok(mut state) = state.lock() {
                    if state.connection_status == ConnectionStatus::Error(_) && 
                       state.reconnect_attempts < config.reconnect_attempts {
                        
                        state.reconnect_attempts += 1;
                        state.connection_status = ConnectionStatus::Reconnecting;
                        
                        let _ = command_sender.send(NetworkCommand::Reconnect);
                    }
                }
            }
        });
        
        self.reconnect_handle = Some(handle);
    }
    
    fn stop_heartbeat(&mut self) {
        if let Some(handle) = self.heartbeat_handle.take() {
            handle.abort();
        }
    }
    
    fn stop_reconnect_monitor(&mut self) {
        if let Some(handle) = self.reconnect_handle.take() {
            handle.abort();
        }
    }
    
    fn update_status(&self, status: ConnectionStatus) {
        if let Ok(mut state) = self.state.lock() {
            state.connection_status = status;
            
            if status == ConnectionStatus::Connected {
                state.reconnect_attempts = 0;
            }
        }
    }
}

pub fn spawn_enhanced_network_task(
    config: NetworkConfig,
    shared_state: Arc<Mutex<NetworkState>>,
) -> (mpsc::UnboundedSender<NetworkCommand>, mpsc::UnboundedReceiver<NetworkMessage>) {
    let (manager, message_receiver) = EnhancedNetworkManager::new(config);
    let command_sender = manager.command_sender.clone();
    
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(async move {
            let mut manager = manager;
            // Handle commands in WASM context
        });
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        tokio::spawn(async move {
            let mut manager = manager;
            // Handle commands in native context
        });
    }
    
    (command_sender, message_receiver)
}
