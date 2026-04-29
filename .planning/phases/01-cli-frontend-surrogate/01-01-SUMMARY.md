---
phase: 01-cli-frontend-surrogate
plan: 01
subsystem: cli
tags: [rust, cli, rime-abi, yune-rime-api, ffi]

requires: []
provides:
  - Explicit ABI-backed yune-cli frontend command surface with required runtime paths
  - Centralized rime_get_api/RimeApi lifecycle wrapper in rime_frontend.rs
  - Deterministic JSON frontend run output with per-key event capture
affects: [01-cli-frontend-surrogate, frontend-abi-validation, transcript-replay]

tech-stack:
  added: [yune-rime-api path dependency]
  patterns:
    - main.rs remains orchestration-only
    - unsafe RIME ABI pointer, CString, struct, lifecycle, and free-pairing code is centralized in rime_frontend.rs
    - CLI error copy follows error/next corrective-action format

key-files:
  created: []
  modified:
    - Cargo.lock
    - crates/yune-cli/Cargo.toml
    - crates/yune-cli/src/args.rs
    - crates/yune-cli/src/main.rs
    - crates/yune-cli/src/rime_frontend.rs

key-decisions:
  - "Kept existing run and check commands on the core-backed path while adding a separate frontend command for ABI-backed execution."
  - "Kept deterministic frontend JSON serialization inside rime_frontend.rs for this foundation slice because later plans own transcript module expansion."
  - "Used an RAII cleanup guard so destroy_session, cleanup_all_sessions, and finalize run on success and error paths."

patterns-established:
  - "Command::Frontend carries shared_data_dir, user_data_dir, schema_id, and sequence explicitly from args.rs to rime_frontend.rs."
  - "rime_frontend.rs acquires all ABI functions through rime_get_api()/RimeApi and pairs each populated commit/context/status read with its matching free function."

requirements-completed: [CLI-01, CLI-02, CLI-03, QUAL-02]

duration: 47min
completed: 2026-04-29
---

# Phase 1 Plan 01: ABI-Backed CLI Frontend Command Foundation Summary

**Explicit yune-cli frontend command path using yune-rime-api RimeApi lifecycle calls while preserving core-backed run/check behavior**

## Performance

- **Duration:** 47 min
- **Started:** 2026-04-29T00:00:00Z
- **Completed:** 2026-04-29T00:47:00Z
- **Tasks:** 3
- **Files modified:** 5

## Accomplishments

- Added `yune-rime-api` as a direct `yune-cli` dependency and introduced `Command::Frontend` with required `--shared-data-dir`, `--user-data-dir`, `--schema`, and `--sequence` flags.
- Implemented the ABI-backed frontend lifecycle in `crates/yune-cli/src/rime_frontend.rs`: setup, initialize, deploy, create session, select schema, process key events, read commit/context/status after each key, cleanup sessions, and finalize.
- Wired `main.rs` as orchestration glue only: parse command, call the owning module, print deterministic JSON on success, and preserve existing stderr failure behavior.
- Preserved existing core-backed `run` and `check` paths.

## Task Commits

No commits were created at user request.

## Files Created/Modified

- `Cargo.lock` - Records the new `yune-rime-api` dependency edge for `yune-cli`.
- `crates/yune-cli/Cargo.toml` - Adds `yune-rime-api = { path = "../yune-rime-api" }`.
- `crates/yune-cli/src/args.rs` - Adds explicit frontend command parsing, required runtime-path validation, plain help text, and focused parser tests.
- `crates/yune-cli/src/main.rs` - Dispatches frontend commands to `rime_frontend::run_frontend` while keeping orchestration-only responsibilities.
- `crates/yune-cli/src/rime_frontend.rs` - Owns all unsafe ABI calls, C string conversion, versioned struct initialization, lifecycle cleanup, allocation/free pairing, per-key event capture, and deterministic JSON output for this foundation.

## Decisions Made

- Kept `run` and `check` unchanged as core-backed compatibility paths, and added `frontend` as an explicit ABI-backed command.
- Required shared/user runtime directories at parse time before any ABI setup call.
- Kept all direct `unsafe` RIME ABI code in `rime_frontend.rs`; `main.rs` contains no ABI structs, pointer handling, or serialization details.
- Used `rime_get_api()` / `RimeApi` exclusively for frontend lifecycle calls; the frontend path does not route through `yune_core` engine APIs directly.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added main dispatch while validating Task 1 parser tests**
- **Found during:** Task 1 (frontend command parsing)
- **Issue:** Adding `Command::Frontend` made the existing `main.rs` match non-exhaustive, blocking compilation of focused args tests.
- **Fix:** Wired minimal frontend dispatch to `rime_frontend::run_frontend` so parser tests could compile, then completed the planned Task 3 behavior.
- **Files modified:** `crates/yune-cli/src/main.rs`
- **Verification:** `cargo test -p yune-cli args -- --nocapture`, `cargo test -p yune-cli main -- --nocapture`, and `cargo test -p yune-cli` passed.
- **Committed in:** Not committed per user request.

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Required for compilation after adding the new enum variant; no scope expansion beyond planned dispatch.

## Issues Encountered

- The active shell did not initially have Cargo on `PATH`; verification commands were run with `PATH="/Users/trenton/.cargo/bin:$PATH"`.
- Worktree startup context lacked tracked phase files, so the implementation was copied back to the requested repository root at `/Users/trenton/Projects/yune` before final verification.

## Verification

All required commands passed in `/Users/trenton/Projects/yune`:

- `cargo test -p yune-cli args -- --nocapture` — 6 passed.
- `cargo test -p yune-cli rime_frontend -- --nocapture` — 3 passed.
- `cargo test -p yune-cli main -- --nocapture` — 0 matched tests, command passed.
- `cargo test -p yune-cli` — 12 passed.
- `cargo fmt --check` — passed.

## Known Stubs

None found in modified files.

## Self-Check: PASSED

- Found summary file at `/Users/trenton/Projects/yune/.planning/phases/01-cli-frontend-surrogate/01-01-SUMMARY.md`.
- Found all modified implementation files under `/Users/trenton/Projects/yune`.
- Commit checks skipped because the user explicitly requested no commits.

## Threat Flags

None beyond the plan threat model. New CLI-to-ABI trust boundaries are the planned `args.rs` runtime path validation and centralized `rime_frontend.rs` FFI lifecycle surface.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Plan 01 establishes the explicit ABI lifecycle foundation needed for later transcript/render work. Plan 02 can build on `FrontendRun`/`FrontendEvent` outputs and move or extend deterministic transcript serialization as planned.

---
*Phase: 01-cli-frontend-surrogate*
*Completed: 2026-04-29*
