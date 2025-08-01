#!/bin/bash

# Make this script executable: chmod +x scripts/debug-workflow.sh

# DUI CLI Workflow Debug Script
# This script helps debug failed GitHub Actions workflows

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO="ukhan1219/dui"

echo -e "${BLUE}🔍 Debugging DUI CLI Workflow${NC}"
echo

# Check if GitHub CLI is available
if ! command -v gh &> /dev/null; then
    echo -e "${RED}❌ GitHub CLI (gh) is required${NC}"
    echo -e "${YELLOW}Install it with: brew install gh${NC}"
    exit 1
fi

echo -e "${BLUE}📊 Recent workflow runs:${NC}"
gh run list --repo "$REPO" --limit 5
echo

# Get the latest failed run
LATEST_RUN=$(gh run list --repo "$REPO" --limit 1 --json databaseId,status,conclusion,url | jq -r '.[0].databaseId')

if [ "$LATEST_RUN" = "null" ]; then
    echo -e "${RED}❌ No workflow runs found${NC}"
    exit 1
fi

echo -e "${BLUE}🔍 Latest run ID: $LATEST_RUN${NC}"
echo

# Check run status
RUN_STATUS=$(gh run view "$LATEST_RUN" --repo "$REPO" --json status,conclusion,url)
STATUS=$(echo "$RUN_STATUS" | jq -r '.status')
CONCLUSION=$(echo "$RUN_STATUS" | jq -r '.conclusion')
URL=$(echo "$RUN_STATUS" | jq -r '.url')

echo -e "${BLUE}📋 Run Status:${NC}"
echo -e "  Status: $STATUS"
echo -e "  Conclusion: $CONCLUSION"
echo -e "  URL: $URL"
echo

if [ "$CONCLUSION" = "failure" ]; then
    echo -e "${RED}❌ Workflow failed!${NC}"
    echo
    echo -e "${BLUE}🔍 Common failure causes:${NC}"
    echo -e "1. ${YELLOW}Rust toolchain issues${NC}"
    echo -e "2. ${YELLOW}Missing dependencies${NC}"
    echo -e "3. ${YELLOW}Workflow syntax errors${NC}"
    echo -e "4. ${YELLOW}Permission issues${NC}"
    echo -e "5. ${YELLOW}Build target configuration${NC}"
    echo
    
    echo -e "${BLUE}📋 Getting detailed logs...${NC}"
    echo -e "Run this command to see detailed logs:"
    echo -e "gh run view $LATEST_RUN --repo $REPO --log"
    echo
    
    echo -e "${BLUE}🔧 Quick fixes to try:${NC}"
    echo -e "1. ${GREEN}Check workflow syntax:${NC}"
    echo -e "   cat .github/workflows/release.yml"
    echo -e "2. ${GREEN}Test locally:${NC}"
    echo -e "   cargo build --release"
    echo -e "3. ${GREEN}Check Rust toolchain:${NC}"
    echo -e "   rustc --version"
    echo -e "   cargo --version"
    echo -e "4. ${GREEN}Re-run workflow:${NC}"
    echo -e "   gh run rerun $LATEST_RUN --repo $REPO"
    echo
    
    echo -e "${BLUE}📋 View logs:${NC}"
    echo -e "gh run view $LATEST_RUN --repo $REPO --log"
    echo
    
    # Check local build
    echo -e "${BLUE}🧪 Testing local build...${NC}"
    if cargo build --release; then
        echo -e "${GREEN}✅ Local build successful${NC}"
    else
        echo -e "${RED}❌ Local build failed${NC}"
        echo -e "${YELLOW}This suggests a code issue, not workflow issue${NC}"
    fi
else
    echo -e "${GREEN}✅ Workflow status: $CONCLUSION${NC}"
fi

echo
echo -e "${BLUE}📋 Next steps:${NC}"
echo -e "1. Check the detailed logs above"
echo -e "2. Fix any issues found"
echo -e "3. Re-run the workflow: gh run rerun $LATEST_RUN --repo $REPO"
echo -e "4. Or create a new version: ./scripts/trigger-release.sh" 