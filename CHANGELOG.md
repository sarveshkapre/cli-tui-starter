# CHANGELOG

## 0.1.2 - 2026-02-01
- Upgrade `ratatui` and `crossterm` to clear `cargo audit` warnings and modernize the stack.
- Honor `NO_COLOR`, `CLICOLOR=0`, and `TERM=dumb` to automatically disable color output.

## 0.1.1 - 2026-02-01
- Make the demo UI responsive on narrow terminals (single-column layout).
- Make the help popup size itself safely for small terminals.
- Improve header status labels and reduce motion/no-color animation noise.

## 0.1.0 - 2026-02-01
- Initial scaffold with CLI commands, themes, and accessibility toggles.
