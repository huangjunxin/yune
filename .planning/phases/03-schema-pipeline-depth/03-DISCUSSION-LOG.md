# Phase 3: Schema Pipeline Depth - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-29
**Phase:** 03-schema-pipeline-depth
**Areas discussed:** Processor and Segmentor Depth, Remaining Gear Policy, Distribution Schema Comparison, Spelling OpenCC Correction Boundaries

---

## Processor and Segmentor Depth

| Option | Description | Selected |
|--------|-------------|----------|
| ABI-facing chain-level tests first | Drive schema-loaded sessions through the RIME ABI path, compare user-visible behavior, then implement focused gaps. | ✓ |
| Core-only isolated tests first | Add behavior in core-only units before proving ABI/schema-chain effects. | |
| Broad implementation before comparisons | Implement larger behavior first, then add tests after. | |

**User's choice:** Auto-selected recommended default.
**Notes:** Existing Phase 1 and Phase 2 decisions require ABI-facing compatibility evidence and module/test ownership for new slices.

---

## Remaining Gear Policy

| Option | Description | Selected |
|--------|-------------|----------|
| Feasible increments plus structured deferrals | Add small compatibility increments when possible; otherwise document observed librime role, reason for deferral, and target phase. | ✓ |
| Implement all remaining gears now | Attempt full `memory`, `poet`/`grammar`, `contextual_translation`, and `unity_table_encoder` parity in Phase 3. | |
| Defer all remaining gears without investigation | Leave all remaining gears untouched and undocumented until later. | |

**User's choice:** Auto-selected recommended default.
**Notes:** This keeps Phase 3 honest without pulling compiled-data or userdb-heavy work forward.

---

## Distribution Schema Comparison

| Option | Description | Selected |
|--------|-------------|----------|
| Direct comparison converted to focused fixtures | Compare selected larger schema chains against librime and convert differences into targeted tests or findings. | ✓ |
| Large golden snapshots only | Record broad outputs without isolating behavior ownership. | |
| Skip comparisons until compiled data is implemented | Avoid larger schema comparisons in Phase 3. | |

**User's choice:** Auto-selected recommended default.
**Notes:** Phase 3 success criteria explicitly require documented comparisons against librime for larger distribution schema chains.

---

## Spelling OpenCC Correction Boundaries

| Option | Description | Selected |
|--------|-------------|----------|
| Targeted expansion with compiled-data deferrals | Expand schema-visible lookup/filter behavior now; defer compiled-payload-dependent correction/tolerance and full OpenCC data-chain work. | ✓ |
| Full parity in Phase 3 | Attempt complete spelling algebra, correction/tolerance, and OpenCC compatibility now. | |
| Document only, no behavior increments | Avoid behavior changes and only record gaps. | |

**User's choice:** Auto-selected recommended default.
**Notes:** This preserves the Phase 3 boundary while preventing small built-in maps or metadata-only support from being overstated as full parity.

---

## Claude's Discretion

- Exact fixture names, selected distribution schemas, comparison script shape, and findings format are left to planning/execution.
- The planner may split work by behavior ownership if that avoids mixing mechanical test moves with semantic changes.

## Deferred Ideas

- Compiled table/prism/reverse payload consumption remains Phase 4 scope.
- LevelDB/userdb learning and storage behavior remains Phase 5 scope.
- Plugin ABI and AI-native input behavior remain future milestone scope.
