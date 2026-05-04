---
phase: 07-wasm-build-and-export-contract
plan: 02
subsystem: wasm-build-contract
tags: [rust, wasm, emscripten, typeduck, exports, shell]

requires:
  - phase: 07-wasm-build-and-export-contract
    provides: Canonical TypeDuck export list and Emscripten blocker contract from Plan 07-01
provides:
  - Executable TypeDuck WASM build/check command path
  - Native dynamic-library export verification against scripts/typeduck-exports.txt
  - Deterministic browser-toolchain blocker path with native adapter fallback tests
affects: [phase-07-wasm-build-and-export-contract, phase-08-typescript-bridge, phase-10-typeduck-web-e2e]

tech-stack:
  added: []
  patterns:
    - POSIX shell build contract rooted relative to the script path
    - Shared adapter export list consumed for native and Emscripten artifact verification

key-files:
  created:
    - scripts/typeduck-wasm-build.sh
  modified: []

key-decisions:
  - "Implement the TypeDuck WASM build/export contract as a POSIX shell script that verifies native exports before browser prerequisite detection."
  - "Treat missing wasm32-unknown-emscripten, emcc, or emar as deterministic blockers only when cargo test -p yune-rime-api --test typeduck_web passes."

patterns-established:
  - "Native export gate: cargo build -p yune-rime-api followed by nm -g checks for every symbol in scripts/typeduck-exports.txt, accepting macOS leading underscores."
  - "Browser export gate: Emscripten RUSTFLAGS are generated from the same export list, then artifacts are inspected with wasm-nm, wasm-objdump, or JS text scan fallback."

requirements-completed:
  - TYPEDUCK-WASM-01
  - TYPEDUCK-WASM-02
  - TYPEDUCK-WASM-03

duration: 8min
completed: 2026-05-04
---

# Phase 07 Plan 02: Deterministic TypeDuck WASM Build Script Summary

**Executable TypeDuck WASM build/check script with native yune_typeduck export verification, Emscripten prerequisite blockers, generated export flags, and native fallback tests.**

## Performance

- **Duration:** 8 min
- **Started:** 2026-05-04T07:18:29Z
- **Completed:** 2026-05-04T07:26:50Z
- **Tasks:** 2 completed
- **Files modified:** 1

## Accomplishments

- Added `scripts/typeduck-wasm-build.sh` as the single command path for TypeDuck adapter build/export validation.
- The script reads `scripts/typeduck-exports.txt`, rejects missing/empty export lists, builds `yune-rime-api`, locates the platform native dynamic library, and verifies all adapter symbols with `nm` while accepting optional leading underscores.
- The script detects `wasm32-unknown-emscripten`, `emcc`, and `emar`; missing browser tooling prints deterministic blocker text and runs `cargo test -p yune-rime-api --test typeduck_web` before exiting successfully only if fallback tests pass.
- When browser tooling is present, the script builds `--target wasm32-unknown-emscripten` with `EXPORTED_FUNCTIONS` and `EXPORTED_RUNTIME_METHODS=ccall,cwrap,UTF8ToString`, then verifies artifacts with `wasm-nm`, `wasm-objdump`, or JS text scan fallback.

## Task Commits

Each task was committed atomically where file changes existed:

1. **Task 1: Create deterministic TypeDuck WASM build/check script** - `d39a2bd` (feat)
2. **Task 2: Run script in local fallback mode and fix deterministic output** - no code changes after verification; covered by `d39a2bd`

**Plan metadata:** pending final docs commit

## Files Created/Modified

- `scripts/typeduck-wasm-build.sh` - Executable POSIX shell build/check path for native export verification, Emscripten prerequisite detection, browser artifact inspection, and native fallback tests.

## Decisions Made

- Implemented the plan as a repo-root-resolving POSIX shell script rather than a Cargo alias or Rust helper so missing external browser tooling can be reported before requiring new dependencies.
- Kept export verification adapter-specific by consuming `scripts/typeduck-exports.txt` for native and browser checks; no `RimeApi` or `lib.rs` behavior was inspected or mutated.
- Used deterministic blocker-plus-fallback success as the local outcome because the current environment does not have the `wasm32-unknown-emscripten` Rust target installed.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- An initial manual overall verification command used `cargo --manifest-path`, which this Cargo version rejects. The command was corrected to `cargo fmt --manifest-path ...` and `cargo test --manifest-path ...`; both corrected checks passed. No repository files changed.

## User Setup Required

None - no external service configuration required. For an actual browser build, a developer may install the Rust target and activate Emscripten as printed by the script, but this plan accepts the deterministic blocker plus native fallback path.

## Verification

- `test -x scripts/typeduck-wasm-build.sh` passed.
- Acceptance greps for `set -eu`, `typeduck-exports.txt`, `cargo build -p yune-rime-api`, `cargo test -p yune-rime-api --test typeduck_web`, `wasm32-unknown-emscripten`, `emcc`, `emar`, blocker messages, `EXPORTED_FUNCTIONS`, `EXPORTED_RUNTIME_METHODS=ccall,cwrap,UTF8ToString`, `wasm-nm`, and `wasm-objdump` passed.
- `PATH="/Users/trenton/.cargo/bin:$PATH" /Users/trenton/Projects/yune/scripts/typeduck-wasm-build.sh` passed with native exports verified, missing `wasm32-unknown-emscripten` blocker output, and native fallback tests passing: 4 tests, 0 failures.
- `test "$(grep -v '^#' scripts/typeduck-wasm-build.sh | grep -c '/Users/trenton/' || true)" = "0"` passed.
- `PATH="/Users/trenton/.cargo/bin:$PATH" cargo fmt --manifest-path /Users/trenton/Projects/yune/Cargo.toml --all -- --check` passed.
- `PATH="/Users/trenton/.cargo/bin:$PATH" cargo test --manifest-path /Users/trenton/Projects/yune/Cargo.toml -p yune-rime-api --test typeduck_web` passed: 4 tests, 0 failures.

## Known Stubs

None.

## Threat Flags

None - the plan added a local developer build/check script with the trust boundaries already listed in the plan threat model. It introduced no network endpoints, auth paths, credential reads, schema changes, or new runtime application file access.

## Next Phase Readiness

Ready for 07-03 to extend adapter tests/docs around browser target constraints and fallback blocker behavior. Phase 8 can depend on a stable command path and canonical export verification semantics once Phase 7 documentation is complete.

## Self-Check: PASSED

- Found `scripts/typeduck-wasm-build.sh`.
- Found `.planning/phases/07-wasm-build-and-export-contract/07-02-SUMMARY.md`.
- Found task commit `d39a2bd` in git history.

---
*Phase: 07-wasm-build-and-export-contract*
*Completed: 2026-05-04*
