



# Advanced Matchbox + egui P2P Integration Guide

This guide provides comprehensive documentation for advanced P2P networking features using Matchbox with egui, including production deployment strategies.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Advanced Features](#advanced-features)
3. [Performance Optimization](#performance-optimization)
4. [Production Deployment](#production-deployment)
5. [Docker Configuration](#docker-configuration)
6. [WebAssembly Optimization](#webassembly-optimization)
7. [Security Considerations](#security-considerations)
8. [Troubleshooting](#troubleshooting)

## Architecture Overview

The enhanced architecture consists of:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   egui UI Layer │    │  Network Layer  │    │  Matchbox Core  │
├─────────────────┤    ├─────────────────┤    ├─────────────────┤
│   Game State    │◄──►│ Message Queue   │◄──►│ WebRTC Socket   │
│   Chat System   │    │ Error Handling  │    │ Signaling       │
│   Debug Panel   │    │ Reconnection    │    │ P2P Channels    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## Advanced Features

### 1. Multiple Channel Support

The enhanced implementation supports multiple data channels with different reliability guarantees:

```rust
// Reliable channel for chat messages
socket_builder = socket_builder.add_reliable_channel();

// Unreliable channel for game state updates
socket_builder = socket_builder.add_unreliable_channel();
```

### 2. Message Batching

For high-frequency updates, implement message batching:

```rust
struct MessageBatch {
    messages: Vec<Vec<u8>>,
    last_send: Instant,
    max_batch_size: usize,
    max_wait_time: Duration,
}
```

### 3. Automatic Reconnection

The system includes automatic reconnection with exponential backoff:

```rust
pub struct ReconnectConfig {
    max_attempts: u32,
    initial_delay: Duration,
    max_delay: Duration,
    backoff_multiplier: f32,
}
```

### 4. Network Statistics

Real-time network monitoring:

- Messages sent/received
- Bytes transferred
- Packet loss percentage
- Round-trip time (ping)
- Connection quality metrics

## Performance Optimization

### 1. WebAssembly Bundle Size

Optimize WASM bundle size:

```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
```

### 2. Memory Management

Use object pooling for frequently allocated objects:

```rust
struct ObjectPool<T> {
    objects: Vec<T>,
    max_size: usize,
}
```

### 3. Rendering Optimization

- Use `ctx.request_repaint()` judiciously
- Implement frame rate limiting
- Cache expensive calculations

## Production Deployment

### 1. Docker Deployment

#### Quick Start with Docker

```bash
# Build and run with Docker Compose
docker-compose up --build

# Or build manually
docker build -t matchbox-p2p-demo .
docker run -p 8080:80 matchbox-p2p-demo
```

#### Production Configuration

```yaml
# docker-compose.prod.yml
version: '3.8'
services:
  matchbox-server:
    image: matchbox_server:latest
    ports:
      - "3536:3536"
    environment:
      - RUST_LOG=info
    restart: unless-stopped
    
  app:
    build:
      context: .
      dockerfile: Dockerfile.prod
    ports:
      - "80:80"
      - "443:443"
    depends_on:
      - matchbox-server
```

### 2. Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: matchbox-p2p-demo
spec:
  replicas: 3
  selector:
    matchLabels:
      app: matchbox-p2p-demo
  template:
    metadata:
      labels:
        app: matchbox-p2p-demo
    spec:
      containers:
      - name: matchbox-server
        image: matchbox_server:latest
        ports:
        - containerPort: 3536
      - name: app
        image: matchbox-p2p-demo:latest
        ports:
        - containerPort: 80
```

### 3. Cloud Deployment

#### AWS ECS

```json
{
  "family": "matchbox-p2p-demo",
  "networkMode": "awsvpc",
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "256",
  "memory": "512",
  "containerDefinitions": [
    {
      "name": "matchbox-server",
      "image": "matchbox_server:latest",
      "portMappings": [{"containerPort": 3536}]
    },
    {
      "name": "app",
      "image": "matchbox-p2p-demo:latest",
      "portMappings": [{"containerPort": 80}]
    }
  ]
}
```

#### Google Cloud Run

```yaml
apiVersion: serving.knative.dev/v1
kind: Service
metadata:
  name: matchbox-p2p-demo
spec:
  template:
    spec:
      containers:
      - image: gcr.io/PROJECT/matchbox-p2p-demo
        ports:
        - containerPort: 8080
```

## WebAssembly Optimization

### 1. Build Configuration

```bash
# Install trunk
cargo install trunk

# Build for production
trunk build --release

# Optimize WASM size
wasm-opt -Oz -o dist/app_bg.wasm dist/app_bg.wasm
```

### 2. Runtime Configuration

```javascript
// index.html optimization
<script>
  // Preload critical resources
  const link = document.createElement('link');
  link.rel = 'preload';
  link.as = 'fetch';
  link.href = './pkg/matchbox_p2p_bg.wasm';
  document.head.appendChild(link);
</script>
```

## Security Considerations

### 1. Signaling Server Security

```rust
// Add authentication to signaling server
let socket = WebRtcSocketBuilder::new("wss://your-domain.com/signaling")
    .ice_server("stun:stun.l.google.com:19302")
    .ice_server("turn:your-turn-server.com:3478")
    .build();
```

### 2. HTTPS Configuration

```nginx
# nginx.conf SSL configuration
server {
    listen 443 ssl http2;
    ssl_certificate /etc/ssl/certs/cert.pem;
    ssl_certificate_key /etc/ssl/private/key.pem;
    
    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

### 3. CORS Configuration

```rust
// Add CORS headers for web deployment
.add_header("Access-Control-Allow-Origin", "*")
.add_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
```

## Troubleshooting

### Common Issues

#### 1. Connection Problems

**Issue**: Cannot connect to signaling server
```bash
# Check if server is running
curl http://localhost:3536/health

# Check firewall settings
sudo ufw allow 3536
```

#### 2. WebRTC Issues

**Issue**: WebRTC connection fails
- Check STUN/TURN server configuration
- Verify firewall rules for UDP traffic
- Test with different networks (NAT traversal)

#### 3. WebAssembly Issues

**Issue**: WASM module fails to load
```bash
# Check browser console for errors
# Verify MIME type configuration
# Check CORS headers
```

### Debug Mode

Enable debug logging:

```bash
# Set log level
export RUST_LOG=debug

# Run with verbose output
RUST_LOG=debug cargo run --bin matchbox_p2p
```

### Performance Profiling

```bash
# Profile with perf
perf record -g cargo run --release
perf report

# Memory profiling
valgrind --tool=massif cargo run --release
```

## Advanced Configuration Examples

### 1. Custom STUN/TURN Configuration

```rust
let config = NetworkConfig {
    room_url: "wss://your-domain.com/signaling".to_string(),
    ice_servers: vec![
        "stun:stun.l.google.com:19302".to_string(),
        "turn:your-turn-server.com:3478".to_string(),
    ],
    ..Default::default()
};
```

### 2. Game-Specific Configuration

```rust
let game_config = NetworkConfig {
    heartbeat_interval: Duration::from_millis(100),
    max_message_size: 1024 * 16, // 16KB for game state
    enable_unreliable_channel: true,
    enable_reliable_channel: true,
    ..Default::default()
};
```

### 3. Production Monitoring

```rust
// Add metrics collection
struct NetworkMetrics {
    connection_duration: Duration,
    message_latency: Vec<Duration>,
    bandwidth_usage: u64,
}
```

## Contributing

To contribute to this example:

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Update documentation
5. Submit a pull request

## License

MIT License - see LICENSE file for details.



