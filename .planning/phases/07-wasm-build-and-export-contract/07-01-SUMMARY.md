---
phase: 07-wasm-build-and-export-contract
plan: 01
subsystem: wasm-build-contract
tags: [rust, wasm, emscripten, typeduck, exports]

requires:
  - phase: 06-real-frontend-validation-and-benchmark
    provides: TypeDuck-Web-style browser/WASM validation gaps and seeded adapter direction
provides:
  - Canonical adapter-owned TypeDuck export list for later build/export checks
  - Documented wasm32-unknown-emscripten target and Emscripten flag contract
  - Deterministic local-toolchain blocker messages for missing Rust target or Emscripten linker
affects: [phase-07-wasm-build-and-export-contract, phase-08-typescript-bridge, phase-10-typeduck-web-e2e]

tech-stack:
  added: []
  patterns:
    - Plain text adapter export contract consumed by docs and later scripts
    - Emscripten export retention documented with underscore-prefixed native symbols

key-files:
  created:
    - scripts/typeduck-exports.txt
  modified:
    - docs/typeduck-web-adapter.md

key-decisions:
  - "Use scripts/typeduck-exports.txt as the canonical non-prefixed TypeDuck adapter export list."
  - "Document wasm32-unknown-emscripten plus Emscripten EXPORTED_FUNCTIONS/EXPORTED_RUNTIME_METHODS as the browser build contract without changing lib.rs facade wiring."

patterns-established:
  - "Adapter-owned export list: one non-prefixed yune_typeduck_* symbol per line, no RimeApi symbols."
  - "Missing browser toolchain state is documented as an actionable blocker with native adapter tests as fallback, not as successful browser validation."

requirements-completed:
  - TYPEDUCK-WASM-01
  - TYPEDUCK-WASM-02

duration: 3min
completed: 2026-05-04
---

# Phase 07 Plan 01: Define WASM Build And Export Contract Summary

**TypeDuck browser build contract with canonical yune_typeduck exports, wasm32-unknown-emscripten target documentation, and deterministic Emscripten blocker semantics.**

## Performance

- **Duration:** 3 min
- **Started:** 2026-05-04T07:03:36Z
- **Completed:** 2026-05-04T07:06:50Z
- **Tasks:** 2 completed
- **Files modified:** 2

## Accomplishments

- Added `scripts/typeduck-exports.txt` with exactly the 11 required non-prefixed `yune_typeduck_*` adapter symbols.
- Extended `docs/typeduck-web-adapter.md` with the `wasm32-unknown-emscripten` target, `./scripts/typeduck-wasm-build.sh` command path, blocker messages, `EXPORTED_FUNCTIONS`, and `EXPORTED_RUNTIME_METHODS`.
- Kept the contract adapter-specific and did not broaden the librime-shaped `RimeApi` surface or add owned behavior to `crates/yune-rime-api/src/lib.rs`.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add canonical TypeDuck export list** - `41839c7` (feat)
2. **Task 2: Document target, flags, and blocker contract** - `9cb0c77` (docs)

**Plan metadata:** pending final docs commit

## Files Created/Modified

- `scripts/typeduck-exports.txt` - Canonical adapter-owned export list with exactly 11 non-prefixed symbols.
- `docs/typeduck-web-adapter.md` - Documents the WASM build/export contract, command path, blocker output, and Emscripten export/runtime flags.

## Decisions Made

- Used a plain UTF-8 newline-delimited export list so future scripts and reviewers can share one canonical adapter contract.
- Documented the Emscripten command path and flag shape now, while leaving the actual build script implementation to the subsequent Phase 7 plan.
- Kept the adapter in `crates/yune-rime-api` and did not modify `crates/yune-rime-api/src/lib.rs`, preserving facade-only wiring.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- Initial `cargo` verification failed because the shell PATH did not include `/Users/trenton/.cargo/bin`. Re-running the same plan checks with that PATH prefix succeeded; no code or docs change was needed.

## User Setup Required

None - no external service configuration required.

## Verification

- `test -f scripts/typeduck-exports.txt && test "$(grep -v '^$' scripts/typeduck-exports.txt | wc -l | tr -d ' ')" = "11"` passed.
- Documentation grep checks for `wasm32-unknown-emscripten`, `./scripts/typeduck-wasm-build.sh`, both blocker messages, `-sEXPORTED_FUNCTIONS=...`, and `-sEXPORTED_RUNTIME_METHODS=ccall,cwrap,UTF8ToString` passed.
- `PATH="/Users/trenton/.cargo/bin:$PATH" cargo fmt --manifest-path /Users/trenton/Projects/yune/Cargo.toml --all -- --check` passed.
- `PATH="/Users/trenton/.cargo/bin:$PATH" cargo test --manifest-path /Users/trenton/Projects/yune/Cargo.toml -p yune-rime-api --test typeduck_web` passed: 4 tests, 0 failures.

## Known Stubs

None.

## Threat Flags

None - this plan added docs/export contract artifacts only and introduced no new network endpoints, auth paths, file access code, or schema trust-boundary changes beyond the planned export-list/documentation surface.

## Next Phase Readiness

Ready for 07-02 to add build-script or command coverage that verifies adapter symbol availability using the canonical export list and documented blocker semantics.

## Self-Check: PASSED

- Found `scripts/typeduck-exports.txt`.
- Found `docs/typeduck-web-adapter.md`.
- Found `.planning/phases/07-wasm-build-and-export-contract/07-01-SUMMARY.md`.
- Found task commits `41839c7` and `9cb0c77` in git history.

---
*Phase: 07-wasm-build-and-export-contract*
*Completed: 2026-05-04*
