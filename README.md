# Egui-Matchbox Integration Guide

## Introduction

This project demonstrates how to integrate **Matchbox P2P networking** with **egui** to create real-time multiplayer applications. The integration enables peer-to-peer communication between egui-based clients, allowing for collaborative interfaces, multiplayer games, and distributed applications without requiring a central server for data relay.

The goal is to provide a robust foundation for building egui applications that can seamlessly connect users directly to each other, leveraging WebRTC for low-latency communication and Matchbox for efficient peer discovery and signaling.

## Core Architecture

The integration follows an **async task and shared state pattern** that separates networking concerns from the egui UI logic:

- **Async Networking Task**: Runs in the background, handling all WebRTC connection management, message sending/receiving, and peer discovery
- **Shared State**: Uses thread-safe data structures (Arc<Mutex<T>>) to communicate between the networking task and egui UI
- **Message Queues**: Implements producer-consumer patterns for handling incoming/outgoing messages without blocking the UI

This architecture ensures that the egui UI remains responsive while the networking layer handles connection establishment, NAT traversal, and message routing in the background.

```rust
// Core architecture pattern
struct AppState {
    network: Arc<Mutex<NetworkState>>,
    message_queue: Arc<Mutex<Vec<NetworkMessage>>>,
}

// Async networking task
async fn network_task(state: Arc<Mutex<NetworkState>>) {
    // Handle connections, send/receive messages
}
```

## Advanced Channel Configuration

Matchbox provides flexible channel configuration through `WebRtcSocketBuilder`, allowing you to create multiple channels with different reliability guarantees optimized for specific use cases.

### Creating Multiple Channels

```rust
use matchbox_socket::WebRtcSocketBuilder;

// Create socket with multiple channels for different purposes
let (socket, message_loop) = WebRtcSocketBuilder::new("ws://localhost:3536")
    .add_channel(ChannelConfig {
        // Unreliable channel for game state - low latency, possible packet loss
        channel_id: 0,
        max_retransmits: None, // No retransmission
        ordered: false,        // Don't guarantee order
    })
    .add_channel(ChannelConfig {
        // Reliable channel for chat messages - guaranteed delivery
        channel_id: 1,
        max_retransmits: Some(15), // Retry up to 15 times
        ordered: true,             // Maintain message order
    })
    .build();

// Spawn the message loop
tokio::spawn(message_loop);
```

### Channel Use Cases

- **Channel 0 (Unreliable)**: Game state updates, player positions, real-time data
  - Acceptable packet loss for smooth gameplay
  - Minimal latency overhead
  - Out-of-order delivery is acceptable

- **Channel 1 (Reliable)**: Chat messages, UI interactions, critical data
  - Must arrive completely and in order
  - Higher latency acceptable for reliability
  - Ensures no data loss

## Error Handling and Reliability

Robust connection state management is crucial for maintaining stable P2P connections, especially when dealing with network instability, peer disconnections, and NAT traversal failures.

### NetworkState Enum

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum NetworkState {
    Disconnected,
    Connecting,
    Connected {
        peer_count: usize,
        connection_quality: f32, // 0.0 to 1.0
    },
    ConnectionFailed {
        error: String,
        retry_count: u32,
    },
    Reconnecting {
        attempt: u32,
        max_attempts: u32,
    },
}

impl NetworkState {
    pub fn should_reconnect(&self) -> bool {
        match self {
            NetworkState::Disconnected => true,
            NetworkState::ConnectionFailed { retry_count, .. } => *retry_count < 3,
            NetworkState::Reconnecting { attempt, max_attempts } => *attempt < *max_attempts,
            _ => false,
        }
    }
}
```

### Error Handling Strategy

```rust
async fn handle_network_errors(state: Arc<Mutex<NetworkState>>) {
    let mut retry_count = 0;
    const MAX_RETRIES: u32 = 3;
    
    loop {
        let current_state = {
            let state = state.lock().unwrap();
            state.clone()
        };
        
        match current_state {
            NetworkState::ConnectionFailed { error, .. } => {
                if retry_count < MAX_RETRIES {
                    retry_count += 1;
                    // Exponential backoff
                    let delay = Duration::from_secs(2u64.pow(retry_count));
                    tokio::time::sleep(delay).await;
                    
                    // Attempt reconnection
                    *state.lock().unwrap() = NetworkState::Reconnecting {
                        attempt: retry_count,
                        max_attempts: MAX_RETRIES,
                    };
                    
                    if let Err(e) = establish_connection().await {
                        *state.lock().unwrap() = NetworkState::ConnectionFailed {
                            error: e.to_string(),
                            retry_count,
                        };
                    }
                } else {
                    // Max retries reached, notify user
                    *state.lock().unwrap() = NetworkState::Disconnected;
                }
            }
            _ => tokio::time::sleep(Duration::from_secs(1)).await,
        }
    }
}
```

## Performance Optimization

Message batching is essential for reducing network overhead and improving performance, especially when dealing with high-frequency updates like game state synchronization.

### MessageBatch Struct

```rust
use std::collections::HashMap;
use instant::Instant;

#[derive(Debug, Clone)]
pub struct MessageBatch {
    messages: Vec<Vec<u8>>,
    max_batch_size: usize,
    max_wait_time: Duration,
    last_flush: Instant,
    channel_id: u8,
}

impl MessageBatch {
    pub fn new(channel_id: u8, max_batch_size: usize, max_wait_time: Duration) -> Self {
        Self {
            messages: Vec::new(),
            max_batch_size,
            max_wait_time,
            last_flush: Instant::now(),
            channel_id,
        }
    }
    
    pub fn add_message(&mut self, data: Vec<u8>) -> Option<Vec<Vec<u8>>> {
        self.messages.push(data);
        
        // Flush conditions:
        // 1. Batch size reached
        // 2. Max wait time exceeded
        if self.messages.len() >= self.max_batch_size 
            || self.last_flush.elapsed() >= self.max_wait_time {
            self.flush()
        } else {
            None
        }
    }
    
    pub fn flush(&mut self) -> Option<Vec<Vec<u8>>> {
        if self.messages.is_empty() {
            return None;
        }
        
        let batched = std::mem::take(&mut self.messages);
        self.last_flush = Instant::now();
        
        Some(batched)
    }
    
    pub fn should_flush(&self) -> bool {
        !self.messages.is_empty() && (
            self.messages.len() >= self.max_batch_size 
            || self.last_flush.elapsed() >= self.max_wait_time
        )
    }
}

// Usage in networking task
async fn process_outgoing_messages(
    socket: &mut WebRtcSocket,
    batches: &mut HashMap<u8, MessageBatch>
) -> Result<(), Box<dyn std::error::Error>> {
    for (channel_id, batch) in batches.iter_mut() {
        if let Some(messages) = batch.flush() {
            // Serialize batched messages
            let serialized = bincode::serialize(&messages)?;
            socket.send(serialized, *channel_id);
        }
    }
    Ok(())
}
```

### Batching Logic Explanation

The message batching system works by:

1. **Accumulating messages** until either:
   - The batch reaches maximum size (e.g., 100 messages)
   - Maximum wait time is exceeded (e.g., 16ms for 60fps)

2. **Reducing network overhead** by:
   - Minimizing packet headers
   - Reducing WebRTC data channel calls
   - Optimizing for MTU sizes

3. **Maintaining responsiveness** through:
   - Configurable batch sizes based on use case
   - Time-based flushing for real-time requirements
   - Priority queuing for critical messages

## Production Deployment

Deploying egui-matchbox applications to production requires careful consideration of signaling server hosting, NAT traversal, and scalability.

### Signaling Server Hosting

The **matchbox_server** acts as a signaling server for WebRTC peer discovery. In production, this must be hosted on a cloud provider with proper SSL/TLS termination.

#### Docker Compose Configuration

```yaml
# docker-compose.yml
version: '3.8'

services:
  matchbox_server:
    image: matchbox_server:latest
    ports:
      - "3536:3536"
    environment:
      - RUST_LOG=info
      - MATCHBOX_PORT=3536
      - MATCHBOX_TLS_CERT_PATH=/certs/cert.pem
      - MATCHBOX_TLS_KEY_PATH=/certs/key.pem
    volumes:
      - ./certs:/certs:ro
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3536/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - ./certs:/etc/nginx/certs:ro
    depends_on:
      - matchbox_server
    restart: unless-stopped
```

#### Nginx Configuration for SSL Termination

```nginx
# nginx.conf
upstream matchbox_backend {
    server matchbox_server:3536;
}

server {
    listen 443 ssl http2;
    server_name your-domain.com;
    
    ssl_certificate /etc/nginx/certs/cert.pem;
    ssl_certificate_key /etc/nginx/certs/key.pem;
    
    location / {
        proxy_pass http://matchbox_backend;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # WebSocket specific settings
        proxy_read_timeout 86400;
        proxy_send_timeout 86400;
    }
}
```

### STUN/TURN Configuration

STUN (Session Traversal Utilities for NAT) and TURN (Traversal Using Relays around NAT) servers are essential for NAT traversal in production environments.

#### Why STUN/TURN is Necessary

- **STUN**: Helps peers discover their public IP addresses and open ports
- **TURN**: Acts as a relay when direct peer-to-peer connection fails (symmetric NAT, firewall restrictions)
- **Production requirement**: Essential for reliable connectivity across diverse network environments

#### Configuring STUN/TURN in WebRtcSocketBuilder

```rust
use matchbox_socket::{WebRtcSocketBuilder, RtcIceServer};

// Production STUN/TURN configuration
let ice_servers = vec![
    // Google's public STUN servers (free, but rate-limited)
    RtcIceServer {
        urls: vec![
            "stun:stun.l.google.com:19302".to_string(),
            "stun:stun1.l.google.com:19302".to_string(),
        ],
        username: None,
        credential: None,
    },
    // Your TURN server (recommended for production)
    RtcIceServer {
        urls: vec![
            "turn:your-turn-server.com:3478".to_string(),
            "turns:your-turn-server.com:5349".to_string(), // TLS
        ],
        username: Some("your-username".to_string()),
        credential: Some("your-password".to_string()),
    },
];

let (socket, message_loop) = WebRtcSocketBuilder::new("wss://your-domain.com:3536")
    .ice_servers(ice_servers)
    .add_channel(ChannelConfig {
        channel_id: 0,
        max_retransmits: None,
        ordered: false,
    })
    .build();
```

#### Setting Up Your Own TURN Server

For production deployments, consider hosting your own TURN server using **coturn**:

```bash
# Install coturn on Ubuntu/Debian
sudo apt update && sudo apt install coturn

# Configure /etc/turnserver.conf
listening-port=3478
tls-listening-port=5349
listening-ip=YOUR_SERVER_IP
relay-ip=YOUR_SERVER_IP
external-ip=YOUR_PUBLIC_IP
realm=your-domain.com
server-name=your-turn-server
lt-cred-mech
user=your-username:your-password
cert=/path/to/cert.pem
pkey=/path/to/key.pem

# Start the TURN server
sudo systemctl start coturn
```

### Production Checklist

- [ ] Host matchbox_server on cloud provider (AWS, GCP, Azure)
- [ ] Configure SSL/TLS certificates for signaling server
- [ ] Set up domain name with proper DNS records
- [ ] Deploy TURN server for reliable NAT traversal
- [ ] Configure firewall rules for ports 3478, 5349 (TURN)
- [ ] Implement health checks and monitoring
- [ ] Set up log aggregation (ELK stack, CloudWatch, etc.)
- [ ] Configure auto-scaling for high-traffic applications
- [ ] Implement rate limiting to prevent abuse
- [ ] Set up backup signaling servers for redundancy

## Getting Started

To get started with egui-matchbox integration:

1. **Add dependencies** to your `Cargo.toml`:
```toml
[dependencies]
egui = "0.25"
matchbox_socket = { version = "0.8", features = ["ggrs"] }
tokio = { version = "1.0", features = ["full"] }
```

2. **Clone and run the signaling server**:
```bash
git clone https://github.com/johanhelsing/matchbox
cd matchbox
cargo run --bin matchbox_server -- --port 3536
```

3. **Run the example application**:
```bash
cargo run --example simple_p2p
```

## Additional Resources

- [Matchbox Documentation](https://docs.rs/matchbox_socket)
- [egui Documentation](https://docs.rs/egui)
- [WebRTC Specification](https://www.w3.org/TR/webrtc/)
- [NAT Traversal Guide](https://www.rfc-editor.org/rfc/rfc5389)
- [Production Deployment Examples](https://github.com/johanhelsing/matchbox/tree/main/examples)
- [WebRTC Data Channels](https://developer.mozilla.org/en-US/docs/Web/API/WebRTC_API/Using_data_channels)
- [Rust Async Programming](https://rust-lang.github.io/async-book/)
