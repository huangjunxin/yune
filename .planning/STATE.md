---
gsd_state_version: 1.0
milestone: typeduck-web-browser-validation
milestone_name: TypeDuck-Web Browser Validation (web-first)
status: in-progress
stopped_at: Re-sequenced to web-first — TypeDuck-Web browser validation (Phase 17) reopened as current priority; TypeDuck-Windows (Phases 11–16) parked
last_updated: "2026-06-17T00:00:00.000Z"
last_activity: 2026-06-17
progress:
  total_phases: 17
  completed_phases: 15
  total_plans: 41
  completed_plans: 37
  percent: 90
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-28)

**Core value:** Existing RIME schemas and frontends should behave predictably through Yune's Rust implementation, with every compatibility difference measurable against librime before it is accepted.
**Current focus:** Web-first — finish and validate the TypeDuck-Web browser path (Phase 17: build the WASM artifact, fix adapter mismatches, run a real-browser E2E for an evidence-based GO/NO-GO) before resuming the parked TypeDuck-Windows platform work.

## Current Position

Phase: 17 — TypeDuck-Web Browser Validation
Plan: 17-01 — Build the WASM artifact and fix TypeDuck-Web adapter mismatches (not started)
Next phase: 17-02 real-browser E2E → 17-03 shared engine parity; TypeDuck-Windows platform work (Phases 11–16) stays parked until web is validated
Status: Web-first re-sequencing; a limited local dev-server smoke exists, but the full Emscripten-backed TypeDuck-Web E2E has not run
Last activity: 2026-06-17

Progress: Foundation + TypeDuck-Web build-out + TypeDuck-Windows first pass done; the critical full browser validation (Phase 17) is the current priority and not yet complete

## Performance Metrics

**Velocity:**

- Total plans completed: 38
- Planned Phase 06 plans: 4
- Average duration: -
- Total execution time: -

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 01 | 3/3 | - | - |
| 02 | 3/3 | - | - |
| 03 | 4 | - | - |
| 04 | 4/4 | - | - |
| 05 | 4/4 | - | - |
| 06 | 0/4 | - | - |
| 07 | 3 | - | - |
| 09 | 3 | - | - |

**Recent Trend:**

- Last 5 plans: 02-01, 02-03, 02-02
- Trend: Phase 2 completed after verification gap closure

*Updated after each phase completion*
| Phase 03-schema-pipeline-depth P01 | 2h 5m | 3 tasks | 2 files |
| Phase 04-compiled-dictionary-data P03 | 7min | 3 tasks | 8 files |
| Phase 04-compiled-dictionary-data P04 | 16min | 3 tasks | 8 files |
| Phase 06 P03 | 27min | 3 tasks | 6 files |
| Phase 06-real-frontend-validation-and-benchmark P04 | 12min | 3 tasks | 7 files |
| Phase 07-wasm-build-and-export-contract P01 | 3min | 2 tasks | 2 files |
| Phase 07-wasm-build-and-export-contract P03 | 4min | 3 tasks | 6 files |
| Phase 09-browser-filesystem-and-persistence P03 | 5min | 3 tasks | 3 files |
| Phase 10 P04 | 8m 47s | 3 tasks | 2 files |
| Phase 11 P01 | - | completed | 8 files |
| Phase 12 P01 | - | completed | 8 files |
| Phase 13 P01 | - | completed | 2 files |
| Phase 14 P01 | - | completed | 9 files |
| Phase 15 P01 | - | completed | 2 files |
| Phase 16 P01 | - | partial/blocked | 2 files |

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- Initialization: Existing `docs/analysis.md`, `docs/roadmap.md`,
  `docs/plans/refactor-plan.md`, and `.planning/codebase/` are the source context for
  this GSD project.

- Initialization: External research was skipped for setup because current scope
  is driven by existing project docs and direct librime comparison.

- Initialization: Future compatibility slices must choose module ownership,
  test ownership, and librime comparison target before implementation.

- Phase 2: Native ABI validation uses a Cargo integration-test dynamic loader,
  resolves `rime_get_api`, and exercises the returned `RimeApi` function table
  against the real Cargo-built `yune-rime-api` cdylib.

- Phase 2: Runtime safety fixes are limited to ABI/runtime boundaries; schema
  semantics, compiled dictionary behavior, and userdb storage compatibility stay
  deferred to later phases.

- 03-01: Non-auto previous-match splitting is owned in processors/speller.rs without lib.rs dispatch changes.
- 03-01: Existing editor/navigator/selector and chord/punctuation/fallback behavior is now locked by schema-loaded ABI fixtures.
- 04-03: Represent advanced source and compiled dictionary data on TableDictionary instead of parallel runtime-specific structs.
- 04-03: Use bounded local Yune fixture markers for Phase 04 advanced compiled payload parity while rejecting unsupported librime sections structurally.
- 04-03: Carry compiled reverse dict_settings into runtime table dictionaries so ReverseLookupTranslator observes source and compiled settings through the same API.
- 04-03: Keep LevelDB/userdb learning, predictive updates, plugin translators, and AI-native ranking out of DATA-03.
- 04-04: Correction/tolerance data is represented as normalized TableDictionary metadata and merged through compiled/source dictionary paths.
- 04-04: Lookup expansion preserves original input first, then correction canonicalization, then tolerance candidates.
- 04-04: Correction/tolerance parser counts are capped before allocation and malformed compiled sections fall back with structured diagnostics.
- 06-03: Squirrel/macOS validation is represented as a source-modeled RimeApi lifecycle fixture plus documented direct-run blocker rather than a mandatory app bundle or input-method registration step.
- 06-03: Linux ibus-rime and fcitx-rime validation remains follow-up documentation with safe ABI source-model markers in native_frontends.rs, not a required daemon dependency for cargo test.
- 06-03: Native frontend mismatch capture continues to reuse the Phase 06 host trace schema and sanitized fixture rules instead of inventing a new target-specific trace format.
- Frontend benchmark baselines use a dependency-free Cargo bench target instead of Criterion to preserve MSRV safety and avoid unnecessary benchmark infrastructure.
- BENCH-01/BENCH-02 measurements stay at the rime_get_api / RimeApi function-table boundary rather than direct yune-core calls.
- AI-native readiness is GO WITH CONDITIONS, based on Phase 6 validation and benchmarks while keeping providers, rankers, context policy, memory policy, and privacy controls out of scope.
- TypeDuck-Web adapter seed work added a Yune-owned `yune_typeduck_*` Rust C/WASM bridge, native adapter contract tests, and browser filesystem documentation before formal Phase 7 planning.
- TypeDuck-Web browser integration should proceed through WASM export contract, TypeScript bridge, browser filesystem persistence, and app-shaped E2E before AI-native frontend exposure.
- 07-01: Use scripts/typeduck-exports.txt as the canonical non-prefixed TypeDuck adapter export list.
- 07-01: Document wasm32-unknown-emscripten plus Emscripten EXPORTED_FUNCTIONS/EXPORTED_RUNTIME_METHODS as the browser build contract without changing lib.rs facade wiring.
- Treat missing browser schema/dictionary assets as an init-time failure before starting the process-global RIME service.
- Document Phase 7 as a handoff contract: one active process-global service, host-owned MEMFS/IDBFS layout and sync, and deterministic verified-or-blocked build output.
- Represent stale deployed config recovery as a deterministic test fixture over existing helpers instead of adding metadata heuristics the helper cannot know.
- Keep recovery documentation local-first and caller-owned: explicit assets, explicit sync boundaries, and no browser app/network/cache policy in Phase 09.
- Document userdb persistence as an explicit host sync boundary because current native exports do not expose userdb mutation notifications.
- D-12/TYPEDUCK-E2E-04: Final findings separate TypeDuck-Web app/source blockers, Yune adapter/runtime mismatches, and environment/tooling blockers.
- D-13/TYPEDUCK-E2E-04: Phase ends with NO-GO recommendation for AI-native frontend exposure due to browser validation blockers.
- D-14: AI-native provider calls, candidate generation, ranking, context, memory, privacy controls, and new first-party Yune frontend remain deferred.
- D-15/WIN-TEST-01: TypeDuck-Windows native IME is the next tracked milestone; first unblock Windows test trust before feature work.
- D-16/WIN-ABI-01: Fork-only config list append APIs are the first feature slice after the Windows baseline because they need no external oracle.
- D-17/WIN-ORACLE-01: Comment semantics and Cantonese/Jyutping parity must be driven by TypeDuck-HK/librime v1.1.2 goldens or documented blockers.
- D-18/WIN-ABI-01: TypeDuck fork list append fields are inserted after `config_list_size` and before `config_begin_list`, matching the fork `RimeApi` order; scalar append values follow the existing string-backed `RimeConfigSet*` representation.
- D-19/WIN-ORACLE-01: The v1.1.2 oracle uses `TypeDuck-HK/librime` commit `74cb52b78fb2411137a7643f6c8bc6517acfde69`, `rime-dictionary-lookup-filter` commit `3e4605c4fae99f068df2edb85aaeab5a97752795`, and `TypeDuck-HK/schema` commit `1bed1ae6a0ab48055f073774d7dfd152a171c548`.
- D-20/WIN-COMMENT-01: Candidate comments for TypeDuck-Windows are represented as source-row dictionary lookup payloads (`\f\r1,...\r0,...`) through `dictionary_lookup_filter`; captured source rows now assert byte output against the v1.1.2 fixture. Normal reverse lookup joins use `"; "`, but that join and schema-name prompt parity still need dedicated oracle coverage.
- D-21/WIN-BUILD-01: The native Windows package is produced by `scripts/package-typeduck-windows.ps1`, which builds `yune-rime-api` for `x86_64-pc-windows-msvc`, renames the DLL/import library to `rime.dll`/`rime.lib`, copies TypeDuck fork headers, and smoke-checks `rime_get_api` plus the `config_list_append_string` slot.
- D-22/WIN-PARITY-01: The Cantonese/Jyutping parity suite locks the captured v1.1.2 schema/menu/comment behavior and keeps uncaptured option, completion, correction, schema-menu, and userdb pronunciation behaviors as explicit ignored tests until dedicated oracle fixtures are captured.
- D-23/SEQUENCING: Re-sequenced to web-first per the original plan — validate Yune in a real web browser (Phase 17) before resuming TypeDuck-Windows platform work. Phase 10's NO-GO reflected absent browser evidence (the WASM artifact was never built), not a failed seam. Shared engine slices (comment shaping, Cantonese goldens, baseline fix) are reused by the web path; Windows-specific native packaging is parked until browser validation succeeds.

### Pending Todos

Web-first (current priority):
- Build the TypeDuck-Web WASM artifact (install Emscripten; run the documented build).
- Fix the TypeDuck-Web adapter shape mismatches (`candidate.text` / `candidate.comment` / `context.highlighted`, not the non-existent context-level keys).
- Run the real-browser TypeDuck-Web E2E and record an evidence-based GO/NO-GO.

Shared engine parity (benefits web and Windows):
- Extend the (now non-circular, fixed on main) comment byte-parity test with the `"; "` reverse-lookup joiner and schema-name-in-prompt oracle cases; ideally drive it from real shipped `.dict.yaml` rows rather than authored ones.
- Capture dedicated v1.1.2 goldens for the 5 ignored Cantonese/Jyutping parity cases.

Parked (TypeDuck-Windows platform work; resume after web validated):
- Run the native Windows packaging smoke check on an MSVC host before claiming a verified artifact.

### Blockers/Concerns

- Web validation (Phase 17) needs an Emscripten toolchain to build the WASM artifact — this is the single blocker that prevented the real browser run, and the reason Phase 10 read NO-GO.
- Native Windows packaging (parked) may require MSVC target/toolchain availability outside the Rust workspace.
- Full Cantonese/Jyutping parity and the schema-name-in-prompt sub-contract remain blocked on uncaptured v1.1.2 goldens.

## Deferred Items

Items acknowledged and carried forward from previous milestone close:

| Category | Item | Status | Deferred At |
|----------|------|--------|-------------|
| Plugin compatibility | librime C++ plugin ABI, Lua, octagram, predict, proto | Deferred | Initialization |
| Product frontend | New graphical end-user frontend | Deferred | Initialization |
| AI extension layer | Production local model bridge and opt-in contextual suggestions | Deferred | Initialization |
| Schema semantics | Deeper librime gear behavior beyond ABI/runtime safety | Deferred | Phase 2 |
| Compiled data | `.table.bin`, `.prism.bin`, `.reverse.bin` payload consumption | Deferred | Phase 2 |
| UserDB storage | LevelDB/userdb compatibility beyond plain text shim | Deferred | Phase 2 |

## Session Continuity

Last summary: 2026-06-17
Summary file: .planning/reports/MILESTONE_SUMMARY-v1.0.md

Last session: 2026-06-17T00:00:00.000Z
Stopped at: Re-sequenced to web-first — reopened TypeDuck-Web browser validation as Phase 17; parked TypeDuck-Windows platform work
Resume file: None

**Current Phase:** 17 (TypeDuck-Web Browser Validation) — active; limited local smoke done, full E2E pending — 2026-06-17
**Next Phase:** 17-01 build the WASM artifact + fix adapter mismatches → 17-02 real-browser E2E → 17-03 shared engine parity

**Active Milestone:** TypeDuck-Web Browser Validation (web-first) — prove the engine in a real browser before resuming TypeDuck-Windows
