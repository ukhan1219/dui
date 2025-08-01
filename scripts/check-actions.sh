#!/bin/bash

# Make this script executable: chmod +x scripts/check-actions.sh

# DUI CLI GitHub Actions Status Checker
# This script helps check the status of GitHub Actions

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO="ukhan1219/dui"

echo -e "${BLUE}🔍 Checking GitHub Actions Status for DUI CLI${NC}"
echo

# Check if GitHub CLI is available
if command -v gh &> /dev/null; then
    echo -e "${GREEN}✅ GitHub CLI found${NC}"
    echo -e "${BLUE}📋 Recent workflow runs:${NC}"
    gh run list --repo "$REPO" --limit 5
    echo
else
    echo -e "${YELLOW}⚠️  GitHub CLI not found${NC}"
    echo -e "${YELLOW}Install it with: brew install gh${NC}"
    echo
fi

# Check workflow file
echo -e "${BLUE}📄 Checking workflow file...${NC}"
if [ -f ".github/workflows/release.yml" ]; then
    echo -e "${GREEN}✅ Workflow file exists${NC}"
    
    # Check for common issues
    if grep -q "matrix:" .github/workflows/release.yml; then
        echo -e "${GREEN}✅ Matrix configuration found${NC}"
    else
        echo -e "${RED}❌ Matrix configuration missing${NC}"
    fi
    
    if grep -q "on:" .github/workflows/release.yml; then
        echo -e "${GREEN}✅ Trigger configuration found${NC}"
    else
        echo -e "${RED}❌ Trigger configuration missing${NC}"
    fi
else
    echo -e "${RED}❌ Workflow file missing${NC}"
fi
echo

# Check recent tags
echo -e "${BLUE}🏷️  Recent tags:${NC}"
git tag --sort=-version:refname | head -5
echo

# Check if tags exist on GitHub
echo -e "${BLUE}🌐 Checking tags on GitHub...${NC}"
LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "none")
if [ "$LATEST_TAG" != "none" ]; then
    echo -e "${GREEN}✅ Latest local tag: $LATEST_TAG${NC}"
    
    # Check if tag exists on GitHub
    if git ls-remote --tags origin | grep -q "$LATEST_TAG"; then
        echo -e "${GREEN}✅ Tag exists on GitHub${NC}"
    else
        echo -e "${RED}❌ Tag not found on GitHub${NC}"
        echo -e "${YELLOW}Try: git push origin $LATEST_TAG${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  No tags found locally${NC}"
fi
echo

# Manual trigger instructions
echo -e "${BLUE}🔧 Manual Actions Trigger:${NC}"
echo -e "If Actions aren't running automatically:"
echo -e "1. Go to: https://github.com/$REPO/actions"
echo -e "2. Click 'Release' workflow"
echo -e "3. Click 'Run workflow'"
echo -e "4. Select branch: main"
echo -e "5. Click 'Run workflow'"
echo

# Common troubleshooting
echo -e "${BLUE}🔍 Common Issues:${NC}"
echo -e "1. ${YELLOW}Actions not showing:${NC} Check repository permissions"
echo -e "2. ${YELLOW}Workflow not triggered:${NC} Verify tag format (v*.*.*)"
echo -e "3. ${YELLOW}Build failures:${NC} Check Rust toolchain and dependencies"
echo -e "4. ${YELLOW}Permission errors:${NC} Ensure GITHUB_TOKEN has write permissions"
echo

# Quick fixes
echo -e "${BLUE}⚡ Quick Fixes:${NC}"
echo -e "1. ${GREEN}Re-push tag:${NC} git push origin v3.2.0"
echo -e "2. ${GREEN}Force push:${NC} git push --force origin v3.2.0"
echo -e "3. ${GREEN}Delete and recreate tag:${NC}"
echo -e "   git tag -d v3.2.0"
echo -e "   git push origin :refs/tags/v3.2.0"
echo -e "   git tag v3.2.0"
echo -e "   git push origin v3.2.0"
echo

echo -e "${YELLOW}📋 Next steps:${NC}"
echo -e "1. Check Actions tab: https://github.com/$REPO/actions"
echo -e "2. If Actions aren't running, try manual trigger"
echo -e "3. Once release is created, run: ./scripts/get-sha256.sh" 