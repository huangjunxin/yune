# Phase 3: Schema Pipeline Depth - Context

**Gathered:** 2026-04-29
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 3 expands schema-loaded behavior beyond the current focused subset into deeper librime processor, segmentor, translator, filter, and remaining gear semantics. It covers ABI-visible behavior for deeper `speller`, `editor`, `navigator`, `selector`, `chord_composer`, shape, punctuation, and fallback interactions; explicit compatibility increments or documented deferrals for remaining librime gears; direct larger-schema comparisons against librime; and targeted broadening of spelling algebra, correction/tolerance, and OpenCC behavior where current focused coverage is insufficient. It does not implement compiled `.table.bin`/`.prism.bin`/`.reverse.bin` payload consumption, LevelDB/userdb storage and learning behavior, plugin ABI compatibility, or AI-native input behavior.

</domain>

<decisions>
## Implementation Decisions

### Processor And Segmentor Depth
- **D-01:** Deeper `speller`, `editor`, `navigator`, `selector`, `chord_composer`, shape, punctuation, and fallback behavior should be proven through ABI-facing tests that drive schema-loaded sessions, not only isolated core tests.
- **D-02:** Processor and segmentor work should prioritize larger-chain interactions where existing focused paths can pass while librime differs: previous-match segment splitting, non-auto-commit composition, segment/selection span changes, navigator fallback, raw/fallback segment exclusion, punctuation segment ordering, shape formatting on commits, and chord raw-sequence lifecycle cleanup.
- **D-03:** Add focused failing comparisons or fixtures before changing dispatch order or state mutation in key-processing paths. Each new behavior slice should name its owning implementation module, owning test module, and librime comparison target before code changes.
- **D-04:** Existing processor modules under `crates/yune-rime-api/src/processors/`, `schema_install.rs`, and schema-selection/session state should remain the ownership anchors. Do not move behavior back into `lib.rs` except for unavoidable ABI export glue.

### Remaining Gear Policy
- **D-05:** `memory`, `poet`/`grammar`, `contextual_translation`, and `unity_table_encoder` must not stay invisible. Each needs either a Phase 3 compatibility increment or a structured deferral that states the observed librime role, why it is out of this phase's implementation slice, and which future phase owns it.
- **D-06:** Prefer small compatibility increments when they can be modeled without compiled dictionary payloads or userdb learning. Examples include schema installation recognition, no-op/diagnostic behavior that preserves chain determinism, or focused candidate weighting/annotation behavior when it can be compared directly against librime.
- **D-07:** Defer behavior that depends on compiled reverse data, UniTE compiled payloads, LevelDB-backed learning, or plugin ecosystems rather than adding incomplete shims that imply compatibility.

### Distribution Schema Comparison
- **D-08:** Larger real-world schema-chain work should compare Yune directly against librime and then convert differences into focused fixtures or documented findings. Avoid broad snapshot churn that is hard to diagnose.
- **D-09:** Distribution-scale comparisons should emphasize chain semantics before performance: component order, segment tags, generated spellings, OpenCC/filter behavior, punctuation/fallback behavior, and candidate differences that users would see.
- **D-10:** When a comparison reveals a gap outside Phase 3, record it as a structured finding with observed Yune behavior, expected librime behavior when known, scope decision, and target phase. Compiled dictionary payload gaps should point to Phase 4; userdb learning/storage gaps should point to Phase 5.

### Spelling, OpenCC, Correction, And Tolerance Boundaries
- **D-11:** Broaden spelling algebra only where the current focused `xlit`/`xform`/`erase`/`derive` and generated-spelling penalty coverage is insufficient for schema-loaded lookup compatibility.
- **D-12:** Correction and tolerance-search work should focus on schema-visible lookup/ranking interactions that can be represented without Phase 4 compiled payload consumption. Correction data or tolerance inputs that require compiled prism/table/reverse payloads should be documented for Phase 4.
- **D-13:** OpenCC work should distinguish between filter-chain integration semantics that can be tested now and full OpenCC conversion-data parity, which remains a larger compatibility/data concern. Do not claim full OpenCC compatibility from small built-in maps.

### Claude's Discretion
- Exact fixture names, loop counts, selected distribution schemas, comparison script shape, and whether findings live in test notes or docs are left to the planner/executor, provided the decisions above remain true.
- The planner may split Phase 3's four roadmap plans by behavior ownership if that keeps implementation/test modules focused and avoids mixing mechanical test movement with semantic changes.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope And Requirements
- `.planning/ROADMAP.md` — Phase 3 goal, success criteria, and four planned work slices for schema pipeline depth.
- `.planning/REQUIREMENTS.md` — `SCHEMA-01` through `SCHEMA-05`, which define the required processor, segmentor, remaining-gear, distribution-chain, spelling algebra, correction/tolerance, and OpenCC coverage.
- `.planning/PROJECT.md` — Project constraints: librime as compatibility oracle, typed Rust architecture, schema/data compatibility boundaries, frontend validation caveat, and module/test ownership rules.

### Prior Phase Context
- `.planning/phases/01-cli-frontend-surrogate/01-CONTEXT.md` — Establishes that frontend-surrogate behavior should exercise the RIME ABI path and that every compatibility slice needs implementation/test ownership and a librime comparison target.
- `.planning/phases/02-native-abi-validation-and-runtime-safety/02-CONTEXT.md` — Establishes dynamic-loader/native ABI validation, runtime safety boundaries, and explicit deferral of deeper schema semantics to Phase 3.

### Codebase Maps
- `.planning/codebase/ARCHITECTURE.md` — Schema selection/install flow, key-processing dispatch order, processor/session ownership, and anti-patterns around adding owned behavior to facades or bypassing schema installation.
- `.planning/codebase/INTEGRATIONS.md` — RIME ABI function table, schema/data file integration points, local storage boundaries, and current compiled-data/userdb limitations.
- `.planning/codebase/TESTING.md` — Rust test patterns, ABI/frontend-style test rules, temp runtime setup, fixture conventions, and guidance not to mock the ABI function table for frontend behavior.
- `.planning/codebase/CONCERNS.md` — Fragile key-processing dispatch, large compatibility-suite risks, distribution-scale coverage gaps, and known compiled-data/userdb boundaries.

### Compatibility Strategy
- `docs/analysis.md` — Current schema-pipeline coverage and remaining gaps around deeper processors/segmentors, remaining librime gears, spelling algebra, correction/tolerance, OpenCC, compiled data, and userdb behavior.
- `docs/roadmap.md` — Prior roadmap details for schema pipeline work and how Phase 3 fits between ABI validation and compiled dictionary data.
- `docs/refactor-plan.md` — Module/test ownership rule and refactor guardrails that should constrain all new compatibility slices.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `crates/yune-rime-api/src/processors/speller.rs` — Existing schema-loaded speller behavior for alphabet/delimiter/use-space/autoclear/autoselect paths; extend with previous-match splitting and non-auto-commit composition semantics.
- `crates/yune-rime-api/src/processors/editor.rs` — Existing editor binding and char-handler behavior; extend with deeper segment/selection semantics where librime comparisons show gaps.
- `crates/yune-rime-api/src/processors/navigator.rs` and `crates/yune-rime-api/src/processors/selector.rs` — Existing layout, binding, raw-tag, delimiter, and syllable-jump coverage; extend through candidate/segment span and fallback interactions.
- `crates/yune-rime-api/src/processors/chord_composer.rs` and `crates/yune-rime-api/src/processors/shape.rs` — Existing chord and shape coverage for printable chording, prompt segments, modifiers, full-shape commits, and lifecycle cleanup; extend remaining larger-chain lifecycle edges.
- `crates/yune-rime-api/src/processors/punctuation.rs` — Existing punctuation translator/segmentor hook point for chain-level punctuation behavior.
- `crates/yune-rime-api/src/schema_install.rs` — Installs schema processors, segmentors, translators, filters, and segment tags; use this as the integration point for additional gear recognition or deferral diagnostics.
- `crates/yune-rime-api/src/session.rs` — Owns session-local processor/segmentor state including speller, editor, chord, selector, navigator, punct segmentor, and fallback segmentor flags.
- `crates/yune-core/src/spelling_algebra.rs` and `crates/yune-core/src/translator/mod.rs` — Existing lookup-side spelling algebra and translator behavior to broaden only where Phase 3 requires it.
- `crates/yune-rime-api/src/tests/schema_processors.rs` and `crates/yune-rime-api/src/tests/schema_selection.rs` — Existing ABI-facing schema behavior suites; add focused tests here or split only along clear behavior ownership boundaries.

### Established Patterns
- Schema-loaded behavior should be installed through `apply_schema_to_session` and `schema_install.rs`, not ad hoc session mutation from unrelated API paths.
- ABI-facing compatibility tests use `test_guard()`, unique temp runtime directories, deployed YAML fixtures, session creation/selection, `RimeProcessKey`, and context/status/commit assertions.
- New behavior slices should keep `crates/yune-rime-api/src/lib.rs` as façade/export glue and keep owned logic in processor, schema-install, core translator/filter, or focused helper modules.
- Comparisons should lock down user-visible state: commits, preedit, candidate text/comment/source/order, segment tags, highlight/page state, status flags, and selected schema behavior.

### Integration Points
- `RimeProcessKey` dispatch order in `crates/yune-rime-api/src/lib.rs` is fragile; changes require focused tests before modifying processor ordering, accept/fallback behavior, commit buffering, or segment-tag updates.
- `apply_schema_to_session` in `crates/yune-rime-api/src/schema_selection.rs` resets installed processors, segmentors, translators, filters, paging, composition, buffered input, and unread commits.
- `install_schema_components` and segment-tag helpers in `crates/yune-rime-api/src/schema_install.rs` are the right place to add component recognition, focused support, or structured deferral handling for remaining gears.
- Larger-schema comparison fixtures should reuse the ABI/frontend-style path rather than direct `Engine` shortcuts when validating schema-loaded behavior.

</code_context>

<specifics>
## Specific Ideas

- Treat librime as the oracle for every user-visible schema-pipeline behavior difference; Rust internals can remain idiomatic if the external contract matches.
- Focus Phase 3 on schema semantics and chain behavior, not data payload implementation. Compiled payload gaps discovered during schema comparison should become Phase 4 findings rather than Phase 3 shims.
- Current docs identify `memory`, `poet`/`grammar`, `contextual_translation`, and `unity_table_encoder` as remaining gear gaps; Phase 3 must make an explicit decision for each instead of leaving them implicit.

</specifics>

<deferred>
## Deferred Ideas

- Compiled `.table.bin`, `.prism.bin`, and `.reverse.bin` payload consumption, rebuild execution, pack checksum chaining, and compiled correction/tolerance data belong to Phase 4.
- LevelDB/userdb storage, learning, frequency updates, predictive lookup, recovery, sync, and transaction behavior belong to Phase 5.
- Full librime C++ plugin ABI compatibility, Lua/octagram/predict/proto plugin ecosystems, and AI-native provider/ranking/context/memory behavior remain outside the current compatibility milestone.

</deferred>

---

*Phase: 03-schema-pipeline-depth*
*Context gathered: 2026-04-29*
