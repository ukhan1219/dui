#!/bin/bash

# Make this script executable: chmod +x scripts/monitor-release.sh

# DUI CLI Release Monitor
# This script monitors the GitHub Actions workflow for releases

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO="ukhan1219/dui"

echo -e "${BLUE}🔍 DUI CLI Release Monitor - $(date)${NC}"
echo

# Get current version
VERSION=$(grep "version" Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo -e "${BLUE}📋 Version: ${VERSION}${NC}"
echo

# Check if GitHub CLI is available
if ! command -v gh &> /dev/null; then
    echo -e "${RED}❌ GitHub CLI (gh) is required${NC}"
    echo -e "${YELLOW}Install it with: brew install gh${NC}"
    echo -e "${YELLOW}Or check manually: https://github.com/${REPO}/actions${NC}"
    exit 1
fi

# Check authentication
if ! gh auth status &> /dev/null; then
    echo -e "${RED}❌ GitHub CLI not authenticated${NC}"
    echo -e "${YELLOW}Run: gh auth login${NC}"
    exit 1
fi

echo -e "${BLUE}📊 Checking workflow status...${NC}"

# Get the latest workflow run
LATEST_RUN=$(gh run list --repo "$REPO" --limit 1 --json databaseId,status,conclusion,url,headBranch | jq -r '.[0].databaseId')

if [ "$LATEST_RUN" = "null" ]; then
    echo -e "${RED}❌ No workflow runs found${NC}"
    echo -e "${YELLOW}Check manually: https://github.com/${REPO}/actions${NC}"
    exit 1
fi

echo -e "${BLUE}🔍 Latest run ID: $LATEST_RUN${NC}"

# Check run status
RUN_STATUS=$(gh run view "$LATEST_RUN" --repo "$REPO" --json status,conclusion,url,headBranch)
STATUS=$(echo "$RUN_STATUS" | jq -r '.status')
CONCLUSION=$(echo "$RUN_STATUS" | jq -r '.conclusion')
URL=$(echo "$RUN_STATUS" | jq -r '.url')
BRANCH=$(echo "$RUN_STATUS" | jq -r '.headBranch')

echo -e "${BLUE}📋 Run Details:${NC}"
echo -e "  Status: $STATUS"
echo -e "  Conclusion: $CONCLUSION"
echo -e "  Branch: $BRANCH"
echo -e "  URL: $URL"
echo

if [ "$STATUS" = "completed" ]; then
    if [ "$CONCLUSION" = "success" ]; then
        echo -e "${GREEN}✅ Workflow completed successfully!${NC}"
        echo
        echo -e "${BLUE}📦 Getting SHA256 hashes...${NC}"
        ./scripts/get-sha256.sh
    elif [ "$CONCLUSION" = "failure" ]; then
        echo -e "${RED}❌ Workflow failed!${NC}"
        echo
        echo -e "${BLUE}🔍 Getting detailed logs...${NC}"
        echo -e "Run this command to see detailed logs:"
        echo -e "gh run view $LATEST_RUN --repo $REPO --log"
    else
        echo -e "${YELLOW}⚠️  Workflow completed with status: $CONCLUSION${NC}"
    fi
else
    echo -e "${YELLOW}⏳ Workflow is still running...${NC}"
    echo -e "${BLUE}📊 Current status: $STATUS${NC}"
    echo
    echo -e "${BLUE}🔄 Monitoring progress...${NC}"
    echo -e "Press Ctrl+C to stop monitoring"
    echo
    
    # Monitor in a loop
    while true; do
        sleep 30
        CURRENT_STATUS=$(gh run view "$LATEST_RUN" --repo "$REPO" --json status,conclusion | jq -r '.status')
        CURRENT_CONCLUSION=$(gh run view "$LATEST_RUN" --repo "$REPO" --json status,conclusion | jq -r '.conclusion')
        
        echo -e "$(date): Status: $CURRENT_STATUS, Conclusion: $CURRENT_CONCLUSION"
        
        if [ "$CURRENT_STATUS" = "completed" ]; then
            if [ "$CURRENT_CONCLUSION" = "success" ]; then
                echo -e "${GREEN}✅ Build completed successfully!${NC}"
                echo -e "${BLUE}📦 Getting SHA256 hashes...${NC}"
                ./scripts/get-sha256.sh
            else
                echo -e "${RED}❌ Build failed with conclusion: $CURRENT_CONCLUSION${NC}"
                echo -e "${BLUE}🔍 Check logs: gh run view $LATEST_RUN --repo $REPO --log${NC}"
            fi
            break
        fi
    done
fi

echo
echo -e "${BLUE}📋 Next steps:${NC}"
echo -e "1. Check releases: https://github.com/${REPO}/releases"
echo -e "2. Update Homebrew formula with new SHA256 hashes"
echo -e "3. Test installation: brew install --build-from-source Formula/dui.rb" 