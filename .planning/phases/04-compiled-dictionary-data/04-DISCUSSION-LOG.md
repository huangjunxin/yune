# Phase 4: Compiled Dictionary Data - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-29
**Phase:** 04-compiled-dictionary-data
**Areas discussed:** Payload Scope, Runtime Fallback Policy, Rebuild Semantics, Advanced Compiled Data

---

## Payload Scope

| Option | Description | Selected |
|--------|-------------|----------|
| Minimum usable readers | Implement usable `.table.bin`, `.prism.bin`, and `.reverse.bin` readers beyond metadata, with unsupported sections recorded as findings. | ✓ |
| Metadata only | Keep Phase 4 limited to richer metadata validation without payload consumption. | |
| Full binary parity immediately | Attempt complete librime binary compatibility for every payload section in one pass. | |

**User's choice:** Auto-selected recommended default.
**Notes:** Phase 4 roadmap requires payload consumption beyond checksum metadata; full parity can be staged through focused findings.

---

## Runtime Fallback Policy

| Option | Description | Selected |
|--------|-------------|----------|
| Fresh compiled first, source fallback | Prefer valid fresh compiled payloads and fall back to source YAML when compiled data is missing, stale, unsupported, or invalid. | ✓ |
| Source-first forever | Keep source YAML as runtime default and use compiled data only for tests. | |
| Compiled-only strict mode | Fail whenever compiled data is unavailable, even if source YAML can preserve behavior. | |

**User's choice:** Auto-selected recommended default.
**Notes:** This balances compatibility, deterministic behavior, and incremental rollout while preserving explicit failure when no valid data path exists.

---

## Rebuild Semantics

| Option | Description | Selected |
|--------|-------------|----------|
| Deterministic Rust rebuild | Extend existing checksum/rebuild-plan primitives into local rebuild/freshness behavior without shelling out. | ✓ |
| Shell out to librime compiler | Use librime tools as an implementation dependency for rebuild outputs. | |
| Skip rebuild execution | Only read existing compiled payloads and leave rebuild behavior for later. | |

**User's choice:** Auto-selected recommended default.
**Notes:** Librime remains the oracle, not a runtime dependency; Phase 4 success criteria require rebuild execution/freshness coverage.

---

## Advanced Compiled Data

| Option | Description | Selected |
|--------|-------------|----------|
| Consume representable schema data | Implement stem, reverse `dict_settings`, preset vocabulary, UniTE encoder, correction, and tolerance data where representable in Phase 4 compiled/source paths. | ✓ |
| Defer all advanced data | Leave every advanced data source as a finding for later phases. | |
| Pull userdb/plugin behavior forward | Implement learning, plugin translators, or AI ranking while handling compiled data. | |

**User's choice:** Auto-selected recommended default.
**Notes:** Phase 4 owns compiled-data representation, while LevelDB/userdb learning remains Phase 5 and plugin/AI behavior stays future scope.

---

## Claude's Discretion

- Exact parser module layout, fixture byte construction strategy, selected distribution dictionaries, and findings format are left to planning/execution.
- The planner may split Phase 4 plans by artifact type or runtime flow when that preserves ownership and test focus.

## Deferred Ideas

- LevelDB/userdb learning and storage remain Phase 5.
- Plugin ecosystems and AI-native behavior remain future milestone scope.
- Full OpenCC data-chain parity remains outside Phase 4 except for narrow compiled-lookup integration needs.
