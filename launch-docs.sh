#!/bin/bash
cd ../egui-docs
export SANDBOX_VOLUMES="$(pwd):/workspace:rw"
export OPENHANDS_AGENT="CodeActAgent"  # Simple agent for documentation
export LLM_MODEL="openrouter/moonshotai/kimi-k2"  # Free for writing
openhands
