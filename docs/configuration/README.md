# Configuration Overview

This directory contains comprehensive documentation for all configurable options in `taskwarrior-tui`. The configuration system uses Taskwarrior's User Defined Attributes (UDA) feature to extend the base functionality.

## Configuration Files

- **[keys.md](keys.md)** - Key binding configuration
- **[colors.md](colors.md)** - Color scheme configuration  
- **[advanced.md](advanced.md)** - Advanced behavior and appearance settings

## Configuration Categories

### 1. Key Bindings (`uda.taskwarrior-tui.keyconfig.*`)

All key bindings can be customized by setting the appropriate UDA value in your `~/.taskrc` file. See [keys.md](keys.md) for a complete list of configurable keys.

### 2. Colors (`color.*`)

`taskwarrior-tui` inherits Taskwarrior's color configuration and extends it with calendar-specific colors. See [colors.md](colors.md) for details.

### 3. Visual Appearance (`uda.taskwarrior-tui.style.*`)

Customize the look of various interface elements including:
- Selection indicators and styles
- Scrollbar appearance
- Navigation bar styling
- Calendar appearance

### 4. Task Report Behavior (`uda.taskwarrior-tui.task-report.*`)

Control how tasks are displayed and manipulated:
- Information panel visibility
- Automatic quoting behavior
- Filtering and completion settings
- Time formatting preferences

### 5. Interface Behavior (`uda.taskwarrior-tui.*`)

General application behavior:
- UI refresh rates
- Tab navigation
- Context menu behavior
- Quick tag functionality

### 6. Backend Configuration (`uda.taskwarrior-tui.backend.*`)

Configure the backend used for task management:
- CLI backend (default for older builds)
- TaskChampion backend (default for newer builds)
- Server synchronization settings

## Quick Start

To get started with configuration, add any of the following to your `~/.taskrc` file:

```bash
# Example: Change the quit key from 'q' to 'Q'
uda.taskwarrior-tui.keyconfig.quit=Q

# Example: Disable the information panel
uda.taskwarrior-tui.task-report.show-info=false

# Example: Change the calendar layout to 6 months per row
uda.taskwarrior-tui.calendar.months-per-row=6

# Example: Set custom selection indicator
uda.taskwarrior-tui.selection.indicator=>
```

## Environment Variables

The following environment variables can also be used to configure `taskwarrior-tui`:

- `TASKWARRIOR_TUI_LOG_LEVEL` - Set log level (off, warn, info, debug, trace)
- `TASKWARRIOR_TUI_CONFIG` - Custom config directory
- `TASKWARRIOR_TUI_DATA` - Custom data directory
- `TASKRC` - Custom taskrc file location
- `TASKDATA` - Custom taskdata directory

## Configuration Discovery

To see all available configuration options with their current values, run:

```bash
task show | grep taskwarrior-tui
```

This will show you all currently configured `taskwarrior-tui` options and their values.

## Configuration Validation

`taskwarrior-tui` validates key bindings to ensure no conflicts exist. If duplicate keys are detected, the application will display an error message on startup.

## See Also

- [Taskwarrior Color Documentation](https://taskwarrior.org/docs/color/)
- [Taskwarrior UDA Documentation](https://taskwarrior.org/docs/udas/)
- [Taskwarrior Configuration Documentation](https://taskwarrior.org/docs/configuration/)