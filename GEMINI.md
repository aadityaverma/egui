# Gemini Code Assistant Report: `egui`

This document provides a comprehensive overview of the `egui` project, an immediate mode GUI library for Rust. It is intended to be a quick-start guide for developers working on this codebase.

## Project Overview

`egui` is a pure Rust library that provides a simple, fast, and highly portable immediate mode GUI. It can be used to create web applications, native desktop applications, and can be integrated into game engines. The library is designed to be easy to use and to have minimal dependencies.

The project is structured as a Rust workspace with several crates, including:

*   `egui`: The core `egui` library.
*   `eframe`: The official `egui` framework for building apps for web and native platforms.
*   `egui_glow`: A backend for rendering `egui` with `glow` (a Rust wrapper for OpenGL/WebGL).
*   `egui-wgpu`: A backend for rendering `egui` with `wgpu` (a Rust wrapper for WebGPU).
*   `egui_demo_app`: A demo application that showcases the features of `egui`.

## Building and Running

### Running the Demo

To run the demo application, use the following command:

```bash
cargo run --release -p egui_demo_app
```

On Linux, you may need to install some dependencies first:

```bash
sudo apt-get install -y libclang-dev libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
```

On Fedora Rawhide, you may need to install some dependencies first:

```bash
dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel
```

### Building the Project

To build the entire project, you can use the following command:

```bash
cargo build --all
```

### Running Tests

To run the tests, you can use the following command:

```bash
cargo test --all
```

## Development Conventions

*   **Code Style**: The project uses `rustfmt` to enforce a consistent code style.
*   **Linting**: The project uses `clippy` to catch common mistakes and to enforce a set of linting rules. The linting rules are defined in the `clippy.toml` file and in the `[workspace.lints]` section of the `Cargo.toml` file.
*   **Contributing**: The `CONTRIBUTING.md` file provides guidelines for contributing to the project.
*   **Dependencies**: The project uses `cargo-deny` to check for security vulnerabilities and to enforce a set of rules for dependencies.
