# Codebase Structure

**Analysis Date:** 2026-04-28

## Directory Layout

```text
yune/
|-- Cargo.toml                         # Workspace manifest and shared Rust metadata
|-- Cargo.lock                         # Locked Rust dependency graph
|-- README.md                          # Project overview, goals, and compatibility scope
|-- crates/
|   |-- yune-core/                     # Deterministic core engine crate
|   |   |-- Cargo.toml
|   |   `-- src/
|   |       |-- lib.rs                 # Public Rust facade and core tests
|   |       |-- engine.rs              # Engine state machine and candidate refresh
|   |       |-- state.rs               # Candidate, context, status, snapshot structs
|   |       |-- key.rs                 # RIME-style key sequence parser and typed keys
|   |       |-- punctuation.rs         # Punctuation translator
|   |       |-- spelling_algebra.rs    # Spelling algebra formulas
|   |       |-- comment_format.rs      # Candidate comment formatting formulas
|   |       |-- dictionary/            # RIME table source, compiled metadata, encoder
|   |       |-- translator/            # Core translator implementations
|   |       `-- filter/                # Core candidate filter implementations
|   |-- yune-rime-api/                 # Librime/RIME-shaped C ABI compatibility crate
|   |   |-- Cargo.toml
|   |   |-- tests/frontend_client.rs   # Frontend-style integration client
|   |   `-- src/
|   |       |-- lib.rs                 # ABI facade, key routing, shared glue
|   |       |-- abi.rs                 # C ABI structs and function table types
|   |       |-- api_table.rs           # Static RimeApi/RimeLeversApi builders
|   |       |-- session.rs             # Session registry and SessionState
|   |       |-- runtime.rs             # Runtime path and trait handling
|   |       |-- schema_install.rs      # Schema component installation
|   |       |-- schema_selection.rs    # Schema selection/reset workflow
|   |       |-- processors/            # Schema-loaded key processors
|   |       `-- tests/                 # Focused ABI/unit compatibility modules
|   |-- yune-schema/                   # Standalone typed RIME schema subset parser
|   |   |-- Cargo.toml
|   |   `-- src/lib.rs
|   `-- yune-cli/                      # Deterministic fixture CLI crate
|       |-- Cargo.toml
|       `-- src/
|           |-- main.rs                # CLI entry point
|           |-- args.rs                # Command parsing
|           |-- sample_core.rs         # Sample core-backed runner
|           |-- fixture.rs             # Fixture comparison
|           |-- transcript.rs          # Deterministic JSON output
|           |-- render.rs              # Help rendering
|           `-- rime_frontend.rs       # Reserved RIME API frontend module
|-- docs/
|   |-- analysis.md                    # Compatibility strategy and gaps
|   |-- refactor-plan.md               # Module ownership/refactor guidance
|   `-- roadmap.md                     # Completed milestones and next work
|-- fixtures/
|   |-- sample-backspace.json          # CLI fixture output
|   |-- sample-composing.json          # CLI fixture output
|   |-- sample-nihao.json              # CLI fixture output
|   `-- sample-punctuation.json        # CLI fixture output
|-- .planning/codebase/                # Generated GSD codebase maps
`-- .gnhf/runs/                        # Local GSD run artifacts
```

## Directory Purposes

**Root:**
- Purpose: Workspace-level project definition and top-level docs.
- Contains: `Cargo.toml`, `Cargo.lock`, `README.md`, `.gitignore`.
- Key files: `Cargo.toml`, `README.md`

**`crates/`:**
- Purpose: Houses all Rust workspace members.
- Contains: `yune-core`, `yune-schema`, `yune-rime-api`, `yune-cli`.
- Key files: `crates/yune-core/Cargo.toml`, `crates/yune-rime-api/Cargo.toml`, `crates/yune-schema/Cargo.toml`, `crates/yune-cli/Cargo.toml`

**`crates/yune-core/src/`:**
- Purpose: Deterministic Rust engine and reusable compatibility primitives.
- Contains: engine state machine, public traits, key parsing, candidates, dictionary parsing, translators, filters, punctuation, spelling algebra, comment formatting.
- Key files: `crates/yune-core/src/lib.rs`, `crates/yune-core/src/engine.rs`, `crates/yune-core/src/state.rs`, `crates/yune-core/src/key.rs`

**`crates/yune-core/src/dictionary/`:**
- Purpose: RIME dictionary source and compiled-data compatibility helpers.
- Contains: source `.dict.yaml` parsing, imports, packs, preset vocabulary, table encoder, checksum and compiled metadata helpers.
- Key files: `crates/yune-core/src/dictionary/source.rs`, `crates/yune-core/src/dictionary/encoder.rs`, `crates/yune-core/src/dictionary/compiled.rs`

**`crates/yune-core/src/translator/`:**
- Purpose: Core candidate generation components.
- Contains: echo, table, reverse lookup, history, switch, folded switch, and schema-list translators.
- Key files: `crates/yune-core/src/translator/mod.rs`

**`crates/yune-core/src/filter/`:**
- Purpose: Core candidate post-processing components.
- Contains: uniquifier, single-char, charset, tagged, simplifier, reverse-lookup filters.
- Key files: `crates/yune-core/src/filter/mod.rs`

**`crates/yune-rime-api/src/`:**
- Purpose: C ABI and runtime compatibility surface for RIME-style frontends.
- Contains: ABI structs, exported functions, function tables, session lifecycle, config APIs, context/status/commit APIs, schema deployment/selection, levers, userdb, runtime path APIs, notifications, modules, key tables.
- Key files: `crates/yune-rime-api/src/lib.rs`, `crates/yune-rime-api/src/abi.rs`, `crates/yune-rime-api/src/api_table.rs`, `crates/yune-rime-api/src/session.rs`

**`crates/yune-rime-api/src/processors/`:**
- Purpose: Schema-loaded key processing behavior before falling through to `yune-core`.
- Contains: ascii composer, chord composer, editor, key binder, navigator, punctuation, recognizer, selector, shape, speller modules.
- Key files: `crates/yune-rime-api/src/processors/mod.rs`, `crates/yune-rime-api/src/processors/speller.rs`, `crates/yune-rime-api/src/processors/key_binder.rs`, `crates/yune-rime-api/src/processors/chord_composer.rs`

**`crates/yune-rime-api/src/tests/`:**
- Purpose: Focused unit-level ABI and compatibility tests within the RIME API crate.
- Contains: ABI layout, candidate, config, context/status, deployment, levers, runtime, schema API, schema processor, schema selection, session, and userdb tests.
- Key files: `crates/yune-rime-api/src/tests/mod.rs`, `crates/yune-rime-api/src/tests/schema_processors.rs`, `crates/yune-rime-api/src/tests/deployment.rs`

**`crates/yune-rime-api/tests/`:**
- Purpose: Integration tests that drive the exported `RimeApi` function table as a frontend would.
- Contains: frontend-style ABI client tests.
- Key files: `crates/yune-rime-api/tests/frontend_client.rs`

**`crates/yune-schema/src/`:**
- Purpose: Standalone typed parser for a minimal RIME schema subset.
- Contains: schema metadata, engine component lists, parse errors, unit tests.
- Key files: `crates/yune-schema/src/lib.rs`

**`crates/yune-cli/src/`:**
- Purpose: Local deterministic runner for sample input sequences and fixtures.
- Contains: command parser, sample engine setup, fixture comparison, JSON transcript formatting, help renderer.
- Key files: `crates/yune-cli/src/main.rs`, `crates/yune-cli/src/sample_core.rs`, `crates/yune-cli/src/fixture.rs`, `crates/yune-cli/src/transcript.rs`

**`docs/`:**
- Purpose: Human-readable compatibility strategy and planning context.
- Contains: analysis, roadmap, refactor guidance.
- Key files: `docs/analysis.md`, `docs/roadmap.md`, `docs/refactor-plan.md`

**`fixtures/`:**
- Purpose: Checked-in deterministic CLI fixture outputs.
- Contains: JSON fixtures consumed by `crates/yune-cli/src/fixture.rs`.
- Key files: `fixtures/sample-nihao.json`, `fixtures/sample-composing.json`, `fixtures/sample-punctuation.json`, `fixtures/sample-backspace.json`

**`.planning/codebase/`:**
- Purpose: GSD-generated codebase maps for planning and execution.
- Contains: generated architecture and structure maps for this focus.
- Key files: `.planning/codebase/ARCHITECTURE.md`, `.planning/codebase/STRUCTURE.md`

**`.gnhf/runs/`:**
- Purpose: Local GSD run artifacts.
- Contains: run directories and subagent artifacts.
- Key files: Not applicable.

## Key File Locations

**Entry Points:**
- `Cargo.toml`: Rust workspace entry point.
- `crates/yune-core/src/lib.rs`: public Rust API for the core engine crate.
- `crates/yune-rime-api/src/lib.rs`: exported C ABI facade and key processing path.
- `crates/yune-rime-api/src/api_table.rs`: `rime_get_api` and function table construction.
- `crates/yune-cli/src/main.rs`: CLI binary entry point.
- `crates/yune-schema/src/lib.rs`: schema parser library entry point.

**Configuration:**
- `Cargo.toml`: workspace member list, shared lints, edition, MSRV.
- `Cargo.lock`: locked dependencies.
- `crates/yune-core/Cargo.toml`: core dependency on `regex`.
- `crates/yune-rime-api/Cargo.toml`: ABI dependencies on `libc`, `regex`, `serde_yaml`, and `yune-core`.
- `crates/yune-schema/Cargo.toml`: schema parser dependencies on `serde` and `serde_yaml`.
- `crates/yune-cli/Cargo.toml`: CLI dependency on `yune-core`.
- `.gitignore`: ignores build outputs and local IDE/macOS artifacts.

**Core Logic:**
- `crates/yune-core/src/engine.rs`: composition, candidate refresh, paging, selection, commits.
- `crates/yune-core/src/key.rs`: RIME key sequence parsing and key model.
- `crates/yune-core/src/state.rs`: candidate/context/status data model.
- `crates/yune-core/src/dictionary/source.rs`: source dictionary parsing.
- `crates/yune-core/src/dictionary/encoder.rs`: table encoder formula support.
- `crates/yune-core/src/dictionary/compiled.rs`: compiled dictionary metadata and checksum helpers.
- `crates/yune-core/src/translator/mod.rs`: candidate generation.
- `crates/yune-core/src/filter/mod.rs`: candidate filtering.
- `crates/yune-core/src/punctuation.rs`: punctuation translation.
- `crates/yune-core/src/spelling_algebra.rs`: lookup-side spelling algebra.
- `crates/yune-core/src/comment_format.rs`: comment formatting formulas.

**RIME ABI Logic:**
- `crates/yune-rime-api/src/abi.rs`: ABI type layout.
- `crates/yune-rime-api/src/session.rs`: session registry and `SessionState`.
- `crates/yune-rime-api/src/context_api.rs`: context, status, and commit reads.
- `crates/yune-rime-api/src/candidate_api.rs`: candidate list iterators.
- `crates/yune-rime-api/src/config.rs`: YAML config state and scalar/path helpers.
- `crates/yune-rime-api/src/config_api.rs`: RIME config entrypoints.
- `crates/yune-rime-api/src/config_compiler.rs`: include/patch/custom patch compilation.
- `crates/yune-rime-api/src/deployment.rs`: initialize, finalize, deploy, sync, maintenance.
- `crates/yune-rime-api/src/runtime.rs`: runtime trait/path handling.
- `crates/yune-rime-api/src/schema_install.rs`: translator/filter/segmentor installation from schemas.
- `crates/yune-rime-api/src/schema_selection.rs`: selected schema workflow.
- `crates/yune-rime-api/src/key_table.rs`: key name/code lookup tables.
- `crates/yune-rime-api/src/levers.rs`: levers/custom settings/user dictionary manager API.
- `crates/yune-rime-api/src/userdb.rs`: plain user dictionary operations.

**Processor Logic:**
- `crates/yune-rime-api/src/processors/ascii_composer.rs`: ascii mode switch handling.
- `crates/yune-rime-api/src/processors/chord_composer.rs`: chord input handling.
- `crates/yune-rime-api/src/processors/editor.rs`: editor variants, bindings, char handlers.
- `crates/yune-rime-api/src/processors/key_binder.rs`: schema key bindings and redirects.
- `crates/yune-rime-api/src/processors/navigator.rs`: candidate and syllable navigation.
- `crates/yune-rime-api/src/processors/punctuation.rs`: schema punctuation processor/translator install.
- `crates/yune-rime-api/src/processors/recognizer.rs`: recognizer processor.
- `crates/yune-rime-api/src/processors/selector.rs`: selector and alternative select keys.
- `crates/yune-rime-api/src/processors/shape.rs`: full-shape ASCII formatting.
- `crates/yune-rime-api/src/processors/speller.rs`: speller gate, auto-clear, auto-select.

**Testing:**
- `crates/yune-core/src/lib.rs`: core unit tests.
- `crates/yune-schema/src/lib.rs`: schema parser unit tests.
- `crates/yune-cli/src/fixture.rs`: fixture check test that scans `fixtures/`.
- `crates/yune-rime-api/src/tests/mod.rs`: shared ABI test helpers and module index.
- `crates/yune-rime-api/src/tests/schema_processors.rs`: focused schema-loaded processor tests.
- `crates/yune-rime-api/tests/frontend_client.rs`: function-table integration tests.

**Documentation:**
- `README.md`: overview, goals, current compatibility surface.
- `docs/analysis.md`: strategy, compatibility layers, gaps.
- `docs/refactor-plan.md`: module ownership and split guidance.
- `docs/roadmap.md`: completed milestones and active next work.

## Naming Conventions

**Files:**
- Rust modules use snake_case: `schema_install.rs`, `config_compiler.rs`, `frontend_client.rs`.
- Module directories with a single aggregate file use `mod.rs`: `crates/yune-core/src/translator/mod.rs`, `crates/yune-rime-api/src/processors/mod.rs`.
- Test modules mirror feature/API areas: `config_api.rs`, `schema_selection.rs`, `schema_processors.rs`.
- Fixture files use `sample-<case>.json`: `fixtures/sample-nihao.json`.
- Documentation files use lowercase kebab-case except generated GSD maps: `docs/refactor-plan.md`, `.planning/codebase/ARCHITECTURE.md`.

**Directories:**
- Crates use `yune-<area>` names: `crates/yune-core`, `crates/yune-rime-api`.
- Focused implementation submodules use concept names: `dictionary`, `translator`, `filter`, `processors`, `tests`.
- Generated planning artifacts live under `.planning/`.
- Local run artifacts live under `.gnhf/`.

## Where to Add New Code

**New Core Engine Behavior:**
- Primary code: `crates/yune-core/src/engine.rs` when behavior changes the generic engine state machine.
- Supporting types: `crates/yune-core/src/state.rs` for candidate/context/status shape changes.
- Tests: `crates/yune-core/src/lib.rs` unless a new focused test module is introduced with a matching implementation split.

**New Core Translator:**
- Implementation: `crates/yune-core/src/translator/mod.rs`.
- Public export: `crates/yune-core/src/lib.rs`.
- Schema installation: `crates/yune-rime-api/src/schema_install.rs` when the translator is schema-driven.
- Tests: core tests in `crates/yune-core/src/lib.rs` plus ABI schema tests in `crates/yune-rime-api/src/tests/schema_processors.rs` when installed from RIME config.

**New Core Filter:**
- Implementation: `crates/yune-core/src/filter/mod.rs`.
- Public export: `crates/yune-core/src/lib.rs`.
- Schema installation: `crates/yune-rime-api/src/schema_install.rs`.
- Tests: `crates/yune-core/src/lib.rs` and the matching `crates/yune-rime-api/src/tests/*.rs` module.

**New Dictionary or Encoder Behavior:**
- Source dictionary parsing: `crates/yune-core/src/dictionary/source.rs`.
- Compiled metadata/checksum behavior: `crates/yune-core/src/dictionary/compiled.rs`.
- Table encoder rules: `crates/yune-core/src/dictionary/encoder.rs`.
- Public exports: `crates/yune-core/src/dictionary/mod.rs` and `crates/yune-core/src/lib.rs`.
- Tests: core tests in `crates/yune-core/src/lib.rs`; ABI deployment/schema tests when runtime config loads the behavior.

**New RIME ABI Function:**
- ABI struct/function table shape: `crates/yune-rime-api/src/abi.rs` and `crates/yune-rime-api/src/api_table.rs`.
- Implementation: add to the owning module such as `context_api.rs`, `candidate_api.rs`, `config_api.rs`, `deployment.rs`, `levers.rs`, `schema_api.rs`, `schema_selection.rs`, `runtime.rs`, `userdb.rs`, or `modules.rs`.
- Facade export: `crates/yune-rime-api/src/lib.rs`.
- Tests: matching `crates/yune-rime-api/src/tests/*.rs` module and `crates/yune-rime-api/tests/frontend_client.rs` when the function table exposes it.

**New Schema Processor:**
- Implementation: `crates/yune-rime-api/src/processors/<processor>.rs`.
- Aggregator export: `crates/yune-rime-api/src/processors/mod.rs`.
- Session state fields: `crates/yune-rime-api/src/session.rs` when state must persist per session.
- Shared processor enums/types: `crates/yune-rime-api/src/lib.rs` only when multiple processor modules need them.
- Installer call: `crates/yune-rime-api/src/schema_selection.rs` and/or `crates/yune-rime-api/src/schema_install.rs`.
- Tests: `crates/yune-rime-api/src/tests/schema_processors.rs`.

**New Config Behavior:**
- Scalar/path helpers and in-memory config state: `crates/yune-rime-api/src/config.rs`.
- Public config entrypoints: `crates/yune-rime-api/src/config_api.rs`.
- Include/patch/build freshness behavior: `crates/yune-rime-api/src/config_compiler.rs`.
- Runtime config lookup helpers: `crates/yune-rime-api/src/lib.rs` near `load_runtime_config_root`.
- Tests: `crates/yune-rime-api/src/tests/config_api.rs` or `crates/yune-rime-api/src/tests/deployment.rs`.

**New Deployment or Runtime Behavior:**
- Runtime trait/path fields: `crates/yune-rime-api/src/runtime.rs`.
- Maintenance, prebuild, deploy, sync: `crates/yune-rime-api/src/deployment.rs`.
- Notifications: `crates/yune-rime-api/src/notifications.rs`.
- Tests: `crates/yune-rime-api/src/tests/deployment.rs` and `crates/yune-rime-api/src/tests/runtime.rs`.

**New CLI Behavior:**
- Command parsing: `crates/yune-cli/src/args.rs`.
- Main command dispatch: `crates/yune-cli/src/main.rs`.
- Core-backed sample behavior: `crates/yune-cli/src/sample_core.rs`.
- RIME ABI-backed frontend behavior: `crates/yune-cli/src/rime_frontend.rs`.
- Output formatting: `crates/yune-cli/src/transcript.rs` or `crates/yune-cli/src/render.rs`.
- Tests: same module or `crates/yune-cli/src/fixture.rs` when fixture output changes.

**New Schema Parser Behavior:**
- Implementation: `crates/yune-schema/src/lib.rs`.
- Tests: `crates/yune-schema/src/lib.rs`.

**New Fixtures:**
- JSON fixture file: `fixtures/sample-<case>.json`.
- Fixture generation/check logic: `crates/yune-cli/src/sample_core.rs`, `crates/yune-cli/src/fixture.rs`, `crates/yune-cli/src/transcript.rs`.

**New Documentation:**
- Project/user-facing overview: `README.md`.
- Compatibility strategy and gaps: `docs/analysis.md`.
- Roadmap state: `docs/roadmap.md`.
- Refactor/module ownership guidance: `docs/refactor-plan.md`.
- Generated codebase maps: `.planning/codebase/`.

**Utilities:**
- Core reusable helpers belong in the focused core module that owns their domain.
- ABI reusable helpers belong in `crates/yune-rime-api/src/config.rs`, `runtime.rs`, `ffi_memory.rs`, or `lib.rs` only when shared across multiple ABI modules.
- Avoid creating a generic utility module unless two or more existing ownership areas need the same helper.

## Special Directories

**`target/`:**
- Purpose: Cargo build output.
- Generated: Yes.
- Committed: No.

**`.planning/`:**
- Purpose: GSD planning and codebase-map artifacts.
- Generated: Yes.
- Committed: Repository-dependent; this session writes only `.planning/codebase/ARCHITECTURE.md` and `.planning/codebase/STRUCTURE.md`.

**`.gnhf/`:**
- Purpose: Local GSD run artifacts.
- Generated: Yes.
- Committed: Not part of normal source ownership.

**`fixtures/`:**
- Purpose: Deterministic sample output fixtures.
- Generated: Produced by CLI runs, then checked in as compatibility data.
- Committed: Yes.

**`docs/`:**
- Purpose: Human-authored project context and roadmap.
- Generated: No.
- Committed: Yes.

**Per-crate `.gitignore` files:**
- Purpose: Ignore crate-local `target` directories.
- Generated: No.
- Committed: Yes.

---

*Structure analysis: 2026-04-28*
