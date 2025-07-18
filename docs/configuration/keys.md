# Key configuration

Configure `taskwarrior-tui` using `~/.taskrc`:

`taskwarrior-tui` reads values from your `taskwarrior`'s `taskrc` file (default: `~/.taskrc`).

```plaintext
# Navigation keys
uda.taskwarrior-tui.keyconfig.quit=q
uda.taskwarrior-tui.keyconfig.refresh=r
uda.taskwarrior-tui.keyconfig.go-to-bottom=G
uda.taskwarrior-tui.keyconfig.go-to-top=g
uda.taskwarrior-tui.keyconfig.down=j
uda.taskwarrior-tui.keyconfig.up=k
uda.taskwarrior-tui.keyconfig.page-down=J
uda.taskwarrior-tui.keyconfig.page-up=K
uda.taskwarrior-tui.keyconfig.next-tab=]
uda.taskwarrior-tui.keyconfig.previous-tab=[

# Task manipulation keys
uda.taskwarrior-tui.keyconfig.delete=x
uda.taskwarrior-tui.keyconfig.done=d
uda.taskwarrior-tui.keyconfig.start-stop=s
uda.taskwarrior-tui.keyconfig.quick-tag=t
uda.taskwarrior-tui.keyconfig.undo=u
uda.taskwarrior-tui.keyconfig.edit=e
uda.taskwarrior-tui.keyconfig.duplicate=y
uda.taskwarrior-tui.keyconfig.modify=m
uda.taskwarrior-tui.keyconfig.add=a
uda.taskwarrior-tui.keyconfig.annotate=A
uda.taskwarrior-tui.keyconfig.log=l

# Selection keys
uda.taskwarrior-tui.keyconfig.select=v
uda.taskwarrior-tui.keyconfig.select-all=V

# Priority keys
uda.taskwarrior-tui.keyconfig.priority-h=H
uda.taskwarrior-tui.keyconfig.priority-m=M
uda.taskwarrior-tui.keyconfig.priority-l=L
uda.taskwarrior-tui.keyconfig.priority-n=N

# Interface keys
uda.taskwarrior-tui.keyconfig.help=?
uda.taskwarrior-tui.keyconfig.shell=!
uda.taskwarrior-tui.keyconfig.filter=/
uda.taskwarrior-tui.keyconfig.zoom=z
uda.taskwarrior-tui.keyconfig.context-menu=c

# Shortcut keys
uda.taskwarrior-tui.keyconfig.shortcut0=0
uda.taskwarrior-tui.keyconfig.shortcut1=1
uda.taskwarrior-tui.keyconfig.shortcut2=2
uda.taskwarrior-tui.keyconfig.shortcut3=3
uda.taskwarrior-tui.keyconfig.shortcut4=4
uda.taskwarrior-tui.keyconfig.shortcut5=5
uda.taskwarrior-tui.keyconfig.shortcut6=6
uda.taskwarrior-tui.keyconfig.shortcut7=7
uda.taskwarrior-tui.keyconfig.shortcut8=8
uda.taskwarrior-tui.keyconfig.shortcut9=9
```

## Key Configuration Options

### Navigation Keys

- `quit` - Exit the application (default: q)
- `refresh` - Refresh the task list (default: r)
- `go-to-bottom` - Jump to the bottom of the list (default: G)
- `go-to-top` - Jump to the top of the list (default: g)
- `down` - Move down in the list (default: j)
- `up` - Move up in the list (default: k)
- `page-down` - Move down one page (default: J)
- `page-up` - Move up one page (default: K)
- `next-tab` - Switch to the next tab (default: ])
- `previous-tab` - Switch to the previous tab (default: [)

### Task Manipulation Keys

- `delete` - Delete the selected task (default: x)
- `done` - Mark the selected task as done (default: d)
- `start-stop` - Start or stop the selected task (default: s)
- `quick-tag` - Add/remove quick tag to/from selected task (default: t)
- `undo` - Undo the last action (default: u)
- `edit` - Edit the selected task (default: e)
- `duplicate` - Duplicate the selected task (default: y)
- `modify` - Modify the selected task (default: m)
- `add` - Add a new task (default: a)
- `annotate` - Add an annotation to the selected task (default: A)
- `log` - Log time against the selected task (default: l)

### Selection Keys

- `select` - Select/deselect the current task (default: v)
- `select-all` - Select/deselect all tasks (default: V)

### Priority Keys

- `priority-h` - Set task priority to High (default: H)
- `priority-m` - Set task priority to Medium (default: M)
- `priority-l` - Set task priority to Low (default: L)
- `priority-n` - Remove task priority (default: N)

### Interface Keys

- `help` - Show help screen (default: ?)
- `shell` - Open shell command (default: !)
- `filter` - Enter filter mode (default: /)
- `zoom` - Zoom into selected task (default: z)
- `context-menu` - Open context menu (default: c)

### Shortcut Keys

- `shortcut0` through `shortcut9` - Execute user-defined shortcuts (default: 0-9)

These shortcuts can be mapped to custom commands using the `uda.taskwarrior-tui.shortcuts.N` configuration options described in the advanced configuration section.
