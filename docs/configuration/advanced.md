# Advanced configuration

`taskwarrior-tui` parses the output of `task show` to get configuration data. This allows
`taskwarrior-tui` to use the same defaults as `taskwarrior` and configure additional options as
required.

## `taskrc` config file options:

Other `taskwarrior-tui` configuration options are possible using the user defined attribute feature
of `taskwarrior`. All `taskwarrior-tui` specific configuration options will begin with
`uda.taskwarrior-tui.`. The following is a full list of all the options available and their default
values implemented by `taskwarrior-tui` if not defined in your `taskrc` file.

```plaintext
# Selection and visual indicators
uda.taskwarrior-tui.selection.indicator=•
uda.taskwarrior-tui.selection.bold=yes
uda.taskwarrior-tui.selection.italic=no
uda.taskwarrior-tui.selection.dim=no
uda.taskwarrior-tui.selection.blink=no
uda.taskwarrior-tui.selection.reverse=no
uda.taskwarrior-tui.mark.indicator=✔
uda.taskwarrior-tui.unmark.indicator=
uda.taskwarrior-tui.mark-selection.indicator=⦿
uda.taskwarrior-tui.unmark-selection.indicator=⦾
uda.taskwarrior-tui.scrollbar.indicator=█
uda.taskwarrior-tui.scrollbar.area=║

# Calendar settings
uda.taskwarrior-tui.calendar.months-per-row=4

# Task report behavior
uda.taskwarrior-tui.task-report.show-info=true
uda.taskwarrior-tui.task-report.looping=true
uda.taskwarrior-tui.task-report.jump-on-task-add=true
uda.taskwarrior-tui.task-report.prompt-on-undo=false
uda.taskwarrior-tui.task-report.prompt-on-delete=false
uda.taskwarrior-tui.task-report.prompt-on-done=false
uda.taskwarrior-tui.task-report.auto-insert-double-quotes-on-add=true
uda.taskwarrior-tui.task-report.auto-insert-double-quotes-on-annotate=true
uda.taskwarrior-tui.task-report.auto-insert-double-quotes-on-log=true
uda.taskwarrior-tui.task-report.reset-filter-on-esc=true
uda.taskwarrior-tui.task-report.task-detail-prefetch=10
uda.taskwarrior-tui.task-report.use-all-tasks-for-completion=false
uda.taskwarrior-tui.task-report.pre-fill-task-meta-data=false
uda.taskwarrior-tui.task-report.date-time-vague-more-precise=false
uda.taskwarrior-tui.task-report.duration-human-readable=true
uda.taskwarrior-tui.task-report.next.filter=$(task show report.next.filter)

# Interface behavior
uda.taskwarrior-tui.context-menu.select-on-move=false
uda.taskwarrior-tui.tabs.change-focus-rotate=false
uda.taskwarrior-tui.tick-rate=250

# Quick tag functionality
uda.taskwarrior-tui.quick-tag.name=next

# Backend configuration
uda.taskwarrior-tui.backend=taskchampion
uda.taskwarrior-tui.taskchampion.data-dir=
uda.taskwarrior-tui.taskchampion.server-config=

# Style settings
uda.taskwarrior-tui.style.report.selection=
uda.taskwarrior-tui.style.context.active=black on rgb444
uda.taskwarrior-tui.style.calendar.title=black on rgb444
uda.taskwarrior-tui.style.calendar.today=bold
uda.taskwarrior-tui.style.navbar=reverse
uda.taskwarrior-tui.style.command=reverse
uda.taskwarrior-tui.style.report.scrollbar=black
uda.taskwarrior-tui.style.report.scrollbar.area=white
uda.taskwarrior-tui.style.report.completion-pane=black on rgb(223,223,223)
uda.taskwarrior-tui.style.report.completion-pane-highlight=black on rgb(223,223,223)
```

## Configuration Options Explained

### Selection and Visual Indicators

- `uda.taskwarrior-tui.selection.indicator` - Character used to indicate selected tasks (default: •)
- `uda.taskwarrior-tui.selection.bold` - Apply bold formatting to selected tasks (default: yes)
- `uda.taskwarrior-tui.selection.italic` - Apply italic formatting to selected tasks (default: no)
- `uda.taskwarrior-tui.selection.dim` - Apply dim formatting to selected tasks (default: no)
- `uda.taskwarrior-tui.selection.blink` - Apply blinking formatting to selected tasks (default: no)
- `uda.taskwarrior-tui.selection.reverse` - Apply reverse formatting to selected tasks (default: no)
- `uda.taskwarrior-tui.mark.indicator` - Character used for marked tasks (default: ✔)
- `uda.taskwarrior-tui.unmark.indicator` - Character used for unmarked tasks (default: empty)
- `uda.taskwarrior-tui.mark-selection.indicator` - Character used for selected marked tasks (default: ⦿)
- `uda.taskwarrior-tui.unmark-selection.indicator` - Character used for selected unmarked tasks (default: ⦾)
- `uda.taskwarrior-tui.scrollbar.indicator` - Character used for scrollbar (default: █)
- `uda.taskwarrior-tui.scrollbar.area` - Character used for scrollbar area (default: ║)

### Calendar Settings

- `uda.taskwarrior-tui.calendar.months-per-row` - Number of months displayed per row in calendar view (default: 4)

### Task Report Behavior

- `uda.taskwarrior-tui.task-report.show-info` - Show task information panel (default: true)
- `uda.taskwarrior-tui.task-report.looping` - Enable looping navigation in task lists (default: true)
- `uda.taskwarrior-tui.task-report.jump-on-task-add` - Jump to newly added tasks (default: true)
- `uda.taskwarrior-tui.task-report.prompt-on-undo` - Prompt before undoing actions (default: false)
- `uda.taskwarrior-tui.task-report.prompt-on-delete` - Prompt before deleting tasks (default: false)
- `uda.taskwarrior-tui.task-report.prompt-on-done` - Prompt before marking tasks as done (default: false)
- `uda.taskwarrior-tui.task-report.auto-insert-double-quotes-on-add` - Automatically add quotes when adding tasks (default: true)
- `uda.taskwarrior-tui.task-report.auto-insert-double-quotes-on-annotate` - Automatically add quotes when annotating (default: true)
- `uda.taskwarrior-tui.task-report.auto-insert-double-quotes-on-log` - Automatically add quotes when logging (default: true)
- `uda.taskwarrior-tui.task-report.reset-filter-on-esc` - Reset filter when pressing escape (default: true)
- `uda.taskwarrior-tui.task-report.task-detail-prefetch` - Number of tasks to prefetch for details (default: 10)
- `uda.taskwarrior-tui.task-report.use-all-tasks-for-completion` - Use all tasks for tab completion (default: false)
- `uda.taskwarrior-tui.task-report.pre-fill-task-meta-data` - Pre-fill task metadata in forms (default: false)
- `uda.taskwarrior-tui.task-report.date-time-vague-more-precise` - Show more precise datetime information (default: false)
- `uda.taskwarrior-tui.task-report.duration-human-readable` - Display durations in human-readable format (default: true)
- `uda.taskwarrior-tui.task-report.next.filter` - Default filter for the next report (default: $(task show report.next.filter))

### Interface Behavior

- `uda.taskwarrior-tui.context-menu.select-on-move` - Automatically select items when moving in context menu (default: false)
- `uda.taskwarrior-tui.tabs.change-focus-rotate` - Rotate focus when changing tabs (default: false)
- `uda.taskwarrior-tui.tick-rate` - UI refresh rate in milliseconds (default: 250)

### Quick Tag Functionality

- `uda.taskwarrior-tui.quick-tag.name` - Tag name used for quick tagging (default: next)

### Backend Configuration

- `uda.taskwarrior-tui.backend` - Backend to use for task management (default: taskchampion or cli)
- `uda.taskwarrior-tui.taskchampion.data-dir` - Data directory for TaskChampion backend (optional)
- `uda.taskwarrior-tui.taskchampion.server-config` - Server configuration for TaskChampion backend (optional)

### Style Settings

- `uda.taskwarrior-tui.style.report.selection` - Style for selected items in reports (default: empty)
- `uda.taskwarrior-tui.style.context.active` - Style for active context (default: black on rgb444)
- `uda.taskwarrior-tui.style.calendar.title` - Style for calendar title (default: black on rgb444)
- `uda.taskwarrior-tui.style.calendar.today` - Style for today's date in calendar (default: bold)
- `uda.taskwarrior-tui.style.navbar` - Style for navigation bar (default: reverse)
- `uda.taskwarrior-tui.style.command` - Style for command line (default: reverse)
- `uda.taskwarrior-tui.style.report.scrollbar` - Style for report scrollbar (default: black)
- `uda.taskwarrior-tui.style.report.scrollbar.area` - Style for scrollbar area (default: white)
- `uda.taskwarrior-tui.style.report.completion-pane` - Style for completion pane (default: black on rgb(223,223,223))
- `uda.taskwarrior-tui.style.report.completion-pane-highlight` - Style for highlighted completion pane items (default: matches completion-pane)

The `uda.taskwarrior-tui.task-report.next.filter` variable defines the default view at program
startup. Set this to any preconfigured report (`task reports`), or create your own report in
taskwarrior and specify its name here.

## commandline options:

`-r`: specify a report to be shown, overrides `uda.taskwarrior-tui.task-report.next.filter` for this
instance

## Configure user defined shortcuts:

You can configure shortcuts to execute custom commands from your `taskwarrior`'s `taskrc` file
(default: `~/.taskrc`). You can do this by mapping a shortcut to an executable file:

```plaintext
uda.taskwarrior-tui.shortcuts.1=~/.config/taskwarrior-tui/shortcut-scripts/add-personal-tag.sh
uda.taskwarrior-tui.shortcuts.2=~/.config/taskwarrior-tui/shortcut-scripts/sync.sh
...
```

The executable file can be placed in any location.

To make a file executable:

1. Run `chmod +x /path/to/script` to modify the executable flag.
2. Add `#!/usr/bin/env bash`, `#!/usr/bin/env python` or whatever is appropriate for your script.

By default, keys `1`-`9` are available to run shortcuts.

When you hit the shortcut, the script will be executed with the `selected_tasks_uuid` as an
argument:

```bash
~/.config/taskwarrior-tui/shortcut-scripts/add-personal-tag.sh $selected_tasks_uuid
```

For example, you can add the `personal` tag to the currently selected task with the following script
in `~/.config/taskwarrior-tui/shortcut-scripts/add-personal-tag.sh` :

```plaintext
task rc.bulk=0 rc.confirmation=off rc.dependency.confirmation=off rc.recurrence.confirmation=off "$@" modify +personal
```

By default, shortcuts are linked to the `1-9` number row keys. They can be customized as any other
keys through `uda.taskwarrior-tui.keyconfig.shortcut1=<key>`. For example:

```plaintext
uda.taskwarrior-tui.keyconfig.shortcut1=n
```

You can set up shortcuts to run `task sync` or any custom bash script that you'd like.

## Configure one background task

You can configure one background task to run periodically:

```plaintext
uda.taskwarrior-tui.background_process=task sync
uda.taskwarrior-tui.background_process_period=60
```

This will run `task sync` every 60 seconds. If the `background_process` is an empty string
(default), then no process will be run. Only if the `background_process` is defined and if the
`background_process` runs successfully, it'll be run every `background_process_period` number of
seconds (default: 60 seconds). However, if it fails even once it won't be run again till
`taskwarrior-tui` is restarted.
