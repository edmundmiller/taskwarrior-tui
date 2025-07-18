# Timewarrior Integration

This document describes the timewarrior integration features in taskwarrior-tui.

## Overview

The timewarrior integration provides automatic time tracking when you start and stop tasks in taskwarrior-tui. This integration uses a combination of:

- A taskwarrior hook script that automatically starts/stops timewarrior tracking
- Enhanced TUI functionality that shows timewarrior status and provides management commands

## Features

### Automatic Time Tracking

When you start a task using the `s` key in taskwarrior-tui, timewarrior will automatically begin tracking time for that task. When you stop the task, timewarrior tracking will automatically stop.

### Timewarrior Status Display

The task details view shows comprehensive timewarrior integration information:

- Integration status (available/not available)
- Hook installation status
- Active tracking information
- Current tags and duration

### Enhanced Start/Stop Feedback

The `s` key now provides feedback about timewarrior integration status when starting/stopping tasks.

## Setup Instructions

### Prerequisites

1. **Install timewarrior** (if not already installed):
   ```bash
   # macOS
   brew install timewarrior
   
   # Ubuntu/Debian
   sudo apt install timewarrior
   
   # Fedora/RHEL
   sudo dnf install timewarrior
   ```

2. **Verify installation**:
   ```bash
   timew --version
   ```

### Quick Setup

1. **Install the hook** (from the project directory):
   ```bash
   # Copy the hook to your taskwarrior hooks directory
   cp hooks/on-modify.timewarrior ~/.task/hooks/
   chmod +x ~/.task/hooks/on-modify.timewarrior
   ```

2. **Enable integration**:
   ```bash
   task config uda.timewarrior.enabled true
   ```

3. **Test the integration**:
   ```bash
   # Start a task
   task 1 start
   
   # Check timewarrior status
   timew
   
   # Stop the task
   task 1 stop
   ```

### Configuration Options

The integration can be configured through taskwarrior's User Defined Attributes (UDA):

```bash
# Enable/disable integration (default: true)
task config uda.timewarrior.enabled true

# Tag prefix for timewarrior entries (default: none)
task config uda.timewarrior.tag.prefix "tw_"

# Include project in timewarrior tags (default: true)
task config uda.timewarrior.include.project true

# Include task description in timewarrior tags (default: false)
task config uda.timewarrior.include.description false

# Set logging level (default: info)
task config uda.timewarrior.log.level info
```

## Usage

### Basic Workflow

1. **Start a task**: Press `s` while a task is selected in taskwarrior-tui
   - The task will start in taskwarrior
   - Timewarrior will automatically begin tracking
   - You'll see a confirmation message

2. **Stop a task**: Press `s` again while an active task is selected
   - The task will stop in taskwarrior
   - Timewarrior will automatically stop tracking
   - You'll see a confirmation message

3. **View timewarrior status**: Navigate to the task details view to see:
   - Integration status
   - Active tracking information
   - Current tags and duration

### Tag Mapping

The integration automatically maps taskwarrior data to timewarrior tags:

- **Project**: Added as `project:projectname`
- **Task UUID**: Added as `uuid:task-uuid-here`
- **Task tags**: Included as-is (if configured)
- **Description**: Included as `desc:description` (if configured)

### Example Timewarrior Output

```
$ timew
Tracking project:work "uuid:32dc5992-397c-4e0f-9e74-9f7bfc648986"
  Started 2025-07-17T14:39:30
  Current                  1:23
  Total               0:01:23
```

## Troubleshooting

### Common Issues

1. **Hook not working**:
   - Verify hook is executable: `ls -la ~/.task/hooks/on-modify.timewarrior`
   - Check hook logs: `tail -f ~/.local/share/taskwarrior-tui/timewarrior-hook.log`

2. **Integration disabled**:
   - Check configuration: `task _get rc.uda.timewarrior.enabled`
   - Enable if needed: `task config uda.timewarrior.enabled true`

3. **Timewarrior not available**:
   - Verify installation: `timew --version`
   - Check PATH: `which timew`

### Log Files

The integration creates log files for debugging:

- **Hook logs**: `~/.local/share/taskwarrior-tui/timewarrior-hook.log`
- **TUI logs**: `~/.local/share/taskwarrior-tui/taskwarrior-tui.log`

### Manual Testing

Test the hook manually:

```bash
# Test hook with sample data
echo -e '{}' | echo '{"uuid":"test-uuid","description":"Test task","start":"20231217T120000Z"}' | ~/.task/hooks/on-modify.timewarrior
```

## Architecture

### Hook Implementation

The `on-modify.timewarrior` hook script:
- Monitors task modifications via taskwarrior's hook system
- Detects start/stop events by comparing old and new task states
- Automatically calls appropriate timewarrior commands
- Provides comprehensive error handling and logging

### TUI Integration

The TUI integration provides:
- Status checking and display
- Hook installation/management
- Enhanced user feedback
- Configuration management

### Configuration Storage

Configuration is stored in taskwarrior's `.taskrc` file using User Defined Attributes (UDA), ensuring consistency across all taskwarrior tools.

## Future Enhancements

Potential future features:
- Direct timewarrior commands from TUI
- Time tracking visualization
- Integration with task reports
- Custom tag mapping rules
- Multi-project time tracking