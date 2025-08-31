
# Matchbox + egui P2P Demo

A comprehensive example demonstrating P2P WebRTC networking with egui integration using Matchbox.

## Features

- **P2P Chat Application**: Real-time messaging between peers
- **WebRTC Networking**: Direct peer-to-peer connections via WebRTC
- **Cross-platform**: Works on native and WASM targets
- **Error Handling**: Comprehensive network state management
- **Web Deployment**: Ready for web deployment with Trunk

## Quick Start

### Native Build

```bash
# Install matchbox_server (signaling server)
cargo install matchbox_server

# Run the signaling server
matchbox_server

# In another terminal, run the demo
cargo run --bin matchbox_p2p
```

### Web Build

```bash
# Install trunk
cargo install trunk

# Build and serve for development
trunk serve

# Build for production
trunk build --release
```

## Architecture

The application uses a modular architecture with:

- **Network Layer**: WebRTC P2P networking via Matchbox
- **State Management**: Shared state with async message passing
- **UI Layer**: egui immediate-mode GUI
- **Error Handling**: Comprehensive error management and recovery

## Configuration

### Signaling Server

The signaling server runs on port 3536 by default. For production, configure:

```bash
matchbox_server --port 8080 --public-ip your-ip
```

### Web Deployment

For web deployment, use the provided Trunk configuration:

```toml
# Trunk.toml
[build]
target = "index.html"
dist = "dist"
```

## Usage

1. **Connect**: Enter a room URL and click "Connect"
2. **Chat**: Send messages to connected peers
3. **Disconnect**: Click "Disconnect" to leave the room

## Development

### Running Tests

```bash
cargo test
```

### Building for Production

```bash
cargo build --release
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

MIT License - see LICENSE file for details
```

## License

MIT License - see LICENSE file for details
