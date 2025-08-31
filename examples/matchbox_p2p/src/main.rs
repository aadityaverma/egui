use eframe::egui;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

mod network;
use network::{NetworkCommand, NetworkMessage, NetworkState};

#[derive(Default)]
struct NetworkedEguiApp {
    // UI State
    message_text: String,
    received_messages: Vec<String>,
    connected_peers: Vec<String>,
    room_url: String,
    
    // Networking
    network_sender: Option<mpsc::UnboundedSender<NetworkCommand>>,
    shared_state: Arc<Mutex<NetworkState>>,
}

impl NetworkedEguiApp {
    fn new() -> Self {
        let shared_state = Arc::new(Mutex::new(NetworkState::default()));
        
        // Setup networking in background
        let network_sender = network::spawn_network_task(shared_state.clone());

        Self {
            shared_state,
            network_sender,
            room_url: "ws://localhost:3536/chat_room".to_string(),
            ..Default::default()
        }
    }
}

impl eframe::App for NetworkedEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Read from shared state without blocking
        let current_state = if let Ok(state) = self.shared_state.try_lock() {
            state.clone()
        } else {
            NetworkState::default()
        };
        
        // Update UI with network state
        self.connected_peers = current_state.peers;
        self.received_messages = current_state.messages
            .iter()
            .map(|(peer, msg)| format!("{}: {}", peer, msg))
            .collect();

        // Render UI
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("P2P Chat with Matchbox + egui");
            
            ui.separator();
            
            // Connection status and error display
            ui.horizontal(|ui| {
                ui.label(format!("Status: {}", current_state.connection_status));
                if let Some(error) = &current_state.error {
                    ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
                }
            });
            ui.label(format!("Connected peers: {}", self.connected_peers.len()));
            
            ui.separator();
            
            // Connection controls
            ui.horizontal(|ui| {
                ui.label("Room URL:");
                ui.text_edit_singleline(&mut self.room_url);
                
                if ui.button("Connect").clicked() {
                    if let Some(sender) = &self.network_sender {
                        sender.send(NetworkCommand::Connect(self.room_url.clone())).ok();
                    }
                }
                
                if ui.button("Disconnect").clicked() {
                    if let Some(sender) = &self.network_sender {
                        sender.send(NetworkCommand::Disconnect).ok();
                    }
                }
            });
            
            ui.separator();
            
            // Message input
            ui.horizontal(|ui| {
                let is_connected = !self.connected_peers.is_empty();
                
                ui.add_enabled_ui(is_connected, |ui| {
                    ui.text_edit_singleline(&mut self.message_text);
                    if ui.button("Send").clicked() && !self.message_text.is_empty() {
                        if let Some(sender) = &self.network_sender {
                            let message = NetworkMessage::Chat(self.message_text.clone());
                            sender.send(NetworkCommand::SendMessage(message)).ok();
                            self.message_text.clear();
                        }
                    }
                });
                
                if !is_connected {
                    ui.label(egui::RichText::new("Connect to send messages").color(egui::Color32::GRAY));
                }
            });
            
            ui.separator();
            
            // Peers list
            ui.collapsing("Connected Peers", |ui| {
                for peer in &self.connected_peers {
                    ui.label(peer);
                }
            });
            
            ui.separator();
            
            // Messages display
            ui.label("Chat Messages:");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for message in &self.received_messages {
                    ui.label(message);
                }
            });
        });
        
        // Request repaint for real-time updates
        ctx.request_repaint();
    }
}


fn main() -> eframe::Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        tracing_wasm::set_as_global_default();
    }

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Matchbox + egui P2P Demo",
        options,
        Box::new(|_cc| Box::new(NetworkedEguiApp::new())),
    )
}
