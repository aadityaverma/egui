
pub mod networking;
pub use networking::network_task;

use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct SharedNetworkState {
    pub connection_status: String,
    pub peers: Vec<String>,
    pub messages: Vec<(String, String)>,
}

#[derive(Debug)]
pub enum NetworkMessage {
    SendMessage(String),
    Disconnect,
}

impl Default for SharedNetworkState {
    fn default() -> Self {
        Self {
            connection_status: "Not connected".to_string(),
            peers: Vec::new(),
            messages: Vec::new(),
        }
    }
}
