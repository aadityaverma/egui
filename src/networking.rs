
use crate::{SharedNetworkState, NetworkMessage};
use matchbox_socket::{WebRtcSocket, PeerState};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

pub async fn network_task(
    shared_state: Arc<Mutex<SharedNetworkState>>,
    mut message_rx: mpsc::UnboundedReceiver<NetworkMessage>,
) {
    shared_state.lock().unwrap().connection_status = "Connecting...".to_string();

    let room_url = "ws://localhost:3536/example_room";
    let (mut socket, message_loop) = WebRtcSocket::new_reliable(room_url);

    let message_loop_handle = tokio::spawn(message_loop);

    shared_state.lock().unwrap().connection_status = "Connected to signaling server".to_string();

    let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(16));

    loop {
        tokio::select! {
            Some(ui_message) = message_rx.recv() => {
                match ui_message {
                    NetworkMessage::SendMessage(text) => {
                        let packet = text.as_bytes().to_vec().into_boxed_slice();
                        socket.channel_mut(0).send_to_all(packet);
                    }
                    NetworkMessage::Disconnect => break,
                }
            }

            _ = interval.tick() => {
                for (peer_id, state) in socket.update_peers() {
                    let mut shared = shared_state.lock().unwrap();
                    match state {
                        PeerState::Connected => shared.peers.push(peer_id.to_string()),
                        PeerState::Disconnected => shared.peers.retain(|p| p != &peer_id.to_string()),
                    }
                    shared.connection_status = format!("Connected to {} peers", shared.peers.len());
                }

                for (peer_id, packet) in socket.channel_mut(0).receive() {
                    if let Ok(message) = String::from_utf8(packet.to_vec()) {
                        shared_state.lock().unwrap().messages.push((peer_id.to_string(), message));
                    }
                }
            }

            _ = &mut message_loop_handle => {
                shared_state.lock().unwrap().connection_status = "Disconnected".to_string();
                break;
            }
        }
    }
}
