# Copilot Instructions for egui

## Project Overview
- `egui` is a Rust immediate mode GUI library, designed for simplicity, speed, and portability. It runs natively and on the web (Wasm).
- The repository is organized as a multi-crate workspace. Key crates:
  - `egui`: Core GUI library (depends on `emath`, `epaint`)
  - `eframe`: Official framework for running egui apps (native & web)
  - `egui_demo_app`, `egui_demo_lib`: Demo app and UI examples
  - `egui_extras`, `egui-winit`, `egui_glow`, `egui_kittest`: Extensions, platform bindings, and testing
  - `emath`, `epaint`, `epaint_default_fonts`: Math, painting, and font utilities

## Architecture & Data Flow
- UI code is written in immediate mode style: each frame, the UI is rebuilt from scratch.
- Data flows from user input events (via platform bindings like `egui-winit`) to UI state, then rendered as textured triangles (via `epaint`).
- `eframe` abstracts platform differences, allowing the same app to run on desktop and web.

## Developer Workflows
- **Build & Test:**
  - Run all checks: `./scripts/check.sh` (runs lints, formatting, docs, clippy, tests, wasm checks)
  - Build web demo: `./scripts/build_demo_web.sh [--release] [--wgpu] [--open]`
  - Update snapshot tests: `UPDATE_SNAPSHOTS=true cargo test --workspace --all-features`
  - If CI snapshot mismatch: `./scripts/update_snapshots_from_ci.sh`
- **Testing:**
  - UI tests use `egui_kittest` (see `crates/egui_kittest/README.md`)
  - Snapshots and large files use git-lfs
- **Platform-specific:**
  - Web builds use Wasm, with optional WebGPU (`--wgpu` flag)
  - Native builds use `glow` or `wgpu` via `eframe`

## Conventions & Patterns
- All UI is rebuilt every frame; avoid storing persistent widget state outside the app state.
- Use `ui.*` methods for building UI (see examples in `README.md` and `examples/`)
- Platform event translation handled in `egui-winit`
- Large assets (e.g., images) tracked via git-lfs
- PRs should be small, focused, and squash-merged

## Integration Points
- External engines (e.g., bevy, miniquad) integrate via `egui` API (see ARCHITECTURE.md)
- Platform bindings: `egui-winit` (desktop), `eframe` (web/native)
- UI testing: `egui_kittest` (AccessKit-based)

## Key References
- `ARCHITECTURE.md`: crate relationships and data flow
- `CONTRIBUTING.md`: PR/test/snapshot workflow, git-lfs usage
- `README.md`: project intro, quick start, example code, links to docs and demos
- `examples/`: practical UI code samples
- `scripts/`: build, test, and CI helper scripts
- `crates/egui_kittest/README.md`: UI testing patterns

---
For unclear or missing conventions, review `ARCHITECTURE.md`, `CONTRIBUTING.md`, and crate-level READMEs. Ask maintainers via GitHub Discussions or Discord for edge cases.
