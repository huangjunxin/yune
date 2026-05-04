---
phase: 07-wasm-build-and-export-contract
plan: 03
subsystem: wasm-build-contract
tags: [rust, wasm, emscripten, typeduck, docs, fallback-tests]

requires:
  - phase: 07-wasm-build-and-export-contract
    provides: Canonical TypeDuck export list and deterministic WASM build/check script from Plans 07-01 and 07-02
provides:
  - Native TypeDuck fallback coverage for browser host filesystem layout and missing-asset init behavior
  - Adapter init guard requiring preloaded shared/build schema and dictionary assets
  - Browser handoff documentation for one-active-service lifecycle, filesystem responsibilities, and verified/blocked build semantics
affects: [phase-08-typescript-bridge, phase-09-browser-filesystem-and-persistence, phase-10-typeduck-web-e2e]

tech-stack:
  added: [serde_json]
  patterns:
    - Serialized native integration tests around process-global RIME service state
    - Adapter init fails closed when browser runtime assets are absent
    - Documentation mirrors native fallback constraints for later browser phases

key-files:
  created:
    - crates/yune-rime-api/src/typeduck_web.rs
    - crates/yune-rime-api/tests/typeduck_web.rs
    - .planning/phases/07-wasm-build-and-export-contract/07-03-SUMMARY.md
  modified:
    - Cargo.lock
    - crates/yune-rime-api/Cargo.toml
    - crates/yune-rime-api/src/lib.rs
    - docs/typeduck-web-adapter.md

key-decisions:
  - "Treat missing browser schema/dictionary assets as an init-time failure before starting the process-global RIME service."
  - "Document Phase 7 as a handoff contract: one active process-global service, host-owned MEMFS/IDBFS layout and sync, and deterministic verified-or-blocked build output."

patterns-established:
  - "Native fallback tests use test_guard() for every adapter init path because cleanup finalizes process-global RIME service state."
  - "TypeDuck adapter init checks shared default/schema/dictionary assets and deployed user_data_dir/build configs before setup/initialize."

requirements-completed:
  - TYPEDUCK-WASM-01
  - TYPEDUCK-WASM-02
  - TYPEDUCK-WASM-03

duration: 4min
completed: 2026-05-04
---

# Phase 07 Plan 03: Browser Target Constraints and Fallback Contract Summary

**TypeDuck browser handoff contract with fail-closed asset preload checks, native fallback coverage, and documented one-active-service filesystem/build semantics.**

## Performance

- **Duration:** 4 min
- **Started:** 2026-05-04T07:34:50Z
- **Completed:** 2026-05-04T07:38:48Z
- **Tasks:** 3 completed
- **Files modified:** 6

## Accomplishments

- Added native fallback coverage named `typeduck_adapter_documents_browser_host_layout_constraints` that asserts `shared_data_dir`, `user_data_dir`, and `user_data_dir/build` layout exists before adapter init.
- Added an adapter init guard so missing schema/dictionary preload assets return null deterministically before the process-global RIME service is started.
- Extended TypeDuck browser documentation with one active process-global lifecycle semantics, host-owned MEMFS/IDBFS responsibilities, exact verified/blocked script output, and preserved Phase 8/9/10/AI deferrals.
- Ran focused formatter, native fallback tests, and the TypeDuck WASM build/check script; local browser build remains blocked by the missing `wasm32-unknown-emscripten` target while native fallback tests pass.

## Task Commits

Each task was committed atomically where file changes existed:

1. **Task 1: Lock browser filesystem and lifecycle constraints in native fallback tests** - `484eea8` (feat)
2. **Task 2: Document browser constraints, fallback blockers, and phase deferrals** - `f90938a` (docs)
3. **Task 3: Run final Phase 7 quality gates** - no file changes after verification; covered by `484eea8` and `f90938a`

**Plan metadata:** pending final docs commit

## Files Created/Modified

- `Cargo.lock` - Records `serde_json` dependency for TypeDuck JSON response serialization.
- `crates/yune-rime-api/Cargo.toml` - Adds `serde_json` to `yune-rime-api` dependencies.
- `crates/yune-rime-api/src/lib.rs` - Wires the TypeDuck adapter module into the API crate exports.
- `crates/yune-rime-api/src/typeduck_web.rs` - Exposes the TypeDuck C/WASM bridge and now fails init when required browser runtime assets are absent.
- `crates/yune-rime-api/tests/typeduck_web.rs` - Covers native adapter lifecycle, JSON responses, candidate actions, deploy/customize, null handling, and browser host layout/missing-asset constraints.
- `docs/typeduck-web-adapter.md` - Documents lifecycle, filesystem, preload, persistence, export, and fallback semantics for browser callers.
- `.planning/phases/07-wasm-build-and-export-contract/07-03-SUMMARY.md` - Plan execution record.

## Decisions Made

- Init checks for preloaded runtime assets before `setup`/`initialize` so missing browser files cannot accidentally reuse process-global state or fabricate placeholder data.
- The browser filesystem contract remains host-owned: JS/Emscripten code must mount MEMFS/IDBFS, preload schema/dictionary assets, and perform persistence sync until Phase 9.
- Phase 7 documentation now treats `scripts/typeduck-wasm-build.sh` as the source of truth for verified browser build output versus blocker-plus-native-fallback output.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical Functionality] Added fail-closed TypeDuck asset preload validation**
- **Found during:** Task 1 (Lock browser filesystem and lifecycle constraints in native fallback tests)
- **Issue:** The new RED test showed `yune_typeduck_init` succeeded with empty shared/user browser directories, which violated the plan threat mitigations for missing preloaded assets.
- **Fix:** Added `has_preloaded_runtime_assets` and `has_preloaded_dictionary` checks before adapter setup/initialize. Init now requires shared `default.yaml`, shared schema YAML, at least one shared `.dict.yaml`, and deployed `user_data_dir/build` default/schema configs.
- **Files modified:** `crates/yune-rime-api/src/typeduck_web.rs`
- **Verification:** Focused test and full `cargo test -p yune-rime-api --test typeduck_web` passed.
- **Committed in:** `484eea8`

---

**Total deviations:** 1 auto-fixed (1 Rule 2 missing critical functionality)
**Impact on plan:** The fix was required by T-07-03-02 and keeps the Phase 7 browser handoff fail-closed without expanding into Phase 8/9/10 functionality.

## Issues Encountered

- The RED-phase focused test failed because empty browser directories still initialized successfully. This exposed the missing fail-closed asset check and was resolved in Task 1.
- Local browser tooling still lacks the `wasm32-unknown-emscripten` Rust target. This is the expected blocker path from Plan 07-02; the script exited successfully after native fallback tests passed.

## User Setup Required

None - no external service configuration required. Developers who want a real browser artifact can install the `wasm32-unknown-emscripten` Rust target and activate Emscripten so `emcc` and `emar` are on `PATH`.

## Verification

- `PATH="/Users/trenton/.cargo/bin:$PATH" cargo test --manifest-path /Users/trenton/Projects/yune/Cargo.toml -p yune-rime-api --test typeduck_web typeduck_adapter_documents_browser_host_layout_constraints -- --exact` passed: 1 test, 0 failures.
- `PATH="/Users/trenton/.cargo/bin:$PATH" cargo test --manifest-path /Users/trenton/Projects/yune/Cargo.toml -p yune-rime-api --test typeduck_web` passed: 5 tests, 0 failures.
- Task 1 greps for `typeduck_adapter_documents_browser_host_layout_constraints` and `user_data_dir/build` passed.
- Task 2 documentation greps for lifecycle, cleanup/finalize, `user_data_dir/build`, asset preload, persistence sync, verified/blocked output, native fallback command, removed old Emscripten-missing bullet, and preserved deferrals passed.
- `PATH="/Users/trenton/.cargo/bin:$PATH" cargo fmt --manifest-path /Users/trenton/Projects/yune/Cargo.toml --all -- --check` passed.
- `PATH="/Users/trenton/.cargo/bin:$PATH" /Users/trenton/Projects/yune/scripts/typeduck-wasm-build.sh` passed with native exports verified, missing `wasm32-unknown-emscripten` blocker output, and native fallback tests passing: 5 tests, 0 failures.

## Known Stubs

None. The stub scan matched the documentation word "placeholder" only in the sentence explaining that missing assets must not fabricate placeholder browser data; it is not a code or UI stub.

## Threat Flags

None - the plan intentionally modified the TypeDuck C/WASM adapter trust boundary and browser filesystem documentation already covered by the plan threat model. It introduced no network endpoints, credential reads, auth paths, remote sync, schema migrations, or new application file access beyond explicit preload existence checks on caller-provided data directories.

## Next Phase Readiness

Phase 8 can build a TypeScript wrapper against a documented one-active-service adapter lifecycle and response-freeing contract. Phase 9 has explicit filesystem and persistence responsibilities to implement, including preloading assets and syncing storage before/after lifecycle-changing operations. Phase 10 can rely on `scripts/typeduck-wasm-build.sh` output to distinguish a verified browser artifact from a local toolchain blocker with native fallback coverage.

## Self-Check: PASSED

- Found `crates/yune-rime-api/src/typeduck_web.rs`.
- Found `crates/yune-rime-api/tests/typeduck_web.rs`.
- Found `docs/typeduck-web-adapter.md`.
- Found `.planning/phases/07-wasm-build-and-export-contract/07-03-SUMMARY.md`.
- Found task commit `484eea8` in git history.
- Found task commit `f90938a` in git history.

---
*Phase: 07-wasm-build-and-export-contract*
*Completed: 2026-05-04*
