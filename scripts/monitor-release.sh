#!/bin/bash

# DUI Release Monitor Script
# This script monitors the GitHub Actions workflow progress

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
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

print_workflow() {
    echo -e "${PURPLE}[WORKFLOW]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    if ! command_exists curl; then
        print_error "curl is not installed"
        exit 1
    fi
    
    if ! command_exists jq; then
        print_warning "jq is not installed. Installing..."
        if command_exists brew; then
            brew install jq
        elif command_exists apt-get; then
            sudo apt-get update && sudo apt-get install -y jq
        else
            print_error "Please install jq manually"
            exit 1
        fi
    fi
    
    print_success "Prerequisites check passed"
}

# Function to get latest workflow run
get_latest_workflow() {
    print_status "Getting latest workflow run..."
    
    local response=$(curl -s -H "Authorization: token $GITHUB_TOKEN" \
        "https://api.github.com/repos/ukhan1219/dui/actions/runs?per_page=1")
    
    local run_id=$(echo "$response" | jq -r '.workflow_runs[0].id')
    local status=$(echo "$response" | jq -r '.workflow_runs[0].status')
    local conclusion=$(echo "$response" | jq -r '.workflow_runs[0].conclusion')
    
    echo "$run_id $status $conclusion"
}

# Function to get workflow status
get_workflow_status() {
    local run_id=$1
    
    local response=$(curl -s -H "Authorization: token $GITHUB_TOKEN" \
        "https://api.github.com/repos/ukhan1219/dui/actions/runs/$run_id")
    
    local status=$(echo "$response" | jq -r '.status')
    local conclusion=$(echo "$response" | jq -r '.conclusion')
    local html_url=$(echo "$response" | jq -r '.html_url')
    
    echo "$status $conclusion $html_url"
}

# Function to get job status
get_job_status() {
    local run_id=$1
    
    local response=$(curl -s -H "Authorization: token $GITHUB_TOKEN" \
        "https://api.github.com/repos/ukhan1219/dui/actions/runs/$run_id/jobs")
    
    echo "$response" | jq -r '.jobs[] | "\(.name): \(.status) (\(.conclusion // "in_progress"))"'
}

# Function to show progress bar
show_progress() {
    local current=$1
    local total=$2
    local width=50
    
    local filled=$((current * width / total))
    local empty=$((width - filled))
    
    printf "["
    printf "%${filled}s" | tr ' ' '#'
    printf "%${empty}s" | tr ' ' '-'
    printf "] %d%%\r" $((current * 100 / total))
}

# Function to monitor workflow
monitor_workflow() {
    local run_id=$1
    
    print_workflow "Monitoring workflow run: $run_id"
    print_workflow "View details at: https://github.com/ukhan1219/dui/actions/runs/$run_id"
    
    local completed_jobs=0
    local total_jobs=5  # test, build-x86_64, build-aarch64, release, publish
    
    while true; do
        local status_info=$(get_workflow_status "$run_id")
        local status=$(echo "$status_info" | cut -d' ' -f1)
        local conclusion=$(echo "$status_info" | cut -d' ' -f2)
        local html_url=$(echo "$status_info" | cut -d' ' -f3)
        
        clear
        echo "=========================================="
        echo "           DUI Release Monitor"
        echo "=========================================="
        echo
        
        print_workflow "Workflow ID: $run_id"
        print_workflow "Status: $status"
        print_workflow "Conclusion: $conclusion"
        print_workflow "URL: $html_url"
        echo
        
        # Get job status
        print_status "Job Status:"
        get_job_status "$run_id" | while read -r job_info; do
            local job_name=$(echo "$job_info" | cut -d':' -f1)
            local job_status=$(echo "$job_info" | cut -d':' -f2 | xargs)
            local job_conclusion=$(echo "$job_info" | cut -d'(' -f2 | cut -d')' -f1)
            
            if [ "$job_conclusion" = "success" ]; then
                print_success "$job_name: $job_status"
                ((completed_jobs++))
            elif [ "$job_conclusion" = "failure" ]; then
                print_error "$job_name: $job_status"
            elif [ "$job_status" = "completed" ]; then
                print_success "$job_name: $job_status"
                ((completed_jobs++))
            else
                print_status "$job_name: $job_status"
            fi
        done
        
        echo
        show_progress "$completed_jobs" "$total_jobs"
        
        # Check if workflow is complete
        if [ "$status" = "completed" ]; then
            echo
            echo
            if [ "$conclusion" = "success" ]; then
                print_success "Workflow completed successfully!"
                print_status "Release is now available at: https://github.com/ukhan1219/dui/releases"
                print_status "Next steps:"
                echo "1. Check the release at: https://github.com/ukhan1219/dui/releases"
                echo "2. Update Homebrew formula with SHA256 hashes"
                echo "3. Submit Homebrew formula to main repository"
            else
                print_error "Workflow failed!"
                print_status "Check the logs at: $html_url"
            fi
            break
        fi
        
        sleep 10
    done
}

# Function to get SHA256 hashes
get_sha256_hashes() {
    print_status "Getting SHA256 hashes for Homebrew formula..."
    
    local latest_tag=$(curl -s https://api.github.com/repos/ukhan1219/dui/releases/latest | jq -r '.tag_name')
    local version=${latest_tag#v}
    
    echo "SHA256 hashes for version $version:"
    echo
    
    # Download and calculate hashes
    local temp_dir=$(mktemp -d)
    cd "$temp_dir"
    
    # Download x86_64 binary
    curl -L -o "dui-x86_64-apple-darwin.tar.gz" \
        "https://github.com/ukhan1219/dui/releases/download/v$version/dui-x86_64-apple-darwin.tar.gz"
    
    # Download aarch64 binary
    curl -L -o "dui-aarch64-apple-darwin.tar.gz" \
        "https://github.com/ukhan1219/dui/releases/download/v$version/dui-aarch64-apple-darwin.tar.gz"
    
    echo "x86_64-apple-darwin:"
    shasum -a 256 "dui-x86_64-apple-darwin.tar.gz"
    echo
    
    echo "aarch64-apple-darwin:"
    shasum -a 256 "dui-aarch64-apple-darwin.tar.gz"
    echo
    
    # Clean up
    cd - > /dev/null
    rm -rf "$temp_dir"
    
    print_status "Copy these hashes to update your Homebrew formula"
}

# Function to show usage
show_usage() {
    echo "DUI Release Monitor"
    echo
    echo "Usage: $0 [options]"
    echo
    echo "Options:"
    echo "  -h, --help     Show this help message"
    echo "  -s, --sha256   Get SHA256 hashes for Homebrew formula"
    echo "  -w, --workflow <id>  Monitor specific workflow ID"
    echo
    echo "Examples:"
    echo "  $0              # Monitor latest workflow"
    echo "  $0 -s           # Get SHA256 hashes"
    echo "  $0 -w 123456    # Monitor specific workflow"
}

# Main function
main() {
    # Check prerequisites
    check_prerequisites
    
    # Check for GitHub token
    if [ -z "$GITHUB_TOKEN" ]; then
        print_warning "GITHUB_TOKEN not set. Some features may not work."
        print_warning "Set GITHUB_TOKEN environment variable for full functionality."
    fi
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_usage
                exit 0
                ;;
            -s|--sha256)
                get_sha256_hashes
                exit 0
                ;;
            -w|--workflow)
                WORKFLOW_ID="$2"
                shift 2
                ;;
            *)
                print_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done
    
    # Get workflow ID
    if [ -z "$WORKFLOW_ID" ]; then
        print_status "Getting latest workflow..."
        local workflow_info=$(get_latest_workflow)
        WORKFLOW_ID=$(echo "$workflow_info" | cut -d' ' -f1)
    fi
    
    # Monitor workflow
    monitor_workflow "$WORKFLOW_ID"
}

# Run main function
main "$@" 