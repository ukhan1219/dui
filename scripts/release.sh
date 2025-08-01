#!/bin/bash

# DUI Release Script
# This script automates the release process for the DUI CLI tool

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

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    if ! command_exists git; then
        print_error "git is not installed"
        exit 1
    fi
    
    if ! command_exists cargo; then
        print_error "cargo is not installed"
        exit 1
    fi
    
    if ! command_exists cargo-bump; then
        print_warning "cargo-bump is not installed. Installing..."
        cargo install cargo-bump
    fi
    
    print_success "Prerequisites check passed"
}

# Check git status
check_git_status() {
    print_status "Checking git status..."
    
    if [ -n "$(git status --porcelain)" ]; then
        print_error "Working directory is not clean. Please commit or stash your changes."
        git status --short
        exit 1
    fi
    
    current_branch=$(git branch --show-current)
    if [ "$current_branch" != "main" ]; then
        print_warning "You are not on the main branch. Current branch: $current_branch"
        read -p "Do you want to continue? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
    
    print_success "Git status check passed"
}

# Get current version
get_current_version() {
    grep '^version = ' Cargo.toml | cut -d '"' -f2
}

# Bump version
bump_version() {
    local bump_type=$1
    
    print_status "Bumping version ($bump_type)..."
    
    case $bump_type in
        patch|minor|major)
            cargo bump $bump_type
            ;;
        *)
            print_error "Invalid bump type: $bump_type. Use patch, minor, or major"
            exit 1
            ;;
    esac
    
    new_version=$(get_current_version)
    print_success "Version bumped to $new_version"
}

# Create and push tag
create_tag() {
    local version=$1
    
    print_status "Creating git tag v$version..."
    
    # Check if tag already exists
    if git tag -l | grep -q "v$version"; then
        print_error "Tag v$version already exists"
        exit 1
    fi
    
    # Create tag
    git tag -a "v$version" -m "Release v$version"
    
    print_status "Pushing tag to remote..."
    git push origin "v$version"
    
    print_success "Tag v$version created and pushed"
}

# Main release function
release() {
    local bump_type=${1:-patch}
    
    print_status "Starting release process..."
    
    # Run checks
    check_prerequisites
    check_git_status
    
    # Get current version
    current_version=$(get_current_version)
    print_status "Current version: $current_version"
    
    # Bump version
    bump_version $bump_type
    
    # Get new version
    new_version=$(get_current_version)
    
    # Commit version bump
    print_status "Committing version bump..."
    git add Cargo.toml Cargo.lock
    git commit -m "chore: bump version to $new_version"
    
    # Push changes
    print_status "Pushing changes to remote..."
    git push origin main
    
    # Create and push tag
    create_tag $new_version
    
    print_success "Release process completed!"
    print_status "GitHub Actions will now build and publish the release"
    print_status "Monitor the workflow at: https://github.com/ukhan1219/dui/actions"
    
    # Show next steps
    echo
    print_status "Next steps:"
    echo "1. Wait for GitHub Actions to complete (5-10 minutes)"
    echo "2. Check the release at: https://github.com/ukhan1219/dui/releases"
    echo "3. Update Homebrew formula with SHA256 hashes from the workflow"
    echo "4. Submit Homebrew formula to main repository"
}

# Help function
show_help() {
    echo "DUI Release Script"
    echo
    echo "Usage: $0 [bump_type]"
    echo
    echo "bump_type:"
    echo "  patch    - Bump patch version (default)"
    echo "  minor    - Bump minor version"
    echo "  major    - Bump major version"
    echo
    echo "Examples:"
    echo "  $0           # Bump patch version"
    echo "  $0 minor     # Bump minor version"
    echo "  $0 major     # Bump major version"
}

# Main script
if [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
    show_help
    exit 0
fi

release "$@" 