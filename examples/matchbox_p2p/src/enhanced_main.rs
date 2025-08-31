


use eframe::egui;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use uuid::Uuid;
use instant::Instant;

mod enhanced_network;
use enhanced_network::{EnhancedNetworkManager, NetworkConfig, NetworkMessage, NetworkState, ConnectionStatus};

#[derive(Default)]
struct EnhancedP2PApp {
    // UI State
    message_text: String,
    room_url: String,
    player_name: String,
    player_position: (f32, f32),
    
    // Networking
    network_sender: Option<mpsc::UnboundedSender<enhanced_network::NetworkCommand>>,
    shared_state: Arc<Mutex<NetworkState>>,
    
    // Game state
    game_objects: Vec<GameObject>,
    last_update: Instant,
    
    // UI Configuration
    show_debug_panel: bool,
    show_network_stats: bool,
    show_game_panel: bool,
}

#[derive(Debug, Clone)]
struct GameObject {
    id: String,
    position: (f32, f32),
    velocity: (f32, f32),
    peer_id: Option<String>,
}

impl EnhancedP2PApp {
    fn new() -> Self {
        let config = NetworkConfig::default();
        let shared_state = Arc::new(Mutex::new(NetworkState::default()));
        
        // Setup networking
        let network_sender = spawn_enhanced_network_task(config, shared_state.clone());
        
        Self {
            shared_state,
            network_sender,
            room_url: "ws://localhost:3536/game_room".to_string(),
            player_name: format!("Player-{}", Uuid::new_v4().to_string()[..8].to_string()),
            player_position: (400.0, 300.0),
            game_objects: Vec::new(),
            last_update: Instant::now(),
            show_debug_panel: true,
            show_network_stats: true,
            show_game_panel: true,
        }
    }
    
    fn send_chat_message(&mut self) {
        if !self.message_text.is_empty() {
            if let Some(sender) = &self.network_sender {
                let message = NetworkMessage::Chat(format!("{}: {}", self.player_name, self.message_text));
                sender.send(enhanced_network::NetworkCommand::SendMessage(message)).ok();
                self.message_text.clear();
            }
        }
    }
    
    fn send_game_state(&mut self) {
        if let Some(sender) = &self.network_sender {
            let game_state = enhanced_network::GameState {
                player_id: self.player_name.clone(),
                position: self.player_position,
                timestamp: self.last_update.elapsed().as_millis() as u64,
                sequence: 0, // TODO: Implement sequence numbers
            };
            
            let message = NetworkMessage::GameState(game_state);
            sender.send(enhanced_network::NetworkCommand::SendMessage(message)).ok();
        }
    }
    
    fn update_game_objects(&mut self) {
        // Update local player position based on input
        // This would be connected to actual input handling
    }
}

impl eframe::App for EnhancedP2PApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Read network state
        let current_state = if let Ok(state) = self.shared_state.try_lock() {
            state.clone()
        } else {
            NetworkState::default()
        };
        
        // Update game state
        self.update_game_objects();
        
        // Send game state periodically
        if self.last_update.elapsed().as_millis() > 16 { // ~60 FPS
            self.send_game_state();
            self.last_update = Instant::now();
        }
        
        // Main UI
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
                
                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.show_debug_panel, "Debug Panel");
                    ui.checkbox(&mut self.show_network_stats, "Network Stats");
                    ui.checkbox(&mut self.show_game_panel, "Game Panel");
                });
                
                ui.menu_button("Network", |ui| {
                    if ui.button("Connect").clicked() {
                        if let Some(sender) = &self.network_sender {
                            sender.send(enhanced_network::NetworkCommand::Connect(self.room_url.clone())).ok();
                        }
                    }
                    
                    if ui.button("Disconnect").clicked() {
                        if let Some(sender) = &self.network_sender {
                            sender.send(enhanced_network::NetworkCommand::Disconnect).ok();
                        }
                    }
                });
            });
        });
        
        // Left panel - Connection and Chat
        egui::SidePanel::left("connection_panel").min_width(300.0).show(ctx, |ui| {
            ui.heading("P2P Network");
            
            // Connection status
            ui.group(|ui| {
                ui.label("Connection Status:");
                match &current_state.connection_status {
                    ConnectionStatus::Connected => {
                        ui.colored_label(egui::Color32::GREEN, "Connected");
                    }
                    ConnectionStatus::Connecting => {
                        ui.colored_label(egui::Color32::YELLOW, "Connecting...");
                    }
                    ConnectionStatus::Reconnecting => {
                        ui.colored_label(egui::Color32::ORANGE, "Reconnecting...");
                    }
                    ConnectionStatus::Disconnected => {
                        ui.colored_label(egui::Color32::GRAY, "Disconnected");
                    }
                    ConnectionStatus::Error(e) => {
                        ui.colored_label(egui::Color32::RED, format!("Error: {}", e));
                    }
                }
                
                ui.label(format!("Connected peers: {}", current_state.peers.len()));
            });
            
            // Room configuration
            ui.group(|ui| {
                ui.label("Room Configuration:");
                ui.horizontal(|ui| {
                    ui.label("Room URL:");
                    ui.text_edit_singleline(&mut self.room_url);
                });
                
                ui.horizontal(|ui| {
                    ui.label("Player Name:");
                    ui.text_edit_singleline(&mut self.player_name);
                });
            });
            
            // Chat messages
            ui.group(|ui| {
                ui.label("Chat Messages:");
                
                egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    for (peer, message, _timestamp) in &current_state.messages {
                        ui.label(format!("{}: {}", peer, message));
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.message_text);
                    if ui.button("Send").clicked() {
                        self.send_chat_message();
                    }
                });
            });
        });
        
        // Central panel - Game area
        if self.show_game_panel {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Game Area");
                
                // Simple 2D game visualization
                let response = ui.allocate_response(
                    egui::vec2(ui.available_width(), ui.available_height() - 100.0),
                    egui::Sense::click_and_drag()
                );
                
                let painter = ui.painter_at(response.rect);
                
                // Draw grid
                let grid_size = 50.0;
                let rect = response.rect;
                
                for x in (0..rect.width() as i32).step_by(grid_size as usize) {
                    let x = rect.left() + x as f32;
                    painter.line_segment(
                        [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                        egui::Stroke::new(1.0, egui::Color32::GRAY.linear_multiply(0.3))
                    );
                }
                
                for y in (0..rect.height() as i32).step_by(grid_size as usize) {
                    let y = rect.top() + y as f32;
                    painter.line_segment(
                        [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                        egui::Stroke::new(1.0, egui::Color32::GRAY.linear_multiply(0.3))
                    );
                }
                
                // Draw player
                let player_pos = egui::pos2(
                    rect.left() + self.player_position.0,
                    rect.top() + self.player_position.1
                );
                
                painter.circle(
                    player_pos,
                    10.0,
                    egui::Color32::GREEN,
                    egui::Stroke::new(2.0, egui::Color32::WHITE)
                );
                
                // Draw other players
                for obj in &self.game_objects {
                    let obj_pos = egui::pos2(
                        rect.left() + obj.position.0,
                        rect.top() + obj.position.1
                    );
                    
                    let color = if obj.peer_id.is_some() {
                        egui::Color32::BLUE
                    } else {
                        egui::Color32::RED
                    };
                    
                    painter.circle(
                        obj_pos,
                        8.0,
                        color,
                        egui::Stroke::new(2.0, egui::Color32::WHITE)
                    );
                }
                
                // Handle mouse input for player movement
                if response.dragged() {
                    let delta = response.drag_delta();
                    self.player_position.0 += delta.x;
                    self.player_position.1 += delta.y;
                }
            });
        }
        
        // Right panel - Debug and stats
        if self.show_debug_panel {
            egui::SidePanel::right("debug_panel").min_width(250.0).show(ctx, |ui| {
                ui.heading("Debug Panel");
                
                // Network stats
                if self.show_network_stats {
                    ui.group(|ui| {
                        ui.label("Network Statistics:");
                        ui.label(format!("Messages sent: {}", current_state.network_stats.messages_sent));
                        ui.label(format!("Messages received: {}", current_state.network_stats.messages_received));
                        ui.label(format!("Bytes sent: {}", current_state.network_stats.bytes_sent));
                        ui.label(format!("Bytes received: {}", current_state.network_stats.bytes_received));
                        
                        if let Some(ping) = current_state.network_stats.last_ping {
                            ui.label(format!("Last ping: {:.2}ms", ping.as_millis()));
                        }
                        
                        ui.label(format!("Packet loss: {:.1}%", current_state.network_stats.packet_loss * 100.0));
                    });
                }
                
                // Connected peers
                ui.group(|ui| {
                    ui.label("Connected Peers:");
                    for peer in &current_state.peers {
                        ui.label(peer);
                    }
                });
                
                // Performance info
                ui.group(|ui| {
                    ui.label("Performance:");
                    ui.label(format!("FPS: {:.1}", 1.0 / ctx.input(|i| i.unstable_dt)));
                    ui.label(format!("Memory usage: {:.1}MB", 0.0)); // TODO: Add actual memory usage
                });
            });
        }
        
        // Request repaint for real-time updates
        ctx.request_repaint();
    }
}

fn spawn_enhanced_network_task(
    config: NetworkConfig,
    shared_state: Arc<Mutex<NetworkState>>,
) -> mpsc::UnboundedSender<enhanced_network::NetworkCommand> {
    let (command_sender, mut command_receiver) = mpsc::unbounded_channel();
    
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(async move {
            let mut manager = EnhancedNetworkManager::new(config).0;
            
            while let Some(command) = command_receiver.recv().await {
                match command {
                    enhanced_network::NetworkCommand::Connect(room_url) => {
                        let _ = manager.connect(room_url).await;
                    }
                    enhanced_network::NetworkCommand::SendMessage(message) => {
                        let _ = manager.send_message(message);
                    }
                    enhanced_network::NetworkCommand::Disconnect => {
                        manager.disconnect();
                    }
                    enhanced_network::NetworkCommand::Reconnect => {
                        // Handle reconnection
                    }
                    enhanced_network::NetworkCommand::UpdateConfig(config) => {
                        let _ = manager.update_config(config).await;
                    }
                }
            }
        });
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        tokio::spawn(async move {
            let mut manager = EnhancedNetworkManager::new(config).0;
            
            while let Some(command) = command_receiver.recv().await {
                match command {
                    enhanced_network::NetworkCommand::Connect(room_url) => {
                        let _ = manager.connect(room_url).await;
                    }
                    enhanced_network::NetworkCommand::SendMessage(message) => {
                        let _ = manager.send_message(message);
                    }
                    enhanced_network::NetworkCommand::Disconnect => {
                        manager.disconnect();
                    }
                    enhanced_network::NetworkCommand::Reconnect => {
                        // Handle reconnection
                    }
                    enhanced_network::NetworkCommand::UpdateConfig(config) => {
                        let _ = manager.update_config(config).await;
                    }
                }
            }
        });
    }
    
    command_sender
}

fn main() -> eframe::Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        tracing_wasm::set_as_global_default();
    }

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Enhanced Matchbox + egui P2P Demo",
        options,
        Box::new(|_cc| Box::new(EnhancedP2PApp::new())),
    )
}



