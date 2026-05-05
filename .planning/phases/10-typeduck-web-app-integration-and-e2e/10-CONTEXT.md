# Phase 10: TypeDuck-Web App Integration And E2E - Context

**Gathered:** 2026-05-05
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 10 turns the Phase 7–9 TypeDuck/Yune bridge contract into a real app integration. It owns obtaining upstream TypeDuck-Web in a reproducible local test location, identifying the current librime/WASM input-engine seam, patching or configuring that seam to call the repository-owned `@yune-ime/typeduck-runtime` bridge and filesystem helpers, running real browser validation for the TypeDuck-Web flows named in the roadmap, and ending the milestone with a go/no-go recommendation for AI-native frontend exposure.

This phase does not broaden the native `yune_typeduck_*` adapter unless the real TypeDuck-Web seam exposes a blocking mismatch that cannot be solved in TypeScript glue, does not build a new Yune product frontend, does not add AI provider/ranking/context/memory behavior, and does not turn browser asset discovery, CDN/cache policy, service-worker lifecycle, or storage-quota behavior into repository-wide product policy beyond what is necessary to exercise TypeDuck-Web E2E.

</domain>

<decisions>
## Implementation Decisions

### Upstream TypeDuck-Web Source Handling
- **D-01:** Use a reproducible local integration location for upstream TypeDuck-Web rather than copying unknown app code into the core Rust crates. Planning may choose clone-vs-submodule-vs-scripted checkout, but the result must make the exact source revision and setup command auditable.
- **D-02:** Treat upstream TypeDuck-Web as the app under test. Phase 10 should identify and document its existing librime/WASM bridge seam before patching, so later work can distinguish app-source changes from Yune adapter changes.
- **D-03:** Keep TypeDuck-Web source changes isolated and minimal. Prefer a patch/configuration layer that routes the existing engine calls to Yune’s TypeScript runtime over broad rewrites of TypeDuck-Web UI, state, or build behavior.

### Yune Bridge Integration Shape
- **D-04:** The replacement seam should call the Phase 8/9 package surface (`TypeDuckRuntime`, `keyEventToRimeKey`, filesystem preparation, IDBFS/equivalent sync helpers) instead of raw `yune_typeduck_*` exports wherever possible.
- **D-05:** Preserve the one-active-runtime-per-Emscripten-Module constraint from the native adapter and wrapper. Phase 10 should not promise multi-instance browser isolation unless TypeDuck-Web already enforces a compatible single-worker/single-module lifecycle.
- **D-06:** Use explicit TypeDuck-Web-owned assets for `default.yaml`, schema YAML, and dictionary YAML. Do not fabricate fallback schema/dictionary data to make E2E pass; missing or mismatched assets should remain visible integration failures.
- **D-07:** If TypeDuck-Web’s current bridge has concepts the Yune adapter lacks, record the mismatch first and only widen Yune’s adapter with the smallest focused native or TypeScript change needed for the E2E requirement.

### Browser E2E Validation
- **D-08:** Real browser validation is required in this phase. Tests or scripted smoke flows must exercise composition, candidate paging, selection, deletion, commit output, deploy, customize, and persistence smoke behavior through the TypeDuck-Web app seam rather than only package-local fake modules.
- **D-09:** Prefer Playwright or the app’s existing browser test runner if TypeDuck-Web already has one. If local browser tooling is unavailable, the blocker must be recorded reproducibly with the command, missing dependency, and fallback evidence; silent skips are not acceptable.
- **D-10:** E2E assertions should validate user-visible TypeDuck-Web behavior plus bridge-level state where practical. Avoid brittle visual assertions unless the app already uses them.
- **D-11:** Persistence validation should follow the Phase 9 explicit sync contract: populate before init, flush after deploy/customize/userdb-relevant boundaries, reload or reinitialize to prove state survives at least one browser persistence smoke cycle.

### Findings And AI-Native Recommendation
- **D-12:** The final integration findings should separate three classes of outcome: TypeDuck-Web app/source blockers, Yune adapter/runtime mismatches, and environment/tooling blockers.
- **D-13:** The go/no-go recommendation for AI-native frontend exposure should be based on real frontend readiness: whether Yune can drive TypeDuck-Web’s existing flows predictably, whether persistence and lifecycle are stable, and whether remaining gaps are bounded enough for AI-native work not to mask compatibility failures.
- **D-14:** AI-native behavior remains deferred. Phase 10 may recommend the next milestone shape, but it should not add provider calls, AI candidates, ranking policy, context capture, memory, or privacy controls.

### Claude's Discretion
- Choose the exact integration layout, patch format, and E2E command structure during planning after inspecting TypeDuck-Web’s repository and build system.
- Prefer the smallest reliable browser validation harness that proves the roadmap flows over a broad test framework migration.
- If upstream TypeDuck-Web cannot be fetched or built in the local environment, document the blocker with deterministic commands and keep any Yune-side fallback validation clearly labeled as fallback, not as satisfying real browser E2E.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope And Requirements
- `.planning/ROADMAP.md` — Phase 10 goal, success criteria, and planned slices `10-01` through `10-04` for source inspection, seam replacement, real browser E2E, and findings/recommendation.
- `.planning/REQUIREMENTS.md` — `TYPEDUCK-E2E-01` through `TYPEDUCK-E2E-04`, defining reproducible TypeDuck-Web source handling, Yune bridge replacement, browser validation coverage, and final AI-native go/no-go recommendation.
- `.planning/PROJECT.md` — Compatibility-first milestone intent, local-first direction, and AI-native deferral context.
- `.planning/STATE.md` — Current project state showing Phase 10 as ready to plan after Phase 9 completion.

### TypeDuck Adapter Handoff
- `.planning/phases/07-wasm-build-and-export-contract/07-CONTEXT.md` — WASM/export contract decisions and adapter boundary assumptions, if present in the Phase 7 directory.
- `.planning/phases/08-typescript-bridge-and-runtime-package/08-CONTEXT.md` — TypeScript runtime wrapper boundaries, Module injection, response ownership, key mapping, lifecycle, and deferrals.
- `.planning/phases/09-browser-filesystem-and-persistence/09-CONTEXT.md` — Browser filesystem, asset preload, persistence sync, and recovery decisions that Phase 10 must carry forward.
- `docs/typeduck-web-adapter.md` — Current public contract for `yune_typeduck_*` symbols, wrapper usage, browser filesystem layout, IDBFS/equivalent sync, lifecycle constraints, and current non-goals.
- `scripts/typeduck-exports.txt` and `scripts/typeduck-wasm-build.sh` — Canonical adapter export list and verified-or-blocked WASM build command path.

### TypeScript Runtime And Tests
- `packages/yune-typeduck-runtime/src/index.ts` — Package export surface TypeDuck-Web integration should consume.
- `packages/yune-typeduck-runtime/src/typeduck.ts` — `TypeDuckRuntime` lifecycle, key processing, candidate actions, deploy/customize, and cleanup wrapper.
- `packages/yune-typeduck-runtime/src/filesystem.ts` — Browser filesystem layout, asset readiness, persistence mount/sync, deploy/customize sync helpers, and deterministic filesystem error surface.
- `packages/yune-typeduck-runtime/src/keys.ts` — DOM-free browser key mapping that should be used instead of deprecated keyboard APIs.
- `packages/yune-typeduck-runtime/test/fake-module.ts` and `packages/yune-typeduck-runtime/test/fake-filesystem.ts` — Fake test patterns for fallback/non-browser coverage.
- `packages/yune-typeduck-runtime/package.json` and `packages/yune-typeduck-runtime/tsconfig.json` — Package-local TypeScript/Vitest build and module settings.

### Native Adapter And Runtime Internals
- `crates/yune-rime-api/src/typeduck_web.rs` — Native TypeDuck adapter init/preload guard, response generation, lifecycle cleanup, deploy/customize, and process-global constraints.
- `crates/yune-rime-api/tests/typeduck_web.rs` — Native adapter behavior tests covering init, process key, candidate actions, missing assets, lifecycle, and response ownership.
- `crates/yune-rime-api/src/runtime.rs`, `crates/yune-rime-api/src/deployment.rs`, `crates/yune-rime-api/src/config_api.rs`, and `crates/yune-rime-api/src/config_compiler.rs` — Runtime path, deploy, config, and freshness behavior relevant to app integration blockers.
- `crates/yune-rime-api/src/userdb.rs` and `crates/yune-rime-api/src/userdb/` — User dictionary persistence behavior relevant to browser persistence smoke validation.

### Codebase Maps
- `.planning/codebase/STACK.md` — Rust workspace plus TypeScript runtime package context.
- `.planning/codebase/INTEGRATIONS.md` — RIME ABI, filesystem, deployment, config, userdb, and frontend integration boundaries.
- `.planning/codebase/ARCHITECTURE.md` — Layering, process-global runtime state, ABI allocation ownership, and facade constraints.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `packages/yune-typeduck-runtime`: The intended browser-facing Yune integration package; Phase 10 should consume this package from TypeDuck-Web glue rather than duplicating wrapper logic.
- `docs/typeduck-web-adapter.md`: The handoff contract for Emscripten exports, runtime methods, filesystem layout, response ownership, and lifecycle constraints.
- `scripts/typeduck-wasm-build.sh`: The repository-owned verified-or-blocked browser/WASM build check; useful as a preflight before trying to wire TypeDuck-Web to Yune.
- `crates/yune-rime-api/tests/typeduck_web.rs` and `packages/yune-typeduck-runtime/test/*`: Native and fake-module fallback coverage for isolating whether a failure is app-specific or bridge-specific.

### Established Patterns
- Browser integration surfaces stay in the TypeDuck adapter/package layer; `yune-core` remains independent of Emscripten, browser APIs, and TypeDuck-Web application policy.
- The native adapter exposes one active process-global Yune/RIME service; browser glue must treat one Emscripten Module instance as one live `TypeDuckRuntime` at a time.
- Missing browser tooling or upstream app blockers should be reported as blockers with reproducible commands, not hidden behind successful fallback tests.
- Deterministic local-first asset and persistence behavior is preferred over implicit network fetches or app policy invented inside Yune helper code.

### Integration Points
- TypeDuck-Web’s current input-engine binding is the main replacement seam; planners must inspect upstream source before deciding the exact patch.
- Emscripten Module initialization must expose `cwrap`, `UTF8ToString`, `FS`, and the retained `yune_typeduck_*` exports/runtime methods before `TypeDuckRuntime.init` runs.
- Browser filesystem setup must create/write the `sharedDataDir`, `userDataDir`, and `userDataDir/build` paths visible to the native adapter.
- E2E flows should drive TypeDuck-Web UI or app-level APIs far enough to prove composition, candidate actions, deploy/customize, and persistence behavior through the replacement seam.

</code_context>

<specifics>
## Specific Ideas

- A likely plan split is: inspect/record upstream seam, implement a minimal bridge replacement, add browser E2E smoke coverage, then write findings and AI-native readiness recommendation.
- If upstream TypeDuck-Web already has Playwright/Cypress or a build/test script, reuse it instead of introducing a separate browser framework.
- If TypeDuck-Web uses a Web Worker for WASM, preserve the worker boundary and inject Yune’s wrapper there rather than bypassing the app’s concurrency model.
- The final recommendation should be explicit: `GO`, `GO WITH CONDITIONS`, or `NO-GO`, with conditions tied to observed TypeDuck-Web/Yune integration facts.

</specifics>

<deferred>
## Deferred Ideas

- AI-native provider calls, candidate generation/ranking policy, context capture, memory, and privacy controls remain deferred to the future AI-native milestone.
- A new first-party Yune graphical frontend remains deferred; Phase 10 validates against TypeDuck-Web as an existing frontend.
- Multi-instance Yune/RIME service isolation remains deferred unless TypeDuck-Web integration proves it is an immediate blocker.
- Browser product policy for CDN/cache/service worker/storage quota remains deferred unless TypeDuck-Web’s existing build requires a minimal documented choice for E2E.

</deferred>

---

*Phase: 10-typeduck-web-app-integration-and-e2e*
*Context gathered: 2026-05-05*
