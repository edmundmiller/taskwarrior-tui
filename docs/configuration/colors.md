# Color configuration

`taskwarrior-tui` reads values from your `taskwarrior`'s `taskrc` file (default: `~/.taskrc`).

![](https://user-images.githubusercontent.com/1813121/96684390-bf173e80-1338-11eb-971c-ae64233d142e.png)

For example, `color.active` is used to style the active task.
If you would like to try it, open your `taskrc` file and change `color.active=white on blue`.

So `color.active` will take precedence over `color.overdue`. You can see what `color.active` is by running `task show color.active` in your favorite shell prompt.

The following color attributes are supported:

```plaintext
color.deleted
color.completed
color.active
color.overdue
color.scheduled
color.due.today
color.due
color.blocked
color.blocking
color.recurring
color.tagged
```

## Calendar Colors

`taskwarrior-tui` supports calendar-specific color settings that override task colors in the calendar view:

```plaintext
color.calendar.due.today
color.calendar.overdue
color.calendar.weekend
color.calendar.holiday
```

### Calendar Color Examples

To set calendar colors, add these to your `~/.taskrc` file:

```bash
# Highlight tasks due today in the calendar
color.calendar.due.today=color0 on color252

# Make overdue tasks stand out in red
color.calendar.overdue=color1 on color0

# Subtle styling for weekend dates
color.calendar.weekend=color8 on color0

# Special highlighting for holidays
color.calendar.holiday=rgb522 on rgb300
```

### Calendar Color Precedence

Calendar colors follow this precedence order:
1. `color.calendar.overdue` - applied to overdue tasks
2. `color.calendar.due.today` - applied to tasks due today
3. `color.calendar.weekend` - applied to weekend dates
4. `color.calendar.holiday` - applied to holiday dates (requires external calendar data)

### Default Calendar Colors

If no calendar-specific color is set, `taskwarrior-tui` provides these sensible defaults:

- **Due Today**: Black text on yellow background (highlights urgency)
- **Overdue**: White text on red background (clear warning)
- **Weekend**: Dark gray text (subtle, less prominent)
- **Holiday**: No default (requires user configuration)

If you don't want these defaults, you can override them by setting the corresponding `color.calendar.*` values in your `~/.taskrc` file. To disable a default color, you can set it to the same as your default text color.
