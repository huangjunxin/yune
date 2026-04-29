---
phase: 04-compiled-dictionary-data
plan: 03
subsystem: data-compatibility
tags: [rust, rime, dictionary, compiled-table, reverse-lookup, encoder]

requires:
  - phase: 04-compiled-dictionary-data
    provides: compiled dictionary metadata and bounded binary parser groundwork from 04-01
provides:
  - Advanced dictionary contracts for entries, stems, reverse dict_settings, and encoder data
  - Compiled table advanced payload parsing for stems, materialized phrase entries, and encoder rules
  - Compiled reverse payload parsing for dict_settings and stems
  - Schema-loaded source/compiled parity tests for stems, reverse comments, vocabulary injection, and UniTE-style encoder phrases
affects: [compiled-dictionary-data, schema-pipeline-depth, userdb-and-learning]

tech-stack:
  added: []
  patterns:
    - Optional Yune fixture payload markers carry advanced data through existing TableDictionary
    - Runtime compiled dictionary loading merges reverse advanced metadata into table dictionaries

key-files:
  created:
    - .planning/phases/04-compiled-dictionary-data/04-03-SUMMARY.md
  modified:
    - crates/yune-core/src/dictionary/source.rs
    - crates/yune-core/src/dictionary/compiled_table.rs
    - crates/yune-core/src/dictionary/compiled_reverse.rs
    - crates/yune-core/src/dictionary/mod.rs
    - crates/yune-core/src/translator/mod.rs
    - crates/yune-core/src/lib.rs
    - crates/yune-rime-api/src/schema_install.rs
    - crates/yune-rime-api/src/tests/dictionary_data.rs

key-decisions:
  - "Represent advanced source and compiled dictionary data on TableDictionary instead of parallel runtime-specific structs."
  - "Use bounded local Yune fixture markers for Phase 04 advanced compiled payload parity while rejecting unsupported librime sections structurally."
  - "Carry compiled reverse dict_settings into runtime table dictionaries so ReverseLookupTranslator observes source and compiled settings through the same API."
  - "Keep LevelDB/userdb learning, predictive updates, plugin translators, and AI-native ranking out of DATA-03."

patterns-established:
  - "TableDictionary accessors expose read-only advanced data while preserving internal ownership."
  - "Schema installation validates compiled table, prism, and reverse artifacts, then uses source fallback for missing, stale, unsupported, or invalid compiled data."
  - "Schema-loaded dictionary data tests compare source-backed and compiled-backed visible behavior rather than parser-only state."

requirements-completed: [DATA-03]

duration: 7min
completed: 2026-04-29T18:11:43Z
---

# Phase 04 Plan 03: Advanced Dictionary Data Summary

**Source and compiled RIME dictionary parity for stems, reverse dict_settings, vocabulary phrase injection, and UniTE-style encoder payloads without userdb learning.**

## Performance

- **Duration:** 7 min after continuation resume
- **Started:** 2026-04-29T18:04:18Z
- **Completed:** 2026-04-29T18:11:43Z
- **Tasks:** 3
- **Files modified:** 8 implementation/test files plus this summary

## Accomplishments

- Promoted advanced dictionary data to explicit `TableDictionary` contracts with read-only access to entries, stems, reverse `dict_settings`, and encoder state.
- Added source parser support for `dict_settings` and preserved preset vocabulary plus rule-based encoder phrase generation through the same dictionary model.
- Added compiled table support for optional advanced fixture payloads carrying stems, materialized vocabulary/encoder phrase entries, and encoder rules.
- Added compiled reverse support for optional settings/stems payloads and wired runtime loading so reverse `dict_settings` reach reverse lookup comments in compiled-backed schemas.
- Added schema-loaded parity tests for stem-column behavior, reverse comment formatting, preset vocabulary phrase injection, and UniTE-style encoder data while asserting no userdb/predictive dependency.

## Task Commits

Each task was committed atomically using the plan-level TDD flow:

1. **Task 1: Promote dictionary stem, dict_settings, preset vocabulary, and encoder data to explicit core contracts**
   - `f57685a` test(04-03): add failing dictionary contract tests
   - `1d3cad5` feat(04-03): expose advanced dictionary contracts
2. **Task 2: Carry stem, dict_settings, vocabulary, and UniTE encoder payloads through compiled readers and translators**
   - `418cdb3` test(04-03): add failing advanced compiled payload tests
   - `c5b9937` feat(04-03): carry advanced payloads through compiled readers
   - `fae3c30` fix(04-03): align compiled advanced schema behavior
3. **Task 3: Add schema-loaded advanced dictionary behavior tests**
   - `e7a8d22` test(04-03): add failing schema-loaded advanced dictionary tests
   - `fae3c30` fix(04-03): align compiled advanced schema behavior

**Plan metadata:** pending final docs commit.

## Files Created/Modified

- `crates/yune-core/src/dictionary/source.rs` - Stores advanced data on `TableDictionary`, exposes read-only accessors, parses source `dict_settings`, and merges advanced data from reverse artifacts.
- `crates/yune-core/src/dictionary/compiled_table.rs` - Parses optional `YUNE-TABLE-ADV` fixture payloads with checked counts/lengths for stems, phrase entries, and encoder rules.
- `crates/yune-core/src/dictionary/compiled_reverse.rs` - Parses optional `YUNE-REVERSE` settings/stems payload data into `TableDictionaryAdvancedData`.
- `crates/yune-core/src/dictionary/mod.rs` - Re-exports `TableDictionaryAdvancedData` with the public dictionary API.
- `crates/yune-core/src/translator/mod.rs` - Applies reverse dictionary `comment_format` settings when constructing reverse lookup comments.
- `crates/yune-core/src/lib.rs` - Adds core source/compiled advanced dictionary contract tests.
- `crates/yune-rime-api/src/schema_install.rs` - Loads reverse dictionaries through the compiled/source policy and merges compiled reverse advanced data into runtime table dictionaries.
- `crates/yune-rime-api/src/tests/dictionary_data.rs` - Adds schema-loaded source/compiled parity tests and fixture builders for advanced table/reverse payloads.

## Decisions Made

- Represented advanced dictionary payload data directly on `TableDictionary` to keep source and compiled paths convergent.
- Used local optional fixture markers (`YUNE-TABLE-ADV\0`, `YUNE-REVERSE\0`) for bounded advanced payload testing without claiming unsupported librime sections are fully implemented.
- Merged compiled reverse advanced data into compiled table dictionaries during schema installation because runtime translators consume one `TableDictionary` API.
- Treated the plan's exact first yune-core verification command as syntactically invalid Cargo usage and ran equivalent single-filter commands separately.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Preserved compiled reverse dict_settings during runtime loading**
- **Found during:** Task 3 (schema-loaded reverse dict_settings parity)
- **Issue:** `parse_rime_reverse_bin_dictionary` successfully parsed compiled `dict_settings`, but `schema_install.rs` discarded the parsed reverse dictionary after validation, so compiled reverse comments stayed unformatted.
- **Fix:** Added `TableDictionary::with_merged_advanced_data_from` and used it in compiled dictionary loading so reverse advanced data reaches the table dictionary passed into translators.
- **Files modified:** `crates/yune-core/src/dictionary/source.rs`, `crates/yune-rime-api/src/schema_install.rs`
- **Verification:** `cargo test -p yune-rime-api dictionary_data_reverse_dict_settings_comments_match_source_and_compiled_paths -- --nocapture`; `cargo test -p yune-rime-api dictionary_data -- --nocapture`
- **Committed in:** `fae3c30`

**2. [Rule 1 - Bug] Fixed vocabulary parity fixture so source path can encode injected phrases**
- **Found during:** Task 3 (schema-loaded vocabulary phrase injection parity)
- **Issue:** The source fixture expected preset phrase `您好` to encode to `nh`, but source entries did not include encodable single-character rows for `您` and `好`; the compiled fixture had a materialized phrase entry, causing source/compiled mismatch.
- **Fix:** Added source fixture rows for `您` and `好` so the source rule-based encoder can generate the same vocabulary-injected phrase as the compiled payload.
- **Files modified:** `crates/yune-rime-api/src/tests/dictionary_data.rs`
- **Verification:** `cargo test -p yune-rime-api dictionary_data_vocabulary_phrase_injection_matches_source_and_compiled_paths -- --nocapture`; `cargo test -p yune-rime-api dictionary_data -- --nocapture`
- **Committed in:** `fae3c30`

**Total deviations:** 2 auto-fixed (Rule 1 bugs)
**Impact on plan:** Both fixes were required for correctness and source/compiled parity. No Phase 5 scope was introduced.

## Issues Encountered

- The plan command `cargo test -p yune-core dictionary::source dictionary::encoder -- --nocapture` failed because Cargo accepts only one test filter. Equivalent separate commands were run and passed, though these filters currently match zero tests because the relevant tests live in `crates/yune-core/src/lib.rs`.
- Initial shell lookup did not find `cargo`; using the absolute toolchain path at `$HOME/.cargo/bin/cargo` resolved verification execution.

## Verification

- `cargo test -p yune-core dictionary::source dictionary::encoder -- --nocapture` - failed as invalid Cargo syntax (`unexpected argument 'dictionary::encoder'`).
- `cargo test -p yune-core dictionary::source -- --nocapture` - passed, 0 matched tests.
- `cargo test -p yune-core dictionary::encoder -- --nocapture` - passed, 0 matched tests.
- `cargo test -p yune-core dictionary:: -- --nocapture` - passed, 0 matched tests.
- `cargo test -p yune-rime-api dictionary_data -- --nocapture` - passed, 9 tests.
- `cargo test --workspace` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- Acceptance grep checks for `dict_settings`, `stem`, `UnsupportedSection`, schema test coverage terms, and absence of new Phase 5 learning/storage implementation all passed.

## Known Stubs

None found in files created or modified by this plan.

## Threat Flags

| Flag | File | Description |
|------|------|-------------|
| threat_flag: compiled advanced fixture payload | `crates/yune-core/src/dictionary/compiled_table.rs` | Optional `YUNE-TABLE-ADV` bytes become stems, phrase entries, and encoder rules; covered by plan threat model T-04-03-01/T-04-03-02 with checked parsing and tests. |
| threat_flag: compiled reverse settings payload | `crates/yune-core/src/dictionary/compiled_reverse.rs` | Optional `YUNE-REVERSE` bytes become reverse comments/settings; covered by plan threat model T-04-03-01/T-04-03-04. |

## Self-Check: PASSED

- Verified summary and key modified files exist.
- Verified task commits exist: `f57685a`, `1d3cad5`, `418cdb3`, `c5b9937`, `e7a8d22`, `fae3c30`.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

DATA-03 is complete for the Phase 04 compatibility slice. Future work can build on explicit dictionary advanced contracts while keeping Phase 5 userdb/learning behavior separate.

---
*Phase: 04-compiled-dictionary-data*
*Completed: 2026-04-29*
