

# Advanced Matchbox + egui P2P Demo

A comprehensive example demonstrating advanced P2P WebRTC networking with egui integration using Matchbox, featuring production-ready deployment strategies.

## 🚀 Features

### Core Features
- **P2P Chat Application**: Real-time messaging between peers
- **WebRTC Networking**: Direct peer-to-peer connections via WebRTC
- **Cross-platform**: Works on native and WASM targets
- **Error Handling**: Comprehensive network state management
- **Web Deployment**: Ready for web deployment with Trunk

### Advanced Features
- **Multiple Data Channels**: Reliable and unreliable channels for different use cases
- **Message Batching**: Optimized for high-frequency updates
- **Automatic Reconnection**: Exponential backoff with configurable retry logic
- **Network Statistics**: Real-time monitoring and debugging
- **Game State Synchronization**: Example 2D game with multiplayer support
- **Docker Deployment**: Production-ready containerization
- **Kubernetes Support**: Scalable cloud deployment
- **Security Hardening**: HTTPS, CORS, and authentication support

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   egui UI Layer │    │  Network Layer  │    │  Matchbox Core  │
├─────────────────┤    ├─────────────────┤    ├─────────────────┤
│   Game State    │◄──►│ Message Queue   │◄──►│ WebRTC Socket   │
│   Chat System   │    │ Error Handling  │    │ Signaling       │
│   Debug Panel   │    │ Reconnection    │    │ P2P Channels    │
│   Statistics    │    │ Load Balancing  │    │ NAT Traversal   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## 🚀 Quick Start

### Prerequisites

```bash
# Install required tools
cargo install matchbox_server trunk docker-compose
```

### Option 1: Native Development

```bash
# Terminal 1: Start signaling server
matchbox_server --port 3536

# Terminal 2: Run native application
cargo run --bin matchbox_p2p
```

### Option 2: Web Development

```bash
# Start development server
trunk serve --port 8080
```

### Option 3: Docker (Production)

```bash
# Build and run with Docker Compose
docker-compose up --build

# Access at http://localhost:8080
```

### Option 4: Enhanced Features

```bash
# Run enhanced version with game features
cargo run --bin enhanced_matchbox_p2p
```

## 📋 Configuration

### Network Configuration

```rust
let config = NetworkConfig {
    room_url: "wss://your-domain.com/signaling".to_string(),
    heartbeat_interval: Duration::from_secs(5),
    reconnect_attempts: 5,
    reconnect_delay: Duration::from_secs(2),
    max_message_size: 1024 * 64,
    enable_reliable_channel: true,
    enable_unreliable_channel: true,
};
```

### STUN/TURN Configuration

```rust
let socket = WebRtcSocketBuilder::new("wss://your-domain.com/signaling")
    .ice_server("stun:stun.l.google.com:19302")
    .ice_server("turn:your-turn-server.com:3478")
    .build();
```

## 🎮 Usage Guide

### Basic Usage
1. **Connect**: Enter room URL and click "Connect"
2. **Chat**: Send messages to connected peers
3. **Game**: Drag to move your player in the game area
4. **Monitor**: View network statistics in debug panel

### Advanced Usage
1. **Multiple Channels**: Use reliable for chat, unreliable for game state
2. **Message Batching**: Automatic batching for high-frequency updates
3. **Reconnection**: Automatic retry with exponential backoff
4. **Statistics**: Real-time network performance monitoring

## 🐳 Docker Deployment

### Development

```bash
# Build and run development environment
docker-compose up

# Or build specific services
docker-compose up matchbox-server
docker-compose up matchbox-p2p-demo
```

### Production

```bash
# Production build
docker-compose -f docker-compose.prod.yml up --build

# With SSL/TLS
docker-compose -f docker-compose.ssl.yml up --build
```

### Kubernetes

```bash
# Deploy to Kubernetes
kubectl apply -f k8s/

# Check deployment status
kubectl get pods -l app=matchbox-p2p-demo
```

## 🔧 Development

### Running Tests

```bash
# Run all tests
./scripts/test_enhanced.sh

# Run specific tests
cargo test
cargo test --release

# Web tests
trunk build --release
```

### Performance Testing

```bash
# Load testing
cargo install wrk
wrk -t12 -c400 -d30s http://localhost:8080

# Memory profiling
cargo install valgrind
valgrind --tool=massif cargo run --release
```

### Debugging

```bash
# Enable debug logging
RUST_LOG=debug cargo run --bin matchbox_p2p

# Web debugging
RUST_LOG=debug trunk serve
```

## 📊 Monitoring

### Network Statistics

The application provides real-time monitoring:
- **Messages**: Sent/received count and rate
- **Bandwidth**: Bytes transferred
- **Latency**: Round-trip time measurements
- **Packet Loss**: Connection quality metrics
- **Peers**: Connected peer information

### Health Checks

```bash
# Check signaling server health
curl http://localhost:3536/health

# Check application health
curl http://localhost:8080/health
```

## 🔒 Security

### HTTPS Configuration

```nginx
server {
    listen 443 ssl http2;
    ssl_certificate /etc/ssl/certs/cert.pem;
    ssl_certificate_key /etc/ssl/private/key.pem;
    
    location / {
        proxy_pass http://localhost:8080;
    }
}
```

### CORS Configuration

```rust
// Add CORS headers
.add_header("Access-Control-Allow-Origin", "https://your-domain.com")
.add_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
```

## 🌐 WebAssembly Optimization

### Build Optimization

```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
```

### Runtime Optimization

```javascript
// Preload critical resources
<link rel="preload" href="app_bg.wasm" as="fetch" crossorigin>
```

## 🐛 Troubleshooting

### Common Issues

| Issue | Solution |
|-------|----------|
| Connection failed | Check firewall, verify STUN/TURN servers |
| WASM loading error | Check MIME types, verify CORS headers |
| High latency | Use closer STUN/TURN servers, check network |
| Memory leaks | Use profiling tools, check object pooling |

### Debug Commands

```bash
# Check signaling server
curl -v http://localhost:3536/health

# Test WebRTC
chrome://webrtc-internals/

# Network diagnostics
ping stun.l.google.com
```

## 📚 API Reference

### NetworkManager API

```rust
impl EnhancedNetworkManager {
    pub fn new(config: NetworkConfig) -> Self;
    pub async fn connect(&mut self, room_url: String) -> Result<(), String>;
    pub fn disconnect(&mut self);
    pub fn send_message(&self, message: NetworkMessage) -> Result<(), String>;
    pub fn get_stats(&self) -> NetworkStats;
}
```

### Configuration API

```rust
pub struct NetworkConfig {
    pub room_url: String,
    pub heartbeat_interval: Duration,
    pub reconnect_attempts: u32,
    pub reconnect_delay: Duration,
    pub max_message_size: usize,
    pub enable_reliable_channel: bool,
    pub enable_unreliable_channel: bool,
}
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Matchbox](https://github.com/johanhelsing/matchbox) for P2P networking
- [egui](https://github.com/emilk/egui) for the immediate-mode GUI
- [WebRTC](https://webrtc.org/) for peer-to-peer communication
- [Docker](https://docker.com/) for containerization

## 📖 Additional Resources

- [Advanced Guide](ADVANCED_GUIDE.md) - Comprehensive documentation for advanced features
- [Docker Guide](DOCKER.md) - Detailed Docker deployment instructions
- [WebAssembly Guide](WASM.md) - WebAssembly optimization techniques
- [API Documentation](docs/api.md) - Complete API reference

