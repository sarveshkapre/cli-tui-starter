# CLI TUI Starter

Beautiful minimal TUI template with commands, themes, accessibility toggles, and config defaults.

```
CLI TUI Starter  /  demo
Theme: aurora | High contrast: off | No color: off | Reduced motion: off

Commands
- cli-tui-starter demo --theme aurora
- cli-tui-starter themes
- cli-tui-starter keys
```

## Features
- Commands: `demo`, `themes`, `keys`
- Themes with accessible high-contrast and no-color modes
- Reduced-motion toggle
- Config file defaults for `demo` options and key bindings
- Minimal, readable architecture

## Quickstart
```bash
make setup
make dev
```

## CLI usage
```bash
cli-tui-starter demo --theme aurora
cli-tui-starter demo --theme mono --high-contrast
cli-tui-starter demo --config ~/.config/cli-tui-starter/config.toml --color --motion
cli-tui-starter demo --no-tty
cli-tui-starter demo --no-tty --width 100 --height 28
cli-tui-starter themes
cli-tui-starter keys
cli-tui-starter themes --format json
cli-tui-starter keys --format json
cli-tui-starter keys --config ~/.config/cli-tui-starter/config.toml
```

## Config file
Default config path:
- `$XDG_CONFIG_HOME/cli-tui-starter/config.toml` (when `XDG_CONFIG_HOME` is set)
- `~/.config/cli-tui-starter/config.toml` (fallback)

Generate a starter config:
```bash
cli-tui-starter config init
# or print to stdout:
cli-tui-starter config init --stdout
```

Validate config:
```bash
cli-tui-starter config validate
cli-tui-starter config validate --format json
```

Schema:
```toml
[demo]
theme = "aurora"         # aurora | mono | solar
no_color = false
high_contrast = false
reduced_motion = false

[keys]
cycle_theme = "t"
toggle_high_contrast = "h"
toggle_color = "c"
toggle_reduced_motion = "r"
toggle_help = "?"
quit = ["q", "esc"]      # esc and ctrl+c always quit even if not listed
```

Precedence:
1. CLI flags
2. Config file values
3. Built-in defaults and `NO_COLOR`/`CLICOLOR=0`/`TERM=dumb`

## Docs
- `docs/AGENTS.md`
- `docs/PLAN.md`
- `docs/PROJECT.md`

## Windows notes
- Prefer Windows Terminal (or VS Code's integrated terminal). Older consoles can behave differently with alternate-screen TUIs.
- If some key combos do not arrive, check your terminal's own keyboard shortcuts first.
- Use `cli-tui-starter demo --no-tty` to verify rendering without requiring an interactive TTY.
- If colors look wrong, try `--no-color` or set `NO_COLOR=1`.

## Docker
Not applicable. This is a local terminal UI binary.
