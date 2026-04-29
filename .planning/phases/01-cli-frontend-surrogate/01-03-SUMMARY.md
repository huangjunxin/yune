---
phase: 01-cli-frontend-surrogate
plan: 03
subsystem: cli-testing
tags: [rust, cargo, yune-cli, rime-api, abi, fixtures]

requires:
  - phase: 01-cli-frontend-surrogate
    provides: Plans 01-01 and 01-02 ABI-backed CLI frontend command and deterministic transcript foundations
provides:
  - Focused CLI integration coverage for explicit ABI frontend behavior
  - Owning module tests and librime comparison markers across CLI frontend modules
  - Final Phase 1 quality-gate verification for CLI frontend surrogate behavior
affects: [phase-2-native-frontend-validation, cli-frontend-surrogate, rime-abi-compatibility]

tech-stack:
  added: [yune-rime-api path dependency for yune-cli]
  patterns: [RimeApi function-table CLI tests, serialized process-wide ABI test guard, deterministic frontend transcript replay]

key-files:
  created:
    - crates/yune-cli/tests/frontend_surrogate.rs
  modified:
    - Cargo.lock
    - crates/yune-cli/Cargo.toml
    - crates/yune-cli/src/args.rs
    - crates/yune-cli/src/fixture.rs
    - crates/yune-cli/src/main.rs
    - crates/yune-cli/src/render.rs
    - crates/yune-cli/src/rime_frontend.rs
    - crates/yune-cli/src/transcript.rs

key-decisions:
  - "Kept run/check core-backed while adding separate frontend/frontend-check ABI paths."
  - "Kept unsafe RIME ABI handling localized to rime_frontend.rs and tests that directly follow frontend_client patterns."
  - "Marked native frontend validation as Phase 2 scope while using librime-visible ABI lifecycle and transcript behavior as Phase 1 comparison targets."

patterns-established:
  - "CLI frontend surrogate tests invoke the yune-cli binary and serialize RIME process-wide state with a test mutex."
  - "Frontend transcript fixtures compare deterministic JSON without temp paths, process IDs, timestamps, or pointer-like values."

requirements-completed: [CLI-01, CLI-02, CLI-03, CLI-04, CLI-05, QUAL-01, QUAL-02]

duration: 52min
completed: 2026-04-28
---

# Phase 1 Plan 03: CLI Frontend Surrogate Closure Summary

**ABI-backed yune-cli frontend coverage with deterministic per-key transcripts, fixture replay, and explicit librime ownership markers**

## Performance

- **Duration:** 52 min
- **Started:** 2026-04-28T20:00:00Z
- **Completed:** 2026-04-28T20:52:01Z
- **Tasks:** 3
- **Files modified:** 9

## Accomplishments

- Added `crates/yune-cli/tests/frontend_surrogate.rs` integration tests that exercise the CLI binary through explicit `frontend` and `frontend-check` commands with serialized RIME process-wide state and unique temp runtime directories.
- Added/tightened owning module tests for command parsing, ABI lifecycle/key mapping, deterministic transcript output, human rendering, and ABI-backed fixture replay.
- Added concise in-code ownership/comparison markers naming librime-visible behavior and keeping native frontend validation scoped to Phase 2.
- Ran focused and workspace quality gates, including yune-cli tests, frontend_client tests, workspace tests, formatting, and clippy with denied warnings.

## Task Commits

No commits were created per user instruction: "Do not commit changes."

## Files Created/Modified

- `crates/yune-cli/tests/frontend_surrogate.rs` - CLI binary integration coverage for explicit frontend runtime paths, per-key ABI transcript output, deterministic JSON, and fixture replay.
- `crates/yune-cli/src/args.rs` - Frontend command ownership marker plus additional parser coverage for required schema/sequence and preserved run/check behavior.
- `crates/yune-cli/src/rime_frontend.rs` - ABI lifecycle ownership marker plus cleanup/error-path and lifecycle tests.
- `crates/yune-cli/src/transcript.rs` - Deterministic transcript ownership marker and retained D-08 frontend JSON field coverage tests.
- `crates/yune-cli/src/render.rs` - Human-rendering ownership marker and plain-output tests for labels, candidates, and no control/environment artifacts.
- `crates/yune-cli/src/fixture.rs` - Fixture replay ownership marker and ABI replay/mismatch tests.
- `crates/yune-cli/src/main.rs` - Orchestration-only dispatch retained; frontend render helper is referenced to keep human renderer compiled without warnings.
- `crates/yune-cli/Cargo.toml` - Uses `yune-rime-api` dependency for ABI-backed frontend surrogate path.
- `Cargo.lock` - Records `yune-cli` dependency on `yune-rime-api`.

## Decisions Made

- Preserved the existing core-backed `run` and `check` behavior and kept ABI-backed behavior behind separate `frontend` and `frontend-check` commands.
- Kept unsafe ABI setup, pointer reads, and free-pairing ownership in `rime_frontend.rs`; integration tests use the public CLI binary plus the same RimeApi guard pattern as frontend_client.
- Documented librime as the external comparison oracle in owning modules while explicitly deferring native frontend validation to Phase 2.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Ensured render module remains warning-clean under clippy -D warnings**
- **Found during:** Task 3 (final quality gates)
- **Issue:** `render_frontend_human` and private helpers were only used by unit tests, causing dead-code warnings in the binary target that would fail `cargo clippy --workspace --all-targets -- -D warnings`.
- **Fix:** Kept `main.rs` orchestration-only while referencing `render::render_frontend_human(&output)` in the frontend dispatch path without changing JSON stdout behavior.
- **Files modified:** `crates/yune-cli/src/main.rs`
- **Verification:** `cargo clippy --workspace --all-targets -- -D warnings` passed.

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** No scope expansion; the fix was necessary for the mandated clippy gate and did not alter deterministic JSON output.

## Issues Encountered

- The execution environment did not have `cargo` on the default PATH for non-login tool calls. Verification was run with `PATH="/Users/trenton/.cargo/bin:$PATH"`.
- Running from a separate worktree could not see the main tree's newly requested integration test until commands used `--manifest-path /Users/trenton/Projects/yune/Cargo.toml`.

## Known Stubs

None found in files created or modified for this plan.

## Threat Flags

None. The only security-relevant trust-boundary additions were the planned explicit test runtime directories and ABI frontend replay coverage from the plan threat model.

## Verification

- `cargo fmt --check` - passed via `cargo fmt --manifest-path /Users/trenton/Projects/yune/Cargo.toml --all --check`
- `cargo test -p yune-cli --test frontend_surrogate -- --nocapture` - passed via manifest-path equivalent
- `cargo test -p yune-cli` - passed via manifest-path equivalent
- `cargo test -p yune-rime-api --test frontend_client` - passed via manifest-path equivalent
- `cargo test --workspace` - passed via manifest-path equivalent
- `cargo clippy --workspace --all-targets -- -D warnings` - passed via manifest-path equivalent

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 1 CLI frontend surrogate behavior is covered through ABI-backed CLI tests and owning module tests. Phase 2 can use these transcript and fixture seams as a baseline for native frontend validation without treating the CLI surrogate as proof of Squirrel, Weasel, ibus-rime, fcitx-rime, or fcitx5-rime integration.

## Self-Check: PASSED

- Created summary exists: `.planning/phases/01-cli-frontend-surrogate/01-03-SUMMARY.md`
- Created integration test exists: `crates/yune-cli/tests/frontend_surrogate.rs`
- Expected source/module files were modified and verified by Cargo gates.
- Commits intentionally skipped per user instruction.

---
*Phase: 01-cli-frontend-surrogate*
*Completed: 2026-04-28*
