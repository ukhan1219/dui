#!/bin/bash

# Make this script executable: chmod +x scripts/get-sha256.sh

# DUI CLI SHA256 Hash Generator
# This script helps get SHA256 hashes from GitHub releases

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

echo -e "${BLUE}🔍 Getting SHA256 hashes for DUI CLI${NC}"
echo

# Get latest version if not specified
if [ "$VERSION" = "latest" ]; then
    VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    echo -e "${YELLOW}Using latest version: $VERSION${NC}"
fi

echo -e "${BLUE}📋 Checking GitHub Actions status...${NC}"
echo -e "Visit: https://github.com/$REPO/actions"
echo

# Check if release exists
RELEASE_URL="https://api.github.com/repos/$REPO/releases/tags/$VERSION"
RELEASE_DATA=$(curl -s "$RELEASE_URL")

if echo "$RELEASE_DATA" | grep -q '"message":"Not Found"'; then
    echo -e "${RED}❌ Release $VERSION not found${NC}"
    echo -e "${YELLOW}This might mean:${NC}"
    echo -e "1. GitHub Actions haven't run yet"
    echo -e "2. The release hasn't been created"
    echo -e "3. Check the Actions tab: https://github.com/$REPO/actions"
    exit 1
fi

echo -e "${GREEN}✅ Release $VERSION found${NC}"
echo

# Get assets from release
ASSETS=$(echo "$RELEASE_DATA" | grep '"browser_download_url"' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$ASSETS" ]; then
    echo -e "${YELLOW}⚠️  No assets found in release${NC}"
    echo -e "${YELLOW}This might mean GitHub Actions are still running${NC}"
    echo -e "Check: https://github.com/$REPO/actions"
    exit 1
fi

echo -e "${BLUE}📦 Found assets:${NC}"
echo "$ASSETS" | while read -r asset_url; do
    filename=$(basename "$asset_url")
    echo -e "  📄 $filename"
done
echo

# Download and calculate SHA256
echo -e "${BLUE}🔢 Calculating SHA256 hashes...${NC}"
echo

TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

echo "$ASSETS" | while read -r asset_url; do
    filename=$(basename "$asset_url")
    echo -e "${BLUE}📥 Downloading $filename...${NC}"
    
    if curl -L -o "$TEMP_DIR/$filename" "$asset_url"; then
        sha256=$(sha256sum "$TEMP_DIR/$filename" | cut -d' ' -f1)
        echo -e "${GREEN}✅ $filename: $sha256${NC}"
    else
        echo -e "${RED}❌ Failed to download $filename${NC}"
    fi
done

echo
echo -e "${BLUE}📝 Homebrew Formula Update:${NC}"
echo -e "Update the following in Formula/dui.rb:"
echo

# Generate Homebrew formula updates
echo "$ASSETS" | while read -r asset_url; do
    filename=$(basename "$asset_url")
    if [ -f "$TEMP_DIR/$filename" ]; then
        sha256=$(sha256sum "$TEMP_DIR/$filename" | cut -d' ' -f1)
        
        case "$filename" in
            *linux-aarch64*)
                echo "    url \"$asset_url\""
                echo "    sha256 \"$sha256\""
                ;;
            *linux-x86_64*)
                echo "    url \"$asset_url\""
                echo "    sha256 \"$sha256\""
                ;;
            *macos-aarch64*)
                echo "    url \"$asset_url\""
                echo "    sha256 \"$sha256\""
                ;;
            *macos-x86_64*)
                echo "    url \"$asset_url\""
                echo "    sha256 \"$sha256\""
                ;;
            *windows*)
                echo "    url \"$asset_url\""
                echo "    sha256 \"$sha256\""
                ;;
        esac
    fi
done

echo
echo -e "${YELLOW}📋 Next steps:${NC}"
echo -e "1. Update Formula/dui.rb with the SHA256 hashes above"
echo -e "2. Test the Homebrew formula locally"
echo -e "3. Submit to Homebrew: brew tap ukhan1219/dui" 