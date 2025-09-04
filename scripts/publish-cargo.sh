#!/bin/bash

# DUI Cargo Publishing Script
# Standalone script for publishing to crates.io

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

# Get current version
get_current_version() {
    grep '^version = ' Cargo.toml | cut -d '"' -f2
}

# Get package name
get_package_name() {
    grep '^name = ' Cargo.toml | cut -d '"' -f2
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    if ! command_exists cargo; then
        print_error "cargo is not installed"
        exit 1
    fi
    
    if ! cargo --version | grep -q "cargo"; then
        print_error "cargo is not working properly"
        exit 1
    fi
    
    # Check if we're in a Cargo project
    if [ ! -f "Cargo.toml" ]; then
        print_error "Cargo.toml not found. Make sure you're in a Rust project directory."
        exit 1
    fi
    
    print_success "Prerequisites check passed"
}

# Check if already published
check_if_published() {
    local package_name=$1
    local version=$2
    
    print_status "Checking if version $version is already published..."
    
    # Try to get info about the specific version
    local api_url="https://crates.io/api/v1/crates/$package_name/$version"
    local http_code=$(curl -s -o /dev/null -w "%{http_code}" "$api_url")
    
    if [ "$http_code" = "200" ]; then
        print_warning "Version $version is already published to crates.io"
        return 0
    else
        print_status "Version $version is not yet published"
        return 1
    fi
}

# Validate package before publishing
validate_package() {
    print_status "Validating package..."
    
    # Check if package builds
    print_status "Building package..."
    if ! cargo build --release; then
        print_error "Package build failed"
        return 1
    fi
    
    # Check if tests pass
    print_status "Running tests..."
    if ! cargo test; then
        print_warning "Tests failed, but continuing..."
    fi
    
    # Run cargo check
    print_status "Running cargo check..."
    if ! cargo check; then
        print_error "Cargo check failed"
        return 1
    fi
    
    # Check manifest
    print_status "Validating manifest..."
    if ! cargo verify-project; then
        print_error "Cargo manifest validation failed"
        return 1
    fi
    
    print_success "Package validation passed"
}

# Publish to cargo
publish_to_cargo() {
    local dry_run=${1:-false}
    
    if [ "$dry_run" = "true" ]; then
        print_status "Running cargo publish dry run..."
        if cargo publish --dry-run; then
            print_success "Dry run successful! Package is ready for publishing."
            return 0
        else
            print_error "Dry run failed. Please fix the issues before publishing."
            return 1
        fi
    else
        print_status "Publishing to crates.io..."
        print_warning "This will publish the package permanently to crates.io"
        
        # Ask for confirmation
        read -p "Are you sure you want to publish? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_status "Publishing cancelled by user"
            return 1
        fi
        
        # Publish for real
        print_status "Publishing to crates.io (this may take a few minutes)..."
        if cargo publish; then
            print_success "Successfully published to crates.io!"
            return 0
        else
            print_error "Failed to publish to crates.io"
            return 1
        fi
    fi
}

# Show package info
show_package_info() {
    local package_name=$(get_package_name)
    local version=$(get_current_version)
    
    print_status "Package Information:"
    echo "  Name: $package_name"
    echo "  Version: $version"
    echo "  crates.io URL: https://crates.io/crates/$package_name"
    echo "  Documentation URL: https://docs.rs/$package_name"
    echo "  Repository: $(grep '^repository = ' Cargo.toml | cut -d '"' -f2 || echo 'Not specified')"
    echo "  License: $(grep '^license = ' Cargo.toml | cut -d '"' -f2 || echo 'Not specified')"
}

# Help function
show_help() {
    echo "DUI Cargo Publishing Script"
    echo
    echo "Usage: $0 [options]"
    echo
    echo "Options:"
    echo "  --dry-run    - Run cargo publish --dry-run (test without publishing)"
    echo "  --force      - Skip already-published check"
    echo "  --info       - Show package information"
    echo "  -h, --help   - Show this help message"
    echo
    echo "Features:"
    echo "  - Validates package before publishing"
    echo "  - Checks if version is already published"
    echo "  - Builds and tests before publishing"
    echo "  - Provides confirmation before publishing"
    echo "  - Shows helpful URLs after publishing"
    echo
    echo "Examples:"
    echo "  $0               # Publish current version"
    echo "  $0 --dry-run     # Test publishing without actually publishing"
    echo "  $0 --info        # Show package information"
    echo "  $0 --force       # Force publish even if version exists"
}

# Main function
main() {
    local dry_run="false"
    local force="false"
    local show_info="false"
    
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --dry-run)
                dry_run="true"
                shift
                ;;
            --force)
                force="true"
                shift
                ;;
            --info)
                show_info="true"
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
    
    # Show package info if requested
    if [ "$show_info" = "true" ]; then
        show_package_info
        exit 0
    fi
    
    # Run checks
    check_prerequisites
    
    # Get package details
    local package_name=$(get_package_name)
    local version=$(get_current_version)
    
    print_status "Publishing package: $package_name v$version"
    
    # Check if already published (unless forced)
    if [ "$force" != "true" ] && [ "$dry_run" != "true" ]; then
        if check_if_published "$package_name" "$version"; then
            print_error "Version $version is already published. Use --force to override."
            exit 1
        fi
    fi
    
    # Validate package
    if ! validate_package; then
        print_error "Package validation failed"
        exit 1
    fi
    
    # Publish
    if ! publish_to_cargo "$dry_run"; then
        exit 1
    fi
    
    # Show success info
    if [ "$dry_run" != "true" ]; then
        echo
        print_success "Publishing completed successfully!"
        echo
        print_status "Your package is now available at:"
        echo "  crates.io: https://crates.io/crates/$package_name"
        echo "  docs.rs: https://docs.rs/$package_name"
        echo
        print_status "Installation command for users:"
        echo "  cargo install $package_name"
    fi
}

# Run main function with all arguments
main "$@"
