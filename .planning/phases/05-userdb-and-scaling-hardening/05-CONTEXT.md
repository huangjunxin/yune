# Phase 5: UserDB And Scaling Hardening - Context

**Gathered:** 2026-04-30
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 5 moves user dictionary behavior beyond the current plain text shim into librime-observable personalization compatibility and closes remaining quality/scaling hardening for this milestone. It covers userdb storage abstraction, backup/restore/recovery/sync/rollback semantics, commit-driven learning, frequency updates, predictive lookup, backdated scan behavior, and targeted test-suite/module hardening. It does not implement the future AI-native memory layer, remote or local LLM providers, plugin ABI compatibility, a new graphical frontend, or broad schema/plugin ecosystems beyond what userdb behavior requires.

</domain>

<decisions>
## Implementation Decisions

### UserDB Storage Boundary
- **D-01:** Phase 5 should replace the current plain text-only userdb shim with a focused user dictionary storage abstraction that can model librime-observable LevelDB/userdb behavior while keeping `yune-core` independent from C ABI pointers and storage engine details.
- **D-02:** The implementation should prefer behavior compatibility over cloning LevelDB internals. If direct LevelDB compatibility is too large for this milestone, the acceptable alternative is a documented compatible abstraction with the same observable import/export, lookup, transaction, and recovery behavior for covered fixtures.
- **D-03:** Existing levers/userdb C ABI functions remain the external compatibility boundary. Storage-specific code should live in `crates/yune-rime-api/src/userdb.rs` or focused submodules, with runtime path/resource-ID validation preserved before filesystem access.
- **D-04:** User dictionary names remain logical resource IDs. Phase 5 must keep the Phase 2 resource-ID protections for userdb APIs and must not treat dict names as arbitrary paths, even when adding snapshot/import/export behavior.

### Snapshot, Recovery, Sync, And Rollback Semantics
- **D-05:** Snapshot backup, restore, import, export, sync, and upgrade behavior should be modeled as explicit transaction boundaries with deterministic local file effects. Tests should cover successful changes and failed/partial operations before optimizing storage.
- **D-06:** Recovery and rollback should be fail-closed: interrupted or malformed userdb state should preserve the last valid dictionary state or produce an explicit failure rather than silently truncating, duplicating, or accepting corrupt learning data.
- **D-07:** Sync behavior should be compatibility-oriented and conflict-aware enough for librime-observable semantics. The current whole-file append/dedupe shim is only a baseline to replace or wrap; Phase 5 should not claim sync compatibility from plain line merging alone.
- **D-08:** Deployment maintenance may clean up legacy userdb recovery artifacts only when the cleanup behavior is covered by focused tests and does not destroy valid current userdb data.

### Learning, Frequency, Predictive Lookup, And Backdated Scan
- **D-09:** Learning should be commit-driven through normal runtime/session flows. Candidate commits should update userdb state and later lookup/ranking behavior through deterministic tests, not parser-only checks.
- **D-10:** Frequency updates and predictive lookup should affect runtime candidate quality/order through existing translator/ranker/filter boundaries where possible. They should remain classic RIME personalization behavior, not AI-native ranking or memory.
- **D-11:** Backdated scan behavior should be represented only to the extent it changes observable userdb learning/ranking/prediction semantics for covered schemas. If librime internals are more complex than the current milestone can safely model, record the unsupported parts as structured findings rather than adding vague shims.
- **D-12:** History translator behavior and existing optional `CandidateRanker` hooks are reusable context but are not substitutes for userdb learning. Phase 5 should keep AI/native ranker work out of scope and should not route USERDB requirements through `MockAiRanker` or future AI memory APIs.

### Scaling And Quality Hardening
- **D-13:** Remaining oversized tests should be split only along behavior ownership boundaries when doing so reduces future risk. Mechanical test movement must not be mixed with userdb behavior changes in the same commit.
- **D-14:** Userdb and scaling hardening should preserve the established ownership rule: `lib.rs` and `main.rs` stay facade/orchestration glue; owned behavior belongs in userdb/storage, runtime/deployment, core translator/ranking, or focused test modules.
- **D-15:** Every Phase 5 plan should name the owning implementation module, owning test module, and librime comparison target before implementation. Focused tests should precede behavior changes when ranking, persistence, recovery, or ABI-visible file operations change.
- **D-16:** Phase quality gates should include focused package tests for changed modules, `cargo fmt`, relevant `cargo test` targets, `cargo test --workspace` when shared behavior changes, and `cargo clippy --workspace --all-targets -- -D warnings` for final phase closure.

### Claude's Discretion
- Exact storage backend shape, fixture byte/text formats, module split names, transaction log format, and selected librime comparison schemas are left to research and planning, provided the decisions above remain true.
- The planner may split Phase 5 plans by storage lifecycle, runtime learning/ranking, and quality hardening as the roadmap suggests, or adjust boundaries if dependencies make a different sequence safer.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope And Requirements
- `.planning/ROADMAP.md` — Phase 5 goal, success criteria, and planned slices for userdb storage, learning/ranking, and quality hardening.
- `.planning/REQUIREMENTS.md` — `USERDB-01` through `USERDB-03` and `QUAL-03` through `QUAL-04`, which define Phase 5 storage, lifecycle, learning, scaling, and gate requirements.
- `.planning/PROJECT.md` — Compatibility oracle, typed Rust architecture, local-first AI boundary, privacy expectations, and module/test ownership constraints.

### Prior Phase Context
- `.planning/phases/04-compiled-dictionary-data/04-CONTEXT.md` — Confirms LevelDB/userdb learning, predictive updates, plugin translators, and AI-native ranking were excluded from Phase 4 and deferred to Phase 5 or later.
- `.planning/phases/03-schema-pipeline-depth/03-CONTEXT.md` — Records `memory`, `poet`/`grammar`, contextual translation, compiled-payload-dependent behavior, and userdb learning/storage as explicit deferrals rather than hidden shims.
- `.planning/phases/02-native-abi-validation-and-runtime-safety/02-CONTEXT.md` — Establishes native/frontend-like ABI validation, runtime safety boundaries, and logical resource-ID validation that userdb file APIs must preserve.

### Codebase Maps And Existing Analysis
- `.planning/codebase/ARCHITECTURE.md` — Engine/session architecture, translator/ranker boundaries, RIME ABI facade ownership, runtime paths, and module ownership anti-patterns.
- `.planning/codebase/INTEGRATIONS.md` — Current user dictionary files, sync snapshot paths, levers/userdb APIs, runtime directory inputs, and absence of LevelDB dependency.
- `.planning/codebase/CONCERNS.md` — Plain text userdb sync/scaling limits, full user dictionary behavior gap, large compatibility test concerns, global state risks, and distribution-scale performance gaps.
- `.planning/codebase/TESTING.md` — Existing Rust test organization, ABI test helpers, userdb/levers/frontend-client testing patterns, and workspace quality commands.
- `docs/analysis.md` — Existing compatibility analysis for userdb behavior, learning, predictive lookup, and remaining milestone risks.
- `docs/roadmap.md` — Prior roadmap context for userdb hardening and how it closes the compatibility milestone before AI-native work.
- `docs/refactor-plan.md` — Module/test ownership rule and refactor guardrails that constrain Phase 5 quality hardening.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `crates/yune-rime-api/src/userdb.rs` — Current levers/userdb ABI implementation, plain `.userdb` file handling, backup/restore/import/export/sync helpers, and logical user dict validation call sites.
- `crates/yune-rime-api/src/resource_id.rs` — Existing logical resource-ID validation, including user dictionary name rejection for paths and `.userdb` suffixes.
- `crates/yune-rime-api/src/runtime.rs` — Runtime user, sync, staging, and prebuilt directory resolution that userdb storage must continue to use.
- `crates/yune-rime-api/src/deployment.rs` — Maintenance/sync/cleanup behavior and current legacy userdb artifact cleanup hooks.
- `crates/yune-rime-api/src/levers.rs` and `crates/yune-rime-api/src/api_table.rs` — Levers API surface and function-table exposure for user dictionary operations.
- `crates/yune-core/src/engine.rs` — Commit recording, candidate refresh, translator/filter/ranker order, and the right runtime seam for commit-driven learning effects.
- `crates/yune-core/src/translator/mod.rs` — `HistoryTranslator`, table/reverse lookup, and candidate quality/order behavior that Phase 5 learning and prediction must integrate with carefully.
- `crates/yune-rime-api/src/tests/userdb.rs`, `levers.rs`, `resource_id.rs`, `deployment.rs`, and `tests/frontend_client.rs` — Existing userdb, resource validation, maintenance, and frontend-style coverage that should be extended or split by behavior ownership.

### Established Patterns
- ABI-facing compatibility tests use `test_guard()`, unique runtime dirs, deployed local files, function-table calls where frontend behavior matters, and direct assertions on file outputs and returned status values.
- Resource IDs from C APIs and schema data are validated as logical IDs before runtime-root joins.
- Core behavior remains typed Rust and deterministic; ABI pointer ownership stays in `yune-rime-api` modules.
- Candidate order changes need focused tests before changing translator, filter, ranker, or learning-related quality calculations.
- Large tests should be split by behavior ownership only when it reduces future risk, not as broad mechanical churn mixed with semantic work.

### Integration Points
- `RimeLeversBackupUserDict`, `RimeLeversRestoreUserDict`, `RimeLeversExportUserDict`, `RimeLeversImportUserDict`, and iterator APIs are the ABI-visible userdb boundary.
- `sync_all_user_dicts()` and `user_dict_upgrade()` are current deployment/maintenance integration points for sync and upgrade behavior.
- `Engine::commit_candidate` and commit-history recording are the natural runtime signal for learning updates, but persistent storage effects should be introduced through a narrow owned interface rather than broad session mutation.
- `Engine::refresh_candidates` currently sorts translator output by quality, then filters, then optional rankers. Userdb frequency/prediction integration must preserve deterministic classic behavior and avoid AI ranker coupling.

</code_context>

<specifics>
## Specific Ideas

- Treat `/Users/trenton/Projects/librime` as the oracle for observable userdb import/export, sync, recovery, learning, frequency, and predictive lookup behavior.
- Use normal ABI/session flows for learning and ranking tests wherever possible: deploy/select schema, process keys, commit candidates, reopen or refresh sessions, then observe candidate order or persisted userdb state.
- Keep the current plain text userdb behavior as legacy compatibility input only if it helps migration tests; do not let it define the Phase 5 target semantics.
- Keep Phase 5 classic-personalization only: userdb learning and prediction are not the future AI-native memory layer.

</specifics>

<deferred>
## Deferred Ideas

- AI-native memory, source-labeled AI candidates, local/remote LLM providers, privacy policy UI, and context provider design remain a future milestone.
- Full librime C++ plugin ABI compatibility and Lua/octagram/predict/proto plugin ecosystems remain deferred.
- Real graphical frontend integration remains outside Phase 5 unless a userdb ABI behavior requires a focused frontend-style fixture.

</deferred>

---

*Phase: 05-userdb-and-scaling-hardening*
*Context gathered: 2026-04-30*
