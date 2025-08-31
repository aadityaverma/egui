#!/bin/bash
cd ../egui-matchbox-net
export SANDBOX_VOLUMES="$(pwd):/workspace:rw"
export OPENHANDS_AGENT="ManagerAgent"
export LLM_MODEL="openrouter/deepseek/deepseek-r1-0528" # Free for networking logic
uvx --python 3.12 --from openhands-ai openhands
