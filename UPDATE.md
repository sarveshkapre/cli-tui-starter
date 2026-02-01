# UPDATE

## Summary
- Shipped responsive demo layout for narrow terminals and made the help popup size itself safely.
- Improved header status labels and reduced animation noise in no-color/reduced-motion modes.
- Added root-level `CHANGELOG.md` and root-level `PLAN.md` for standard repo hygiene.
- Upgraded `ratatui`/`crossterm` so `cargo audit` is clean; honor `NO_COLOR`/`CLICOLOR=0`/`TERM=dumb`.
- Make `demo` fail fast when not running in a TTY (prevents CI hangs) and add a smoke test for it.

## Commands run
```bash
make check
```

## Notes
- Work is committed directly on `main` (no PR workflow).
