# Copilot Instructions for egui

## Project Overview

- `egui` is a Rust immediate mode GUI library for desktop and web (Wasm), designed for simplicity, speed, and portability.
- Multi-crate workspace. Key crates:
  - `egui`: Core GUI library (immediate mode, stateless widgets; depends on `emath`, `epaint`)
  - `eframe`: Official framework for running egui apps (cross-platform, abstracts platform differences)
  - `egui_demo_app`, `egui_demo_lib`: Demo app and UI examples (see online at https://egui.rs)
  - `egui_extras`: Add-on features (e.g., image loaders)
  - `egui-winit`, `egui_glow`, `egui-wgpu`: Platform bindings and rendering backends
  - `egui_kittest`: UI testing harness (AccessKit-based)
  - `emath`, `epaint`, `epaint_default_fonts`: Math, painting, and font utilities

## Architecture & Data Flow

- UI is rebuilt every frame (immediate mode): do not persist widget state outside app state.
- Data flow: user input events (via platform bindings, e.g. `egui-winit`) → UI state → rendered as textured triangles (via `epaint`).
- `eframe` enables the same app to run natively or as a web app (Wasm/WebGL/WebGPU).
- See `ARCHITECTURE.md` for crate relationships and integration details.

## Developer Workflows

- **Build & Test:**
  - Run all checks: `./scripts/check.sh` (lints, formatting, docs, clippy, tests, wasm checks)
  - Build web demo: `./scripts/build_demo_web.sh [--release] [--wgpu] [--open]` (see script for options)
  - Update snapshot tests: `UPDATE_SNAPSHOTS=true cargo test --workspace --all-features`
  - If CI snapshot mismatch: `./scripts/update_snapshots_from_ci.sh` (downloads and applies CI-generated snapshots)
- **Testing:**
  - UI tests: use `egui_kittest` (see `crates/egui_kittest/README.md` for harness and snapshot testing)
  - Snapshots and large files tracked via git-lfs (see `.gitattributes` and `CONTRIBUTING.md`)
- **Platform-specific:**
  - Web builds: Wasm, with optional WebGPU (`--wgpu` flag)
  - Native builds: `glow` or `wgpu` via `eframe`
  - Demo app: `cargo run --release -p egui_demo_app` (native) or `./scripts/build_demo_web.sh --open` (web)

## Conventions & Patterns

- UI is rebuilt every frame; keep all persistent state in your app struct, not in widgets.
- Use `ui.*` methods for building UI (see `README.md` and `examples/` for idiomatic patterns).
- Platform event translation: handled in `egui-winit` (desktop) and `eframe` (web/native).
- Large assets (e.g., images) tracked via git-lfs; add new images to LFS and check `.gitattributes`.
- PRs: keep small, focused, and squash-merged (see `CONTRIBUTING.md`).

## Integration Points

- External engines (e.g., bevy, miniquad) integrate via `egui` API (see `ARCHITECTURE.md`)
- Platform bindings: `egui-winit` (desktop), `eframe` (web/native), `egui_glow`, `egui-wgpu`
- UI testing: `egui_kittest` (AccessKit-based, see README for snapshot/image testing guidelines)

## Key References

- `ARCHITECTURE.md`: crate relationships, data flow, and integration boundaries
- `CONTRIBUTING.md`: PR/test/snapshot workflow, git-lfs usage
- `README.md`: project intro, quick start, example code, links to docs and demos
- `examples/`: practical UI code samples (see also online demo at https://egui.rs)
- `scripts/`: build, test, and CI helper scripts (see `check.sh`, `build_demo_web.sh`, `update_snapshots_from_ci.sh`)
- `crates/egui_kittest/README.md`: UI testing and snapshot patterns

---

For unclear or missing conventions, review `ARCHITECTURE.md`, `CONTRIBUTING.md`, and crate-level READMEs. For edge cases, ask maintainers via GitHub Discussions or Discord.
