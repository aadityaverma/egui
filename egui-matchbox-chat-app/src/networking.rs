

use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use crate::{SharedNetworkState, NetworkMessage};

pub async fn network_task(
    shared_state: Arc<Mutex<SharedNetworkState>>,
    mut message_receiver: mpsc::UnboundedReceiver<NetworkMessage>,
) {
    // Initialize the network state
    {
        let mut state = shared_state.lock().unwrap();
        state.connection_status = "Connecting...".to_string();
    }

    // Simulate network initialization delay
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Update connection status
    {
        let mut state = shared_state.lock().unwrap();
        state.connection_status = "Connected".to_string();
        state.peers.push("peer_1".to_string());
        state.peers.push("peer_2".to_string());
    }

    // Main network loop
    loop {
        tokio::select! {
            Some(message) = message_receiver.recv() => {
                match message {
                    NetworkMessage::SendMessage(text) => {
                        let mut state = shared_state.lock().unwrap();
                        state.messages.push(("You".to_string(), text));
                        
                        // Simulate sending to peers
                        for peer in &state.peers {
                            state.messages.push((peer.clone(), format!("Echo: {}", state.messages.last().unwrap().1)));
                        }
                    }
                    NetworkMessage::Disconnect => {
                        let mut state = shared_state.lock().unwrap();
                        state.connection_status = "Disconnected".to_string();
                        state.peers.clear();
                        break;
                    }
                }
            }
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
                // Simulate receiving messages from peers
                let mut state = shared_state.lock().unwrap();
                if state.connection_status == "Connected" && !state.peers.is_empty() {
                    // Occasionally add a random message from a peer
                    if rand::random::<f32>() < 0.1 {
                        let peer = state.peers[rand::random::<usize>() % state.peers.len()].clone();
                        state.messages.push((peer, format!("Random message at {}", chrono::Local::now().format("%H:%M:%S"))));
                    }
                }
            }
        }
    }
}

// Add these dependencies to Cargo.toml if using this networking implementation
// [dependencies]
// rand = "0.8"
// chrono = "0.4"


