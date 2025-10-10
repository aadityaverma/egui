#!/bin/bash
# This script installs and runs the Matchbox signaling server.
echo "Installing matchbox_server..."
cargo install matchbox_server
echo "Starting matchbox_server on default port 3536..."
matchbox_server
