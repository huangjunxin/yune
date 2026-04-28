# Codebase Concerns

**Analysis Date:** 2026-04-28

## Tech Debt

**Workspace lints are declared but not enabled by member crates:**
- Issue: `Cargo.toml` declares `unsafe_code = "forbid"` and clippy lint policy under `[workspace.lints]`, but the member manifests do not opt in with `[lints] workspace = true`. The ABI crate contains extensive required unsafe code, so the intended policy is ambiguous rather than enforceable.
- Files: `Cargo.toml`, `crates/yune-rime-api/Cargo.toml`, `crates/yune-core/Cargo.toml`, `crates/yune-schema/Cargo.toml`, `crates/yune-cli/Cargo.toml`
- Impact: New unsafe code and clippy regressions rely on command-line discipline instead of manifest-enforced policy.
- Fix approach: Add explicit crate-level lint configuration. For `crates/yune-rime-api`, allow unsafe code intentionally and keep `unsafe_op_in_unsafe_fn`/FFI lint expectations explicit; for safe crates, opt into the workspace lint policy.

**Core tests remain concentrated in the public facade:**
- Issue: `crates/yune-core/src/lib.rs` is about 5,082 lines and mostly compatibility tests, while implementation now lives in focused modules.
- Files: `crates/yune-core/src/lib.rs`, `crates/yune-core/src/key.rs`, `crates/yune-core/src/engine.rs`, `crates/yune-core/src/dictionary/source.rs`
- Impact: Test ownership is harder to infer from implementation ownership, and focused changes require scanning a very large facade file.
- Fix approach: Move tests beside their modules (`key.rs`, `engine.rs`, `dictionary/source.rs`, `translator/mod.rs`, `filter/mod.rs`) without behavior changes.

**RIME API facade still owns large cross-module glue:**
- Issue: `crates/yune-rime-api/src/lib.rs` is about 1,851 lines and owns ABI exports, key dispatch, schema switch behavior, context menu settings, config path helpers, and shared utility functions.
- Files: `crates/yune-rime-api/src/lib.rs`, `crates/yune-rime-api/src/processors/mod.rs`, `crates/yune-rime-api/src/schema_install.rs`, `crates/yune-rime-api/src/schema_selection.rs`
- Impact: Key-processing changes can touch unrelated ABI/config/schema concerns, increasing review risk.
- Fix approach: Keep `lib.rs` as the export index and move key dispatch/menu/switch helper groups into focused modules as new behavior slices require them.

**CLI frontend module is a placeholder:**
- Issue: `crates/yune-cli/src/rime_frontend.rs` contains only a reserved comment, and `crates/yune-cli/src/main.rs` still routes `run` through `yune-core` via `sample_core`.
- Files: `crates/yune-cli/src/rime_frontend.rs`, `crates/yune-cli/src/main.rs`, `crates/yune-cli/src/sample_core.rs`, `docs/roadmap.md`, `docs/refactor-plan.md`
- Impact: The CLI does not exercise `yune-rime-api` setup, deployment, schema selection, context/status/commit reads, or real frontend-like lifecycle behavior.
- Fix approach: Implement the RIME API-backed CLI frontend in `rime_frontend.rs`, then keep the existing core runner only as a fixture compatibility path.

## Known Bugs

**No production TODO/FIXME markers detected:**
- Symptoms: The production codebase has no `TODO`, `FIXME`, `HACK`, or `XXX` markers in `crates/*/src` outside tests.
- Files: `crates/yune-core/src`, `crates/yune-rime-api/src`, `crates/yune-cli/src`, `crates/yune-schema/src`
- Trigger: Not applicable.
- Workaround: Use `docs/analysis.md`, `docs/roadmap.md`, and this document as the active issue inventory.

**Switcher settings have no API-level destroy path:**
- Symptoms: `RimeSwitcherSettingsInit` allocates a `RimeSwitcherSettings` and inserts pointer-keyed entries into three global registries, but the levers API exposes no matching switcher-settings destroy function. Tests manually `drop(Box::from_raw(settings))`, leaving registry cleanup as caller/test responsibility.
- Files: `crates/yune-rime-api/src/levers.rs`, `crates/yune-rime-api/src/api_table.rs`, `crates/yune-rime-api/src/tests/levers.rs`
- Trigger: Repeated calls to `RimeSwitcherSettingsInit` in a long-lived process.
- Workaround: Reuse a switcher settings object where possible, or restart the process between frontend sessions.

**Maintenance thread API is synchronous/no-op shaped:**
- Symptoms: `RimeStartMaintenance` runs maintenance inline, `RimeIsMaintenancing` always returns `FALSE`, and `RimeJoinMaintenanceThread` is a no-op.
- Files: `crates/yune-rime-api/src/deployment.rs`, `crates/yune-rime-api/tests/frontend_client.rs`, `crates/yune-rime-api/src/tests/deployment.rs`
- Trigger: A frontend that expects librime-style asynchronous maintenance state.
- Workaround: Treat maintenance calls as synchronous and wait for `RimeStartMaintenance`/`RimeDeployWorkspace` to return.

## Security Considerations

**Runtime resource IDs can contain path separators:**
- Risk: Config IDs, schema dictionary names, custom config IDs, and user dictionary names are joined directly onto runtime roots. Inputs containing `../` or platform separators can escape expected data directories when accepted from C callers or schema YAML.
- Files: `crates/yune-rime-api/src/lib.rs`, `crates/yune-rime-api/src/schema_install.rs`, `crates/yune-rime-api/src/userdb.rs`, `crates/yune-rime-api/src/levers.rs`, `crates/yune-rime-api/src/config_api.rs`
- Current mitigation: Paths are rooted under runtime directories and extension suffixes are appended in several paths; read/write operations check file existence or create parent directories.
- Recommendations: Validate resource IDs as logical IDs, not paths. Reject separators, `..`, absolute paths, drive prefixes, and NUL-derived lossy paths before joining roots. Add tests for config open, dictionary loading, custom settings, and userdb import/export.

**FFI ownership depends on exact API pairing:**
- Risk: `CString::into_raw`, `Box::into_raw`, `Vec::from_raw_parts`, and `std::mem::forget` are used for C ABI returns. Calling the wrong free function, freeing twice, or passing foreign pointers can cause undefined behavior.
- Files: `crates/yune-rime-api/src/context_api.rs`, `crates/yune-rime-api/src/candidate_api.rs`, `crates/yune-rime-api/src/schema_api.rs`, `crates/yune-rime-api/src/config_api.rs`, `crates/yune-rime-api/src/ffi_memory.rs`, `crates/yune-rime-api/src/levers.rs`
- Current mitigation: Entry points check null pointers and versioned `data_size` fields where applicable; ownership comments and ABI tests cover many layout/lifecycle cases.
- Recommendations: Keep FFI allocation/free pairs centralized in `ffi_memory.rs`, add debug-only allocation provenance assertions for iterator/list/context pointers, and document caller ownership in generated C headers when headers exist.

**Process-wide module pointers are caller-owned:**
- Risk: `RimeRegisterModule` stores raw module pointers as `usize` and returns them later; the caller must keep module storage alive.
- Files: `crates/yune-rime-api/src/modules.rs`
- Current mitigation: Safety docs state that caller-owned module storage must remain alive, and the built-in levers module is process-owned.
- Recommendations: Keep this API marked unsafe, avoid registering stack-allocated modules in tests/examples, and consider storing owned copies for Yune-native modules.

**Schema-provided regex patterns are compiled without resource limits:**
- Risk: Recognizer patterns from deployed schema config are compiled directly. Rust `regex` avoids catastrophic backtracking, but untrusted large pattern sets can still consume CPU and memory during deployment/session setup.
- Files: `crates/yune-rime-api/src/schema_install.rs`
- Current mitigation: Invalid regex patterns are skipped.
- Recommendations: Bound pattern length/count for untrusted schemas and report skipped patterns through diagnostics.

## Performance Bottlenecks

**Dictionary candidate lookup is linear:**
- Problem: `StaticTableTranslator` scans `entries` for every input refresh, and sentence mode scans entries for each input position.
- Files: `crates/yune-core/src/translator/mod.rs`, `crates/yune-core/src/engine.rs`
- Cause: Dictionaries are stored as `Vec<(String, Candidate)>`; `refresh_candidates` collects all translator output and sorts the full candidate list on every key event.
- Improvement path: Add prefix indexes/trie or code-range indexes for table translators, produce candidates lazily by page, and cache exact/completion lookup results per dictionary generation.

**Schema dictionary loading reparses source YAML:**
- Problem: Selecting/installing a schema reads `.dict.yaml`, import tables, packs, and optional preset vocabulary from text files and parses them into in-memory dictionaries.
- Files: `crates/yune-rime-api/src/schema_install.rs`, `crates/yune-core/src/dictionary/source.rs`, `crates/yune-core/src/dictionary/compiled.rs`
- Cause: Compiled `.table.bin`/`.prism.bin`/`.reverse.bin` support is metadata/rebuild-plan only; source YAML remains the runtime load path.
- Improvement path: Cache parsed dictionaries by runtime path/checksum and implement compiled payload consumption before distribution-scale schemas are used.

**Session and candidate snapshots clone large structures:**
- Problem: Candidate iteration and context retrieval clone candidates or snapshots before converting to C-owned memory.
- Files: `crates/yune-rime-api/src/session.rs`, `crates/yune-rime-api/src/context_api.rs`, `crates/yune-rime-api/src/candidate_api.rs`, `crates/yune-core/src/engine.rs`
- Cause: `session_candidates_snapshot` clones the full candidate vector; `Engine::snapshot` clones the full context; C APIs then allocate another representation.
- Improvement path: Snapshot only the visible page for `RimeGetContext`, expose iterator state over stable candidate IDs, and avoid full-context clone paths for read-only ABI calls.

**User dictionary sync reads whole files:**
- Problem: Sync merges read complete destination and snapshot files into strings and a `HashSet` of all existing lines.
- Files: `crates/yune-rime-api/src/userdb.rs`
- Cause: The current userdb shim is plain text and append-merge based.
- Improvement path: Move toward the planned userdb storage abstraction or stream line-by-line with atomic temp-file replacement.

## Fragile Areas

**C ABI memory and versioned structs:**
- Files: `crates/yune-rime-api/src/abi.rs`, `crates/yune-rime-api/src/context_api.rs`, `crates/yune-rime-api/src/ffi_memory.rs`, `crates/yune-rime-api/src/config_api.rs`, `crates/yune-rime-api/src/levers.rs`
- Why fragile: Struct field availability depends on caller-provided `data_size`; nested pointers have different ownership rules per API; many functions expose borrowed process/config-owned pointers.
- Safe modification: Add ABI layout and lifecycle tests before changing structs, fields, allocation types, or free functions. Preserve data-size compatibility and pointer lifetime comments.
- Test coverage: Strong focused ABI tests exist, but real frontend lifecycle and misuse behavior are not covered by native clients.

**Key processing dispatch:**
- Files: `crates/yune-rime-api/src/lib.rs`, `crates/yune-rime-api/src/processors/key_binder.rs`, `crates/yune-rime-api/src/processors/speller.rs`, `crates/yune-rime-api/src/processors/editor.rs`, `crates/yune-rime-api/src/processors/chord_composer.rs`, `crates/yune-core/src/engine.rs`
- Why fragile: `RimeProcessKey` validates masks, handles ascii composer switches, key binder redirects, selector/navigator overrides, processor chains, shape processing, and engine fallback in one flow.
- Safe modification: Add focused tests for each branch before changing dispatch order. Keep key mask acceptance, commit buffering, paging, and segment-tag updates observable through ABI tests.
- Test coverage: Broad focused coverage exists, but native frontend modifier timing and release-key sequences remain higher-risk.

**Global process state:**
- Files: `crates/yune-rime-api/src/session.rs`, `crates/yune-rime-api/src/runtime.rs`, `crates/yune-rime-api/src/modules.rs`, `crates/yune-rime-api/src/notifications.rs`, `crates/yune-rime-api/src/levers.rs`, `crates/yune-rime-api/src/api_table.rs`
- Why fragile: Runtime paths, sessions, module pointers, notifications, state-label cache, API tables, and switcher registries are process-wide singletons.
- Safe modification: Reset globals explicitly in tests, avoid holding locks across callbacks or filesystem work, and keep session mutation inside narrow lock scopes.
- Test coverage: Tests use isolation helpers and locks, but there is little multi-threaded concurrency coverage.

**Config path mutation semantics:**
- Files: `crates/yune-rime-api/src/config.rs`, `crates/yune-rime-api/src/config_api.rs`, `crates/yune-rime-api/src/config_compiler.rs`
- Why fragile: Slash paths, list references such as `@next`, `@before`, and `@after`, null-to-container conversion, and lexical map iteration all emulate librime behavior.
- Safe modification: Add compatibility fixtures for each path form before editing `set_config_value`, `list_index`, or config iterator logic.
- Test coverage: Config API and compiler tests are substantial, but path traversal and invalid-resource boundary tests should be added.

**Large compatibility suites:**
- Files: `crates/yune-rime-api/src/tests/schema_processors.rs`, `crates/yune-rime-api/src/tests/schema_selection.rs`, `crates/yune-rime-api/tests/frontend_client.rs`, `crates/yune-core/src/lib.rs`
- Why fragile: Several test files exceed 4,000 lines, making it easy to add near-duplicate fixtures or hide ownership boundaries.
- Safe modification: Split only by behavior ownership and keep fixture helpers shared; avoid mixing mechanical test moves with behavior changes.
- Test coverage: Coverage is broad, but file size slows review and targeted execution.

## Scaling Limits

**Single global session mutex:**
- Current capacity: One process-wide `Mutex<SessionRegistry>` guards all sessions.
- Limit: Concurrent frontend calls across sessions serialize through one lock, and poisoned locks panic through `.expect(...)`.
- Scaling path: Shard session state or store per-session locks after registry lookup; convert poison handling at FFI boundaries into failure returns.

**Unbounded commit history:**
- Current capacity: `Engine` appends every commit to `context.commit_history`; `HistoryTranslator` reads only the configured tail.
- Limit: Long-lived sessions can accumulate unbounded history memory.
- Scaling path: Keep a bounded ring buffer sized by installed history translator needs.

**In-memory source dictionaries:**
- Current capacity: Table dictionaries are loaded into vectors and scanned for lookup.
- Limit: Distribution-scale RIME dictionaries increase startup memory, per-key CPU, candidate sorting, and context clone costs.
- Scaling path: Implement compiled dictionary payload loading, prefix indexes, and lazy candidate paging.

**Plain text userdb sync:**
- Current capacity: User dictionary entries are merged as whole text files.
- Limit: Large user dictionaries require full-file reads, full-line `HashSet` dedupe, and non-atomic overwrites.
- Scaling path: Add LevelDB-compatible/userdb abstraction, atomic writes, and conflict-aware merge semantics.

## Dependencies at Risk

**`serde_yaml`:**
- Risk: RIME compatibility is tied to yaml-cpp/libyaml behavior, while config and schema parsing use `serde_yaml` plus local compatibility shims.
- Impact: Subtle YAML scalar, null, duplicate, merge, or ordering behavior can differ from librime and affect deployed configs/dictionaries.
- Migration plan: Keep compatibility tests for yaml-cpp edge cases, isolate YAML access behind helper functions, and consider a parser layer with explicit librime-compatible normalization.

**`regex`:**
- Risk: Recognizer, spelling algebra, and schema pattern behavior depends on Rust regex semantics rather than librime's exact matching stack.
- Impact: Valid RIME schemas may compile or match differently, especially around unsupported regex constructs.
- Migration plan: Treat each unsupported pattern as a compatibility finding; add diagnostics and fixtures before introducing alternate regex engines.

**`libc`:**
- Risk: `libc` is used for Unix `ctime_r` signature formatting to match librime.
- Impact: Non-Unix builds use a different timestamp format, and signature tests must allow platform differences.
- Migration plan: Keep the Unix-specific path isolated in `crates/yune-rime-api/src/lib.rs` and gate platform-specific tests.

## Missing Critical Features

**Native frontend validation:**
- Problem: ABI tests and `frontend_client.rs` exercise a frontend-style function table, but Squirrel, Weasel, ibus-rime, fcitx-rime, and fcitx5-rime behavior is not represented.
- Blocks: Confidence in struct lifetimes, notification timing, deployment behavior, focus/session lifecycle, and real input-method framework integration.

**RIME API-backed CLI frontend:**
- Problem: `crates/yune-cli/src/rime_frontend.rs` is not implemented, so the CLI cannot act as the planned frontend surrogate.
- Blocks: Scriptable replay through deployment, schema selection, sessions, context/status/commit, and frontend-like rendering.

**Compiled dictionary payload consumption and rebuild execution:**
- Problem: `crates/yune-core/src/dictionary/compiled.rs` handles checksums, metadata, and rebuild-plan primitives, but runtime schema loading still consumes source `.dict.yaml`.
- Blocks: Efficient distribution-scale dictionary startup and compatibility with prebuilt `.table.bin`, `.prism.bin`, and `.reverse.bin` data.

**Full user dictionary behavior:**
- Problem: `crates/yune-rime-api/src/userdb.rs` provides plain file-backed backup/restore/import/export/sync shims rather than LevelDB-backed learning, recovery, transactions, predictive lookup, and frequency updates.
- Blocks: Librime-compatible personalization and production userdb migration.

**Full OpenCC and plugin compatibility:**
- Problem: `SimplifierFilter` uses small built-in character maps, and plugin compatibility is intentionally outside the current compatibility subset.
- Blocks: Real-world schemas depending on OpenCC data chains, Lua/octagram/predict/proto plugins, or C++ plugin ABI.

## Test Coverage Gaps

**Real frontend integration:**
- What's not tested: Native frontend clients and OS input-method framework behavior.
- Files: `crates/yune-rime-api/tests/frontend_client.rs`, `docs/analysis.md`, `docs/roadmap.md`
- Risk: ABI behavior can pass synthetic tests while failing under real callback timing, dynamic loading, focus changes, or frontend memory expectations.
- Priority: High

**Resource path validation:**
- What's not tested: Rejection of `../`, absolute paths, path separators, and odd resource IDs in config IDs, dictionary names, custom config IDs, and user dictionary names.
- Files: `crates/yune-rime-api/src/lib.rs`, `crates/yune-rime-api/src/schema_install.rs`, `crates/yune-rime-api/src/userdb.rs`, `crates/yune-rime-api/src/levers.rs`
- Risk: Untrusted schema or C API inputs can read/write outside intended runtime roots.
- Priority: High

**Concurrency and lock behavior:**
- What's not tested: Multi-threaded session access, notification callbacks under concurrent state changes, poisoned mutex recovery, and switcher registry churn.
- Files: `crates/yune-rime-api/src/session.rs`, `crates/yune-rime-api/src/runtime.rs`, `crates/yune-rime-api/src/notifications.rs`, `crates/yune-rime-api/src/levers.rs`
- Risk: Real frontends may call from multiple threads and expose serialization, panic, or stale pointer behavior.
- Priority: Medium

**Distribution-scale performance:**
- What's not tested: Large dictionaries, large schema chains, many candidates, long user dictionaries, and repeated schema switching under realistic data sizes.
- Files: `crates/yune-core/src/translator/mod.rs`, `crates/yune-core/src/dictionary/source.rs`, `crates/yune-rime-api/src/schema_install.rs`, `crates/yune-rime-api/src/userdb.rs`
- Risk: Focused fixtures can hide per-key linear scans, parse-time costs, and full-file sync bottlenecks.
- Priority: Medium

**Compiled data behavior beyond metadata:**
- What's not tested: Reading table/prism/reverse payloads, rebuild execution, pack checksum chaining at compiled-data level, and source-vs-prebuilt fallback behavior.
- Files: `crates/yune-core/src/dictionary/compiled.rs`, `crates/yune-rime-api/src/schema_install.rs`, `docs/analysis.md`, `docs/roadmap.md`
- Risk: Compiled-data compatibility can regress or remain incomplete while metadata tests pass.
- Priority: Medium

---

*Concerns audit: 2026-04-28*
