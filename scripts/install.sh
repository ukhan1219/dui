#!/bin/bash

# DUI CLI Installation Script
# This script installs the DUI CLI tool on Unix/Linux/macOS systems

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to detect OS and architecture
detect_system() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"
    
    case "$OS" in
        Darwin)
            OS_NAME="macos"
            ;;
        Linux)
            OS_NAME="linux"
            ;;
        *)
            print_error "Unsupported operating system: $OS"
            exit 1
            ;;
    esac
    
    case "$ARCH" in
        x86_64)
            ARCH_NAME="x86_64"
            ;;
        arm64|aarch64)
            ARCH_NAME="aarch64"
            ;;
        *)
            print_error "Unsupported architecture: $ARCH"
            exit 1
            ;;
    esac
    
    print_status "Detected system: $OS_NAME-$ARCH_NAME"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    if ! command_exists curl; then
        print_error "curl is not installed. Please install curl first."
        exit 1
    fi
    
    if ! command_exists tar; then
        print_error "tar is not installed. Please install tar first."
        exit 1
    fi
    
    print_success "Prerequisites check passed"
}

# Function to get latest version
get_latest_version() {
    print_status "Getting latest version..."
    
    # Try to get latest version from GitHub API
    LATEST_VERSION=$(curl -s https://api.github.com/repos/ukhan1219/dui/releases/latest | grep '"tag_name"' | cut -d'"' -f4 | sed 's/v//')
    
    if [ -z "$LATEST_VERSION" ]; then
        print_warning "Could not get latest version from GitHub API, using fallback"
        LATEST_VERSION="3.6.0"
    fi
    
    print_status "Latest version: $LATEST_VERSION"
    echo "$LATEST_VERSION"
}

# Function to download and install
download_and_install() {
    local version=$1
    local asset_name="dui-$OS_NAME-$ARCH_NAME"
    local download_url="https://github.com/ukhan1219/dui/releases/download/v$version/$asset_name.tar.gz"
    local temp_dir=$(mktemp -d)
    
    print_status "Downloading DUI CLI v$version..."
    
    # Download the release
    if ! curl -L -o "$temp_dir/$asset_name.tar.gz" "$download_url"; then
        print_error "Failed to download DUI CLI"
        exit 1
    fi
    
    print_status "Extracting binary..."
    
    # Extract the binary
    cd "$temp_dir"
    tar -xzf "$asset_name.tar.gz"
    
    # Determine install location
    local install_dir="/usr/local/bin"
    if [ ! -w "$install_dir" ]; then
        install_dir="$HOME/.local/bin"
        mkdir -p "$install_dir"
    fi
    
    print_status "Installing to $install_dir..."
    
    # Install the binary
    if sudo cp dui "$install_dir/"; then
        sudo chmod +x "$install_dir/dui"
        print_success "DUI CLI installed successfully!"
    else
        # Try without sudo
        if cp dui "$install_dir/"; then
            chmod +x "$install_dir/dui"
            print_success "DUI CLI installed successfully!"
        else
            print_error "Failed to install DUI CLI"
            exit 1
        fi
    fi
    
    # Clean up
    cd - > /dev/null
    rm -rf "$temp_dir"
    
    # Add to PATH if needed
    if [[ ":$PATH:" != *":$install_dir:"* ]]; then
        print_warning "Please add $install_dir to your PATH"
        echo "Add this line to your shell profile (.bashrc, .zshrc, etc.):"
        echo "export PATH=\"$install_dir:\$PATH\""
    fi
}

# Function to verify installation
verify_installation() {
    print_status "Verifying installation..."
    
    if command_exists dui; then
        print_success "DUI CLI is installed and available in PATH"
        dui --version
    else
        print_warning "DUI CLI installed but not found in PATH"
        print_warning "Please restart your terminal or add the install directory to PATH"
    fi
}

# Function to show usage
show_usage() {
    echo "DUI CLI Installation Script"
    echo
    echo "Usage: $0 [options]"
    echo
    echo "Options:"
    echo "  -h, --help     Show this help message"
    echo "  -v, --version  Install specific version (default: latest)"
    echo
    echo "Examples:"
    echo "  $0              # Install latest version"
    echo "  $0 -v 3.6.0    # Install specific version"
}

# Main installation function
install_dui() {
    local version=${1:-$(get_latest_version)}
    
    print_status "Starting DUI CLI installation..."
    
    # Run checks
    check_prerequisites
    detect_system
    
    # Download and install
    download_and_install "$version"
    
    # Verify installation
    verify_installation
    
    print_success "Installation completed!"
    echo
    print_status "Next steps:"
    echo "1. Restart your terminal or run: source ~/.bashrc (or ~/.zshrc)"
    echo "2. Run 'dui --help' to see available commands"
    echo "3. Make sure Docker is running to use DUI CLI"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_usage
            exit 0
            ;;
        -v|--version)
            VERSION="$2"
            shift 2
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Run installation
install_dui "$VERSION" 