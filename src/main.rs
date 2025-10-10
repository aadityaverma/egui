

use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

use egui_matchbox_net::{network_task, NetworkMessage, SharedNetworkState};

#[tokio::main]
async fn main() {
    // Create shared state
    let shared_state = Arc::new(Mutex::new(SharedNetworkState::default()));
    
    // Create channel for UI messages
    let (message_tx, message_rx) = mpsc::unbounded_channel();
    
    // Start the network task
    network_task(shared_state.clone(), message_rx).await;
}

