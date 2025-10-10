



use crate::{NetworkMessage, SharedNetworkState};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use matchbox_socket::{PeerId, WebRtcSocket};

pub async fn network_task(
    shared_state: Arc<Mutex<SharedNetworkState>>,
    mut message_receiver: mpsc::UnboundedReceiver<NetworkMessage>,
) {
    // Connect to matchbox signaling server
    let room_url = "ws://localhost:3536/room";
    
    let (mut socket, loop_fut) = WebRtcSocket::builder(room_url)
        .add_reliable_channel()
        .build();

    // Update connection status
    {
        let mut state = shared_state.lock().unwrap();
        state.connection_status = "Connecting...".to_string();
    }

    // Spawn the socket event loop
    tokio::spawn(loop_fut);

    // Wait for connection
    socket.wait_for_peers(1).await;
    
    {
        let mut state = shared_state.lock().unwrap();
        state.connection_status = format!("Connected ({} peers)", socket.connected_peers().len());
        state.peers = socket.connected_peers().iter().map(|p| p.to_string()).collect();
    }

    // Main network loop
    loop {
        tokio::select! {
            Some(message) = message_receiver.recv() => {
                match message {
                    NetworkMessage::SendMessage(text) => {
                        let mut state = shared_state.lock().unwrap();
                        state.messages.push(("You".to_string(), text.clone()));
                        
                        // Send message to all connected peers
                        for peer in socket.connected_peers() {
                            socket.send(text.as_bytes().into(), peer);
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
            Some((peer_id, packet)) = socket.next() => {
                if let Ok(text) = String::from_utf8(packet) {
                    let mut state = shared_state.lock().unwrap();
                    state.messages.push((peer_id.to_string(), text));
                    
                    // Update peers list
                    state.peers = socket.connected_peers().iter().map(|p| p.to_string()).collect();
                    state.connection_status = format!("Connected ({} peers)", state.peers.len());
                }
            }
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
                // Keep the connection alive and update peer count
                let mut state = shared_state.lock().unwrap();
                let current_peers = socket.connected_peers();
                if current_peers.len() != state.peers.len() {
                    state.peers = current_peers.iter().map(|p| p.to_string()).collect();
                    state.connection_status = format!("Connected ({} peers)", state.peers.len());
                }
            }
        }
    }
}



