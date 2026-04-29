# Phase 4: Compiled Dictionary Data - Context

**Gathered:** 2026-04-29
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 4 moves dictionary loading and rebuild behavior beyond source `.dict.yaml` parsing and compiled metadata checks into usable compiled librime data compatibility. It covers runtime consumption of `.table.bin`, `.prism.bin`, and `.reverse.bin` payloads; source-vs-prebuilt-vs-staging fallback; rebuild execution/freshness/checksum decisions; and the compiled-data representation needed for stem columns, reverse `dict_settings`, preset vocabulary, UniTE encoder payloads, correction data, and tolerance-search inputs. It does not implement LevelDB/userdb learning/storage, plugin ecosystems, AI-native behavior, or full OpenCC data-chain parity beyond what compiled dictionary lookup requires.

</domain>

<decisions>
## Implementation Decisions

### Payload Scope
- **D-01:** Phase 4 should implement minimum usable readers for librime `.table.bin`, `.prism.bin`, and `.reverse.bin` payloads, not stop at header/checksum metadata.
- **D-02:** Reader work should be behavior-driven against schema-loaded lookup fixtures and librime-derived payload observations. Unsupported binary sections should become structured findings with exact observed format/role and target follow-up, not silent no-ops that imply compatibility.
- **D-03:** Compiled payload parsing belongs in `crates/yune-core/src/dictionary/` or focused submodules owned by the dictionary layer. RIME ABI/schema code should choose resources and install dictionaries, not own binary parsing.

### Runtime Fallback Policy
- **D-04:** Runtime schema installation should prefer valid fresh compiled payloads when available, then fall back to source `.dict.yaml` parsing when compiled payloads are missing, stale, unsupported, or fail validation and source data is available.
- **D-05:** If neither a usable compiled payload nor source dictionary is available, the failure should be explicit and test-covered rather than silently installing an empty dictionary.
- **D-06:** Source and compiled paths must produce the same user-visible candidate ordering for focused fixtures before performance-oriented shortcuts are accepted.

### Rebuild Semantics
- **D-07:** Rebuild execution should be deterministic and local to Yune's Rust implementation, built around existing checksum/rebuild-plan primitives, runtime paths, staging/prebuilt directories, and deployed schema/dictionary resources.
- **D-08:** Do not shell out to librime compilers or depend on external generated artifacts during normal tests. Librime remains the oracle for comparison, not an implementation dependency.
- **D-09:** Freshness decisions should cover table, prism, reverse, source-vs-prebuilt fallback, pack checksum chaining, and forced rebuild flags where librime behavior is observable. Partial rebuild support should be explicit about which artifacts were rebuilt or reused.

### Advanced Compiled Data
- **D-10:** Stem-column data, reverse-db `dict_settings`, preset vocabulary phrase injection, and UniTE-style encoder payloads should be consumed where existing schemas rely on them and where they can be represented from compiled/source data without pulling Phase 5 userdb behavior forward.
- **D-11:** Correction data and tolerance-search inputs should move from Phase 3 schema-visible boundaries into the compiled-data path when the data is present in table/prism/reverse artifacts and can be compared through schema-loaded lookup tests.
- **D-12:** LevelDB/userdb learning, predictive frequency updates, plugin-backed translators, and AI-native ranking/memory remain out of Phase 4 even when compiled-data findings point toward them; record those findings for Phase 5 or future milestones.

### Security And Resource Boundaries
- **D-13:** Compiled-data readers must treat schema-provided dictionary IDs as logical resource IDs and preserve the resource-ID validation established in Phase 2 and Phase 3.
- **D-14:** Binary payload parsing must be bounded and fail closed on malformed lengths, offsets, counts, or unsupported versions. Tests should include malformed local fixture bytes without reading outside the payload or panicking.

### Claude's Discretion
- Exact parser module layout, fixture byte construction strategy, selected distribution dictionaries, and whether findings live in summary sections or focused comparison tests are left to planning/execution, provided ownership and phase boundaries above remain true.
- The planner may split Phase 4 plans by artifact type or by runtime flow if that keeps tests focused and avoids mixing binary parser work with schema-install/rebuild orchestration.

</decisions>

<specifics>
## Specific Ideas

- Treat `/Users/trenton/Projects/librime` as the oracle for compiled payload format and user-visible candidate behavior, but keep Yune's Rust readers idiomatic and bounded.
- Use schema-loaded ABI tests for runtime behavior whenever compiled data affects selected schemas, while lower-level parser tests may live in `yune-core` dictionary modules.
- Preserve the Phase 3 rule that findings outside the active phase are explicit and assigned to Phase 5 or future milestones rather than hidden behind compatibility shims.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope And Requirements
- `.planning/ROADMAP.md` — Phase 4 goal, success criteria, and four planned work slices for compiled dictionary data.
- `.planning/REQUIREMENTS.md` — `DATA-01` through `DATA-04`, which define compiled payload, rebuild, advanced data, and correction/tolerance requirements.
- `.planning/PROJECT.md` — Compatibility oracle, typed Rust architecture, data compatibility boundary, resource-ID security, and module/test ownership constraints.

### Prior Phase Context
- `.planning/phases/03-schema-pipeline-depth/03-CONTEXT.md` — Deferred compiled payload, UniTE payload, correction/tolerance data, and distribution comparison findings into Phase 4.
- `.planning/phases/02-native-abi-validation-and-runtime-safety/02-CONTEXT.md` — Resource-ID validation and ABI/runtime safety boundaries that compiled dictionary file resolution must preserve.
- `.planning/phases/01-cli-frontend-surrogate/01-CONTEXT.md` — ABI/frontend-style test expectations and module/test ownership rules.

### Codebase Maps And Existing Analysis
- `.planning/codebase/ARCHITECTURE.md` — Dictionary model, schema installation flow, runtime paths, and ownership boundaries.
- `.planning/codebase/INTEGRATIONS.md` — RIME schema/data file integration points, staging/prebuilt directories, and current compiled metadata parser location.
- `.planning/codebase/CONCERNS.md` — Compiled-data behavior gap, dictionary scaling limits, source-YAML runtime load path, and resource/security concerns.
- `.planning/codebase/TESTING.md` — Rust test commands, ABI/schema-loaded fixture rules, and workspace quality gates.
- `docs/analysis.md` — Existing compatibility analysis for compiled dictionary payloads, rebuild behavior, correction/tolerance, and userdb boundaries.
- `docs/roadmap.md` — Prior roadmap details for compiled dictionary data and how it fits between schema semantics and userdb hardening.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `crates/yune-core/src/dictionary/compiled.rs` — Existing RIME checksum, compiled metadata, and rebuild-plan primitives; extend into bounded payload readers and richer rebuild decisions.
- `crates/yune-core/src/dictionary/source.rs` — Existing source `.dict.yaml` parsing, imports, packs, preset vocabulary, stem columns, and encoder-adjacent source behavior; use as source fallback and comparison baseline.
- `crates/yune-core/src/translator/mod.rs` — Runtime table/reverse/script lookup and candidate ordering; compiled dictionary data must feed these paths without duplicating translator semantics.
- `crates/yune-rime-api/src/schema_install.rs` — Runtime schema installer that currently loads dictionary resources from deployed YAML/source data; use it to choose compiled vs source dictionaries and preserve logical resource validation.
- `crates/yune-rime-api/src/runtime.rs` and `crates/yune-rime-api/src/deployment.rs` — Runtime path and staging/prebuilt deployment/freshness integration points for compiled artifacts.
- `crates/yune-rime-api/src/tests/distribution_schema_comparison.rs` and `schema_selection.rs` — Existing schema-loaded comparison and selection tests that can host focused compiled-data regressions.

### Established Patterns
- Compatibility changes start with focused tests and an explicit librime comparison target.
- `lib.rs` remains facade/export glue; owned behavior belongs in dictionary, schema-install, runtime/deployment, translator/filter, or focused test modules.
- Resource IDs from schemas and C APIs remain logical IDs and must not become arbitrary filesystem paths.
- Tests should separate low-level parser validity from ABI-visible schema-loaded behavior.

### Integration Points
- `apply_schema_to_session` installs translators/filters after schema selection; compiled dictionary selection should be reproducible from deployed schema config.
- Runtime config lookup prefers staging/prebuilt deployed data roots; compiled artifact lookup must respect the same directory model.
- Rebuild planning already exists in `RimeDictRebuildInput`/`RimeDictRebuildPlan`; Phase 4 should expand this into execution/freshness behavior without bypassing current deployment helpers.

</code_context>

<deferred>
## Deferred Ideas

- LevelDB/userdb storage, learning, frequency updates, predictive lookup, and backdated scan behavior remain Phase 5.
- Full plugin ABI, Lua/octagram/predict/proto ecosystems, and AI-native input behavior remain future milestone scope.
- Full OpenCC conversion-data chain parity remains outside Phase 4 unless a compiled dictionary lookup fixture requires a narrow integration boundary.

</deferred>

---

*Phase: 04-compiled-dictionary-data*
*Context gathered: 2026-04-29*
