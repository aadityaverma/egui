#!/bin/bash
cd ../egui-matchbox-server
export SANDBOX_VOLUMES="$(pwd):/workspace:rw"
export OPENHANDS_AGENT="CodeActAgent"
export LLM_MODEL="openrouter/deepseek/deepseek-r1-0528" # Free for server tasks
uvx --python 3.12 --from openhands-ai openhands
