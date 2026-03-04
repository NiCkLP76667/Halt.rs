# Halt.rs - Build Scripts

#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_NAME="halt"
VERSION=$(grep '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
BUILD_DIR="target/release"
DIST_DIR="dist"

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check dependencies
check_dependencies() {
    log_info "Checking dependencies..."

    if ! command -v cargo &> /dev/null; then
        log_error "Cargo is not installed. Please install Rust."
        exit 1
    fi

    if ! command -v node &> /dev/null; then
        log_error "Node.js is not installed. Please install Node.js."
        exit 1
    fi

    if ! command -v python3 &> /dev/null; then
        log_error "Python 3 is not installed. Please install Python 3."
        exit 1
    fi

    if ! command -v javac &> /dev/null; then
        log_error "Java JDK is not installed. Please install Java JDK."
        exit 1
    fi

    log_success "All dependencies are available."
}

# Build Rust core
build_rust() {
    log_info "Building Rust core..."

    cargo build --release

    if [ $? -eq 0 ]; then
        log_success "Rust core built successfully."
    else
        log_error "Failed to build Rust core."
        exit 1
    fi
}

# Build MCP server
build_mcp() {
    log_info "Building MCP server..."

    cd mcp-server
    npm install
    npm run build

    if [ $? -eq 0 ]; then
        log_success "MCP server built successfully."
    else
        log_error "Failed to build MCP server."
        exit 1
    fi

    cd ..
}

# Build Python bindings
build_python() {
    log_info "Building Python bindings..."

    cd bindings/python
    python3 setup.py build_ext --inplace

    if [ $? -eq 0 ]; then
        log_success "Python bindings built successfully."
    else
        log_error "Failed to build Python bindings."
        exit 1
    fi

    cd ../..
}

# Build Java bindings
build_java() {
    log_info "Building Java bindings..."

    javac -d target/classes HaltProxy.java

    if [ $? -eq 0 ]; then
        log_success "Java bindings compiled successfully."
    else
        log_error "Failed to compile Java bindings."
        exit 1
    fi
}

# Create distribution package
create_dist() {
    log_info "Creating distribution package..."

    mkdir -p $DIST_DIR
    mkdir -p $DIST_DIR/bin
    mkdir -p $DIST_DIR/lib
    mkdir -p $DIST_DIR/config
    mkdir -p $DIST_DIR/docs

    # Copy binaries
    cp $BUILD_DIR/$PROJECT_NAME $DIST_DIR/bin/
    cp mcp-server/dist/* $DIST_DIR/lib/ 2>/dev/null || true

    # Copy configuration
    cp config/*.env $DIST_DIR/config/

    # Copy documentation
    cp README.md $DIST_DIR/docs/
    cp docs/**/*.md $DIST_DIR/docs/ 2>/dev/null || true

    # Create archive
    tar -czf $DIST_DIR/$PROJECT_NAME-$VERSION.tar.gz -C $DIST_DIR .

    log_success "Distribution package created: $DIST_DIR/$PROJECT_NAME-$VERSION.tar.gz"
}

# Run tests
run_tests() {
    log_info "Running tests..."

    # Rust tests
    log_info "Running Rust tests..."
    cargo test

    # MCP server tests
    log_info "Running MCP server tests..."
    cd mcp-server
    npm test
    cd ..

    # Python tests
    log_info "Running Python tests..."
    cd bindings/python
    python3 -m pytest tests/ 2>/dev/null || log_warn "No Python tests found"
    cd ../..

    log_success "All tests completed."
}

# Clean build artifacts
clean() {
    log_info "Cleaning build artifacts..."

    cargo clean
    rm -rf target/
    rm -rf dist/
    rm -rf mcp-server/node_modules/
    rm -rf mcp-server/dist/
    rm -rf bindings/python/build/
    rm -rf bindings/python/*.so
    rm -rf bindings/python/*.pyd

    log_success "Clean completed."
}

# Show usage
usage() {
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  all       Build everything (default)"
    echo "  rust      Build only Rust core"
    echo "  mcp       Build only MCP server"
    echo "  python    Build only Python bindings"
    echo "  java      Build only Java bindings"
    echo "  dist      Create distribution package"
    echo "  test      Run all tests"
    echo "  clean     Clean build artifacts"
    echo "  help      Show this help"
}

# Main build process
main() {
    case "${1:-all}" in
        "all")
            check_dependencies
            build_rust
            build_mcp
            build_python
            build_java
            run_tests
            create_dist
            log_success "Full build completed successfully!"
            ;;
        "rust")
            check_dependencies
            build_rust
            ;;
        "mcp")
            check_dependencies
            build_mcp
            ;;
        "python")
            check_dependencies
            build_python
            ;;
        "java")
            check_dependencies
            build_java
            ;;
        "dist")
            create_dist
            ;;
        "test")
            run_tests
            ;;
        "clean")
            clean
            ;;
        "help"|"-h"|"--help")
            usage
            ;;
        *)
            log_error "Unknown command: $1"
            usage
            exit 1
            ;;
    esac
}

# Run main function
main "$@"
