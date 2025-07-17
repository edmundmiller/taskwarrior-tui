# Testing Framework for Taskwarrior TUI

This directory contains comprehensive tests for the Taskwarrior TUI application, organized for easy understanding and maintenance.

## 📁 Test File Organization

### Core Testing Infrastructure

#### `ui_testing.rs` - Testing Utilities and Mock Data
**Purpose**: Provides reusable testing infrastructure for all TUI tests

**Key Components**:
- `TuiTestHelper`: Primary utility for rendering widgets and capturing output
- `MockData`: Generates consistent test data (tasks, projects, tags)
- `UiAssertions`: Helper functions for common UI test assertions
- `strategies`: Property-based testing strategies for random but valid data

**When to use**: Import this module in other test files to access testing utilities

### Test Suites

#### `integration_tests.rs` - End-to-End Workflow Testing
**Purpose**: Tests complete user workflows and state transitions

**Test Categories**:
- **Basic UI Functionality**: Empty lists, single/multiple task display, error handling
- **State Management**: Table state transitions, filtering, selection modes  
- **Responsive Design**: Different terminal sizes and layout adaptation
- **Performance Testing**: Large datasets, Unicode support, rendering benchmarks
- **Property-Based Testing**: Random data validation with invariant checking

**When to run**: `cargo test integration_` - for testing complete user flows

#### `snapshot_tests.rs` - Visual Regression Protection  
**Purpose**: Captures exact terminal output to detect visual changes

**Test Categories**:
- **Basic Widgets**: Title, help text, status bar, progress indicators
- **Task List Scenarios**: Empty, single, multiple, selection states
- **Responsive Design**: Narrow (40x15), wide (120x30), square (60x60) terminals
- **Theme Testing**: Dark, light, high contrast color schemes
- **Layout Testing**: Split pane, tabbed, three-pane interfaces
- **Edge Cases**: Error messages, loading states, confirmation dialogs
- **Regression Tests**: Unicode, special characters, minimal terminal sizes

**When to run**: `cargo test snapshot_` - for visual regression testing

## 🚀 Running Tests

### Quick Commands
```bash
# Run all tests
cargo test --workspace -- --nocapture

# Run specific test categories
cargo test ui_              # UI component tests
cargo test integration_     # Integration tests  
cargo test snapshot_        # Snapshot tests

# Manage snapshots
cargo insta review         # Review snapshot changes
cargo insta accept         # Accept all snapshots
```

### Using the Test Runner Script
```bash
# Convenient test runner with organized output
./scripts/test.sh all           # Run complete test suite
./scripts/test.sh ui            # Test UI components
./scripts/test.sh snapshot      # Run snapshot tests
./scripts/test.sh review        # Review snapshot changes
```

## 🎯 Test Writing Guidelines

### For Integration Tests
1. **Use descriptive test names** that explain the scenario
2. **Test complete workflows** rather than individual functions
3. **Verify both functionality and UI state** using UiAssertions
4. **Include snapshot assertions** for UI components when appropriate

### For Snapshot Tests
1. **Create tests for each UI state** you want to protect
2. **Use consistent terminal sizes** within test categories
3. **Include diverse scenarios** (empty, full, error states)
4. **Review snapshots carefully** before accepting changes

### For Property-Based Tests
1. **Use the provided strategies** in `ui_testing::strategies`
2. **Test invariants** that should always hold true
3. **Include shrinking-friendly properties** for better failure diagnosis

## 📊 Test Coverage

The testing framework covers:
- ✅ UI component rendering and layout
- ✅ Task data management and filtering  
- ✅ State transitions and user interactions
- ✅ Error handling and edge cases
- ✅ Performance with large datasets
- ✅ Visual regression protection
- ✅ Responsive design across terminal sizes
- ✅ Theme and accessibility support

## 🔧 Snapshot Management

### When Snapshots Change
1. **Review changes carefully**: `cargo insta review` 
2. **Verify changes are intentional** before accepting
3. **Test across different terminal sizes** if layout changed
4. **Update related documentation** if UI behavior changed

### Snapshot Organization
- Snapshots are stored in `snapshots/` subdirectories
- Names include module path for easy identification
- Filters in `.insta.toml` normalize timestamps and UUIDs

## 🤖 AI Agent Guidelines

When working with these tests as an AI agent:

1. **Start with `ui_testing.rs`** to understand available utilities
2. **Read module-level documentation** for test organization
3. **Use section headers** to navigate to specific test categories
4. **Follow existing patterns** when adding new tests
5. **Update snapshots carefully** and review changes before committing
6. **Reference this README** for understanding test purpose and organization

The testing framework is designed to be:
- **Self-documenting** with extensive comments and clear structure
- **Modular** with reusable utilities and clear separation of concerns
- **Comprehensive** covering UI, functionality, and edge cases
- **Maintainable** with consistent patterns and good organization