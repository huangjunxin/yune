# Coding Conventions

**Analysis Date:** 2026-04-28

## Naming Patterns

**Files:**
- Use Rust module filenames in `snake_case`, with conceptual submodules under focused directories such as `crates/yune-core/src/dictionary/source.rs`, `crates/yune-core/src/dictionary/compiled.rs`, `crates/yune-rime-api/src/processors/key_binder.rs`, and `crates/yune-rime-api/src/schema_selection.rs`.
- Use `mod.rs` only for directory module roots such as `crates/yune-core/src/dictionary/mod.rs`, `crates/yune-core/src/filter/mod.rs`, `crates/yune-core/src/translator/mod.rs`, `crates/yune-rime-api/src/processors/mod.rs`, and `crates/yune-rime-api/src/tests/mod.rs`.
- Keep crate package names in kebab-case in manifests: `crates/yune-core/Cargo.toml`, `crates/yune-schema/Cargo.toml`, `crates/yune-rime-api/Cargo.toml`, and `crates/yune-cli/Cargo.toml`.
- Keep checked-in CLI fixtures under `fixtures/` with `sample-*.json` names such as `fixtures/sample-nihao.json` and `fixtures/sample-punctuation.json`.

**Functions:**
- Use `snake_case` for Rust functions and methods, including behavior-heavy APIs such as `Engine::process_key_event` in `crates/yune-core/src/engine.rs`, `TableDictionary::parse_rime_dict_yaml_with_imports` in `crates/yune-core/src/dictionary/source.rs`, and `Schema::parse_rime_yaml` in `crates/yune-schema/src/lib.rs`.
- Use `RimePascalCase` only for exported librime-shaped C ABI functions with `#[no_mangle]`, for example `RimeConfigOpen` in `crates/yune-rime-api/src/config_api.rs` and `RimeSetup` in `crates/yune-rime-api/src/runtime.rs`.
- Name tests as long, behavior-specific `snake_case` sentences, for example `processes_ascii_keys_and_returns_unread_commit_once` in `crates/yune-rime-api/src/tests/session_api.rs` and `checked_in_fixtures_match_cli_output` in `crates/yune-cli/src/fixture.rs`.

**Variables:**
- Use `snake_case` locals and fields, with `is_` / `has_` prefixes for booleans such as `Status::is_ascii_mode` in `crates/yune-core/src/state.rs`, `has_selectable_candidates` in `crates/yune-core/src/engine.rs`, and `is_last_page` in `crates/yune-rime-api/src/abi.rs`.
- Use descriptive temporary names over abbreviations when crossing ABI or schema boundaries, such as `shared_data_dir`, `user_data_dir`, `prebuilt_data_dir`, and `backup_config_files` in `crates/yune-rime-api/src/runtime.rs`.
- Use `_guard` for intentionally held test mutex guards, as in `crates/yune-rime-api/src/tests/session_api.rs`, to serialize process-wide runtime state.

**Types:**
- Use `UpperCamelCase` for structs, enums, traits, and error types such as `Engine`, `CandidateRanker`, `TableDictionaryParseError`, `RimeConfigIterator`, and `SchemaParseError`.
- Keep C ABI mirror types prefixed with `Rime` and marked `#[repr(C)]` in `crates/yune-rime-api/src/abi.rs`.
- Derive common traits near type declarations. Current types commonly derive combinations of `Clone`, `Copy`, `Debug`, `Default`, `Eq`, `Hash`, and `PartialEq`, as in `KeyModifiers` and `KeyCode` in `crates/yune-core/src/key.rs`.

## Code Style

**Formatting:**
- Use `rustfmt` through `cargo fmt`; no repo-specific `rustfmt.toml` or `.rustfmt.toml` is present.
- Use Rust 2021 syntax with workspace MSRV `1.76` from `Cargo.toml`; avoid newer standard-library helpers unless the MSRV is raised.
- Keep early-return `let Some(...) = ... else { return ...; };` and `let Ok(...) = ... else { return ...; };` patterns for validation-heavy code, as in `crates/yune-rime-api/src/config_api.rs` and `crates/yune-rime-api/src/runtime.rs`.
- Prefer small focused production modules. Keep `crates/yune-core/src/lib.rs` and `crates/yune-rime-api/src/lib.rs` as public facades and glue; add new implementation work to focused modules such as `crates/yune-core/src/key.rs` or `crates/yune-rime-api/src/processors/speller.rs`.

**Linting:**
- Treat the root `Cargo.toml` lint policy as the intended standard: `[workspace.lints.clippy] all = "warn"` and `pedantic = "warn"`.
- Use the documented quality gate from `docs/refactor-plan.md`: `cargo clippy --workspace --all-targets -- -D warnings`.
- Public pure accessors and constructors commonly carry `#[must_use]`, for example `Engine::new` in `crates/yune-core/src/engine.rs`, `Schema::minimal` in `crates/yune-schema/src/lib.rs`, and `TableEntry::new` in `crates/yune-core/src/dictionary/source.rs`.
- FFI boundary functions use explicit `unsafe extern "C" fn` signatures plus `# Safety` docs and local `// SAFETY:` comments, as in `crates/yune-rime-api/src/config_api.rs`, `crates/yune-rime-api/src/runtime.rs`, and `crates/yune-rime-api/src/ffi_memory.rs`.

## Import Organization

**Order:**
1. Standard library imports first, often grouped with `use std::{...};`, as in `crates/yune-rime-api/src/runtime.rs` and `crates/yune-cli/src/main.rs`.
2. External crates next, such as `use regex::Regex;`, `use serde_yaml::Value;`, and `use yune_core::{...};` in `crates/yune-rime-api/src/schema_install.rs`.
3. Local `crate::`, `super::`, and module imports last, commonly grouped with braces in files like `crates/yune-core/src/engine.rs` and `crates/yune-rime-api/src/config_api.rs`.

**Path Aliases:**
- No custom path aliases are configured. Use crate names from workspace manifests, for example `yune_core` in `crates/yune-rime-api/src/schema_install.rs` and `crates/yune-cli/src/sample_core.rs`.
- Within a crate, use `crate::...` for cross-module access and `super::...` for sibling or parent module access, as in `crates/yune-core/src/punctuation.rs`, `crates/yune-core/src/filter/mod.rs`, and `crates/yune-rime-api/src/tests/session_api.rs`.

## Error Handling

**Patterns:**
- Library parsing code returns custom error types implementing `Display` and `Error`, such as `KeySequenceParseError` in `crates/yune-core/src/key.rs` and `SchemaParseError` in `crates/yune-schema/src/lib.rs`.
- CLI code returns `Result<(), String>` from `run` and maps errors to `stderr` plus `ExitCode::FAILURE` in `crates/yune-cli/src/main.rs`.
- C ABI functions return librime-shaped `Bool` values or null pointers instead of panicking. Validate null pointers and invalid C strings at the boundary, as in `crates/yune-rime-api/src/config_api.rs` and `crates/yune-rime-api/src/candidate_api.rs`.
- Use `expect` for internal invariants and test setup, not for ordinary user input. Examples include mutex poisoning checks in `crates/yune-rime-api/src/runtime.rs` and fixture/test setup checks in `crates/yune-rime-api/src/tests/mod.rs`.
- When a function is compatibility-oriented, preserve librime-shaped fallback behavior explicitly, such as missing config open behavior in `crates/yune-rime-api/src/tests/config_api.rs`.

## Logging

**Framework:** console

**Patterns:**
- No `log` or `tracing` dependency is present in workspace manifests. Use `println!` only for CLI user output in `crates/yune-cli/src/main.rs`, `crates/yune-cli/src/render.rs`, and `crates/yune-cli/src/fixture.rs`.
- Use `eprintln!` for CLI errors at the process boundary in `crates/yune-cli/src/main.rs`.
- Library crates should avoid unsolicited output. Return structured values, `Result`, `Bool`, or null pointers instead of logging from `crates/yune-core/src/*`, `crates/yune-schema/src/lib.rs`, and `crates/yune-rime-api/src/*`.

## Comments

**When to Comment:**
- Document FFI safety requirements with Rustdoc `# Safety` sections on unsafe public functions and `// SAFETY:` comments next to unsafe blocks, following `crates/yune-rime-api/src/config_api.rs` and `crates/yune-rime-api/src/runtime.rs`.
- Use comments to explain compatibility behavior, struct layout, ownership, or intentionally unusual fallbacks. Avoid restating straightforward Rust control flow.
- Keep docs under `docs/` for roadmap and refactor guidance; source comments should stay local to non-obvious invariants.

**JSDoc/TSDoc:**
- Not applicable. The repo is Rust-only.
- Use Rustdoc on public APIs when safety, ownership, or compatibility semantics are not self-evident, especially in `crates/yune-rime-api/src/abi.rs`, `crates/yune-rime-api/src/config_api.rs`, and `crates/yune-rime-api/src/runtime.rs`.

## Function Design

**Size:** Keep production functions focused around one compatibility or state transition. Extract repeated behavior into private helpers such as `read_installation_settings` in `crates/yune-rime-api/src/runtime.rs`, `required_field` in `crates/yune-schema/src/lib.rs`, and `parse_key_event_repr` in `crates/yune-core/src/key.rs`.

**Parameters:** Prefer `impl Into<String>` and `impl IntoIterator` for ergonomic Rust-facing APIs, as in `Engine::set_schema`, `Engine::set_property`, and `TableDictionary::new`. Use exact raw pointer types only at ABI boundaries in `crates/yune-rime-api/src/abi.rs` and `crates/yune-rime-api/src/*_api.rs`.

**Return Values:** Use `Option<String>` for optional commits in `crates/yune-core/src/engine.rs`, `Result<T, Error>` for parsers in `crates/yune-core/src/key.rs` and `crates/yune-schema/src/lib.rs`, and librime-compatible `Bool`/pointer returns in `crates/yune-rime-api/src/*_api.rs`.

## Module Design

**Exports:** Keep public re-exports centralized in crate facades. `crates/yune-core/src/lib.rs` re-exports engine, state, dictionary, filter, key, punctuation, and translator types; `crates/yune-rime-api/src/lib.rs` re-exports ABI and API surface modules.

**Barrel Files:** Use Rust module roots as barrels when they define ownership boundaries. Examples include `crates/yune-core/src/dictionary/mod.rs`, `crates/yune-core/src/translator/mod.rs`, `crates/yune-rime-api/src/processors/mod.rs`, and `crates/yune-rime-api/src/tests/mod.rs`.

---

*Convention analysis: 2026-04-28*
