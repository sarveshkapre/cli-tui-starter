# PROJECT_MEMORY

## 2026-02-09 - Deterministic gitleaks CI scan on push events
- Decision: Configure the `gitleaks` CI job to checkout full history (`fetch-depth: 0`) and rely on the action default invocation.
- Why: The previous shallow checkout intermittently broke push-range scanning (`fatal: ambiguous argument <base>^..<head>`), creating flaky CI failures.
- Evidence:
  - Failed runs: `21557276579`, `21557279815`, `21557375125`
  - Fixed run after change: `21808825344` (`gitleaks` job passed)
  - Files: `.github/workflows/ci.yml`
- Commit: `01340f4d1bb6e6cea0e44eb1ed3689bf3e376509`
- Confidence: High
- Trust label: Verified from GitHub Actions logs and run outcomes
- Follow-ups:
  - Monitor next 5 push runs for recurrence of git-range scan errors.

## 2026-02-09 - Config file defaults for demo runtime behavior
- Decision: Add `config.toml` support for demo defaults (`theme`, `no_color`, `high_contrast`, `reduced_motion`) with explicit precedence rules.
- Why: This was the highest-impact roadmap item for product usability, allowing persistent defaults without requiring long CLI invocations.
- Evidence:
  - Files: `src/config.rs`, `src/cli.rs`, `src/main.rs`, `src/app.rs`, `tests/smoke.rs`, `README.md`
  - Commands: `cargo test`, `make check`
- Commit: `11513356a9215150c372a851014eb84d847090f1`
- Confidence: High
- Trust label: Verified by automated tests and local smoke commands
- Follow-ups:
  - Add keybinding customization to the same config file format.
