# INCIDENTS

## 2026-02-11 - Local quality-gate sequencing mistake during automation run
- Status: Resolved
- Impact:
  - Local `make check` failed once with a formatting diff before final validation.
  - No broken commit was pushed; failure was caught and corrected in-session.
- Root cause:
  - `make check` was started while formatting changes were still being applied, creating an avoidable transient failure.
- Detection evidence:
  - `make check` output showed `cargo fmt --all -- --check` diffs in `src/app.rs` and `src/ui.rs`.
- Fix:
  - Re-run validation in strict sequence: `cargo fmt` followed by `make check`.
  - Continue only after full pass.
- Prevention rules:
  - Never run repo quality gate commands concurrently with formatter runs.
  - Treat any formatting drift as a stop-the-line signal before further validation.

## 2026-02-09 - Windows CI failures after adding golden snapshot tests
- Status: Resolved
- Impact:
  - `windows-test` failed on `main` for multiple pushes, blocking the "CI green" invariant.
  - Affected runs: `21838593394`, `21838596347`, `21838694459`, `21838708175`.
- Root cause:
  - Snapshot tests compared captured stdout normalized to `\\n` against `include_str!()` expected text files that were checked out with CRLF (`\\r\\n`) on Windows.
  - Additionally, an earlier push occurred before running `make check`, so `cargo fmt -- --check` failed in CI.
- Detection evidence:
  - `windows-test` logs showed `assertion left == right` where the only diff was `\\n` vs `\\r\\n`.
  - `check` job logs showed `cargo fmt -- --check` diffs in `src/ui.rs`.
- Fix:
  - Normalize newlines for both actual stdout and expected snapshot text in `tests/demo_no_tty_snapshots.rs`.
  - Run `cargo fmt` and re-run `make check` before pushing follow-up commits.
  - Verified with successful run: `21838748447` (all jobs passed).
- Prevention rules:
  - Any golden/snapshot text comparison must normalize newlines (at minimum `\\r\\n` -> `\\n`) to be Windows-safe.
  - Always run `make check` locally before pushing to `main` to prevent avoidable CI churn.

## 2026-02-09 - Flaky `gitleaks` job failures on `main` push CI
- Status: Resolved
- Impact:
  - CI showed false failures for healthy commits, blocking trust in release readiness.
  - Affected runs included `21557375125`, `21557279815`, and `21557276579`.
- Root cause:
  - Workflow used shallow checkout (`fetch-depth: 1`).
  - `gitleaks-action@v2` attempted scanning a commit range requiring the parent commit SHA, which was missing in shallow history for some push contexts.
- Detection evidence:
  - Error snippet from logs: `fatal: ambiguous argument '<base>^..<head>'`.
  - GitHub Actions logs showed `failed to scan Git repository` with non-zero exit.
- Fix:
  - Update `gitleaks` job checkout to `fetch-depth: 0` in `.github/workflows/ci.yml`.
  - Remove unsupported `args` input from `gitleaks-action@v2` invocation.
- Prevention rules:
  - Any workflow step that computes commit ranges must fetch enough history to resolve parent SHAs.
  - Treat CI tool warnings about unsupported action inputs as reliability risks and clean them up promptly.
  - Keep a recent successful run reference for every workflow change to validate runtime behavior.

## 2026-02-09 - Quality gate break from newly published RustSec advisory
- Status: Resolved
- Impact:
  - `make check` and CI `check` job failed on `cargo audit` despite no source-code regressions.
  - Affected run: `21808825344` (`check` job failed at Quality gate).
- Root cause:
  - New advisory `RUSTSEC-2026-0009` flagged transitive dependency `time 0.3.46`.
  - Lockfile had not yet advanced to patched `time >= 0.3.47`.
- Detection evidence:
  - `cargo audit` output reported medium severity DoS advisory with dependency tree via `ratatui`.
- Fix:
  - Run `cargo update -p time --precise 0.3.47` and commit updated `Cargo.lock`.
  - Re-run `make check` locally to verify audit, tests, lint, build all pass.
- Prevention rules:
  - Treat audit failures as release blockers and remediate in the same session.
  - Re-run full `make check` after any CI/workflow fix commit to catch unrelated ecosystem drift.
  - Keep dependency lockfile updates scoped and evidence-backed (command + resulting version diff).

### 2026-02-12T20:01:29Z | Codex execution failure
- Date: 2026-02-12T20:01:29Z
- Trigger: Codex execution failure
- Impact: Repo session did not complete cleanly
- Root Cause: codex exec returned a non-zero status
- Fix: Captured failure logs and kept repository in a recoverable state
- Prevention Rule: Re-run with same pass context and inspect pass log before retrying
- Evidence: pass_log=logs/20260212-101456-cli-tui-starter-cycle-2.log
- Commit: pending
- Confidence: medium

### 2026-02-12T20:04:57Z | Codex execution failure
- Date: 2026-02-12T20:04:57Z
- Trigger: Codex execution failure
- Impact: Repo session did not complete cleanly
- Root Cause: codex exec returned a non-zero status
- Fix: Captured failure logs and kept repository in a recoverable state
- Prevention Rule: Re-run with same pass context and inspect pass log before retrying
- Evidence: pass_log=logs/20260212-101456-cli-tui-starter-cycle-3.log
- Commit: pending
- Confidence: medium

### 2026-02-12T20:08:24Z | Codex execution failure
- Date: 2026-02-12T20:08:24Z
- Trigger: Codex execution failure
- Impact: Repo session did not complete cleanly
- Root Cause: codex exec returned a non-zero status
- Fix: Captured failure logs and kept repository in a recoverable state
- Prevention Rule: Re-run with same pass context and inspect pass log before retrying
- Evidence: pass_log=logs/20260212-101456-cli-tui-starter-cycle-4.log
- Commit: pending
- Confidence: medium

### 2026-02-12T20:11:54Z | Codex execution failure
- Date: 2026-02-12T20:11:54Z
- Trigger: Codex execution failure
- Impact: Repo session did not complete cleanly
- Root Cause: codex exec returned a non-zero status
- Fix: Captured failure logs and kept repository in a recoverable state
- Prevention Rule: Re-run with same pass context and inspect pass log before retrying
- Evidence: pass_log=logs/20260212-101456-cli-tui-starter-cycle-5.log
- Commit: pending
- Confidence: medium

### 2026-02-12T20:15:26Z | Codex execution failure
- Date: 2026-02-12T20:15:26Z
- Trigger: Codex execution failure
- Impact: Repo session did not complete cleanly
- Root Cause: codex exec returned a non-zero status
- Fix: Captured failure logs and kept repository in a recoverable state
- Prevention Rule: Re-run with same pass context and inspect pass log before retrying
- Evidence: pass_log=logs/20260212-101456-cli-tui-starter-cycle-6.log
- Commit: pending
- Confidence: medium

### 2026-02-12T20:18:58Z | Codex execution failure
- Date: 2026-02-12T20:18:58Z
- Trigger: Codex execution failure
- Impact: Repo session did not complete cleanly
- Root Cause: codex exec returned a non-zero status
- Fix: Captured failure logs and kept repository in a recoverable state
- Prevention Rule: Re-run with same pass context and inspect pass log before retrying
- Evidence: pass_log=logs/20260212-101456-cli-tui-starter-cycle-7.log
- Commit: pending
- Confidence: medium

### 2026-02-12T20:22:21Z | Codex execution failure
- Date: 2026-02-12T20:22:21Z
- Trigger: Codex execution failure
- Impact: Repo session did not complete cleanly
- Root Cause: codex exec returned a non-zero status
- Fix: Captured failure logs and kept repository in a recoverable state
- Prevention Rule: Re-run with same pass context and inspect pass log before retrying
- Evidence: pass_log=logs/20260212-101456-cli-tui-starter-cycle-8.log
- Commit: pending
- Confidence: medium

### 2026-02-12T20:25:51Z | Codex execution failure
- Date: 2026-02-12T20:25:51Z
- Trigger: Codex execution failure
- Impact: Repo session did not complete cleanly
- Root Cause: codex exec returned a non-zero status
- Fix: Captured failure logs and kept repository in a recoverable state
- Prevention Rule: Re-run with same pass context and inspect pass log before retrying
- Evidence: pass_log=logs/20260212-101456-cli-tui-starter-cycle-9.log
- Commit: pending
- Confidence: medium

### 2026-02-12T20:29:30Z | Codex execution failure
- Date: 2026-02-12T20:29:30Z
- Trigger: Codex execution failure
- Impact: Repo session did not complete cleanly
- Root Cause: codex exec returned a non-zero status
- Fix: Captured failure logs and kept repository in a recoverable state
- Prevention Rule: Re-run with same pass context and inspect pass log before retrying
- Evidence: pass_log=logs/20260212-101456-cli-tui-starter-cycle-10.log
- Commit: pending
- Confidence: medium

### 2026-02-12T20:32:59Z | Codex execution failure
- Date: 2026-02-12T20:32:59Z
- Trigger: Codex execution failure
- Impact: Repo session did not complete cleanly
- Root Cause: codex exec returned a non-zero status
- Fix: Captured failure logs and kept repository in a recoverable state
- Prevention Rule: Re-run with same pass context and inspect pass log before retrying
- Evidence: pass_log=logs/20260212-101456-cli-tui-starter-cycle-11.log
- Commit: pending
- Confidence: medium

### 2026-02-12T20:36:28Z | Codex execution failure
- Date: 2026-02-12T20:36:28Z
- Trigger: Codex execution failure
- Impact: Repo session did not complete cleanly
- Root Cause: codex exec returned a non-zero status
- Fix: Captured failure logs and kept repository in a recoverable state
- Prevention Rule: Re-run with same pass context and inspect pass log before retrying
- Evidence: pass_log=logs/20260212-101456-cli-tui-starter-cycle-12.log
- Commit: pending
- Confidence: medium
