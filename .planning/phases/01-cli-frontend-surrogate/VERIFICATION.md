---
phase: 01-cli-frontend-surrogate
verified: 2026-04-28T21:14:31Z
status: passed
score: 7/7 must-haves verified
overrides_applied: 0
re_verification:
  previous_status: passed
  previous_score: 7/7
  gaps_closed:
    - "Code review remediation verified: schema IDs are validated as logical IDs in CLI parsing and frontend fixture replay."
    - "Code review remediation verified: fixture comparison preserves whitespace inside JSON strings."
    - "Code review remediation verified: fixture schema_id/sequence extraction scans top-level root fields only."
    - "Code review remediation verified: frontend transcript input uses RimeApi::get_input instead of preedit duplication."
    - "Code review remediation verified: {Tab} maps to the RIME Tab keycode."
    - "Code review artifact is marked resolved."
  gaps_remaining: []
  regressions: []
---

# Phase 1: CLI Frontend Surrogate Verification Report

**Phase Goal:** Developers can use `yune-cli` as a scriptable frontend surrogate that exercises `yune-rime-api` setup, schema selection, key processing, and transcript replay.
**Verified:** 2026-04-28T21:14:31Z
**Status:** passed
**Re-verification:** Yes — final re-verification after code review remediation

## Goal Achievement

Phase 1 goal is achieved. The implementation provides an explicit ABI-backed `yune-cli frontend` path with runtime roots, schema selection, key processing, per-key transcript output in JSON or human text, and ABI transcript replay comparison through `frontend-check`. Code review remediation was verified in source and tests, not by trusting summaries.

### Observable Truths

| # | Truth | Status | Evidence |
|---|---|---|---|
| 1 | Developer can initialize the RIME service from `yune-cli` with explicit shared/user data paths. | VERIFIED | `crates/yune-cli/src/args.rs` defines `Command::Frontend { shared_data_dir, user_data_dir, schema_id, sequence, output_mode }` and rejects missing runtime paths with corrective errors. `crates/yune-cli/src/main.rs` passes both paths into `rime_frontend::FrontendOptions`. `crates/yune-cli/src/rime_frontend.rs` converts both paths to `RimeTraits.shared_data_dir` and `RimeTraits.user_data_dir` before ABI `setup` and `initialize`. Integration test `frontend_command_rejects_missing_runtime_paths_before_abi_calls` passed. |
| 2 | Developer can deploy, select a schema, create a session, process keys, and destroy the session through ABI calls. | VERIFIED | `rime_frontend.rs` obtains the `RimeApi` table via `rime_get_api`, requires `setup`, `initialize`, `deploy`, `create_session`, `select_schema`, and `process_key`, then captures state via ABI reads. `CleanupGuard::drop` calls `destroy_session`, `cleanup_all_sessions`, and `finalize` after initialized runs. Focused tests and workspace tests passed. |
| 3 | Developer can inspect commit text, preedit, candidates, highlight index, and status after each CLI key event. | VERIFIED | `FrontendTranscript::to_json()` emits per-event `commits`, `context.input`, `context.preedit`, `context.candidates`, `context.highlighted`, page metadata, select labels, and `status`. `render_frontend_human` renders event/key/handled/commit/preedit/caret/highlighted/candidates/status as plain text and is reachable through `frontend --output human`. Integration tests `frontend_command_uses_explicit_runtime_schema_and_per_key_abi_events` and `frontend_command_can_render_human_transcript` passed. |
| 4 | Developer can replay a key transcript through the ABI and compare deterministic output. | VERIFIED | `Command::FrontendCheck` parses `frontend-check <fixture.json> --shared-data-dir <path> --user-data-dir <path>`. `fixture::check_frontend_fixture` reads top-level `schema_id` and `sequence`, validates schema ID, reruns `rime_frontend::run_frontend`, serializes deterministic JSON, normalizes expected/actual JSON while preserving string whitespace, and compares. Integration test `frontend_check_replays_expected_fixture_through_abi_transcript` passed. |
| 5 | Every new behavior added in this phase lives in an owned module with matching focused tests, not in `main.rs` or `lib.rs`. | VERIFIED | CLI parsing is in `args.rs`; ABI lifecycle and unsafe code are in `rime_frontend.rs`; deterministic JSON is in `transcript.rs`; human rendering is in `render.rs`; replay comparison is in `fixture.rs`; binary behavior is covered by `tests/frontend_surrogate.rs`. `main.rs` only dispatches parsed commands and prints selected output. No `crates/yune-cli/src/lib.rs` exists. |
| 6 | QUAL-01: compatibility slice has owning implementation module, owning tests, and explicit librime comparison target. | VERIFIED | Owning modules have focused unit tests: `args.rs`, `rime_frontend.rs`, `transcript.rs`, `render.rs`, and `fixture.rs`; integration tests cover the CLI boundary. In-code comparison markers identify librime-visible seams, including the `rime_frontend.rs` lifecycle target `setup/initialize/deploy/select/create-session/process-key/read-state/destroy/finalize`. |
| 7 | QUAL-02: `lib.rs`/`main.rs` remain facades/orchestration glue. | VERIFIED | `main.rs` matches `Command` variants, calls owning modules, and prints output. It contains no RIME ABI struct handling, unsafe function-table access, fixture parsing, JSON serialization, or human transcript formatting. There is no CLI `lib.rs`. |

**Score:** 7/7 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `crates/yune-cli/Cargo.toml` | `yune-rime-api` dependency | VERIFIED | Contains `yune-rime-api = { path = "../yune-rime-api" }`. GSD artifact verification for plan 01 passed. |
| `crates/yune-cli/src/args.rs` | Frontend command parsing with runtime paths, schema, sequence, replay command, output selector, and logical schema validation | VERIFIED | Defines `Command::Frontend`, `Command::FrontendCheck`, and `FrontendOutputMode::{Json, Human}`. Parses `--output json|human`, defaults to JSON, validates schema IDs with alphanumeric/underscore/hyphen logical-ID rules, and tests invalid `../default` rejection. |
| `crates/yune-cli/src/main.rs` | Orchestration-only dispatch | VERIFIED | Dispatches `Frontend` to `rime_frontend::run_frontend`, selects JSON or human output, and dispatches `FrontendCheck` to `fixture::check_frontend_fixture`. No unsafe ABI or serialization logic exists here. |
| `crates/yune-cli/src/rime_frontend.rs` | Centralized unsafe ABI lifecycle wrapper and per-key state capture | VERIFIED | Uses `rime_get_api`, ABI function-table calls, C string conversion, RimeTraits setup, deploy/select/session/process calls, commit/context/status/input capture, matching free calls, and cleanup guard. Review remediation confirmed `get_input` use and `{Tab}` mapping to `0xff09`. |
| `crates/yune-cli/src/transcript.rs` | Deterministic frontend JSON serialization | VERIFIED | `FrontendTranscript::to_json()` emits stable top-level `schema_id`, `sequence`, `events`, `commits`, `context`, and `status`; tests assert exact output and omit environment-dependent values. |
| `crates/yune-cli/src/render.rs` | Plain human rendering for frontend events | VERIFIED | `render_frontend_human` is substantive, tested, and reachable from CLI through `FrontendOutputMode::Human`. It does not call JSON serialization and tests reject control/path/pointer leakage. |
| `crates/yune-cli/src/fixture.rs` | ABI frontend transcript fixture comparison | VERIFIED | `check_frontend_fixture` replays through `run_frontend`, validates logical schema IDs, extracts root fields only, normalizes JSON without deleting string whitespace, and compares deterministic output. |
| `crates/yune-cli/tests/frontend_surrogate.rs` | Integration coverage for ABI-backed CLI frontend surrogate | VERIFIED | Contains end-to-end binary tests for missing runtime path rejection, explicit runtime/schema per-key JSON, human output mode, deterministic output omissions, and ABI replay comparison. `cargo test -p yune-cli --test frontend_surrogate -- --nocapture` passed with 5 tests. |
| `.planning/phases/01-cli-frontend-surrogate/01-REVIEW.md` | Code review artifact marked resolved after remediation | VERIFIED | Frontmatter has `status: resolved`, `critical: 0`, `warning: 0`, `info: 0`, and the body records CR-01, CR-02, WR-01, WR-02, and WR-03 as resolved. |

Note: `gsd-sdk query verify.artifacts` for plan 03 reported a literal-pattern miss for `rime_frontend.rs` requiring exact text `librime comparison target`. Manual source verification shows the substantive comparison target comment exists and names the lifecycle target; this is not a blocker.

### Key Link Verification

| From | To | Via | Status | Details |
|---|---|---|---|---|
| `args.rs` | `main.rs` | `Command::Frontend { ..., output_mode }` | WIRED | `main.rs` matches `Command::Frontend` and consumes `output_mode`. |
| `args.rs` | CLI output selector | `FrontendOutputMode` and `--output` | WIRED | `parse_output_mode` accepts `json` and `human`; unsupported values return a corrective error. |
| `args.rs` | logical schema validation | `validate_schema_id` | WIRED | `parse_frontend` validates `--schema`; `fixture.rs` reuses `validate_schema_id` for fixture `schema_id`. |
| `main.rs` | `rime_frontend.rs` | `rime_frontend::run_frontend(FrontendOptions { ... })` | WIRED | ABI-backed frontend command is routed through the owning lifecycle wrapper. |
| `main.rs` | `render.rs` | `FrontendOutputMode::Human => render_frontend_human` | WIRED | Previous human-renderer orphan gap remains closed; integration test covers CLI stdout. |
| `main.rs` | `transcript.rs` | `FrontendOutputMode::Json => output.to_json()` | WIRED | JSON remains default path and is tested through binary invocation. |
| `main.rs` | `fixture.rs` | `FrontendCheck => check_frontend_fixture` | WIRED | ABI transcript replay command is CLI-reachable. |
| `rime_frontend.rs` | `yune-rime-api RimeApi` | `rime_get_api` function table | WIRED | Lifecycle uses ABI table entries, not direct `yune_core` fixture setup. |
| `rime_frontend.rs` | ABI input/context/status/commit reads | `get_input`, `get_commit`, `get_context`, `get_status` with free calls | WIRED | Review remediation verified `context.input` comes from `RimeApi::get_input`; commit/context/status structs are freed through matching ABI free functions. |
| `rime_frontend.rs` | RIME cleanup functions | cleanup guard | WIRED | Drop path destroys session, cleans all sessions when available, and finalizes after initialization. |
| `rime_frontend.rs` | `transcript.rs` | `FrontendRun::to_json()` delegates to `FrontendTranscript` | WIRED | Owned event data flows to deterministic transcript serialization. |
| `transcript.rs` | `fixture.rs` | normalized expected/actual comparison | WIRED | `check_frontend_fixture` compares `run_frontend(...).to_json()` against fixture JSON. |
| `tests/frontend_surrogate.rs` | binary/API behavior | `CARGO_BIN_EXE_yune-cli` and `rime_get_api` guard | WIRED | Integration tests invoke the real CLI binary and guard process-wide RIME state. |

Note: `gsd-sdk query verify.key-links` for plan 03 reported one literal-plan issue because `from: "owned implementation modules"` is not a file path. Manual verification of module-local tests and integration tests passes.

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|---|---|---|---|---|
| `args.rs` | `shared_data_dir`, `user_data_dir`, `schema_id`, `sequence`, `output_mode` | CLI flags parsed by `Command::parse` | Yes | FLOWING |
| `main.rs` | `output` | `rime_frontend::run_frontend(FrontendOptions { ... })?` | Yes | FLOWING |
| `rime_frontend.rs` | `FrontendRun.events` | ABI `process_key` per parsed key followed by ABI state reads | Yes | FLOWING |
| `rime_frontend.rs` | `FrontendContext.input` | `api.get_input(session_id)` | Yes | FLOWING |
| `rime_frontend.rs` | Tab keycode | `KeyCode::Tab => XK_TAB` where `XK_TAB = 0xff09` | Yes | FLOWING |
| `transcript.rs` | JSON transcript fields | Owned `FrontendRun`/`FrontendEvent` values from ABI lifecycle | Yes | FLOWING |
| `render.rs` | Human transcript fields | Same owned `FrontendRun` events selected by `--output human` | Yes | FLOWING |
| `fixture.rs` | fixture `schema_id`/`sequence` | `root_field_values` scans only root-object fields, then `parse_json_string` extracts string value | Yes | FLOWING |
| `fixture.rs` | `actual` transcript JSON | `run_frontend(FrontendOptions { ... })?.to_json()` | Yes | FLOWING |
| `tests/frontend_surrogate.rs` | CLI stdout/stderr assertions | Actual `CARGO_BIN_EXE_yune-cli` subprocess output | Yes | FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|---|---|---|---|
| Formatting gate | `cd /Users/trenton/Projects/yune && /Users/trenton/.cargo/bin/cargo fmt --check` | Exit 0 as part of focused gate chain. | PASS |
| Frontend integration suite | `cd /Users/trenton/Projects/yune && /Users/trenton/.cargo/bin/cargo test -p yune-cli --test frontend_surrogate -- --nocapture` | 5 passed, 0 failed. Tests cover human output, JSON output, missing paths, deterministic omissions, and fixture replay. | PASS |
| Full `yune-cli` test suite | `cd /Users/trenton/Projects/yune && /Users/trenton/.cargo/bin/cargo test -p yune-cli` | 27 unit tests and 5 integration tests passed. | PASS |
| Workspace tests | `cd /Users/trenton/Projects/yune && /Users/trenton/.cargo/bin/cargo test --workspace` | Workspace tests passed, including 27 `yune-cli` unit tests, 5 `frontend_surrogate` tests, 141 `yune_core` tests, 222 `yune_rime_api` tests, 33 `frontend_client` tests, and 3 `yune_schema` tests. | PASS |
| Workspace clippy | `cd /Users/trenton/Projects/yune && /Users/trenton/.cargo/bin/cargo clippy --workspace --all-targets -- -D warnings` | Finished successfully with warnings denied. | PASS |
| GSD artifact/key-link checks | `gsd-sdk query verify.artifacts` and `gsd-sdk query verify.key-links` for plans 01-03 from project root | Plans 01-02 passed. Plan 03 had non-blocking literal-pattern/path limitations; manual source verification confirmed the underlying artifacts and links. | PASS WITH NOTE |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|---|---|---|---|---|
| CLI-01 | 01-01, 01-03 | Initialize `yune-rime-api` from `yune-cli` with explicit shared data and user data directories. | SATISFIED | Parser requires both dirs; main passes them to `FrontendOptions`; `rime_frontend.rs` writes them into `RimeTraits` before ABI setup/initialize; integration test covers missing path rejection before ABI calls. |
| CLI-02 | 01-01, 01-03 | Deploy and select schemas through the CLI using the RIME ABI path, not direct `yune-core` fixture setup. | SATISFIED | `rime_frontend.rs` calls ABI `deploy()` and `select_schema(session_id, schema_id)`. Schema IDs are now logical-ID validated at CLI and fixture replay boundaries. |
| CLI-03 | 01-01, 01-03 | Create/destroy RIME sessions and process interactive key events through `RimeProcessKey`. | SATISFIED | `create_session`, `process_key`, and `destroy_session` are used through `RimeApi`; cleanup guard handles process-wide cleanup/finalize; per-key integration tests passed. |
| CLI-04 | 01-02, 01-03 | Render commit text, preedit, candidate page, highlight index, and status after each CLI key event. | SATISFIED | JSON output exposes required per-key state; human output is reachable via `--output human` and renders the same ABI-backed event data in plain text. |
| CLI-05 | 01-02, 01-03 | Replay transcript key sequences through the RIME ABI and compare output. | SATISFIED | `frontend-check` extracts top-level `schema_id`/`sequence`, validates schema, runs ABI path, serializes deterministic JSON, normalizes preserving string whitespace, and compares. |
| QUAL-01 | 01-03 | Compatibility slice has owning implementation module, owning tests, and explicit librime comparison target. | SATISFIED | Owned modules and focused tests exist; lifecycle and transcript comments identify librime-visible comparison seams. |
| QUAL-02 | 01-01, 01-02, 01-03 | `lib.rs`/`main.rs` remain facades/orchestration glue. | SATISFIED | `main.rs` is dispatch-only; no CLI `lib.rs`; implementation logic is in owned modules. |

No orphaned Phase 1 requirements were found. `.planning/REQUIREMENTS.md` maps CLI-01 through CLI-05 and QUAL-01/QUAL-02 to Phase 1, and all are claimed by phase plans and satisfied by implementation evidence.

### Code Review Remediation Verification

| Review Item | Status | Evidence |
|---|---|---|
| CR-01: schema IDs validated as logical IDs in CLI args and fixture replay | VERIFIED | `args.rs` has `validate_schema_id` accepting only non-empty ASCII alphanumeric, `_`, and `-`, rejecting `.`, `..`, separators, and traversal. `parse_frontend` applies it to `--schema`; `fixture.rs` applies it to fixture `schema_id`; tests cover invalid `../default`. |
| CR-02: fixture comparison preserves whitespace inside JSON strings | VERIFIED | `fixture.rs::normalize_json` tracks `in_string` and only removes whitespace outside strings; test `fixture_comparison_preserves_whitespace_inside_strings` asserts `"ni "` and `"ni"` mismatch. |
| WR-01: fixture extraction scans top-level root fields only | VERIFIED | `root_field_values` parses the root object and skips nested JSON values with string-aware balanced skipping; test `fixture_field_reader_only_accepts_top_level_fields` rejects nested `events[].schema_id`. |
| WR-02: transcript input uses `RimeApi::get_input`, not preedit duplication | VERIFIED | `capture_context` obtains `api.get_input`, reads `get_input(session_id)`, and passes that input to `copy_context`; preedit is copied separately from `context.composition.preedit`. |
| WR-03: `{Tab}` maps to RIME Tab keycode | VERIFIED | `XK_TAB` is `0xff09`; `key_event_to_rime` maps `KeyCode::Tab => XK_TAB`; test `maps_tab_key_name_to_rime_keycode` passed. |
| Review artifact resolved | VERIFIED | `.planning/phases/01-cli-frontend-surrogate/01-REVIEW.md` frontmatter has `status: resolved` and zero findings. |

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|---|---:|---|---|---|
| None | - | No TODO/FIXME/placeholder/stub/console-log-only patterns found in Phase 1 CLI source or integration test files. | - | No blocker anti-patterns. |

### Human Verification Required

None. Phase 1 produces deterministic CLI/API behavior that was verified through code inspection, focused integration tests, workspace tests, clippy, and targeted data-flow tracing. Visual/native frontend behavior is explicitly Phase 2 scope and is not required for this phase.

### Gaps Summary

No blocking gaps remain. The previous functional gaps remain closed and the code review remediation is now verified in the implementation:

1. Human transcript rendering is exposed through `yune-cli frontend --output human`.
2. CLI output selector exists via `--output json|human`, defaulting to JSON.
3. Schema IDs are validated as logical IDs for both CLI frontend runs and frontend fixture replay.
4. Fixture replay comparison preserves whitespace inside JSON strings and extracts only root-level `schema_id`/`sequence` fields.
5. Frontend transcript input comes from `RimeApi::get_input`.
6. `{Tab}` maps to the RIME Tab keycode.

The CLI frontend surrogate satisfies the Phase 1 goal: it exercises `yune-rime-api` setup, schema selection, key processing, per-key transcript rendering, and transcript replay from the `yune-cli` workflow while preserving owned module boundaries and facade-only `main.rs` orchestration.

---

_Verified: 2026-04-28T21:14:31Z_
_Verifier: Claude (gsd-verifier)_
