# Yune

## What This Is

Yune is a Rust input-method engine project that uses librime as the external
compatibility oracle while avoiding a direct C++ architecture clone. It already
has a deterministic core, a focused RIME-style C ABI shim, schema-loaded
compatibility slices, and frontend-style ABI tests. The next milestone is to
turn that compatibility surface into a stronger frontend and data-compatibility
validation path without losing the clean module boundaries established by the
recent refactor.

## Core Value

Existing RIME schemas and frontends should behave predictably through Yune's
Rust implementation, with every compatibility difference measurable against
librime before it is accepted.

## Requirements

### Validated

- ✓ Rust workspace with `yune-core`, `yune-schema`, `yune-rime-api`, and
  `yune-cli` crates — existing
- ✓ Deterministic core fixture runner for composition, candidates, commits, and
  status snapshots — existing
- ✓ Focused RIME-style C ABI subset for sessions, context/status/commit, config,
  levers, schema lists, deployment helpers, modules, runtime options, and key
  processing — existing
- ✓ Frontend-style compatibility client that drives `yune-rime-api` through the
  exported `RimeApi` function table — existing
- ✓ Schema-loaded compatibility subsets for high-value processors, segmentors,
  translators, and filters including `key_binder`, `punctuator`, `recognizer`,
  `ascii_composer`, `speller`, `abc_segmentor`, `ascii_segmentor`, `matcher`,
  `affix_segmentor`, `table_translator`, `script_translator`,
  `r10n_translator`, `reverse_lookup_translator`, `history_translator`,
  `switch_translator`, `schema_list_translator`, `simplifier`, `uniquifier`,
  `single_char_filter`, `charset_filter`/`cjk_minifier`, and
  `reverse_lookup_filter` — existing
- ✓ Source `.dict.yaml` compatibility coverage for many librime/yaml-cpp edge
  cases, dictionary imports, preset vocabulary, table encoder primitives,
  checksum metadata, and rebuild-plan groundwork — existing
- ✓ Mechanical module split for current `yune-core`, `yune-rime-api`,
  `yune-rime-api` unit tests, and preparatory `yune-cli` modules — existing

### Active

- [ ] Build `yune-cli` into a frontend-surrogate input method that drives
  `yune-rime-api` rather than `yune-core` directly.
- [ ] Validate the current ABI against real frontend clients or native
  frontend-like loading paths and record the resulting compatibility gaps.
- [ ] Broaden schema coverage toward remaining librime gear behavior and deeper
  semantics beyond the current focused subset.
- [ ] Expand dictionary compatibility from source parsing and metadata checks
  toward compiled payload consumption and rebuild execution.
- [ ] Expand user dictionary compatibility beyond plain text shims toward
  librime-style storage, recovery, learning, and frequency behavior.
- [ ] Preserve module and test ownership boundaries for every new compatibility
  slice so future work does not collapse back into single-file accumulation.

### Out of Scope

- Full C++ librime plugin ABI compatibility — defer until a real frontend or
  distribution requirement makes it necessary.
- Cloud inference as a required runtime dependency — classic input behavior must
  remain local-first and low latency.
- A new graphical end-user frontend — the CLI frontend is a validation
  surrogate, not native UI integration.
- Rewriting working compatibility slices during mechanical refactors — preserve
  observable behavior unless a commit explicitly targets behavior.

## Context

Yune's planning documents live in `docs/analysis.md`, `docs/roadmap.md`, and
`docs/refactor-plan.md`. They establish that librime is the behavior oracle for
schema semantics, config behavior, candidate output, C ABI expectations,
deployed data compatibility, and frontend integration. The implementation is
allowed to remain idiomatic Rust internally when the external contract remains
compatible.

The codebase map in `.planning/codebase/` identifies the main architecture:
`crates/yune-core/src/` owns reusable engine behavior, `crates/yune-rime-api/src/`
owns the librime-shaped ABI and schema installation layer, `crates/yune-cli/src/`
currently owns fixture execution and has a reserved RIME frontend entry point,
and `crates/yune-schema/src/lib.rs` owns the typed schema subset parser.

The strongest current coverage is in ABI surface tests, schema-loaded focused
behavior, config/deployment compatibility, source dictionary parsing, and
mechanical module organization. The highest-risk remaining areas are native
frontend lifecycle behavior, resource path validation, process-wide ABI state,
compiled dictionary payloads, LevelDB/userdb behavior, distribution-scale
performance, and unmodeled librime gear components.

## Constraints

- **Compatibility**: `/Users/trenton/Projects/librime` is the external behavior
  oracle for user-visible behavior, schema semantics, ABI contracts, and
  migration support.
- **Architecture**: Prefer typed, idiomatic Rust modules over cloning librime's
  internal C++ structure when the boundary contract is preserved.
- **Testing**: Run focused tests for each behavior slice and `cargo test
  --workspace` after broader phases; use `cargo clippy --workspace --all-targets
  -- -D warnings` as the quality gate when implementation changes warrant it.
- **Frontend validation**: The CLI frontend is an intermediate validation layer;
  it is not proof that Squirrel, Weasel, ibus-rime, fcitx-rime, or fcitx5-rime
  integration is complete.
- **Data compatibility**: Source `.dict.yaml` support is not enough for
  production-scale compatibility; compiled `.table.bin`, `.prism.bin`, and
  `.reverse.bin` payloads remain a required direction.
- **Security**: Runtime resource identifiers must be treated as logical IDs, not
  arbitrary filesystem paths.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Use librime as compatibility oracle, not architecture template | Existing schemas and frontends depend on librime contracts, but Rust can model internals more cleanly | ✓ Good |
| Build compatibility fixtures and ABI tests before replacing deeper engine modules | Behavior must be measurable before differences can be classified as improvements or regressions | ✓ Good |
| Keep AI ranking optional and local-first | Classic input must remain predictable and low latency without network access | — Pending |
| Treat the recent refactor as a structural rule for future feature work | Large single-file accumulation slowed review, search, focused testing, and extraction | — Pending |
| Keep plugin ABI compatibility deferred | Plugin compatibility is expensive and not yet required by a concrete frontend or schema migration path | — Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `$gsd-transition`):
1. Requirements invalidated? -> Move to Out of Scope with reason
2. Requirements validated? -> Move to Validated with phase reference
3. New requirements emerged? -> Add to Active
4. Decisions to log? -> Add to Key Decisions
5. "What This Is" still accurate? -> Update if drifted

**After each milestone** (via `$gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-04-28 after initialization from existing docs and codebase map*
