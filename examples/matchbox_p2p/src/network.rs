
use matchbox_socket::{WebRtcSocket, PeerState};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum NetworkEvent {
    PeerConnected(String),
    PeerDisconnected(String),
    MessageReceived(String, String), // (peer_id, message)
    ConnectionStatusChanged(String),
    Error(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkMessage {
    Chat(String),
    Ping,
    Pong,
}

pub struct NetworkManager {
    socket: Option<WebRtcSocket>,
    event_sender: mpsc::UnboundedSender<NetworkEvent>,
    message_sender: mpsc::UnboundedSender<NetworkMessage>,
    shared_state: Arc<Mutex<NetworkState>>,
}

#[derive(Default, Clone)]
pub struct NetworkState {
    pub peers: Vec<String>,
    pub messages: Vec<(String, String)>,
    pub connection_status: String,
    pub error: Option<String>,
}

impl NetworkManager {
    pub fn new(
        event_sender: mpsc::UnboundedSender<NetworkEvent>,
        shared_state: Arc<Mutex<NetworkState>>,
    ) -> (Self, mpsc::UnboundedReceiver<NetworkMessage>) {
        let (message_sender, message_receiver) = mpsc::unbounded_channel();
        
        let manager = Self {
            socket: None,
            event_sender,
            message_sender,
            shared_state,
        };
        
        (manager, message_receiver)
    }
    
    pub async fn connect(&mut self, room_url: String) -> Result<(), String> {
        self.update_status("Connecting...".to_string());
        
        let (socket, message_loop) = WebRtcSocket::new_reliable(room_url);
        
        // Start the matchbox message loop
        let handle = tokio::spawn(message_loop);
        
        self.socket = Some(socket);
        
        self.update_status("Connected to signaling server".to_string());
        
        // Start network processing
        self.start_network_processing(handle);
        
        Ok(())
    }
    
    pub fn disconnect(&mut self) {
        self.socket = None;
        self.update_status("Disconnected".to_string());
        
        if let Ok(mut state) = self.shared_state.lock() {
            state.peers.clear();
            state.error = None;
        }
    }
    
    fn start_network_processing(&mut self, message_loop_handle: tokio::task::JoinHandle<()>) {
        let event_sender = self.event_sender.clone();
        let shared_state = self.shared_state.clone();
        let mut socket = self.socket.take().unwrap();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(16));
            
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        // Handle peer state changes
                        for (peer_id, state) in socket.update_peers() {
                            match state {
                                PeerState::Connected => {
                                    event_sender.send(NetworkEvent::PeerConnected(peer_id.to_string())).ok();
                                }
                                PeerState::Disconnected => {
                                    event_sender.send(NetworkEvent::PeerDisconnected(peer_id.to_string())).ok();
                                }
                            }
                        }
                        
                        // Handle incoming messages
                        for (peer_id, packet) in socket.channel_mut(0).receive() {
                            if let Ok(message) = bincode::deserialize::<NetworkMessage>(&packet) {
                                match message {
                                    NetworkMessage::Chat(text) => {
                                        event_sender.send(NetworkEvent::MessageReceived(
                                            peer_id.to_string(),
                                            text
                                        )).ok();
                                    }
                                    NetworkMessage::Ping => {
                                        let response = bincode::serialize(&NetworkMessage::Pong).unwrap();
                                        socket.channel_mut(0).send_to_all(response.into_boxed_slice());
                                    }
                                    NetworkMessage::Pong => {
                                        // Handle pong response
                                    }
                                }
                            }
                        }
                    }
                    
                    _ = &mut message_loop_handle => {
                        event_sender.send(NetworkEvent::Error("Message loop ended".to_string())).ok();
                        break;
                    }
                }
            }
        });
    }
    
    pub fn send_message(&mut self, message: NetworkMessage) -> Result<(), String> {
        if let Some(socket) = &mut self.socket {
            let packet = bincode::serialize(&message)
                .map_err(|e| format!("Serialization error: {}", e))?;
            socket.channel_mut(0).send_to_all(packet.into_boxed_slice());
            Ok(())
        } else {
            Err("Not connected".to_string())
        }
    }
    
    fn update_status(&self, status: String) {
        if let Ok(mut state) = self.shared_state.lock() {
            state.connection_status = status;
        }
        
        self.event_sender.send(NetworkEvent::ConnectionStatusChanged(status)).ok();
    }
    
    pub fn get_connection_info(&self) -> (usize, String) {
        if let Ok(state) = self.shared_state.lock() {
            (state.peers.len(), state.connection_status.clone())
        } else {
            (0, "Unknown".to_string())
        }
    }
}

pub fn spawn_network_task(
    shared_state: Arc<Mutex<NetworkState>>,
) -> mpsc::UnboundedSender<NetworkCommand> {
    let (command_sender, mut command_receiver) = mpsc::unbounded_channel();
    let (event_sender, mut event_receiver) = mpsc::unbounded_channel();
    
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(async move {
            run_network_task(shared_state, command_receiver, event_receiver).await;
        });
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        tokio::spawn(async move {
            run_network_task(shared_state, command_receiver, event_receiver).await;
        });
    }
    
    command_sender
}

async fn run_network_task(
    shared_state: Arc<Mutex<NetworkState>>,
    mut command_receiver: mpsc::UnboundedReceiver<NetworkCommand>,
    mut event_receiver: mpsc::UnboundedReceiver<NetworkEvent>,
) {
    let mut network_manager = None;
    
    loop {
        tokio::select! {
            Some(command) = command_receiver.recv() => {
                match command {
                    NetworkCommand::Connect(room_url) => {
                        let (manager, message_receiver) = NetworkManager::new(
                            event_sender.clone(),
                            shared_state.clone(),
                        );
                        
                        if let Err(e) = manager.connect(room_url).await {
                            if let Ok(mut state) = shared_state.lock() {
                                state.error = Some(e);
                            }
                        } else {
                            network_manager = Some(manager);
                        }
                    }
                    NetworkCommand::SendMessage(message) => {
                        if let Some(manager) = &mut network_manager {
                            if let Err(e) = manager.send_message(message) {
                                if let Ok(mut state) = shared_state.lock() {
                                    state.error = Some(e);
                                }
                            }
                        }
                    }
                    NetworkCommand::Disconnect => {
                        if let Some(manager) = &mut network_manager {
                            manager.disconnect();
                        }
                        network_manager = None;
                    }
                }
            }
            
            Some(event) = event_receiver.recv() => {
                match event {
                    NetworkEvent::PeerConnected(peer_id) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.peers.push(peer_id);
                            state.connection_status = format!("Connected to {} peers", state.peers.len());
                        }
                    }
                    NetworkEvent::PeerDisconnected(peer_id) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.peers.retain(|p| p != &peer_id);
                            state.connection_status = format!("Connected to {} peers", state.peers.len());
                        }
                    }
                    NetworkEvent::MessageReceived(peer_id, message) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.messages.push((peer_id, message));
                        }
                    }
                    NetworkEvent::ConnectionStatusChanged(status) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.connection_status = status;
                        }
                    }
                    NetworkEvent::Error(error) => {
                        if let Ok(mut state) = shared_state.lock() {
                            state.error = Some(error);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum NetworkCommand {
    Connect(String),
    SendMessage(NetworkMessage),
    Disconnect,
}
