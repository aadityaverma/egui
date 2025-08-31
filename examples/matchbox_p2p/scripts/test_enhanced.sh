#!/bin/bash

# Enhanced Matchbox P2P Demo Test Script

set -e

echo "🚀 Testing Enhanced Matchbox P2P Demo"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if required tools are installed
check_dependencies() {
    print_status "Checking dependencies..."
    
    # Check for Rust
    if ! command -v cargo &> /dev/null; then
        print_error "Rust is not installed. Please install Rust first."
        exit 1
    fi
    
    # Check for matchbox_server
    if ! command -v matchbox_server &> /dev/null; then
        print_warning "matchbox_server not found. Installing..."
        cargo install matchbox_server
    fi
    
    # Check for trunk (for web builds)
    if ! command -v trunk &> /dev/null; then
        print_warning "trunk not found. Installing..."
        cargo install trunk
    fi
    
    print_status "All dependencies are installed ✓"
}

# Test native build
test_native() {
    print_status "Testing native build..."
    
    # Start matchbox server in background
    print_status "Starting matchbox server..."
    matchbox_server &
    MATCHBOX_PID=$!
    
    # Wait for server to start
    sleep 2
    
    # Build and run native version
    print_status "Building native version..."
    cargo build --release
    
    # Test if build succeeds
    if [ $? -eq 0 ]; then
        print_status "Native build successful ✓"
    else
        print_error "Native build failed"
        kill $MATCHBOX_PID 2>/dev/null || true
        exit 1
    fi
    
    # Clean up
    kill $MATCHBOX_PID 2>/dev/null || true
}

# Test web build
test_web() {
    print_status "Testing web build..."
    
    # Build web version
    print_status "Building web version..."
    trunk build --release
    
    if [ $? -eq 0 ]; then
        print_status "Web build successful ✓"
    else
        print_error "Web build failed"
        exit 1
    fi
    
    # Check if dist directory exists
    if [ -d "dist" ]; then
        print_status "Web assets generated in dist/ ✓"
    else
        print_error "dist directory not found"
        exit 1
    fi
}

# Test Docker build
test_docker() {
    print_status "Testing Docker build..."
    
    # Check if Docker is installed
    if ! command -v docker &> /dev/null; then
        print_warning "Docker not found. Skipping Docker tests."
        return
    fi
    
    # Test Docker build
    print_status "Building Docker image..."
    docker build -t matchbox-p2p-test .
    
    if [ $? -eq 0 ]; then
        print_status "Docker build successful ✓"
    else
        print_error "Docker build failed"
        exit 1
    fi
    
    # Clean up
    docker rmi matchbox-p2p-test 2>/dev/null || true
}

# Run tests
main() {
    print_status "Starting enhanced P2P demo tests..."
    
    check_dependencies
    test_native
    test_web
    test_docker
    
    print_status "All tests completed successfully! 🎉"
    print_status "You can now:"
    print_status "  • Run native: cargo run --bin matchbox_p2p"
    print_status "  • Run web: trunk serve"
    print_status "  • Run Docker: docker-compose up"
}

# Run main function
main "$@"
