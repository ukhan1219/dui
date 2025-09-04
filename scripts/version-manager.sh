#!/bin/bash

# DUI Version Management Script
# Utility for version management and iteration

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

# Get current version from Cargo.toml
get_current_version() {
    grep '^version = ' Cargo.toml | cut -d '"' -f2
}

# Get package name
get_package_name() {
    grep '^name = ' Cargo.toml | cut -d '"' -f2
}

# Parse version string into components
parse_version() {
    local version=$1
    echo "$version" | tr '.' ' '
}

# Calculate next version
calculate_next_version() {
    local current_version=$1
    local bump_type=$2
    
    local version_parts=($(parse_version "$current_version"))
    local major=${version_parts[0]}
    local minor=${version_parts[1]}
    local patch=${version_parts[2]}
    
    case $bump_type in
        major)
            echo "$((major + 1)).0.0"
            ;;
        minor)
            echo "$major.$((minor + 1)).0"
            ;;
        patch)
            echo "$major.$minor.$((patch + 1))"
            ;;
        *)
            print_error "Invalid bump type: $bump_type"
            return 1
            ;;
    esac
}

# Set version in Cargo.toml
set_version_cargo() {
    local new_version=$1
    
    print_status "Updating version in Cargo.toml to $new_version..."
    
    if command_exists cargo-bump; then
        # Use cargo-bump if available
        cargo bump --version "$new_version"
    else
        # Manual replacement
        sed -i.bak "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
        rm -f Cargo.toml.bak
    fi
    
    print_success "Cargo.toml updated"
}

# Update version in README.md
update_version_readme() {
    local new_version=$1
    
    if [ ! -f "README.md" ]; then
        print_warning "README.md not found, skipping..."
        return 0
    fi
    
    print_status "Updating version references in README.md..."
    
    # Update cargo install command
    sed -i.bak "s/cargo install dui-cli --version [0-9]\+\.[0-9]\+\.[0-9]\+/cargo install dui-cli --version $new_version/g" README.md
    
    # Update version badges
    sed -i.bak "s/dui-cli-[0-9]\+\.[0-9]\+\.[0-9]\+/dui-cli-$new_version/g" README.md
    
    # Update any other version references
    sed -i.bak "s/v[0-9]\+\.[0-9]\+\.[0-9]\+/v$new_version/g" README.md
    
    rm -f README.md.bak
    print_success "README.md updated"
}

# Update version in Homebrew formula
update_version_formula() {
    local new_version=$1
    
    if [ ! -f "Formula/dui.rb" ]; then
        print_warning "Formula/dui.rb not found, skipping..."
        return 0
    fi
    
    print_status "Updating version in Homebrew formula..."
    
    # Update the URL with new version
    sed -i.bak "s/refs\/tags\/v[0-9]\+\.[0-9]\+\.[0-9]\+/refs\/tags\/v$new_version/g" Formula/dui.rb
    
    rm -f Formula/dui.rb.bak
    print_success "Homebrew formula updated"
    print_warning "Note: SHA256 hash will need to be updated after release"
}

# Update version in SETUP.md
update_version_setup() {
    local new_version=$1
    
    if [ ! -f "SETUP.md" ]; then
        print_warning "SETUP.md not found, skipping..."
        return 0
    fi
    
    print_status "Updating version references in SETUP.md..."
    
    sed -i.bak "s/v[0-9]\+\.[0-9]\+\.[0-9]\+/v$new_version/g" SETUP.md
    
    rm -f SETUP.md.bak
    print_success "SETUP.md updated"
}

# Update all version references
update_all_versions() {
    local new_version=$1
    local dry_run=${2:-false}
    
    if [ "$dry_run" = "true" ]; then
        print_status "DRY RUN: Would update version to $new_version in:"
        echo "  - Cargo.toml"
        [ -f "README.md" ] && echo "  - README.md"
        [ -f "Formula/dui.rb" ] && echo "  - Formula/dui.rb"
        [ -f "SETUP.md" ] && echo "  - SETUP.md"
        return 0
    fi
    
    set_version_cargo "$new_version"
    update_version_readme "$new_version"
    update_version_formula "$new_version"
    update_version_setup "$new_version"
    
    print_success "All version references updated to $new_version"
}

# Show current version info
show_version_info() {
    local current_version=$(get_current_version)
    local package_name=$(get_package_name)
    
    print_status "Version Information:"
    echo "  Package: $package_name"
    echo "  Current Version: $current_version"
    echo
    
    print_status "Next Possible Versions:"
    echo "  Patch: $(calculate_next_version "$current_version" "patch")"
    echo "  Minor: $(calculate_next_version "$current_version" "minor")"
    echo "  Major: $(calculate_next_version "$current_version" "major")"
    echo
    
    print_status "Files with version references:"
    [ -f "Cargo.toml" ] && echo "  ✓ Cargo.toml"
    [ -f "README.md" ] && echo "  ✓ README.md" || echo "  ✗ README.md (not found)"
    [ -f "Formula/dui.rb" ] && echo "  ✓ Formula/dui.rb" || echo "  ✗ Formula/dui.rb (not found)"
    [ -f "SETUP.md" ] && echo "  ✓ SETUP.md" || echo "  ✗ SETUP.md (not found)"
}

# Set specific version
set_version() {
    local new_version=$1
    local dry_run=${2:-false}
    
    if [ -z "$new_version" ]; then
        print_error "Version is required"
        return 1
    fi
    
    # Validate version format
    if ! echo "$new_version" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$'; then
        print_error "Invalid version format. Expected: X.Y.Z"
        return 1
    fi
    
    local current_version=$(get_current_version)
    
    if [ "$current_version" = "$new_version" ]; then
        print_warning "Version is already $new_version"
        return 0
    fi
    
    print_status "Changing version: $current_version → $new_version"
    
    update_all_versions "$new_version" "$dry_run"
}

# Bump version
bump_version() {
    local bump_type=$1
    local dry_run=${2:-false}
    
    local current_version=$(get_current_version)
    local new_version=$(calculate_next_version "$current_version" "$bump_type")
    
    print_status "Bumping $bump_type version: $current_version → $new_version"
    
    update_all_versions "$new_version" "$dry_run"
}

# Validate all version references
validate_versions() {
    local current_version=$(get_current_version)
    local issues_found=false
    
    print_status "Validating version consistency across files..."
    
    # Check README.md
    if [ -f "README.md" ]; then
        local readme_versions=$(grep -o 'v[0-9]\+\.[0-9]\+\.[0-9]\+' README.md | sort | uniq)
        if [ -n "$readme_versions" ]; then
            for version in $readme_versions; do
                if [ "$version" != "v$current_version" ]; then
                    print_warning "README.md contains outdated version: $version (current: v$current_version)"
                    issues_found=true
                fi
            done
        fi
    fi
    
    # Check Formula
    if [ -f "Formula/dui.rb" ]; then
        local formula_version=$(grep -o 'v[0-9]\+\.[0-9]\+\.[0-9]\+' Formula/dui.rb | head -1)
        if [ -n "$formula_version" ] && [ "$formula_version" != "v$current_version" ]; then
            print_warning "Formula/dui.rb contains outdated version: $formula_version (current: v$current_version)"
            issues_found=true
        fi
    fi
    
    if [ "$issues_found" = "false" ]; then
        print_success "All version references are consistent"
    else
        print_warning "Some version inconsistencies found. Run 'set $current_version' to fix them."
    fi
}

# Help function
show_help() {
    echo "DUI Version Management Script"
    echo
    echo "Usage: $0 <command> [options]"
    echo
    echo "Commands:"
    echo "  info                 - Show current version information"
    echo "  bump <type>          - Bump version (patch|minor|major)"
    echo "  set <version>        - Set specific version (e.g., 1.2.3)"
    echo "  validate             - Validate version consistency"
    echo
    echo "Options:"
    echo "  --dry-run           - Preview changes without applying them"
    echo "  -h, --help          - Show this help message"
    echo
    echo "Examples:"
    echo "  $0 info             # Show current version info"
    echo "  $0 bump patch       # Bump patch version"
    echo "  $0 bump minor       # Bump minor version"
    echo "  $0 bump major       # Bump major version"
    echo "  $0 set 1.2.3        # Set version to 1.2.3"
    echo "  $0 validate         # Check version consistency"
    echo "  $0 bump patch --dry-run  # Preview patch version bump"
}

# Main function
main() {
    if [ $# -eq 0 ]; then
        show_help
        exit 1
    fi
    
    local command=$1
    shift
    
    local dry_run="false"
    
    # Check for dry-run flag
    if [ "${!#}" = "--dry-run" ]; then
        dry_run="true"
        set -- "${@:1:$(($#-1))}"  # Remove last argument
    fi
    
    # Check for help flag
    if [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
        show_help
        exit 0
    fi
    
    # Check prerequisites
    if [ ! -f "Cargo.toml" ]; then
        print_error "Cargo.toml not found. Make sure you're in a Rust project directory."
        exit 1
    fi
    
    case $command in
        info)
            show_version_info
            ;;
        bump)
            if [ $# -eq 0 ]; then
                print_error "Bump type is required (patch|minor|major)"
                exit 1
            fi
            bump_version "$1" "$dry_run"
            ;;
        set)
            if [ $# -eq 0 ]; then
                print_error "Version is required"
                exit 1
            fi
            set_version "$1" "$dry_run"
            ;;
        validate)
            validate_versions
            ;;
        *)
            print_error "Unknown command: $command"
            show_help
            exit 1
            ;;
    esac
}

# Run main function
main "$@"
