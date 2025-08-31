Strategy for Multi-Agent Egui Development with OpenHands:

  The core idea is to leverage OpenHands' multi-agent capabilities to parallelize development efforts on the
  egui project. Instead of a single agent trying to tackle all aspects, we've created specialized
  "micro-agents," each focusing on a distinct area of the egui architecture.

   1. Specialized Micro-Agents:
       * Core Library Agent: Focuses on the fundamental egui, emath, and epaint crates. This agent would handle
         low-level UI patterns, widget optimization, and rendering primitives.
       * Demo Development Agent: Concentrates on egui_demo_lib, responsible for creating compelling widget
         showcases, interactive examples, and improving the overall user experience of the demos.
       * Web Integration Agent: Specializes in web_demo and WASM compilation, optimizing for web performance,
         JavaScript-Rust interop, and browser compatibility.
       * Framework Agent: Handles eframe, focusing on cross-platform framework development, backend integrations
         (glow, wgpu), and application lifecycle management.
       * Documentation Agent: Dedicated to examples and general documentation, ensuring clear API docs,
         tutorials, and comprehensive code examples.

   2. Git Worktrees for Isolation:
       * The use of git worktree allows each agent to work on a separate, isolated branch and directory
         (egui-core, egui-demo, etc.). This prevents conflicts during parallel development and allows agents to
         focus on their specific codebase without interference.

   3. Dedicated Launch Scripts:
       * Each launch-*.sh script sets up the environment for a specific agent, including the working directory,
         sandbox volumes, the OpenHands agent type (e.g., ManagerAgent, CodeActAgent), and a tailored LLM model.
         This ensures each agent has the optimal tools and context for its specialized tasks.

   4. LLM Model Specialization:
       * Different LLM models are assigned to agents based on their strengths (e.g., deepseek-coder for systems
         programming, claude-3.5-sonnet for creative UI work and writing, gpt-4o for web technologies). This is
         a best practice to maximize agent effectiveness and efficiency.

   5. MCP Server Configuration:
       * The config.toml includes rust-analyzer and cargo-helper as MCP (Multi-Agent Communication Protocol)
         stdio servers. This provides the agents with essential Rust development tools, enabling them to
         understand and interact with the Rust codebase effectively.

  Development Workflow:

   * Parallel Understanding: All agents can be launched simultaneously to analyze their respective parts of the
     project, building a deep understanding of their domain.
   * Specialized Development Tasks: Once understanding is established, specific tasks can be assigned to each
     agent (e.g., "Implement a new widget type" for the Core Library Agent, "Optimize WASM bundle size" for the
     Web Integration Agent).
   * Integration and Testing: After individual tasks are completed, changes from each worktree can be merged
     back into the main egui repository. Comprehensive tests (e.g., cargo test --workspace, build_demo_web.sh)
     would then be run to ensure all changes integrate correctly.

  This multi-agent approach aims to accelerate development by allowing parallel work, leveraging specialized
  expertise, and reducing context switching, ultimately leading to a more efficient and robust development
  process for egui.

  You can now proceed with creating the git worktrees and launching your OpenHands agents as described in the
  provided documentation.
