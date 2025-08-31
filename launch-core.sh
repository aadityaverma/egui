#!/bin/bash
cd ../egui-core
export SANDBOX_VOLUMES="$(pwd):/workspace:rw"
export OPENHANDS_AGENT="ManagerAgent"
export LLM_MODEL="openrouter/deepseek/deepseek-r1-0528"  # Free for systems programming
openhands
