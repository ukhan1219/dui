#!/bin/bash

# Make this script executable: chmod +x scripts/monitor-release.sh

# DUI CLI Release Monitor
# This script monitors GitHub Actions progress and gets SHA256 hashes when complete

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO="ukhan1219/dui"
VERSION=${1:-"latest"}

echo -e "${BLUE}🔍 Monitoring DUI CLI Release${NC}"
echo

# Check if GitHub CLI is available
if ! command -v gh &> /dev/null; then
    echo -e "${RED}❌ GitHub CLI (gh) is required${NC}"
    echo -e "${YELLOW}Install it with: brew install gh${NC}"
    exit 1
fi

# Get latest version if not specified
if [ "$VERSION" = "latest" ]; then
    VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
fi

echo -e "${BLUE}📋 Monitoring release: $VERSION${NC}"
echo

# Function to check workflow status
check_workflow() {
    echo -e "${BLUE}📊 Current workflow runs:${NC}"
    gh run list --repo "$REPO" --limit 3
    
    echo
    echo -e "${BLUE}🔗 Actions URL: https://github.com/$REPO/actions${NC}"
    echo -e "${BLUE}🔗 Releases URL: https://github.com/$REPO/releases${NC}"
    echo
}

# Function to check release status
check_release() {
    local release_url="https://api.github.com/repos/$REPO/releases/tags/$VERSION"
    local release_data=$(curl -s "$release_url")
    
    if echo "$release_data" | grep -q '"message":"Not Found"'; then
        echo -e "${YELLOW}⏳ Release not found yet - workflow still running${NC}"
        return 1
    fi
    
    local assets=$(echo "$release_data" | grep '"browser_download_url"' | sed -E 's/.*"([^"]+)".*/\1/')
    
    if [ -z "$assets" ]; then
        echo -e "${YELLOW}⏳ Release exists but no assets yet - builds still running${NC}"
        return 1
    fi
    
    echo -e "${GREEN}✅ Release complete with assets!${NC}"
    echo -e "${BLUE}📦 Assets found:${NC}"
    echo "$assets" | while read -r asset_url; do
        filename=$(basename "$asset_url")
        echo -e "  📄 $filename"
    done
    return 0
}

# Function to get SHA256 hashes
get_sha256() {
    echo -e "${BLUE}🔢 Getting SHA256 hashes...${NC}"
    ./scripts/get-sha256.sh "$VERSION"
}

# Main monitoring loop
echo -e "${BLUE}🚀 Starting monitoring...${NC}"
echo -e "${YELLOW}Press Ctrl+C to stop monitoring${NC}"
echo

while true; do
    clear
    echo -e "${BLUE}🔍 DUI CLI Release Monitor - $(date)${NC}"
    echo -e "${BLUE}📋 Version: $VERSION${NC}"
    echo
    
    # Check workflow status
    check_workflow
    
    # Check release status
    if check_release; then
        echo
        echo -e "${GREEN}🎉 Release is complete!${NC}"
        echo
        get_sha256
        echo
        echo -e "${BLUE}📋 Next steps:${NC}"
        echo -e "1. Update Formula/dui.rb with the SHA256 hashes"
        echo -e "2. Test the Homebrew formula locally"
        echo -e "3. Submit to Homebrew: brew tap ukhan1219/dui"
        echo
        break
    fi
    
    echo -e "${YELLOW}⏳ Waiting 30 seconds before next check...${NC}"
    echo -e "${YELLOW}Press Ctrl+C to stop monitoring${NC}"
    sleep 30
done 