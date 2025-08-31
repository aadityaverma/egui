#!/bin/bash
cd ../egui-matchbox-app
export SANDBOX_VOLUMES="$(pwd):/workspace:rw"
export OPENHANDS_AGENT="ManagerAgent"
export LLM_MODEL="openrouter/moonshotai/kimi-k2" # Free for creative UI work and integration
uvx --python 3.12 --from openhands-ai openhands
