#!/bin/bash
cd ../egui-matchbox-docs
export SANDBOX_VOLUMES="$(pwd):/workspace:rw"
export OPENHANDS_AGENT="CodeActAgent"
export LLM_MODEL="openrouter/moonshotai/kimi-k2" # Free for documentation and examples
uvx --python 3.12 --from openhands-ai openhands
