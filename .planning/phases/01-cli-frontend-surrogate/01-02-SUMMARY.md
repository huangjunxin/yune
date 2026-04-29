---
phase: 01-cli-frontend-surrogate
plan: 02
subsystem: cli
tags: [rust, cli, transcript, render, fixture-replay, rime-abi]

requires:
  - 01-01
provides:
  - Deterministic FrontendTranscript JSON serialization for owned ABI frontend events
  - Plain line-oriented frontend transcript rendering separated from JSON serialization
  - ABI-backed frontend-check fixture replay comparison alongside core-backed check
affects: [01-cli-frontend-surrogate, frontend-abi-validation, transcript-replay]

tech-stack:
  added: []
  patterns:
    - handcrafted deterministic JSON with stable key order and two-space indentation
    - render.rs owns human output; transcript.rs owns comparison serialization
    - main.rs remains orchestration-only dispatch

key-files:
  created:
    - .planning/phases/01-cli-frontend-surrogate/01-02-SUMMARY.md
  modified:
    - crates/yune-cli/src/transcript.rs
    - crates/yune-cli/src/render.rs
    - crates/yune-cli/src/fixture.rs
    - crates/yune-cli/src/args.rs
    - crates/yune-cli/src/main.rs
    - crates/yune-cli/src/rime_frontend.rs

key-decisions:
  - "Moved deterministic frontend transcript serialization ownership to transcript.rs while keeping FrontendRun::to_json as a compatibility delegation."
  - "Added frontend-check as a separate ABI-backed fixture replay command so existing core-backed check remains unchanged."
  - "Extended owned FrontendEvent/FrontendContext with keycode, mask, page metadata, select keys, and select labels so serializers never read raw ABI pointers."

patterns-established:
  - "FrontendTranscript::new(&FrontendRun).to_json() serializes schema_id, sequence, events, commits, context, and status in the required order."
  - "render_frontend_human(&FrontendRun) renders plain text labels without ANSI, cursor control, timestamps, paths, memory addresses, or icons."
  - "check_frontend_fixture reads only schema_id and sequence from a fixture before replaying through rime_frontend::run_frontend."

requirements-completed: [CLI-04, CLI-05, QUAL-02]

duration: 35min
completed: 2026-04-29
---

# Phase 1 Plan 02: Frontend Transcript Serialization, Rendering, and Fixture Replay Summary

**Deterministic ABI frontend transcripts with plain human rendering and separate ABI fixture replay comparison**

## Performance

- **Duration:** 35 min
- **Started:** 2026-04-28T20:39:06Z
- **Completed:** 2026-04-29
- **Tasks:** 3
- **Files modified:** 6 implementation files plus this summary

## Accomplishments

- Added `FrontendTranscript` in `crates/yune-cli/src/transcript.rs` with deterministic JSON top-level order: `schema_id`, `sequence`, `events`, `commits`, `context`, `status`.
- Extended owned frontend event/context data in `crates/yune-cli/src/rime_frontend.rs` to include `keycode`, `mask`, `page_size`, `page_no`, `is_last_page`, `select_keys`, and `select_labels`, copied from ABI output before free calls.
- Added `render_frontend_human` in `crates/yune-cli/src/render.rs` with plain line-oriented labels for event, key, handled, commit, preedit, caret, highlighted, candidates, selected marker, and status summary.
- Added `frontend-check <fixture.json> --shared-data-dir <path> --user-data-dir <path>` parsing and dispatch, while preserving existing core-backed `check <fixture.json>` behavior.
- Implemented `check_frontend_fixture` in `crates/yune-cli/src/fixture.rs`, extracting only top-level `schema_id` and `sequence`, replaying through `rime_frontend::run_frontend`, serializing with `FrontendRun::to_json`/`FrontendTranscript`, and comparing normalized JSON.

## Task Commits

No commits were created at user request.

## Files Created/Modified

- `crates/yune-cli/src/transcript.rs` - Adds `FrontendTranscript` and frontend-specific deterministic serializers for events, contexts, candidates, page metadata, select keys/labels, and status flags.
- `crates/yune-cli/src/render.rs` - Adds `render_frontend_human` for stable plain text frontend transcript output.
- `crates/yune-cli/src/fixture.rs` - Adds ABI-backed frontend fixture comparison while retaining the existing core fixture comparison path.
- `crates/yune-cli/src/args.rs` - Adds `Command::FrontendCheck`, parser support, and help text for the new replay command.
- `crates/yune-cli/src/main.rs` - Adds orchestration-only dispatch to `fixture::check_frontend_fixture`.
- `crates/yune-cli/src/rime_frontend.rs` - Extends owned frontend data with D-08 fields and delegates JSON output to `FrontendTranscript`.
- `.planning/phases/01-cli-frontend-surrogate/01-02-SUMMARY.md` - Captures this implementation summary.

## Decisions Made

- Kept `FrontendRun::to_json()` as a small delegating compatibility method so existing command output still works, but the serialization implementation now lives in `transcript.rs`.
- Added `frontend-check` rather than changing `check`, preserving the core-backed fixture command exactly as a compatibility path.
- Used the existing handwritten JSON style instead of serde, preserving deterministic key order and avoiding environment-derived output.
- Shared a single frontend test guard between `rime_frontend` and frontend fixture tests to serialize process-wide RIME runtime state under full test-suite parallelism.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical Functionality] Added owned D-08 event/context fields**
- **Found during:** Task 1
- **Issue:** Plan 01 structs did not yet carry keycode, mask, page metadata, select keys, or select labels required by D-08.
- **Fix:** Extended `FrontendEvent` and `FrontendContext` and copied values from ABI context data before calling `free_context`.
- **Files modified:** `crates/yune-cli/src/rime_frontend.rs`, `crates/yune-cli/src/transcript.rs`, `crates/yune-cli/src/render.rs`
- **Verification:** `cargo test -p yune-cli transcript -- --nocapture` and full `cargo test -p yune-cli` passed.
- **Committed in:** Not committed per user request.

**2. [Rule 1 - Bug] Serialized frontend runtime tests with a shared guard**
- **Found during:** Task 3 full-suite verification
- **Issue:** Frontend fixture tests and existing frontend lifecycle tests each used separate mutexes, allowing process-wide RIME runtime tests to overlap and intermittently fail status reads.
- **Fix:** Added a shared `frontend_test_guard` in `rime_frontend.rs` for all frontend runtime tests.
- **Files modified:** `crates/yune-cli/src/rime_frontend.rs`, `crates/yune-cli/src/fixture.rs`
- **Verification:** Full `cargo test -p yune-cli` passed.
- **Committed in:** Not committed per user request.

---

**Total deviations:** 2 auto-fixed
**Impact on plan:** Both fixes were required to satisfy planned D-08 data completeness and reliable verification; no architectural scope expansion.

## Issues Encountered

- The worktree copy lacks tracked phase plan files, so the implementation and summary were applied in the requested repository root at `/Users/trenton/Projects/yune`.
- `cargo fmt --manifest-path /Users/trenton/Projects/yune/Cargo.toml` reported no format targets from the workspace manifest; formatting was run successfully with the yune-cli crate manifest instead.

## Verification

All required commands passed in `/Users/trenton/Projects/yune` using the root manifest path:

- `cargo test --manifest-path /Users/trenton/Projects/yune/Cargo.toml -p yune-cli transcript -- --nocapture` — 3 passed.
- `cargo test --manifest-path /Users/trenton/Projects/yune/Cargo.toml -p yune-cli render -- --nocapture` — 2 passed.
- `cargo test --manifest-path /Users/trenton/Projects/yune/Cargo.toml -p yune-cli fixture -- --nocapture` — 4 passed.
- `cargo test --manifest-path /Users/trenton/Projects/yune/Cargo.toml -p yune-cli` — 19 passed.
- `cargo fmt --manifest-path /Users/trenton/Projects/yune/crates/yune-cli/Cargo.toml` — passed.

## Known Stubs

None found in modified implementation files.

## Threat Flags

None beyond the plan threat model. The new command path is the planned ABI frontend fixture replay surface and extracts only top-level `schema_id` and `sequence` before replay.

## Self-Check: PASSED

- Found summary file at `/Users/trenton/Projects/yune/.planning/phases/01-cli-frontend-surrogate/01-02-SUMMARY.md`.
- Found all modified implementation files under `/Users/trenton/Projects/yune`.
- Verified required test commands passed.
- Commit checks skipped because the user explicitly requested no commits.

## User Setup Required

None.

## Next Phase Readiness

Plan 02 establishes deterministic frontend transcript serialization, plain human rendering, and ABI fixture replay comparison. Plan 03 can add broader integration coverage on top of the `frontend` and `frontend-check` command surfaces.

---
*Phase: 01-cli-frontend-surrogate*
*Completed: 2026-04-29*
