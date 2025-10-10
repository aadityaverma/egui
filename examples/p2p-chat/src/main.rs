
use eframe::egui;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

mod networking;
use networking::network_task;

#[derive(Default)]
struct NetworkedEguiApp {
    message_text: String,
    network_sender: Option<mpsc::UnboundedSender<NetworkMessage>>,
    shared_state: Arc<Mutex<SharedNetworkState>>,
}

#[derive(Default, Clone)]
pub struct SharedNetworkState {
    pub peers: Vec<String>,
    pub messages: Vec<(String, String)>, // (peer_id, message)
    pub connection_status: String,
}

pub enum NetworkMessage {
    SendMessage(String),
    Disconnect,
}

fn main() -> eframe::Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        tracing_wasm::set_as_global_default();
    }

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 600.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "P2P Chat - Matchbox + egui",
        options,
        Box::new(|_cc| Box::new(NetworkedEguiApp::new())),
    )
}

impl NetworkedEguiApp {
    fn new() -> Self {
        let shared_state = Arc::new(Mutex::new(SharedNetworkState::default()));
        let network_sender = {
            let state_clone = shared_state.clone();
            let (tx, rx) = mpsc::unbounded_channel();

            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_futures::spawn_local(network_task(state_clone, rx));

            #[cfg(not(target_arch = "wasm32"))]
            {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.spawn(network_task(state_clone, rx));
            }
            Some(tx)
        };

        Self { shared_state, network_sender, ..Default::default() }
    }
}

impl eframe::App for NetworkedEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let current_state = if let Ok(state) = self.shared_state.try_lock() {
            state.clone()
        } else {
            // If the lock is held, we use a default state for this frame.
            // This prevents the UI from freezing.
            SharedNetworkState::default()
        };

        let received_messages: Vec<String> = current_state.messages
            .iter()
            .map(|(peer, msg)| format!("{}: {}", peer, msg))
            .collect();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("P2P Chat with Matchbox + egui");
            ui.separator();
            ui.label(format!("Status: {}", current_state.connection_status));
            ui.label(format!("Connected peers: {}", current_state.peers.len()));
            ui.separator();

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.message_text);
                if ui.button("Send").clicked() && !self.message_text.is_empty() {
                    if let Some(sender) = &self.network_sender {
                        sender.send(NetworkMessage::SendMessage(self.message_text.clone())).ok();
                        self.message_text.clear();
                    }
                }
            });

            ui.separator();

            ui.label("Received Messages:");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for message in &received_messages {
                    ui.label(message);
                }
            });

            if ui.button("Disconnect").clicked() {
                if let Some(sender) = &self.network_sender {
                    sender.send(NetworkMessage::Disconnect).ok();
                }
            }
        });

        ctx.request_repaint();
    }
}
