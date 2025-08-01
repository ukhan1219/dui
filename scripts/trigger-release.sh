#!/bin/bash

# Make this script executable: chmod +x scripts/trigger-release.sh

# DUI CLI Release Trigger Script
# This script helps trigger GitHub Actions manually

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO="ukhan1219/dui"
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | cut -d'"' -f2)

echo -e "${BLUE}🚀 DUI CLI Release Trigger${NC}"
echo

# Check if GitHub CLI is available
if ! command -v gh &> /dev/null; then
    echo -e "${RED}❌ GitHub CLI (gh) is required for this script${NC}"
    echo -e "${YELLOW}Install it with: brew install gh${NC}"
    echo -e "${YELLOW}Then authenticate with: gh auth login${NC}"
    exit 1
fi

# Check authentication
if ! gh auth status &> /dev/null; then
    echo -e "${RED}❌ Not authenticated with GitHub CLI${NC}"
    echo -e "${YELLOW}Run: gh auth login${NC}"
    exit 1
fi

echo -e "${GREEN}✅ GitHub CLI authenticated${NC}"
echo

# Show current version
echo -e "${BLUE}📋 Current version in Cargo.toml: $CURRENT_VERSION${NC}"

# Check if tag exists
if git tag -l | grep -q "v$CURRENT_VERSION"; then
    echo -e "${YELLOW}⚠️  Tag v$CURRENT_VERSION already exists${NC}"
    echo -e "${BLUE}Options:${NC}"
    echo -e "1. Trigger workflow manually for existing tag"
    echo -e "2. Create a new patch version (v$CURRENT_VERSION.1)"
    echo -e "3. Bump version and create new tag"
    echo
    read -p "Choose option (1/2/3): " -n 1 -r
    echo
    echo
    
    case $REPLY in
        1)
            echo -e "${BLUE}🔧 Triggering workflow manually for v$CURRENT_VERSION...${NC}"
            gh workflow run release.yml --ref "v$CURRENT_VERSION"
            echo -e "${GREEN}✅ Workflow triggered!${NC}"
            ;;
        2)
            NEW_VERSION="${CURRENT_VERSION}.1"
            echo -e "${BLUE}📝 Creating new patch version: v$NEW_VERSION${NC}"
            
            # Update Cargo.toml
            sed -i.bak "s/version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml
            rm Cargo.toml.bak
            
            # Commit and tag
            git add Cargo.toml
            git commit -m "chore: bump version to $NEW_VERSION"
            git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION"
            git push origin main
            git push origin "v$NEW_VERSION"
            
            echo -e "${GREEN}✅ New version v$NEW_VERSION created and pushed!${NC}"
            ;;
        3)
            echo -e "${BLUE}📝 Bumping version...${NC}"
            echo -e "${YELLOW}Current version: $CURRENT_VERSION${NC}"
            read -p "Enter new version (e.g., 3.3.0): " NEW_VERSION
            
            if [[ ! $NEW_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
                echo -e "${RED}❌ Invalid version format. Use format: x.y.z${NC}"
                exit 1
            fi
            
            # Update Cargo.toml
            sed -i.bak "s/version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml
            rm Cargo.toml.bak
            
            # Commit and tag
            git add Cargo.toml
            git commit -m "chore: bump version to $NEW_VERSION"
            git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION"
            git push origin main
            git push origin "v$NEW_VERSION"
            
            echo -e "${GREEN}✅ New version v$NEW_VERSION created and pushed!${NC}"
            ;;
        *)
            echo -e "${RED}❌ Invalid option${NC}"
            exit 1
            ;;
    esac
else
    echo -e "${BLUE}📝 Creating tag v$CURRENT_VERSION...${NC}"
    git tag -a "v$CURRENT_VERSION" -m "Release v$CURRENT_VERSION"
    git push origin "v$CURRENT_VERSION"
    echo -e "${GREEN}✅ Tag v$CURRENT_VERSION created and pushed!${NC}"
fi

echo
echo -e "${BLUE}📋 Next steps:${NC}"
echo -e "1. Check Actions: https://github.com/$REPO/actions"
echo -e "2. Wait for workflow to complete (usually 5-10 minutes)"
echo -e "3. Check release: https://github.com/$REPO/releases"
echo -e "4. Once complete, run: ./scripts/get-sha256.sh"
echo
echo -e "${YELLOW}⏱️  Expected timeline:${NC}"
echo -e "- Workflow starts: Immediately"
echo -e "- Builds complete: 5-10 minutes"
echo -e "- Release created: 1-2 minutes after builds"
echo
echo -e "${BLUE}🔍 Monitor progress:${NC}"
echo -e "gh run list --repo $REPO --limit 3" 