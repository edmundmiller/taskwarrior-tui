# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is `taskwarrior-tui`, a Terminal User Interface (TUI) for Taskwarrior written in Rust. It provides a vim-like interface for managing Taskwarrior tasks with features like live filtering, multiple selection, tab completion, and colors matching taskwarrior's output.

## Development Commands

### Build & Run
- `cargo build` - Build the project
- `cargo run` - Run the application
- `cargo run -- --report next` - Run with specific report (default is "next")

### Testing
- `cargo test` - Run all tests
- `cargo test --workspace -- --nocapture` - Run tests with output
- `cargo test ui_` - Run UI/TUI component tests specifically
- `cargo test integration_` - Run integration tests
- `cargo test snapshot_` - Run snapshot tests for visual regression
- `cargo insta review` - Review and approve/reject snapshot changes
- `cargo insta test` - Run snapshot tests with insta

### Testing Architecture

**Unit Tests:**
- Backend tests (`src/backend/tests.rs`) - Test task data operations
- Table state tests (`src/table.rs`) - Test table widget state management
- UI component tests (`src/ui.rs`) - Test individual UI widget rendering
- Configuration tests (`src/config.rs`) - Test configuration parsing

**Integration Tests:**
- `tests/ui_testing.rs` - TUI testing utilities and mock data generators
- `tests/integration_tests.rs` - End-to-end workflow testing
- `tests/snapshot_tests.rs` - Visual regression testing with snapshots

**Testing Dependencies:**
- `insta` - Snapshot testing for UI regression detection
- `proptest` - Property-based testing for robust edge case coverage
- `assert_matches` - Pattern matching assertions
- `similar` - Better diff output for test failures

**Snapshot Testing:**
Snapshots are stored in `snapshots/` directory and capture the exact terminal output for UI components. This helps detect visual regressions when making changes to the UI. Use `cargo insta review` to approve changes to the UI after intentional modifications.

### Code Quality
- `cargo check` - Check code for compilation errors
- `cargo fmt --all -- --check` - Check code formatting (use nightly toolchain)
- `cargo clippy -- -D warnings` - Run clippy lints with warnings as errors

### Release Build
- `cargo build --release` - Build optimized release version

## Code Architecture

### Core Components

**Main Application (`src/app.rs`)**
- `TaskwarriorTui` - Main application struct handling TUI state and logic
- Contains the main event loop and terminal management
- Manages task data synchronization with taskwarrior backend

**User Interface (`src/ui.rs`)**
- Handles all TUI rendering using the `ratatui` crate
- Manages different UI modes and layouts

**Event System (`src/event.rs`, `src/action.rs`)**
- `Event` - Input events from terminal
- `Action` - Application actions triggered by events
- Async event handling with crossterm

**Key Components:**
- `src/task_report.rs` - Task reporting and display logic
- `src/table.rs` - Table widget for task display
- `src/config.rs` - Configuration management
- `src/keyconfig.rs` - Key binding configuration
- `src/completion.rs` - Tab completion system
- `src/pane/` - Different panes (context, project views)

### Dependencies

**Core TUI Libraries:**
- `ratatui` - Terminal UI framework
- `crossterm` - Cross-platform terminal handling
- `tokio` - Async runtime

**Taskwarrior Integration:**
- `task-hookrs` - Taskwarrior data handling

**Other Key Dependencies:**
- `clap` - Command line argument parsing
- `rustyline` - Line editing with history
- `log4rs` - Logging framework
- `chrono` - Date/time handling

## Configuration

The application reads configuration from taskwarrior's `.taskrc` file and supports custom key bindings and colors through taskwarrior's UDA (User Defined Attributes) system.

## Development Notes

- Uses stable Rust toolchain
- Logging is configured to write to `~/.local/share/taskwarrior-tui/taskwarrior-tui.log`
- Environment variables:
  - `TASKWARRIOR_TUI_LOG_LEVEL` - Set log level (off, warn, info, debug, trace)
  - `TASKWARRIOR_TUI_CONFIG` - Custom config directory
  - `TASKWARRIOR_TUI_DATA` - Custom data directory
  - `TASKRC` - Custom taskrc file location
  - `TASKDATA` - Custom taskdata directory

## Testing Requirements

Tests require taskwarrior to be installed and use test data from the `taskwarrior-testdata` repository. The CI setup shows the testing environment configuration.