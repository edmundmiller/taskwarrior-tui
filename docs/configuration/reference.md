# Configuration Reference

This is a comprehensive reference of all configuration options available in `taskwarrior-tui`.

## Key Configuration Reference

| Configuration Key | Default | Description |
|-------------------|---------|-------------|
| `uda.taskwarrior-tui.keyconfig.quit` | `q` | Exit the application |
| `uda.taskwarrior-tui.keyconfig.refresh` | `r` | Refresh the task list |
| `uda.taskwarrior-tui.keyconfig.go-to-bottom` | `G` | Jump to the bottom of the list |
| `uda.taskwarrior-tui.keyconfig.go-to-top` | `g` | Jump to the top of the list |
| `uda.taskwarrior-tui.keyconfig.down` | `j` | Move down in the list |
| `uda.taskwarrior-tui.keyconfig.up` | `k` | Move up in the list |
| `uda.taskwarrior-tui.keyconfig.page-down` | `J` | Move down one page |
| `uda.taskwarrior-tui.keyconfig.page-up` | `K` | Move up one page |
| `uda.taskwarrior-tui.keyconfig.next-tab` | `]` | Switch to the next tab |
| `uda.taskwarrior-tui.keyconfig.previous-tab` | `[` | Switch to the previous tab |
| `uda.taskwarrior-tui.keyconfig.delete` | `x` | Delete the selected task |
| `uda.taskwarrior-tui.keyconfig.done` | `d` | Mark the selected task as done |
| `uda.taskwarrior-tui.keyconfig.start-stop` | `s` | Start or stop the selected task |
| `uda.taskwarrior-tui.keyconfig.quick-tag` | `t` | Add/remove quick tag to/from selected task |
| `uda.taskwarrior-tui.keyconfig.undo` | `u` | Undo the last action |
| `uda.taskwarrior-tui.keyconfig.edit` | `e` | Edit the selected task |
| `uda.taskwarrior-tui.keyconfig.duplicate` | `y` | Duplicate the selected task |
| `uda.taskwarrior-tui.keyconfig.modify` | `m` | Modify the selected task |
| `uda.taskwarrior-tui.keyconfig.add` | `a` | Add a new task |
| `uda.taskwarrior-tui.keyconfig.annotate` | `A` | Add an annotation to the selected task |
| `uda.taskwarrior-tui.keyconfig.log` | `l` | Log time against the selected task |
| `uda.taskwarrior-tui.keyconfig.select` | `v` | Select/deselect the current task |
| `uda.taskwarrior-tui.keyconfig.select-all` | `V` | Select/deselect all tasks |
| `uda.taskwarrior-tui.keyconfig.priority-h` | `H` | Set task priority to High |
| `uda.taskwarrior-tui.keyconfig.priority-m` | `M` | Set task priority to Medium |
| `uda.taskwarrior-tui.keyconfig.priority-l` | `L` | Set task priority to Low |
| `uda.taskwarrior-tui.keyconfig.priority-n` | `N` | Remove task priority |
| `uda.taskwarrior-tui.keyconfig.help` | `?` | Show help screen |
| `uda.taskwarrior-tui.keyconfig.shell` | `!` | Open shell command |
| `uda.taskwarrior-tui.keyconfig.filter` | `/` | Enter filter mode |
| `uda.taskwarrior-tui.keyconfig.zoom` | `z` | Zoom into selected task |
| `uda.taskwarrior-tui.keyconfig.context-menu` | `c` | Open context menu |
| `uda.taskwarrior-tui.keyconfig.shortcut0` | `0` | Execute user-defined shortcut 0 |
| `uda.taskwarrior-tui.keyconfig.shortcut1` | `1` | Execute user-defined shortcut 1 |
| `uda.taskwarrior-tui.keyconfig.shortcut2` | `2` | Execute user-defined shortcut 2 |
| `uda.taskwarrior-tui.keyconfig.shortcut3` | `3` | Execute user-defined shortcut 3 |
| `uda.taskwarrior-tui.keyconfig.shortcut4` | `4` | Execute user-defined shortcut 4 |
| `uda.taskwarrior-tui.keyconfig.shortcut5` | `5` | Execute user-defined shortcut 5 |
| `uda.taskwarrior-tui.keyconfig.shortcut6` | `6` | Execute user-defined shortcut 6 |
| `uda.taskwarrior-tui.keyconfig.shortcut7` | `7` | Execute user-defined shortcut 7 |
| `uda.taskwarrior-tui.keyconfig.shortcut8` | `8` | Execute user-defined shortcut 8 |
| `uda.taskwarrior-tui.keyconfig.shortcut9` | `9` | Execute user-defined shortcut 9 |

## Visual Configuration Reference

| Configuration Key | Default | Description |
|-------------------|---------|-------------|
| `uda.taskwarrior-tui.selection.indicator` | `•` | Character used to indicate selected tasks |
| `uda.taskwarrior-tui.selection.bold` | `yes` | Apply bold formatting to selected tasks |
| `uda.taskwarrior-tui.selection.italic` | `no` | Apply italic formatting to selected tasks |
| `uda.taskwarrior-tui.selection.dim` | `no` | Apply dim formatting to selected tasks |
| `uda.taskwarrior-tui.selection.blink` | `no` | Apply blinking formatting to selected tasks |
| `uda.taskwarrior-tui.selection.reverse` | `no` | Apply reverse formatting to selected tasks |
| `uda.taskwarrior-tui.mark.indicator` | `✔` | Character used for marked tasks |
| `uda.taskwarrior-tui.unmark.indicator` | ` ` | Character used for unmarked tasks |
| `uda.taskwarrior-tui.mark-selection.indicator` | `⦿` | Character used for selected marked tasks |
| `uda.taskwarrior-tui.unmark-selection.indicator` | `⦾` | Character used for selected unmarked tasks |
| `uda.taskwarrior-tui.scrollbar.indicator` | `█` | Character used for scrollbar |
| `uda.taskwarrior-tui.scrollbar.area` | `║` | Character used for scrollbar area |

## Behavior Configuration Reference

| Configuration Key | Default | Description |
|-------------------|---------|-------------|
| `uda.taskwarrior-tui.calendar.months-per-row` | `4` | Number of months displayed per row in calendar view |
| `uda.taskwarrior-tui.task-report.show-info` | `true` | Show task information panel |
| `uda.taskwarrior-tui.task-report.looping` | `true` | Enable looping navigation in task lists |
| `uda.taskwarrior-tui.task-report.jump-on-task-add` | `true` | Jump to newly added tasks |
| `uda.taskwarrior-tui.task-report.prompt-on-undo` | `false` | Prompt before undoing actions |
| `uda.taskwarrior-tui.task-report.prompt-on-delete` | `false` | Prompt before deleting tasks |
| `uda.taskwarrior-tui.task-report.prompt-on-done` | `false` | Prompt before marking tasks as done |
| `uda.taskwarrior-tui.task-report.auto-insert-double-quotes-on-add` | `true` | Automatically add quotes when adding tasks |
| `uda.taskwarrior-tui.task-report.auto-insert-double-quotes-on-annotate` | `true` | Automatically add quotes when annotating |
| `uda.taskwarrior-tui.task-report.auto-insert-double-quotes-on-log` | `true` | Automatically add quotes when logging |
| `uda.taskwarrior-tui.task-report.reset-filter-on-esc` | `true` | Reset filter when pressing escape |
| `uda.taskwarrior-tui.task-report.task-detail-prefetch` | `10` | Number of tasks to prefetch for details |
| `uda.taskwarrior-tui.task-report.use-all-tasks-for-completion` | `false` | Use all tasks for tab completion |
| `uda.taskwarrior-tui.task-report.pre-fill-task-meta-data` | `false` | Pre-fill task metadata in forms |
| `uda.taskwarrior-tui.task-report.date-time-vague-more-precise` | `false` | Show more precise datetime information |
| `uda.taskwarrior-tui.task-report.duration-human-readable` | `true` | Display durations in human-readable format |
| `uda.taskwarrior-tui.context-menu.select-on-move` | `false` | Automatically select items when moving in context menu |
| `uda.taskwarrior-tui.tabs.change-focus-rotate` | `false` | Rotate focus when changing tabs |
| `uda.taskwarrior-tui.tick-rate` | `250` | UI refresh rate in milliseconds |
| `uda.taskwarrior-tui.quick-tag.name` | `next` | Tag name used for quick tagging |

## Style Configuration Reference

| Configuration Key | Default | Description |
|-------------------|---------|-------------|
| `uda.taskwarrior-tui.style.report.selection` | `` | Style for selected items in reports |
| `uda.taskwarrior-tui.style.context.active` | `black on rgb444` | Style for active context |
| `uda.taskwarrior-tui.style.calendar.title` | `black on rgb444` | Style for calendar title |
| `uda.taskwarrior-tui.style.calendar.today` | `bold` | Style for today's date in calendar |
| `uda.taskwarrior-tui.style.navbar` | `reverse` | Style for navigation bar |
| `uda.taskwarrior-tui.style.command` | `reverse` | Style for command line |
| `uda.taskwarrior-tui.style.report.scrollbar` | `black` | Style for report scrollbar |
| `uda.taskwarrior-tui.style.report.scrollbar.area` | `white` | Style for scrollbar area |
| `uda.taskwarrior-tui.style.report.completion-pane` | `black on rgb(223,223,223)` | Style for completion pane |
| `uda.taskwarrior-tui.style.report.completion-pane-highlight` | `black on rgb(223,223,223)` | Style for highlighted completion pane items |

## Backend Configuration Reference

| Configuration Key | Default | Description |
|-------------------|---------|-------------|
| `uda.taskwarrior-tui.backend` | `taskchampion` | Backend to use for task management |
| `uda.taskwarrior-tui.taskchampion.data-dir` | `` | Data directory for TaskChampion backend |
| `uda.taskwarrior-tui.taskchampion.server-config` | `` | Server configuration for TaskChampion backend |
| `uda.taskwarrior-tui.background_process` | `` | Background process to run periodically |
| `uda.taskwarrior-tui.background_process_period` | `60` | Background process period in seconds |

## Color Configuration Reference

| Configuration Key | Description |
|-------------------|-------------|
| `color.deleted` | Color for deleted tasks |
| `color.completed` | Color for completed tasks |
| `color.active` | Color for active tasks |
| `color.overdue` | Color for overdue tasks |
| `color.scheduled` | Color for scheduled tasks |
| `color.due.today` | Color for tasks due today |
| `color.due` | Color for tasks with due dates |
| `color.blocked` | Color for blocked tasks |
| `color.blocking` | Color for tasks that are blocking other tasks |
| `color.recurring` | Color for recurring tasks |
| `color.tagged` | Color for tagged tasks |
| `color.calendar.due.today` | Color for tasks due today in calendar view |
| `color.calendar.overdue` | Color for overdue tasks in calendar view |
| `color.calendar.weekend` | Color for weekend dates in calendar view |
| `color.calendar.holiday` | Color for holiday dates in calendar view |

## Shortcut Configuration Reference

| Configuration Key | Description |
|-------------------|-------------|
| `uda.taskwarrior-tui.shortcuts.0` | Path to script for shortcut 0 |
| `uda.taskwarrior-tui.shortcuts.1` | Path to script for shortcut 1 |
| `uda.taskwarrior-tui.shortcuts.2` | Path to script for shortcut 2 |
| `uda.taskwarrior-tui.shortcuts.3` | Path to script for shortcut 3 |
| `uda.taskwarrior-tui.shortcuts.4` | Path to script for shortcut 4 |
| `uda.taskwarrior-tui.shortcuts.5` | Path to script for shortcut 5 |
| `uda.taskwarrior-tui.shortcuts.6` | Path to script for shortcut 6 |
| `uda.taskwarrior-tui.shortcuts.7` | Path to script for shortcut 7 |
| `uda.taskwarrior-tui.shortcuts.8` | Path to script for shortcut 8 |
| `uda.taskwarrior-tui.shortcuts.9` | Path to script for shortcut 9 |