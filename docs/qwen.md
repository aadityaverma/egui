# egui: An Easy-to-Use GUI in Pure Rust

## Project Overview

egui is a simple, fast, and highly portable immediate mode GUI library for Rust. It runs on the web (via Wasm), natively on desktop (Linux, Mac, Windows), and can be integrated into game engines. The project is part of the egui ecosystem, which includes several interdependent crates that provide GUI functionality, rendering, and application frameworks.

The main crates in the egui ecosystem are:
- **egui**: The core GUI library with widgets, layouts, and UI logic
- **emath**: Minimal 2D math library (Vec2, Rect, Pos2, etc.)
- **epaint**: 2D shapes and text rendering system
- **eframe**: Official framework for building cross-platform apps (web & native)
- **egui-winit**: Integration with the winit windowing library
- **egui_glow**: OpenGL-based renderer for native and web
- **egui-wgpu**: WebGPU-based renderer
- **egui_extras**: Additional features built on top of egui

The project emphasizes ease of use, immediate mode GUI patterns, and cross-platform compatibility. It's developed and maintained by Emil Ernerfeldt and sponsored by Rerun, a company building SDKs for visualizing multimodal data streams.

## Building and Running

### Prerequisites
- Rust 1.88 or later
- For native development on Linux: `sudo apt-get install -y libclang-dev libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

### Build Commands
- `cargo build`: Build all crates
- `cargo run -p egui_demo_app`: Run the demo application locally
- `cargo run --release -p egui_demo_app`: Run the demo with optimizations
- `./scripts/check.sh`: Run all CI-like checks (formatting, clippy, tests, etc.)

### Web Development
- To build for web: `./scripts/build_demo_web.sh`
- To serve web demo: `./scripts/start_server.sh` (then visit http://localhost:8765)
- Install web target: `rustup target add wasm32-unknown-unknown`

### Testing
- `cargo test`: Run all tests
- `cargo test --workspace --all-features`: Run all tests with all features enabled
- `UPDATE_SNAPSHOTS=true cargo test --workspace --all-features`: Update snapshot tests
- `cargo test --doc`: Run documentation tests

### Linting and Code Quality
- `cargo fmt --all -- --check`: Check formatting (requires rustfmt)
- `cargo clippy --all-targets --all-features -- -D warnings`: Run lints
- `typos`: Check for typos
- `./scripts/check.sh`: Run comprehensive CI checks

## Development Conventions

### Code Style
- Follow Rust API Guidelines
- Use `TODO(username): comment` instead of `FIXME`
- Add blank lines around all `fn`, `struct`, `enum`, etc.
- Use `// Comment like this.` format (with space after //)
- Write idiomatic Rust code
- Add docstrings to types, struct fields, and all `pub fn`
- Include example code (doc-tests) where appropriate
- Before making a function longer, consider adding a helper function
- Coordinate system: (0,0) is top-left, X increases right, Y increases down
- Use logical "points" as coordinate system (related to physical pixels by `pixels_per_point`)

### Architecture Principles
- Avoid `unsafe` code (egui is "unsafe forbidden")
- Avoid `unwrap` and `expect` - use proper error handling
- Use good names for everything
- Leave the code cleaner than how you found it
- Keep pull requests small and focused
- Respect immediate mode GUI principles (no callbacks, state stored outside GUI)

### Crate Structure
- `egui`: Core GUI logic and widgets
- `emath`: Mathematics (Vec2, Rect, etc.)
- `epaint`: Painting and rendering primitives
- `eframe`: Framework for cross-platform apps
- Each crate has specific responsibilities with minimal dependencies between them

### Testing and Quality Assurance
- Run `./scripts/check.sh` before submitting PRs
- Update snapshot tests if UI changes are expected
- Use `egui_kittest` for accessibility testing
- For non-macOS systems, snapshot tests might need updates from CI

### Git and Workflow
- Use git-lfs for large files (images, etc.)
- Run `git lfs install` after cloning
- Small commits are fine - they'll be squashed on merge
- Keep PRs focused on a single feature or bug fix
- Open draft PRs early to get early feedback