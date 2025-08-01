#!/bin/bash

# Make this script executable: chmod +x scripts/install.sh

# DUI CLI Installer
# An intuitive Docker management CLI built in Rust

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO="ukhan1219/dui"
LATEST_VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
BINARY_NAME="dui"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $ARCH in
    x86_64) ARCH="x86_64" ;;
    aarch64) ARCH="aarch64" ;;
    arm64) ARCH="aarch64" ;;
    *) echo -e "${RED}Unsupported architecture: $ARCH${NC}" && exit 1 ;;
esac

case $OS in
    linux) OS="linux" ;;
    darwin) OS="macos" ;;
    *) echo -e "${RED}Unsupported OS: $OS${NC}" && exit 1 ;;
esac

# Download URL
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_VERSION/dui-$OS-$ARCH"

echo -e "${BLUE}🐳 Installing DUI CLI...${NC}"
echo -e "${YELLOW}Version: $LATEST_VERSION${NC}"
echo -e "${YELLOW}Platform: $OS-$ARCH${NC}"
echo

# Check if Docker is available
if ! command -v docker &> /dev/null; then
    echo -e "${RED}❌ Docker is not installed or not in PATH${NC}"
    echo -e "${YELLOW}Please install Docker first: https://docs.docker.com/get-docker/${NC}"
    exit 1
fi

# Check if Docker is running
if ! docker info &> /dev/null; then
    echo -e "${RED}❌ Docker is not running${NC}"
    echo -e "${YELLOW}Please start Docker and try again${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Docker is available and running${NC}"

# Create installation directory
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Download binary
echo -e "${BLUE}📥 Downloading DUI CLI...${NC}"
if curl -L -o "$INSTALL_DIR/$BINARY_NAME" "$DOWNLOAD_URL"; then
    chmod +x "$INSTALL_DIR/$BINARY_NAME"
    echo -e "${GREEN}✅ Download completed${NC}"
else
    echo -e "${RED}❌ Download failed${NC}"
    exit 1
fi

# Add to PATH if not already there
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${BLUE}📝 Adding to PATH...${NC}"
    
    # Detect shell
    if [[ "$SHELL" == *"zsh"* ]]; then
        SHELL_RC="$HOME/.zshrc"
    elif [[ "$SHELL" == *"bash"* ]]; then
        SHELL_RC="$HOME/.bashrc"
    else
        SHELL_RC="$HOME/.profile"
    fi
    
    echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_RC"
    echo -e "${GREEN}✅ Added to $SHELL_RC${NC}"
    echo -e "${YELLOW}Please restart your terminal or run: source $SHELL_RC${NC}"
fi

# Test installation
echo -e "${BLUE}🧪 Testing installation...${NC}"
if "$INSTALL_DIR/$BINARY_NAME" --version &> /dev/null; then
    echo -e "${GREEN}✅ Installation successful!${NC}"
    echo
    echo -e "${BLUE}🎉 DUI CLI is now installed!${NC}"
    echo -e "${YELLOW}Usage:${NC}"
    echo -e "  dui --help"
    echo -e "  dui containers list"
    echo -e "  dui interactive"
    echo
    echo -e "${BLUE}📚 Documentation: https://github.com/$REPO${NC}"
else
    echo -e "${RED}❌ Installation test failed${NC}"
    exit 1
fi 