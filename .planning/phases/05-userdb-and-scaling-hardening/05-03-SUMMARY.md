---
phase: 05-userdb-and-scaling-hardening
plan: 03
subsystem: testing
tags: [rust, yune-core, test-ownership, librime-compatibility]

requires:
  - phase: 05-userdb-and-scaling-hardening
    provides: 05-01 userdb storage boundary and 05-02 core userdb learning/ranking behavior tests already in owned modules
provides:
  - Behavior-owned yune-core engine, translator, and filter test modules split from the core facade
  - Core facade retained as exports plus key/dictionary facade-specific tests only
  - Focused formatting and yune-core engine/translator/filter gate results for Phase 05 closure
affects: [05-04, phase-05-quality-closure, core-test-ownership]

tech-stack:
  added: []
  patterns:
    - Rust unit tests mounted through a focused crate-private tests module tree
    - Mechanical assertion-preserving test movement by behavior ownership

key-files:
  created:
    - crates/yune-core/src/tests/mod.rs
    - crates/yune-core/src/tests/engine.rs
    - crates/yune-core/src/tests/translator.rs
    - crates/yune-core/src/tests/filter.rs
  modified:
    - crates/yune-core/src/lib.rs

key-decisions:
  - "Split only engine, translator, and filter behavior tests from lib.rs; leave key, dictionary, and facade-specific tests in place until their owning modules are targeted."
  - "Treat existing core userdb tests in crates/yune-core/src/userdb.rs as already behavior-owned rather than moving them into translator.rs."

patterns-established:
  - "crates/yune-core/src/lib.rs stays a facade plus facade-owned tests; cross-cutting behavior tests live under crates/yune-core/src/tests/*.rs."
  - "Mechanical test splits must preserve assertions and production behavior, with rustfmt as a separate verification commit when needed."

requirements-completed: [QUAL-03, QUAL-04]

duration: 6min
completed: 2026-04-30T04:19:45Z
---

# Phase 05 Plan 03: Core Test Ownership Split Summary

**Yune-core engine, translator, and filter behavior tests split into focused owner modules while preserving assertions and facade boundaries.**

## Performance

- **Duration:** 6 min
- **Started:** 2026-04-30T04:13:48Z
- **Completed:** 2026-04-30T04:19:45Z
- **Tasks:** 3
- **Files modified:** 5

## Accomplishments

- Inventoried remaining `yune-core` facade tests by behavior ownership and confirmed the plan scope was mechanical-only.
- Moved 49 engine behavior tests into `crates/yune-core/src/tests/engine.rs`, including composition, selection, commit, paging, runtime options/properties, and ranker-order coverage.
- Moved translator-owned reverse lookup, history, and punctuation translator tests into `crates/yune-core/src/tests/translator.rs`.
- Moved filter-owned reverse lookup filter, uniquifier, single-char, charset, simplifier, tagged-filter, and table-translator interaction tests into `crates/yune-core/src/tests/filter.rs`.
- Kept `crates/yune-core/src/lib.rs` as facade/export glue plus tests that remain key, dictionary, compiled payload, encoder, or facade-specific.

## Task Commits

Each task was committed atomically:

1. **Task 1: Inventory core tests by behavior ownership** - `c6776be` (chore)
2. **Task 2: Move core tests without changing assertions** - `8c12e96` (test)
3. **Task 3: Verify core split boundaries and summarize D-15 coverage** - `7e7e8ae` (style)

## Files Created/Modified

- `crates/yune-core/src/tests/mod.rs` - Mounts the focused core behavior test modules.
- `crates/yune-core/src/tests/engine.rs` - Owns core engine composition, commit, selection, paging, option/property, and ranker-flow behavior tests.
- `crates/yune-core/src/tests/translator.rs` - Owns reverse lookup translator, history translator, and punctuation translator behavior tests.
- `crates/yune-core/src/tests/filter.rs` - Owns reverse lookup, uniquifier, single char, charset, simplifier, tagged filter, and table-translator/filter interaction tests.
- `crates/yune-core/src/lib.rs` - Adds the crate-private test module declaration and retains facade plus still-owned key/dictionary/compiled-data tests.

## Core Test Inventory

### Moved: Engine ownership

- Composition and commit flow: `commits_table_candidate_before_echo_candidate`, raw/script/comment commit tests, direct composition controls, and sequence snapshot status.
- Selector/navigation behavior: numeric/keypad selection, page movement, highlight movement, direct candidate select/delete, and out-of-page digit handling.
- Editor/navigator behavior: escape, delete, backspace, syllable jumps, caret movement, home/end behavior, and modified key fallbacks.
- Runtime engine state: options, properties, candidate refresh after backspace, and optional ranker ordering/pending behavior.

### Moved: Translator ownership

- Reverse lookup translator comments, completion opt-in, and segment-tag gating.
- History translator recent commit behavior and tag gating.
- Punctuation translator half/full shape, symbol fallback, comment, and commit behavior.

### Moved: Filter ownership

- Reverse lookup filter comment mutation and projection formatting.
- Uniquifier, single-char, charset, simplifier, tagged-filter behavior.
- Table translator tests that exercise filter-facing option/tag/comment/quality interactions were placed with filter coverage because they are used by the focused `cargo test -p yune-core filter` gate.

### Already behavior-owned

- Core userdb candidate, learning, predictive lookup, and backdated scan tests remain in `crates/yune-core/src/userdb.rs`; moving them to `crates/yune-core/src/tests/translator.rs` would reduce ownership clarity because their implementation owner is `crates/yune-core/src/userdb.rs`, not the translator module.
- Key parsing tests remain in `crates/yune-core/src/lib.rs` for this plan because `key.rs` was outside the target split buckets and moving key tests belongs to a separate ownership cleanup.
- Dictionary/source, compiled payload, rebuild-plan, and encoder tests remain in `crates/yune-core/src/lib.rs` because this plan targeted engine, translator/userdb-candidate, and filter buckets only.

### Move would require semantic change

- None. All moves were mechanical test relocation, import adjustment, and module declaration/formatting only.

## D-15 Ownership Coverage

| Core behavior | Implementation module | Owning test module | Librime comparison target |
|---|---|---|---|
| Engine composition, commit, selection, paging, and runtime state | `crates/yune-core/src/engine.rs` | `crates/yune-core/src/tests/engine.rs` | librime engine/editor/selector/navigator observable key and candidate behavior |
| Userdb learning, predictive lookup, and backdated scan | `crates/yune-core/src/userdb.rs` with engine seams in `crates/yune-core/src/engine.rs` | `crates/yune-core/src/userdb.rs` | librime userdb learning, prediction, frequency, and backdated scan behavior |
| Reverse lookup, history, punctuation translator behavior | `crates/yune-core/src/translator/mod.rs` and `crates/yune-core/src/punctuation.rs` | `crates/yune-core/src/tests/translator.rs` | librime translator completion, tags, comments, history, and punctuation candidates |
| Candidate filtering and filter-visible translator interactions | `crates/yune-core/src/filter/mod.rs` and translator option/tag seams | `crates/yune-core/src/tests/filter.rs` | librime reverse lookup filter, simplifier/OpenCC-like tips, uniquifier, charset, single-char, and tag-gated filter behavior |
| Key parsing and dictionary/compiled data retained for future ownership cleanup | `crates/yune-core/src/key.rs`, `crates/yune-core/src/dictionary/*` | `crates/yune-core/src/lib.rs` retained tests | librime key names, dict compiler, source dictionary, compiled table/prism/reverse behavior |

## Verification Commands Run

- `grep -R "mod tests" crates/yune-core/src/lib.rs | grep -v '^#' || true` — reported only the crate-private `mod tests;` declaration after split.
- `($HOME/.cargo/bin/cargo test -p yune-core engine -- --nocapture && $HOME/.cargo/bin/cargo test -p yune-core translator -- --nocapture && $HOME/.cargo/bin/cargo test -p yune-core filter -- --nocapture) || (cargo test -p yune-core engine -- --nocapture && cargo test -p yune-core translator -- --nocapture && cargo test -p yune-core filter -- --nocapture)` — passed.
- `($HOME/.cargo/bin/cargo fmt --check && $HOME/.cargo/bin/cargo test -p yune-core engine && $HOME/.cargo/bin/cargo test -p yune-core translator && $HOME/.cargo/bin/cargo test -p yune-core filter) || (cargo fmt --check && cargo test -p yune-core engine && cargo test -p yune-core translator && cargo test -p yune-core filter)` — failed first on formatting, then passed after `cargo fmt`.
- `grep -R "fn main" crates/yune-core/src/lib.rs | grep -v '^#' || true` — no output; facade contains no `fn main` implementation logic.

## Decisions Made

- Split only behavior groups that matched this plan's engine, translator/userdb-candidate, and filter targets.
- Left userdb tests in `crates/yune-core/src/userdb.rs` because they are already owned by their implementation module.
- Left key/dictionary/compiled-data tests in the facade because moving them would exceed this plan's specified buckets and should be handled by a future ownership plan.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Applied rustfmt after format gate failed**
- **Found during:** Task 3 (Verify core split boundaries and summarize D-15 coverage)
- **Issue:** `cargo fmt --check` reported formatting differences introduced by the mechanical test split.
- **Fix:** Ran `$HOME/.cargo/bin/cargo fmt` and committed the formatting-only changes separately.
- **Files modified:** `crates/yune-core/src/lib.rs`, `crates/yune-core/src/tests/engine.rs`, `crates/yune-core/src/tests/filter.rs`, `crates/yune-core/src/tests/translator.rs`
- **Verification:** Re-ran `cargo fmt --check` plus focused `yune-core` engine, translator, and filter tests successfully.
- **Committed in:** `7e7e8ae`

---

**Total deviations:** 1 auto-fixed (Rule 3 blocking)
**Impact on plan:** Formatting was required by D-16 and did not alter runtime semantics.

## Issues Encountered

- The initial focused test command fell back to `cargo` after the first compile error, but `cargo` was not on shell PATH. Subsequent successful verification used `$HOME/.cargo/bin/cargo` directly through the plan's preferred command path.
- `cargo fmt --check` failed after the split; resolved with a formatting-only commit.

## User Setup Required

None - no external service configuration required.

## Known Stubs

None. The stub-pattern scan only matched existing dictionary tests that intentionally use the word `placeholder` as fixture data.

## Threat Flags

None. This plan introduced no new network endpoints, auth paths, file access patterns, or schema/trust-boundary changes; changes were limited to test module organization and a facade module declaration.

## Self-Check: PASSED

- Found `crates/yune-core/src/tests/mod.rs`
- Found `crates/yune-core/src/tests/engine.rs`
- Found `crates/yune-core/src/tests/translator.rs`
- Found `crates/yune-core/src/tests/filter.rs`
- Found `.planning/phases/05-userdb-and-scaling-hardening/05-03-SUMMARY.md`
- Found task commits `c6776be`, `8c12e96`, and `7e7e8ae`

## Next Phase Readiness

- Phase 05 quality closure can run focused and workspace gates against behavior-owned core test modules.
- Remaining facade tests are intentionally retained because their behavior owners are key/dictionary/compiled-data rather than the engine/translator/filter buckets in this plan.

---
*Phase: 05-userdb-and-scaling-hardening*
*Completed: 2026-04-30T04:19:45Z*
