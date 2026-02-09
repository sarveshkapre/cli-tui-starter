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
- Config file defaults for `demo` options
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
cli-tui-starter themes
cli-tui-starter keys
```

## Config file
Default config path:
- `$XDG_CONFIG_HOME/cli-tui-starter/config.toml` (when `XDG_CONFIG_HOME` is set)
- `~/.config/cli-tui-starter/config.toml` (fallback)

Schema:
```toml
[demo]
theme = "aurora"         # aurora | mono | solar
no_color = false
high_contrast = false
reduced_motion = false
```

Precedence:
1. CLI flags
2. Config file values
3. Built-in defaults and `NO_COLOR`/`CLICOLOR=0`/`TERM=dumb`

## Docs
- `docs/AGENTS.md`
- `docs/PLAN.md`
- `docs/PROJECT.md`

## Docker
Not applicable. This is a local terminal UI binary.
