#!/bin/bash
cd ../egui-matchbox-wasm
export SANDBOX_VOLUMES="$(pwd):/workspace:rw"
export OPENHANDS_AGENT="ManagerAgent"
export LLM_MODEL="openrouter/moonshotai/kimi-k2" # Free for web deployment
uvx --python 3.12 --from openhands-ai openhands
