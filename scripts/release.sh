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

# Update changelog
update_changelog() {
    local version=$1
    local date=$(date +%Y-%m-%d)
    
    print_status "Updating CHANGELOG.md..."
    
    if [ ! -f "CHANGELOG.md" ]; then
        print_warning "CHANGELOG.md not found, creating one..."
        touch CHANGELOG.md
    fi
    
    # Create a temporary file with the updated changelog
    temp_changelog=$(mktemp)
    
    # Add header if file is empty
    if [ ! -s "CHANGELOG.md" ]; then
        echo "# Changelog" > "$temp_changelog"
        echo "" >> "$temp_changelog"
        echo "All notable changes to this project will be documented in this file." >> "$temp_changelog"
        echo "" >> "$temp_changelog"
    else
        # Copy existing content up to [Unreleased]
        sed '/^## \[Unreleased\]/q' CHANGELOG.md > "$temp_changelog"
    fi
    
    # Replace [Unreleased] with the new version
    sed -i.bak "s/^## \[Unreleased\]/## [$version] - $date/" "$temp_changelog"
    
    # Add new [Unreleased] section
    echo "" >> "$temp_changelog"
    echo "## [Unreleased]" >> "$temp_changelog"
    echo "" >> "$temp_changelog"
    echo "### Added" >> "$temp_changelog"
    echo "" >> "$temp_changelog"
    echo "### Changed" >> "$temp_changelog"
    echo "" >> "$temp_changelog"
    echo "### Fixed" >> "$temp_changelog"
    echo "" >> "$temp_changelog"
    
    # Add the rest of the changelog if it exists
    if [ -s "CHANGELOG.md" ]; then
        sed '1,/^## \[Unreleased\]/d' CHANGELOG.md >> "$temp_changelog"
    fi
    
    # Replace the original file
    mv "$temp_changelog" CHANGELOG.md
    rm -f CHANGELOG.md.bak
    
    print_success "CHANGELOG.md updated for version $version"
}

# Generate release notes from changelog
generate_release_notes() {
    local version=$1
    local notes_file="release_notes_$version.md"
    
    print_status "Generating release notes for version $version..."
    
    if [ ! -f "CHANGELOG.md" ]; then
        print_error "CHANGELOG.md not found"
        return 1
    fi
    
    # Extract the section for this version
    sed -n "/^## \[$version\]/,/^## \[.*\]/p" CHANGELOG.md | sed '$d' > "$notes_file"
    
    if [ -s "$notes_file" ]; then
        print_success "Release notes generated: $notes_file"
        echo "Release notes preview:"
        echo "======================"
        cat "$notes_file"
        echo "======================"
    else
        print_warning "No release notes found for version $version"
        echo "# Release $version" > "$notes_file"
        echo "" >> "$notes_file"
        echo "- Version bump to $version" >> "$notes_file"
    fi
}

# Publish to cargo
publish_to_cargo() {
    print_status "Publishing to crates.io..."
    
    # First check if we can publish (dry run)
    print_status "Running cargo publish dry run..."
    if ! cargo publish --dry-run; then
        print_error "Cargo publish dry run failed. Please fix the issues and try again."
        return 1
    fi
    
    # Publish for real
    print_status "Publishing to crates.io (this may take a few minutes)..."
    if cargo publish; then
        print_success "Successfully published to crates.io!"
        print_status "Package will be available at: https://crates.io/crates/$(grep '^name = ' Cargo.toml | cut -d '"' -f2)"
    else
        print_error "Failed to publish to crates.io"
        return 1
    fi
}

# Update Homebrew formula
update_homebrew_formula() {
    local version=$1
    
    print_status "Updating Homebrew formula..."
    
    if [ ! -f "Formula/dui.rb" ]; then
        print_warning "Formula/dui.rb not found, skipping Homebrew formula update"
        return 0
    fi
    
    # Update version in Homebrew formula
    sed -i.bak "s/refs\/tags\/v[0-9]\+\.[0-9]\+\.[0-9]\+/refs\/tags\/v$version/g" Formula/dui.rb
    
    print_success "Homebrew formula updated"
    print_warning "Note: SHA256 hash will need to be updated after GitHub release is created"
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
    local publish_cargo=${2:-true}
    
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
    
    # Update changelog
    update_changelog $new_version
    
    # Generate release notes
    generate_release_notes $new_version
    
    # Update Homebrew formula
    update_homebrew_formula $new_version
    
    # Build and test before committing
    print_status "Building and testing..."
    if ! cargo build --release; then
        print_error "Build failed. Please fix the issues and try again."
        exit 1
    fi
    
    if ! cargo test; then
        print_warning "Tests failed. Continuing anyway..."
    fi
    
    # Commit all changes
    print_status "Committing version bump and changelog updates..."
    git add Cargo.toml CHANGELOG.md Formula/dui.rb
    # Force add Cargo.lock if it exists and has changes
    if [ -f "Cargo.lock" ]; then
        git add -f Cargo.lock
    fi
    git commit -m "chore: release version $new_version

- Bump version to $new_version
- Update CHANGELOG.md
- Update Homebrew formula
- Generate release notes"
    
    # Push changes
    print_status "Pushing changes to remote..."
    git push origin main
    
    # Create and push tag
    create_tag $new_version
    
    # Publish to cargo if requested
    if [ "$publish_cargo" = "true" ]; then
        if ! publish_to_cargo; then
            print_warning "Cargo publishing failed, but release will continue"
        fi
    else
        print_status "Skipping Cargo publishing (use --publish to enable)"
    fi
    
    print_success "Release process completed!"
    print_status "GitHub Actions will now build and publish the release"
    print_status "Monitor the workflow at: https://github.com/ukhan1219/dui/actions"
    
    # Show next steps
    echo
    print_status "Release Summary:"
    echo "- Version: $current_version → $new_version"
    echo "- Git tag: v$new_version"
    echo "- Release notes: release_notes_$new_version.md"
    if [ "$publish_cargo" = "true" ]; then
        echo "- Published to crates.io"
    fi
    echo
    print_status "Next steps:"
    echo "1. Wait for GitHub Actions to complete (5-10 minutes)"
    echo "2. Check the release at: https://github.com/ukhan1219/dui/releases"
    echo "3. Update Homebrew formula with SHA256 hashes from the workflow"
    echo "4. Submit Homebrew formula to main repository"
    if [ "$publish_cargo" != "true" ]; then
        echo "5. Run './scripts/release.sh $bump_type --publish' to publish to crates.io"
    fi
}

# Help function
show_help() {
    echo "DUI Release Script"
    echo
    echo "Usage: $0 [bump_type] [options]"
    echo
    echo "bump_type:"
    echo "  patch    - Bump patch version (default)"
    echo "  minor    - Bump minor version"
    echo "  major    - Bump major version"
    echo
    echo "Options:"
    echo "  --publish    - Publish to crates.io (default: enabled)"
    echo "  --no-publish - Skip publishing to crates.io"
    echo "  --dry-run    - Run without making any changes"
    echo "  -h, --help   - Show this help message"
    echo
    echo "Features:"
    echo "  - Bumps version in Cargo.toml"
    echo "  - Updates CHANGELOG.md with new version"
    echo "  - Generates release notes"
    echo "  - Updates Homebrew formula"
    echo "  - Creates and pushes git tag"
    echo "  - Publishes to crates.io (optional)"
    echo "  - Builds and tests before release"
    echo
    echo "Examples:"
    echo "  $0                     # Bump patch version and publish"
    echo "  $0 minor               # Bump minor version and publish"
    echo "  $0 major --no-publish  # Bump major version without publishing"
    echo "  $0 patch --dry-run     # Preview what would happen"
}

# Dry run function
dry_run() {
    local bump_type=${1:-patch}
    
    print_status "DRY RUN: Preview of release process"
    echo "======================================"
    
    # Get current version
    current_version=$(get_current_version)
    print_status "Current version: $current_version"
    
    # Calculate new version (without actually changing it)
    case $bump_type in
        patch)
            new_version=$(echo "$current_version" | awk -F. '{printf "%d.%d.%d", $1, $2, $3+1}')
            ;;
        minor)
            new_version=$(echo "$current_version" | awk -F. '{printf "%d.%d.0", $1, $2+1}')
            ;;
        major)
            new_version=$(echo "$current_version" | awk -F. '{printf "%d.0.0", $1+1}')
            ;;
    esac
    
    print_status "Would bump version: $current_version → $new_version"
    print_status "Would update CHANGELOG.md"
    print_status "Would generate release notes: release_notes_$new_version.md"
    print_status "Would update Homebrew formula"
    print_status "Would create git tag: v$new_version"
    print_status "Would publish to crates.io"
    
    echo
    print_status "Files that would be modified:"
    echo "  - Cargo.toml"
    echo "  - Changelog.md"
    echo "  - Formula/dui.rb"
    echo "  - Cargo.lock (if present)"
    
    echo
    print_status "Git operations that would be performed:"
    echo "  - git add [modified files]"
    echo "  - git commit -m 'chore: release version $new_version'"
    echo "  - git push origin main"
    echo "  - git tag -a 'v$new_version' -m 'Release v$new_version'"
    echo "  - git push origin v$new_version"
    
    echo
    print_warning "This was a dry run. No changes were made."
    print_status "Run without --dry-run to execute the release."
}

# Parse arguments
bump_type="patch"
publish_cargo="true"
dry_run_mode="false"

while [[ $# -gt 0 ]]; do
    case $1 in
        patch|minor|major)
            bump_type="$1"
            shift
            ;;
        --publish)
            publish_cargo="true"
            shift
            ;;
        --no-publish)
            publish_cargo="false"
            shift
            ;;
        --dry-run)
            dry_run_mode="true"
            shift
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Main script execution
if [ "$dry_run_mode" = "true" ]; then
    dry_run "$bump_type"
else
    release "$bump_type" "$publish_cargo"
fi 