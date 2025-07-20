# Developer Guide

## Quick Start

### Initial Setup

```bash
git clone https://github.com/kdheepak/taskwarrior-tui
cd taskwarrior-tui

# Clone test data repository
git clone https://github.com/kdheepak/taskwarrior-testdata tests/data
source .envrc

# Build and test
cargo build
cargo test
```

### Running the Application

```bash
# Debug build
cargo run

# Release build
cargo run --release

# With specific report
cargo run -- --report next
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test --workspace -- --nocapture

# Run specific test categories
cargo test ui_                # UI/TUI component tests
cargo test integration_       # Integration tests
cargo test snapshot_          # Snapshot tests for visual regression
```

### Testing Individual Functions

If you want to test the `test_taskwarrior_timing` function in `src/app.rs`:

```bash
cargo test -- app::tests::test_taskwarrior_timing --nocapture
```

### Snapshot Testing

```bash
# Run snapshot tests
cargo insta test

# Review and approve/reject snapshot changes
cargo insta review
```

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

## Code Quality

```bash
# Check code for compilation errors
cargo check

# Check code formatting (requires nightly toolchain)
cargo fmt --all -- --check

# Run clippy lints with warnings as errors
cargo clippy -- -D warnings
```

## Logging and Debugging

### Getting Logs

```bash
# Set log level and run
export TASKWARRIOR_TUI_LOG_LEVEL=debug
taskwarrior-tui

# OR for trace level
export TASKWARRIOR_TUI_LOG_LEVEL=trace
cargo run
```

### Environment Variables

- `TASKWARRIOR_TUI_LOG_LEVEL` - Set log level (off, warn, info, debug, trace)
- `TASKWARRIOR_TUI_CONFIG` - Custom config directory
- `TASKWARRIOR_TUI_DATA` - Custom data directory
- `TASKRC` - Custom taskrc file location
- `TASKDATA` - Custom taskdata directory

Log files are written to `~/.local/share/taskwarrior-tui/taskwarrior-tui.log`

## Architecture Overview

### Internals

`taskwarrior-tui` is a state-driven terminal user interface. Keyboard events are read asynchronously and communicated using channels. Most of the logic is implemented in `src/app.rs`. The difference between the previous state and the current state of the TUI is rendered every `Tick` by `ratatui`. The `app.draw_...` functions are responsible for rendering the UI. Actions for key presses are taken in [`app.handle_input(&mut self, input: Key)`](https://github.com/kdheepak/taskwarrior-tui/blob/main/src/app.rs).

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
- `src/timewarrior.rs` - TimeWarrior integration with caching

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

## Contributing to Documentation

See `docs/` folder in the repository: <https://github.com/kdheepak/taskwarrior-tui>

When you make a PR to the repository, a preview of the documentation is rendered and a link is posted to the PR.

## TimeWarrior Integration Performance

The TimeWarrior integration includes several performance optimizations to maintain UI responsiveness:

### Caching Architecture

**Cache Structure (`src/timewarrior.rs:37-66`):**
- `TrackingCache` struct with configurable cache duration (default: 5 seconds)
- Uses `RefCell` for interior mutability to allow cache updates from immutable references
- Tracks last update time and automatically expires stale data

**Cache Management:**
- Only queries TimeWarrior when cache is expired
- Single batch query retrieves all tracked task UUIDs at once
- Subsequent task checks use cached data without system calls

### Query Optimization

**Before Optimization:**
- Called `timew get dom.active.tag.1` for each task individually
- N system calls per UI render (where N = number of tasks)
- Could cause UI lag with large task lists

**After Optimization:**
- Single call to `timew get dom.active.tag` gets all tracked UUIDs
- Maximum 2 system calls every 5 seconds regardless of task count
- ~90%+ reduction in system calls for typical usage

### Implementation Details

```rust
// Cache refresh only when expired
if self.tracking_cache.borrow().is_expired() {
    self.refresh_tracking_cache()?;
}

// Fast lookup from cached data
self.tracking_cache.borrow().contains(task_uuid)
```

**Error Handling:**
- Graceful fallback when TimeWarrior is unavailable
- Cache is cleared on errors to prevent stale state
- Logging for debugging integration issues

**Testing:**
- Comprehensive test coverage for cache behavior
- Performance regression tests ensure optimizations are maintained
- Mock integration for testing without TimeWarrior dependency

## Development Notes

- Uses stable Rust toolchain
- Snapshots are stored in `snapshots/` directory and capture exact terminal output
- Use `cargo insta review` to approve UI changes after intentional modifications
- Tests require taskwarrior to be installed and use test data from the `taskwarrior-testdata` repository