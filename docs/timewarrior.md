# TimeWarrior Integration

`taskwarrior-tui` provides comprehensive integration with [TimeWarrior](https://timewarrior.net/), allowing for automatic time tracking when you start and stop tasks. This integration includes visual indicators for tracked tasks and seamless workflow integration.

## Features

- **Automatic Time Tracking**: When you start or stop tasks using the `s` key, TimeWarrior automatically begins or ends time tracking
- **Visual Highlighting**: Tasks currently being tracked by TimeWarrior are highlighted in the task list with configurable styling
- **Hook-Based Integration**: Uses a Python hook script that monitors task modifications and automatically manages TimeWarrior tracking
- **Configurable Tag Mapping**: Maps taskwarrior data to timewarrior tags (project, UUID, description, etc.)
- **Performance Optimized**: Uses intelligent caching to provide fast highlighting without impacting UI responsiveness

## Quick Start

1. **Install TimeWarrior**: Follow the [TimeWarrior installation guide](https://timewarrior.net/download/)

2. **Enable Integration**: Add to your `~/.taskrc`:
   ```bash
   uda.timewarrior.enabled=true
   ```

3. **Install Hook**: Use the TimeWarrior integration commands in taskwarrior-tui to install the hook script

4. **Start Tracking**: Press `s` on any task to start/stop time tracking

## Installation

### Prerequisites

- [TimeWarrior](https://timewarrior.net/) must be installed and available in your PATH
- Python 3 (required for the hook script)

### Hook Installation

The TimeWarrior integration requires a hook script to automatically start and stop time tracking when tasks are started or stopped in taskwarrior.

**Automatic Installation (Recommended):**
1. Open taskwarrior-tui
2. Navigate to the task details view
3. Look for TimeWarrior integration status
4. Follow the provided installation instructions

**Manual Installation:**
1. Copy the hook script from `hooks/on-modify.timewarrior` to your taskwarrior hooks directory (usually `~/.task/hooks/`)
2. Make the script executable: `chmod +x ~/.task/hooks/on-modify.timewarrior`

## Configuration

All TimeWarrior integration settings are configured through taskwarrior's User Defined Attributes (UDA) system.

### Basic Configuration

```bash
# Enable TimeWarrior integration
uda.timewarrior.enabled=true

# Configure which data to include in TimeWarrior tags
uda.timewarrior.include.project=true
uda.timewarrior.include.description=false

# Set log level for debugging
uda.timewarrior.log.level=info
```

### Visual Styling

Configure how tracked tasks appear in the task list:

```bash
# Style for tasks currently being tracked (default: green bold)
uda.taskwarrior-tui.style.timewarrior.tracking=green bold

# Examples of other styles:
# uda.taskwarrior-tui.style.timewarrior.tracking=blue on_yellow
# uda.taskwarrior-tui.style.timewarrior.tracking=red underline
# uda.taskwarrior-tui.style.timewarrior.tracking=white on_green bold
```

### Advanced Configuration

```bash
# Tag prefix for TimeWarrior entries (optional)
uda.timewarrior.tag.prefix=tw_

# Include task description in TimeWarrior tags (can be verbose)
uda.timewarrior.include.description=true

# Set detailed logging for troubleshooting
uda.timewarrior.log.level=debug
```

## Usage

### Starting and Stopping Time Tracking

1. **Navigate to a task** in the task list
2. **Press `s`** to start tracking the task
   - TimeWarrior will automatically start tracking
   - The task will be highlighted with the configured style
   - A confirmation message will appear
3. **Press `s` again** to stop tracking
   - TimeWarrior will stop the current tracking session
   - The highlighting will be removed

### Visual Indicators

- **Highlighted Tasks**: Tasks currently being tracked by TimeWarrior are visually highlighted
- **Style Customization**: The highlighting style can be customized using the `uda.taskwarrior-tui.style.timewarrior.tracking` configuration
- **Real-time Updates**: The highlighting updates automatically when tracking status changes

### Integration Status

View the current TimeWarrior integration status:

1. Select any task and view the task details panel
2. Look for the "TimeWarrior Integration" section
3. This shows:
   - Whether TimeWarrior is available
   - Hook installation status
   - Current tracking information
   - Setup instructions if needed

## Tag Mapping

When a task is tracked, the following information is mapped to TimeWarrior tags:

- **Project**: `project:projectname` (if `uda.timewarrior.include.project=true`)
- **UUID**: `uuid:task-uuid` (always included for task identification)
- **Task Tags**: Existing task tags are included as-is
- **Description**: `desc:description` (if `uda.timewarrior.include.description=true`)

### Example TimeWarrior Entry

For a task with project "work" and tags "urgent", the TimeWarrior entry might look like:
```
project:work uuid:a1b2c3d4-e5f6-7890-abcd-ef1234567890 urgent
```

## Troubleshooting

### TimeWarrior Not Found
- Ensure TimeWarrior is installed and available in your PATH
- Test by running `timew --version` in your terminal

### Hook Not Working
- Verify the hook script is in the correct location (`~/.task/hooks/on-modify.timewarrior`)
- Ensure the script is executable: `chmod +x ~/.task/hooks/on-modify.timewarrior`
- Check the script has the correct shebang line: `#!/usr/bin/env python3`

### Integration Disabled
- Check that `uda.timewarrior.enabled=true` is set in your `~/.taskrc`
- Restart taskwarrior-tui after configuration changes

### No Visual Highlighting
- Verify that TimeWarrior is actually tracking (run `timew` to check)
- Check your terminal supports the configured colors/styles
- Try a simpler style like `uda.taskwarrior-tui.style.timewarrior.tracking=bold`

### Performance Issues
The integration uses intelligent caching to minimize performance impact:
- Tracking status is cached for 5 seconds
- Only one TimeWarrior query is made per cache refresh
- Cache automatically refreshes when expired

If you experience performance issues:
- Check TimeWarrior is responding quickly: `time timew get dom.active`
- Verify the hook script is not causing delays

## Commands and Shortcuts

| Key | Action |
|-----|--------|
| `s` | Start/stop time tracking for selected task |

## Advanced Usage

### Multiple Task Tracking
TimeWarrior supports tracking multiple tasks simultaneously. The visual highlighting in taskwarrior-tui will show all currently tracked tasks.

### Manual TimeWarrior Management
You can still use TimeWarrior commands directly:
- `timew start project:work meeting` - Start manual tracking
- `timew stop` - Stop current tracking
- `timew summary` - View time summary

The taskwarrior-tui integration will automatically reflect these changes.

### Custom Tag Prefixes
Use tag prefixes to organize TimeWarrior entries:
```bash
uda.timewarrior.tag.prefix=tui_
```
This will prefix all auto-generated tags with "tui_".

## Performance Optimizations

The TimeWarrior integration includes several performance optimizations:

- **Intelligent Caching**: Tracking status is cached for 5 seconds to reduce system calls
- **Batch Queries**: All tracked task UUIDs are retrieved in a single TimeWarrior query
- **Automatic Cache Expiration**: Cache automatically refreshes when stale
- **Error Handling**: Graceful fallback when TimeWarrior is unavailable

These optimizations ensure that the integration remains responsive even with large task lists.