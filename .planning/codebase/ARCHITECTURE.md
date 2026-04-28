<!-- refreshed: 2026-04-28 -->
# Architecture

**Analysis Date:** 2026-04-28

## System Overview

```text
+----------------------+          +------------------------------+
| RIME frontends / ABI  |          | Local fixture CLI            |
| `yune-rime-api` C ABI |          | `crates/yune-cli/src/main.rs`|
+----------+-----------+          +---------------+--------------+
           |                                      |
           v                                      v
+---------------------------------------------------------------+
| Compatibility and runtime adapter layer                       |
| `crates/yune-rime-api/src/lib.rs`                             |
| sessions, key conversion, config, deployment, schema install   |
+---------------------+-------------------------+---------------+
                      |                         |
                      v                         v
+----------------------------------+    +------------------------+
| Core input engine                |    | Schema model subset    |
| `crates/yune-core/src/engine.rs` |    | `crates/yune-schema/`  |
| translators, filters, rankers    |    | parse RIME YAML shape |
+----------------+-----------------+    +------------------------+
                 |
                 v
+---------------------------------------------------------------+
| Runtime state, candidates, dictionaries, fixture output        |
| `crates/yune-core/src/state.rs`, `crates/yune-core/src/`       |
+---------------------------------------------------------------+
```

## Component Responsibilities

| Component | Responsibility | File |
|-----------|----------------|------|
| Workspace | Defines the Rust workspace, crate membership, shared edition, MSRV, license, and lint level. | `Cargo.toml` |
| Core facade | Exposes the stable Rust API for engine state, translators, filters, dictionary helpers, key parsing, and the AI ranking hook. | `crates/yune-core/src/lib.rs` |
| Core engine | Owns composition state mutation, candidate refresh, selection, commits, paging, and trait invocation order. | `crates/yune-core/src/engine.rs` |
| Core state | Defines candidates, composition, context, status, snapshots, and commit history records. | `crates/yune-core/src/state.rs` |
| Key model | Converts RIME-style sequence names into typed `KeyEvent` values used by the core and ABI shim. | `crates/yune-core/src/key.rs` |
| Dictionary model | Parses RIME table dictionaries, imports, packs, preset vocabulary, compiled metadata, checksums, and table encoder rules. | `crates/yune-core/src/dictionary/` |
| Translators | Provide echo, table, reverse lookup, history, switch, and schema-list candidate generation. | `crates/yune-core/src/translator/mod.rs` |
| Filters | Reorder, deduplicate, convert, tag-gate, or annotate candidates after translation. | `crates/yune-core/src/filter/mod.rs` |
| RIME ABI facade | Exports C ABI entrypoints, key routing, public function table exports, and cross-module glue. | `crates/yune-rime-api/src/lib.rs` |
| ABI layout | Defines librime-shaped structs, type aliases, and function table structs. | `crates/yune-rime-api/src/abi.rs` |
| API table | Builds static `RimeApi` and `RimeLeversApi` function tables. | `crates/yune-rime-api/src/api_table.rs` |
| Session registry | Owns process-wide session IDs, session state, lifecycle checks, cleanup, and session lookup helpers. | `crates/yune-rime-api/src/session.rs` |
| Runtime paths | Stores process-wide RIME traits and resolves shared, user, prebuilt, staging, sync, and log paths. | `crates/yune-rime-api/src/runtime.rs` |
| Schema install | Converts deployed schema YAML into core translators, filters, segment tags, and segmentor data. | `crates/yune-rime-api/src/schema_install.rs` |
| Schema selection | Applies schema resets and installs all session processors and core chains for a selected schema. | `crates/yune-rime-api/src/schema_selection.rs` |
| Processor modules | Implement schema-loaded key handling for ascii composer, chord composer, editor, key binder, navigator, punctuation, recognizer, selector, shape, and speller. | `crates/yune-rime-api/src/processors/` |
| Config API | Exposes deployed/user config open, scalar access, mutation, iterators, and YAML-backed config state. | `crates/yune-rime-api/src/config_api.rs` |
| Config compiler | Handles librime-style include, patch, custom patch, and build-info freshness behavior. | `crates/yune-rime-api/src/config_compiler.rs` |
| Deployment | Implements initialization, maintenance, deploy, sync, staging, and notification-facing runtime operations. | `crates/yune-rime-api/src/deployment.rs` |
| Candidate/context APIs | Copy engine state into caller-owned C ABI structures and candidate iterators. | `crates/yune-rime-api/src/context_api.rs`, `crates/yune-rime-api/src/candidate_api.rs` |
| Schema crate | Provides a small typed RIME schema subset parser independent of the runtime ABI shim. | `crates/yune-schema/src/lib.rs` |
| CLI harness | Runs deterministic sample sequences and checks JSON fixtures directly against `yune-core`. | `crates/yune-cli/src/` |

## Pattern Overview

**Overall:** Layered Rust workspace with a deterministic core and compatibility adapters.

**Key Characteristics:**
- Keep externally observable RIME compatibility at the boundary in `crates/yune-rime-api/src/`.
- Keep reusable engine behavior in `crates/yune-core/src/` behind Rust traits and typed state.
- Keep `lib.rs` and `main.rs` as facades and orchestration glue; put owned behavior in focused modules.
- Convert deployed YAML configuration into installed session processors, translators, filters, and segment tags.
- Preserve classic input behavior when optional ranking or schema behavior is absent.

## Layers

**Workspace Layer:**
- Purpose: Define crate composition and shared Rust metadata.
- Location: `Cargo.toml`
- Contains: workspace members, resolver, edition, license, MSRV, lint policy.
- Depends on: Not applicable.
- Used by: all crates under `crates/`.

**Core Engine Layer:**
- Purpose: Represent input-method state and deterministic candidate generation.
- Location: `crates/yune-core/src/`
- Contains: `Engine`, `Context`, `Status`, key parsing, translators, filters, dictionary parsing, table encoding, punctuation, spelling algebra, AI ranker trait.
- Depends on: `regex` for parsing and pattern application.
- Used by: `crates/yune-cli`, `crates/yune-rime-api`.

**RIME Compatibility Layer:**
- Purpose: Present a librime-shaped C ABI and translate frontend calls into core engine mutations.
- Location: `crates/yune-rime-api/src/`
- Contains: ABI structs, process-wide runtime state, session registry, config/deployment APIs, function table builders, schema installation, processor routing, FFI memory cleanup.
- Depends on: `yune-core`, `libc`, `regex`, `serde_yaml`.
- Used by: integration tests and any frontend loading the exported ABI symbols.

**Schema Model Layer:**
- Purpose: Parse a minimal standalone RIME schema subset into typed Rust values.
- Location: `crates/yune-schema/src/lib.rs`
- Contains: `Schema`, `EngineSpec`, YAML parsing, missing-field errors.
- Depends on: `serde`, `serde_yaml`.
- Used by: schema compatibility work that needs a small typed schema model.

**CLI Fixture Layer:**
- Purpose: Provide deterministic command-line fixture generation and checking for core behavior.
- Location: `crates/yune-cli/src/`
- Contains: argument parsing, sample core runner, fixture comparison, JSON transcript rendering.
- Depends on: `yune-core`.
- Used by: fixture workflows and smoke checks.

**Documentation and Fixture Layer:**
- Purpose: Describe current compatibility scope and store checked-in expected outputs.
- Location: `README.md`, `docs/`, `fixtures/`
- Contains: roadmap, analysis, refactor guidance, sample JSON fixtures.
- Depends on: source behavior but is not compiled.
- Used by: planning, compatibility context, CLI fixture tests.

## Data Flow

### Primary RIME API Key Path

1. Frontend obtains or calls the ABI through `rime_get_api` and `RimeApi.process_key` (`crates/yune-rime-api/src/api_table.rs:62`, `crates/yune-rime-api/src/api_table.rs:84`).
2. `RimeProcessKey` validates the session, mask, and keycode, then looks up mutable `SessionState` (`crates/yune-rime-api/src/lib.rs:318`, `crates/yune-rime-api/src/session.rs:32`).
3. Keycodes are converted into `yune_core::KeyEvent` values (`crates/yune-rime-api/src/lib.rs:904`, `crates/yune-core/src/key.rs:55`).
4. ABI-level processors run before the core engine: ascii composer, key binder, selector, navigator, chord composer, recognizer, punctuation, alternative selection, speller, editor, and shape processing (`crates/yune-rime-api/src/lib.rs:392`, `crates/yune-rime-api/src/lib.rs:1413`).
5. Unhandled typed keys fall through to `Engine::process_key_event` (`crates/yune-rime-api/src/lib.rs:1470`, `crates/yune-core/src/engine.rs:141`).
6. The core engine refreshes candidates by invoking translators, sorting by quality, applying filters, and allowing rankers to provide ready reorders (`crates/yune-core/src/engine.rs:711`).
7. Commits are buffered in `SessionState.unread_commit` for `RimeGetCommit`, while context/status reads copy snapshots into caller-owned C structs (`crates/yune-rime-api/src/lib.rs:1128`, `crates/yune-rime-api/src/context_api.rs:18`, `crates/yune-rime-api/src/context_api.rs:54`, `crates/yune-rime-api/src/context_api.rs:246`).

### Schema Selection and Installation Flow

1. Frontend selects a schema with `RimeSelectSchema` (`crates/yune-rime-api/src/schema_selection.rs:48`).
2. `apply_schema_to_session` resets core translators, filters, processors, paging, composition, buffered input, and unread commits (`crates/yune-rime-api/src/schema_selection.rs:83`).
3. Runtime config roots load deployed YAML from staging before prebuilt data (`crates/yune-rime-api/src/lib.rs:1551`, `crates/yune-rime-api/src/lib.rs:1566`).
4. Schema installers add segment tags, processors, translators, and filters in fixed order (`crates/yune-rime-api/src/schema_selection.rs:111`).
5. Translator and filter chain installers map `engine/translators` and `engine/filters` component prescriptions to core implementations (`crates/yune-rime-api/src/schema_install.rs:20`, `crates/yune-rime-api/src/schema_install.rs:236`).

### CLI Fixture Flow

1. CLI arguments select `run`, `check`, or help (`crates/yune-cli/src/main.rs:24`, `crates/yune-cli/src/args.rs:11`).
2. `run_sequence` builds a sample `Engine`, installs punctuation and table translators, processes the key sequence, and returns a fixture output (`crates/yune-cli/src/sample_core.rs:19`).
3. `FixtureOutput::to_json` serializes schema, commits, context, candidates, and status using handwritten deterministic JSON (`crates/yune-cli/src/transcript.rs:12`).
4. `check_fixture` extracts the sequence from a JSON fixture, reruns the sample, normalizes whitespace, and compares output (`crates/yune-cli/src/fixture.rs:5`).

**State Management:**
- `yune-core` keeps session-local mutable state inside each `Engine`.
- `yune-rime-api` keeps process-wide mutable runtime state in `OnceLock<Mutex<_>>` registries and an `AtomicBool` service flag.
- FFI functions copy Rust-owned state into caller-owned C structures and pair allocations with explicit `RimeFree*` functions.
- Config data is YAML-backed `serde_yaml::Value` stored behind `RimeConfig.ptr`.

## Key Abstractions

**Engine:**
- Purpose: Own one input-method state machine.
- Examples: `crates/yune-core/src/engine.rs`, `crates/yune-core/src/state.rs`
- Pattern: mutable session object with plug-in translators, filters, and rankers.

**Translator:**
- Purpose: Convert current composition input into candidate vectors.
- Examples: `crates/yune-core/src/lib.rs`, `crates/yune-core/src/translator/mod.rs`, `crates/yune-core/src/punctuation.rs`
- Pattern: `Send + Sync` trait object installed into `Engine.translators`.

**CandidateFilter:**
- Purpose: Mutate candidate vectors after translation.
- Examples: `crates/yune-core/src/lib.rs`, `crates/yune-core/src/filter/mod.rs`
- Pattern: `Send + Sync` trait object with option-aware and context-aware hooks.

**CandidateRanker:**
- Purpose: Allow optional non-blocking candidate reranking without changing fallback order.
- Examples: `crates/yune-core/src/lib.rs`
- Pattern: `RerankResult::Pending` preserves classic order; `RerankResult::Ready` replaces candidate order.

**SessionState:**
- Purpose: Bridge one RIME session to one `Engine` plus schema-loaded processor state.
- Examples: `crates/yune-rime-api/src/session.rs`, `crates/yune-rime-api/src/processors/`
- Pattern: registered by numeric `RimeSessionId` in a mutex-protected process-wide registry.

**Runtime Config:**
- Purpose: Resolve deployed/user YAML and expose librime-style path and config APIs.
- Examples: `crates/yune-rime-api/src/runtime.rs`, `crates/yune-rime-api/src/config.rs`, `crates/yune-rime-api/src/config_api.rs`
- Pattern: process-wide path state plus per-config heap state owned through `RimeConfig.ptr`.

**Schema Component Prescription:**
- Purpose: Split `component@namespace` declarations and install matching processors, translators, filters, and segmentors.
- Examples: `crates/yune-rime-api/src/schema_install.rs`
- Pattern: string component registry implemented with match statements and config helpers.

## Entry Points

**Rust Core API:**
- Location: `crates/yune-core/src/lib.rs`
- Triggers: Rust crates instantiate `Engine` or use public parser/type exports.
- Responsibilities: stable API surface for core engine composition, dictionary parsing, translation/filter/ranking, and key parsing.

**RIME C ABI:**
- Location: `crates/yune-rime-api/src/lib.rs`, `crates/yune-rime-api/src/api_table.rs`
- Triggers: C ABI symbol lookup, `rime_get_api`, direct exported `Rime*` calls.
- Responsibilities: maintain librime-compatible function table, session lifecycle, key processing, context/status/commit reads, config APIs, deployment helpers.

**CLI Binary:**
- Location: `crates/yune-cli/src/main.rs`
- Triggers: `cargo run -p yune-cli -- ...`
- Responsibilities: run sample sequences, check fixtures, print deterministic JSON or help.

**Schema Parser API:**
- Location: `crates/yune-schema/src/lib.rs`
- Triggers: Rust callers parse schema YAML.
- Responsibilities: parse schema metadata and engine component lists into typed structs.

**Workspace Tests:**
- Location: `crates/yune-core/src/lib.rs`, `crates/yune-rime-api/src/tests/`, `crates/yune-rime-api/tests/frontend_client.rs`, `crates/yune-cli/src/fixture.rs`
- Triggers: `cargo test --workspace`.
- Responsibilities: lock down core behavior, ABI compatibility, frontend-style function table use, and fixture stability.

## Architectural Constraints

- **Threading:** Runtime and sessions use mutex-protected process globals in `crates/yune-rime-api/src/session.rs` and `crates/yune-rime-api/src/runtime.rs`; the core engine itself is ordinary single-session mutable state.
- **Global state:** `sessions()`, `service_started()`, `runtime_paths()`, notification handler state, API function tables, module registries, state-label cache, and config/user dictionary process state are all module-level globals under `crates/yune-rime-api/src/`.
- **Unsafe boundary:** C ABI functions dereference caller pointers and allocate C strings. Keep unsafe pointer work in ABI/config/context/candidate/FFI memory modules, not in `yune-core`.
- **Circular imports:** Rust module cycles are not detected by the compiler and are not present. The RIME ABI facade uses `pub use` and `pub(crate) use` re-exports from `crates/yune-rime-api/src/lib.rs`; avoid introducing module dependencies that require moving owned logic back into the facade.
- **Compatibility boundary:** External behavior is shaped by RIME/librime contracts in `crates/yune-rime-api/src/`; internal Rust design can stay idiomatic when the boundary remains compatible.
- **Project skills:** `.codex/skills/` and `.agents/skills/` are not detected in this repository.

## Anti-Patterns

### Adding Owned Behavior To Facades

**What happens:** New engine, processor, config, or ABI behavior is added directly to `crates/yune-core/src/lib.rs`, `crates/yune-rime-api/src/lib.rs`, or `crates/yune-cli/src/main.rs`.
**Why it's wrong:** These files already act as public surfaces and orchestration glue; growing them hides ownership boundaries and makes focused compatibility testing harder.
**Do this instead:** Add core behavior under `crates/yune-core/src/`, ABI behavior under the matching `crates/yune-rime-api/src/*.rs` module, processor behavior under `crates/yune-rime-api/src/processors/`, and CLI behavior under focused `crates/yune-cli/src/*.rs` modules.

### Bypassing Schema Installation

**What happens:** Session-specific translator, filter, option, or processor state is mutated ad hoc from a new API path.
**Why it's wrong:** Schema reset and install order lives in `apply_schema_to_session`; bypassing it creates session state that is not reproducible from deployed config.
**Do this instead:** Extend the installers invoked by `crates/yune-rime-api/src/schema_selection.rs:111` and keep component-specific parsing in `crates/yune-rime-api/src/schema_install.rs` or `crates/yune-rime-api/src/processors/`.

### Leaking ABI Allocation Ownership Into Core

**What happens:** Core types or algorithms start depending on C pointers, `CString`, or caller-owned ABI structures.
**Why it's wrong:** `yune-core` is the deterministic Rust engine layer and is used directly by `yune-cli`; ABI memory ownership belongs to the RIME compatibility layer.
**Do this instead:** Keep C allocation, pointer validation, and `RimeFree*` pairing in `crates/yune-rime-api/src/context_api.rs`, `crates/yune-rime-api/src/candidate_api.rs`, `crates/yune-rime-api/src/config_api.rs`, and `crates/yune-rime-api/src/ffi_memory.rs`.

### Hand-Rolling Duplicate Config Lookup

**What happens:** New code parses slash-separated config paths or scalar coercions independently.
**Why it's wrong:** Existing helpers encode librime-style scalar and path behavior; duplicate parsing drifts from compatibility tests.
**Do this instead:** Use `find_config_value`, `config_scalar_string`, `config_scalar_bool`, `config_scalar_int`, `config_scalar_double`, and `set_config_value` from `crates/yune-rime-api/src/config.rs` and `crates/yune-rime-api/src/lib.rs`.

## Error Handling

**Strategy:** Public Rust APIs use `Result` for parse failures, C ABI entrypoints return librime-style booleans/nulls and leave detailed behavior to tests.

**Patterns:**
- Return typed errors for core parser failures: `TableDictionaryParseError`, `TableEncoderFormulaError`, `KeySequenceParseError`, `SchemaParseError`.
- Return `FALSE` or null from C ABI functions when session IDs, pointers, masks, config handles, or string conversions are invalid.
- Use explicit `// SAFETY:` comments around pointer operations in ABI-facing unsafe functions.
- Convert lossy or invalid C strings defensively at the ABI boundary.
- Keep commit failures as `None` in core engine methods such as `Engine::commit_composition`.

## Cross-Cutting Concerns

**Logging:** No structured logging framework is present. Runtime setup stores `log_dir` and `app_name` in `crates/yune-rime-api/src/runtime.rs`; operational messages use notification callbacks in `crates/yune-rime-api/src/notifications.rs`.

**Validation:** Core parsers validate input through typed parse errors; ABI functions validate null pointers, data sizes, session IDs, key masks, and string conversions before mutating state.

**Authentication:** Not applicable. This repository has no user authentication layer.

**Configuration:** RIME configuration is YAML-backed and resolved through runtime paths. Deployed config lookup prefers staging over prebuilt data; user config lookup uses the user data directory.

**Compatibility Testing:** Source-level unit tests live beside core code and ABI test modules. Frontend-style ABI testing lives in `crates/yune-rime-api/tests/frontend_client.rs`.

---

*Architecture analysis: 2026-04-28*
