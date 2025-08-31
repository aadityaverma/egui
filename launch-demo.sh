#!/bin/bash
cd ../egui-demo
export SANDBOX_VOLUMES="$(pwd):/workspace:rw"
export OPENHANDS_AGENT="ManagerAgent"
export LLM_MODEL="openrouter/moonshotai/kimi-k2"  # Free for creative UI work
openhands
