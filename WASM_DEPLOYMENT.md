# WASM Deployment with Trunk

This project is configured to be built as a WebAssembly application using Trunk.

## Prerequisites

Make sure you have Trunk installed:
```bash
cargo install trunk
```

**Note:** Ensure your `Cargo.toml` contains the necessary WASM dependencies. The project already includes:
- `wasm-bindgen`
- `wasm-bindgen-futures`
- `web-sys`
- `js-sys`

These are configured under the `[target.'cfg(target_arch = "wasm32")'.dependencies]` section in the relevant crates.

## Configuration Files

### Trunk.toml
The project includes a properly configured `Trunk.toml`:
- Builds from `index.html`
- Outputs to `dist/` directory
- Serves on `127.0.0.1:8080`

### index.html
Enhanced HTML file with:
- Mobile viewport scaling
- PWA description meta tags
- Full-screen canvas for egui
- Proper WASM module linking

## Development

Run the development server with live reloading:
```bash
trunk serve
```

This will:
- Build the WASM module
- Start a local server at http://127.0.0.1:8080
- Watch for file changes and rebuild automatically

## Production

Build the optimized application for production:
```bash
trunk build --release
```

This creates optimized output in the `dist/` directory ready for deployment.

## Additional Commands

### Clean build artifacts
```bash
trunk clean
```

### Check without building
```bash
trunk check
```

## Troubleshooting

### Common Issues

1. **Missing WASM target**: Install the WebAssembly target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. **Build failures**: Ensure all dependencies are up to date:
   ```bash
   cargo update
   ```

3. **Browser compatibility**: The application requires a modern browser with WebAssembly and WebGL support.
