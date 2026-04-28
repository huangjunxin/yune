# Testing Patterns

**Analysis Date:** 2026-04-28

## Test Framework

**Runner:**
- Rust built-in test harness through Cargo.
- Config: `Cargo.toml` workspace with crate manifests under `crates/yune-core/Cargo.toml`, `crates/yune-schema/Cargo.toml`, `crates/yune-rime-api/Cargo.toml`, and `crates/yune-cli/Cargo.toml`.

**Assertion Library:**
- Standard Rust assertions: `assert_eq!`, `assert!`, `assert_ne!`, and explicit `panic!` in fixture mismatch handling.
- No third-party assertion, property-test, or snapshot-test crate is declared in the current manifests.

**Run Commands:**
```bash
cargo test --workspace              # Run all tests
cargo test -p yune-rime-api session_api              # Run a focused unit-test module by name
cargo test -p yune-rime-api --test frontend_client              # Run the frontend-style integration client
cargo clippy --workspace --all-targets -- -D warnings              # Lint all production and test targets
```

## Test File Organization

**Location:**
- Unit tests are mostly embedded in source files with `#[cfg(test)] mod tests`, for example `crates/yune-core/src/lib.rs`, `crates/yune-schema/src/lib.rs`, `crates/yune-cli/src/args.rs`, `crates/yune-cli/src/fixture.rs`, and `crates/yune-cli/src/transcript.rs`.
- `yune-rime-api` has a dedicated internal test tree under `crates/yune-rime-api/src/tests/`, mounted from `#[cfg(test)] mod tests;` in `crates/yune-rime-api/src/lib.rs`.
- One Cargo integration test lives at `crates/yune-rime-api/tests/frontend_client.rs`.
- CLI golden fixtures live outside crates under `fixtures/` and are exercised from `crates/yune-cli/src/fixture.rs`.

**Naming:**
- Test functions use behavior descriptions in `snake_case`, such as `config_open_apis_load_runtime_yaml_files` in `crates/yune-rime-api/src/tests/config_api.rs` and `parses_rime_schema_subset` in `crates/yune-schema/src/lib.rs`.
- RIME compatibility tests often include `librime` in the test name when matching external behavior, for example tests in `crates/yune-rime-api/src/tests/schema_processors.rs` and `crates/yune-rime-api/src/tests/schema_selection.rs`.
- Test modules follow implementation or API areas: `session_api.rs`, `config_api.rs`, `candidate_api.rs`, `schema_selection.rs`, `schema_processors.rs`, `deployment.rs`, `levers.rs`, and `userdb.rs` under `crates/yune-rime-api/src/tests/`.

**Structure:**
```text
crates/yune-core/src/lib.rs              # core facade plus large embedded unit-test module
crates/yune-schema/src/lib.rs            # schema parser unit tests
crates/yune-rime-api/src/tests/mod.rs    # shared ABI test helpers
crates/yune-rime-api/src/tests/*.rs      # focused API and compatibility unit tests
crates/yune-rime-api/tests/frontend_client.rs              # integration-style API-table client
crates/yune-cli/src/*                    # small CLI unit tests
fixtures/*.json                         # checked-in CLI output fixtures
```

## Test Structure

**Suite Organization:**
```rust
#[cfg(test)]
mod tests {
    use super::{check_fixture, sequence_from_fixture};

    #[test]
    fn checked_in_fixtures_match_cli_output() {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let fixtures_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .expect("CLI crate should live under workspace crates")
            .join("fixtures");
        // collect fixtures, sort them, and compare each fixture to generated output
    }
}
```

**Patterns:**
- Set up process-wide RIME state through `test_guard()` before ABI tests, as in `crates/yune-rime-api/src/tests/mod.rs`, `crates/yune-rime-api/src/tests/session_api.rs`, and `crates/yune-rime-api/src/tests/config_api.rs`.
- Use focused helper constructors for empty ABI structs, such as `empty_context`, `empty_status`, `empty_config`, `empty_traits`, and `empty_candidate_list_iterator` in `crates/yune-rime-api/src/tests/mod.rs`.
- Use direct assertions against user-visible state: commits, preedit, candidates, schema names, status bits, C strings, and file outputs.
- Use temporary directories with unique names for deployment/config tests, then remove them explicitly. See `unique_temp_dir` in `crates/yune-rime-api/src/tests/mod.rs` and `crates/yune-rime-api/tests/frontend_client.rs`.

## Mocking

**Framework:** hand-written fakes and fixtures

**Patterns:**
```rust
struct CommentTranslator;

impl Translator for CommentTranslator {
    fn name(&self) -> &'static str {
        "comment_translator"
    }

    fn translate(&self, input: &str) -> Vec<Candidate> {
        if input != "ni" {
            return Vec::new();
        }
        vec![Candidate {
            text: "你".to_owned(),
            comment: "first-comment".to_owned(),
            source: CandidateSource::Table,
            quality: 1.0,
        }]
    }
}
```

**What to Mock:**
- Mock translator/ranker/filter behavior at trait boundaries from `crates/yune-core/src/lib.rs`, using local structs like `CommentTranslator` and production helper types like `StaticTableTranslator`.
- Mock frontend modules with `extern "C"` function pointers in `crates/yune-rime-api/src/tests/mod.rs` and `crates/yune-rime-api/tests/frontend_client.rs`.
- Mock runtime config and schema data by writing YAML into unique temp directories in `crates/yune-rime-api/src/tests/config_api.rs`, `crates/yune-rime-api/src/tests/deployment.rs`, and `crates/yune-rime-api/src/tests/schema_selection.rs`.

**What NOT to Mock:**
- Do not mock the ABI function table when testing frontend behavior. `crates/yune-rime-api/tests/frontend_client.rs` drives functions through `rime_get_api`.
- Do not bypass `Engine` for core behavior tests. Use `Engine`, `StaticTableTranslator`, `PunctuationTranslator`, and key sequence helpers from `crates/yune-core/src/lib.rs` and `crates/yune-cli/src/sample_core.rs`.
- Do not replace checked-in CLI fixtures with ad hoc inline expectations when exercising the fixture contract in `crates/yune-cli/src/fixture.rs`.

## Fixtures and Factories

**Test Data:**
```rust
const SAMPLE_DICT: &str = r#"
---
name: sample
version: "0.1"
sort: by_weight
...

你	ni	10
好	hao	10
你好	ni hao	100
"#;
```

**Location:**
- CLI JSON fixtures are in `fixtures/sample-nihao.json`, `fixtures/sample-composing.json`, `fixtures/sample-backspace.json`, and `fixtures/sample-punctuation.json`.
- Shared RIME API test factories live in `crates/yune-rime-api/src/tests/mod.rs`.
- Inline YAML fixtures are common in `crates/yune-rime-api/src/tests/config_api.rs`, `crates/yune-rime-api/src/tests/deployment.rs`, `crates/yune-rime-api/src/tests/schema_processors.rs`, and `crates/yune-rime-api/src/tests/schema_selection.rs`.
- Core dictionary and key-sequence test data is mostly inline in `crates/yune-core/src/lib.rs`.

## Coverage

**Requirements:** None enforced by tooling. No coverage command or threshold is configured in `Cargo.toml`.

**View Coverage:**
```bash
Not configured
```

## Test Types

**Unit Tests:**
- Core parser, dictionary, translator, filter, ranker, key handling, schema, and CLI behavior are tested with `#[test]` functions in `crates/yune-core/src/lib.rs`, `crates/yune-schema/src/lib.rs`, and `crates/yune-cli/src/*.rs`.
- `yune-rime-api` API surfaces are tested by focused modules under `crates/yune-rime-api/src/tests/`.

**Integration Tests:**
- `crates/yune-rime-api/tests/frontend_client.rs` drives the public API table through `rime_get_api`, C-compatible structs, and function pointers to approximate frontend usage.
- Config/deployment/schema tests write temp runtime directories and YAML files to exercise file-backed behavior through public APIs.

**E2E Tests:**
- No browser or OS input-method E2E harness is present.
- CLI fixture checks in `crates/yune-cli/src/fixture.rs` are the closest repo-local end-to-end path for core-backed input sequence output.

## Common Patterns

**Async Testing:**
```rust
Not used. Tests are synchronous Rust `#[test]` functions.
```

**Error Testing:**
```rust
let error = Schema::parse_rime_yaml(
    r#"
schema:
  name: Missing ID
"#,
)
.expect_err("schema without schema_id should fail");

assert_eq!(
    error.to_string(),
    "missing required RIME schema field: schema.schema_id"
);
```

- For C ABI error paths, assert `FALSE`, null pointers, or unchanged output state, as in `crates/yune-rime-api/src/tests/session_api.rs`, `crates/yune-rime-api/src/tests/config_api.rs`, and `crates/yune-rime-api/tests/frontend_client.rs`.
- For parser errors, prefer `expect_err` plus exact user-facing error text when the message is part of the contract, as in `crates/yune-schema/src/lib.rs`.
- For fixture mismatches, return a detailed `Err(String)` from `check_fixture` in `crates/yune-cli/src/fixture.rs` and panic only at the test boundary.

---

*Testing analysis: 2026-04-28*
