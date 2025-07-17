#!/bin/bash

# Test runner script for Taskwarrior TUI
# Provides convenient access to different types of tests

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_help() {
    echo "Taskwarrior TUI Test Runner"
    echo ""
    echo "Usage: $0 [command]"
    echo ""
    echo "Commands:"
    echo "  all         Run all tests"
    echo "  unit        Run unit tests only"
    echo "  integration Run integration tests only"
    echo "  ui          Run UI component tests only"
    echo "  snapshot    Run snapshot tests only"
    echo "  backend     Run backend tests only"
    echo "  review      Review snapshot changes"
    echo "  clean       Clean test artifacts"
    echo "  help        Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 all           # Run complete test suite"
    echo "  $0 ui            # Test UI components"
    echo "  $0 review        # Review snapshot changes after UI modifications"
}

run_command() {
    local desc="$1"
    local cmd="$2"
    
    echo -e "${BLUE}▶${NC} $desc"
    echo -e "${YELLOW}$cmd${NC}"
    eval "$cmd"
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $desc completed successfully"
    else
        echo -e "${RED}✗${NC} $desc failed"
        exit 1
    fi
    echo ""
}

case "${1:-help}" in
    "all")
        echo -e "${GREEN}Running complete test suite...${NC}"
        echo ""
        run_command "Unit tests" "cargo test --lib"
        run_command "Integration tests" "cargo test --test integration_tests"
        run_command "Snapshot tests" "cargo test --test snapshot_tests"
        run_command "UI testing utilities" "cargo test --test ui_testing"
        echo -e "${GREEN}All tests completed successfully!${NC}"
        ;;
    
    "unit")
        run_command "Unit tests" "cargo test --lib"
        ;;
    
    "integration")
        run_command "Integration tests" "cargo test --test integration_tests"
        ;;
    
    "ui")
        run_command "UI component tests" "cargo test ui_"
        ;;
    
    "snapshot")
        run_command "Snapshot tests" "cargo test --test snapshot_tests"
        echo -e "${YELLOW}Note: Use '$0 review' to approve any snapshot changes${NC}"
        ;;
    
    "backend")
        run_command "Backend tests" "cargo test backend"
        ;;
    
    "review")
        echo -e "${BLUE}Reviewing snapshot changes...${NC}"
        cargo insta review
        ;;
    
    "clean")
        echo -e "${YELLOW}Cleaning test artifacts...${NC}"
        cargo clean
        rm -rf snapshots/*.pending-snap
        echo -e "${GREEN}✓${NC} Test artifacts cleaned"
        ;;
    
    "help"|*)
        print_help
        ;;
esac