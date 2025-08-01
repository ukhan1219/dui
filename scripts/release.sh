#!/bin/bash

# Make this script executable: chmod +x scripts/release.sh

# DUI CLI Release Script
# This script creates a git tag and prepares for GitHub release

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get version from Cargo.toml
VERSION=$(grep '^version = ' Cargo.toml | cut -d'"' -f2)

echo -e "${BLUE}🚀 Creating release for DUI CLI v$VERSION${NC}"
echo

# Check if we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    echo -e "${YELLOW}⚠️  Warning: You're not on the main branch (currently on $CURRENT_BRANCH)${NC}"
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check if working directory is clean
if [ -n "$(git status --porcelain)" ]; then
    echo -e "${RED}❌ Working directory is not clean. Please commit or stash your changes.${NC}"
    git status --short
    exit 1
fi

# Check if tag already exists
if git tag -l | grep -q "v$VERSION"; then
    echo -e "${RED}❌ Tag v$VERSION already exists${NC}"
    exit 1
fi

# Pull latest changes
echo -e "${BLUE}📥 Pulling latest changes...${NC}"
git pull origin main

# Create and push tag
echo -e "${BLUE}🏷️  Creating tag v$VERSION...${NC}"
git tag -a "v$VERSION" -m "Release v$VERSION - Comprehensive Docker Management CLI"

echo -e "${BLUE}📤 Pushing tag to GitHub...${NC}"
git push origin "v$VERSION"

echo -e "${GREEN}✅ Tag v$VERSION created and pushed successfully!${NC}"
echo
echo -e "${BLUE}📋 Next steps:${NC}"
echo -e "1. GitHub Actions will automatically build and create a release"
echo -e "2. Check the release at: https://github.com/ukhan1219/dui/releases"
echo -e "3. Update the Homebrew formula with the new SHA256 hashes"
echo
echo -e "${YELLOW}📝 Release notes template:${NC}"
echo "## Release v$VERSION - Comprehensive Docker Management CLI"
echo
echo "### 🎉 Major Features Added"
echo
echo "#### Advanced Container Management"
echo "- Container Creation: Create containers with port mappings, volume mounts, and environment variables"
echo "- Container Inspection: Detailed container information and configuration"
echo "- Size Analysis: Container disk usage and resource consumption"
echo "- Exec Support: Execute commands inside running containers"
echo "- Comprehensive Lifecycle: Full container lifecycle management"
echo
echo "#### Visual Analytics & Charts"
echo "- CPU Usage Charts: Real-time CPU consumption with color-coded bars"
echo "- Memory Usage Charts: Memory consumption with percentage and absolute values"
echo "- Network Traffic Charts: Network I/O visualization"
echo "- Storage Usage Charts: Disk usage analytics"
echo "- Status Charts: Container state distribution"
echo "- Image Size Charts: Docker image size analysis"
echo "- System Pie Charts: Overall system resource distribution"
echo "- Real-time Dashboard: Live system overview"
echo
echo "#### Interactive Mode Features"
echo "- Tab Completion: Intelligent command and container/image name completion"
echo "- Interactive Menus: Numbered lists for easy container/image selection"
echo "- Command History: Persistent command history"
echo "- Syntax Highlighting: Color-coded command input"
echo "- Smart Suggestions: Context-aware command suggestions"
echo
echo "#### Network & Volume Management"
echo "- Network Listing: View all Docker networks with details"
echo "- Volume Management: List and inspect Docker volumes"
echo "- Resource Discovery: Easy discovery of Docker resources"
echo
echo "### 🔧 Technical Improvements"
echo "- Enhanced error handling and validation"
echo "- Improved user interface with better formatting"
echo "- Added comprehensive documentation"
echo "- Better performance and reliability"
echo
echo "### 📚 Documentation"
echo "- Updated README with all new features and examples"
echo "- Added detailed usage instructions"
echo "- Included interactive mode documentation"
echo
echo "### 🚀 Installation"
echo '```bash'
echo 'cargo install dui-cli'
echo '```'
echo
echo "### 🐳 Usage Examples"
echo '```bash'
echo '# Create and manage containers'
echo 'dui containers create web-server nginx:latest -p 8080:80'
echo 'dui containers start web-server'
echo
echo '# Visual analytics'
echo 'dui charts cpu'
echo 'dui charts memory'
echo
echo '# Interactive mode'
echo 'dui interactive'
echo '```'
echo
echo "---"
echo
echo "**Happy Dockering! 🐳**" 