# Phase 9: Browser Filesystem And Persistence - Context

**Gathered:** 2026-05-05
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 9 makes the Phase 8 TypeScript runtime usable in a browser-hosted Emscripten filesystem. It owns browser virtual filesystem layout creation, schema and dictionary asset preload before `TypeDuckRuntime.init`, persistence sync orchestration before init and after deploy/customize/userdb-changing flows, and documented/tested recovery behavior for missing assets, failed sync, and stale deployed configs.

This phase does not clone or patch upstream TypeDuck-Web, does not run real TypeDuck-Web browser E2E, does not broaden the `yune_typeduck_*` native adapter contract unless a filesystem blocker requires a focused fix, does not add broad frontend bundler/app scaffolding, and does not introduce AI-native provider/ranking/context/memory behavior. Those remain Phase 10 and the future AI-native milestone.

</domain>

<decisions>
## Implementation Decisions

### Browser Filesystem Host Shape
- **D-01:** Add browser filesystem orchestration beside the Phase 8 TypeScript runtime package rather than in Rust core. Phase 9 should expose a small host-side helper layer that prepares Emscripten `FS`/`IDBFS` state before `TypeDuckRuntime.init`.
- **D-02:** Keep the helper DOM-free and TypeDuck-Web-app-free. It should be testable with a fake Emscripten filesystem/module in Node/Vitest, similar to the Phase 8 fake Module tests, rather than requiring a real browser or upstream TypeDuck-Web checkout.
- **D-03:** Use explicit caller-provided paths for `sharedDataDir`, `userDataDir`, and `schemaId`, carrying forward the Phase 8 runtime options. The helper may derive `userDataDir/build`, but it must not invent hidden global paths or support multiple simultaneous process-global services.

### Virtual Filesystem Layout And Asset Preload
- **D-04:** The prepared layout must include `shared_data_dir`, `user_data_dir`, and `user_data_dir/build` before init. These path names should be treated as browser virtual filesystem paths passed to the existing adapter, not as arbitrary native filesystem paths.
- **D-05:** Asset preload should be explicit and schema-scoped: preload `default.yaml`, `<schema>.schema.yaml`, and the selected `<dict>.dict.yaml` into `shared_data_dir`, and ensure deployed/preloaded `default.yaml` and `<schema>.schema.yaml` exist under `user_data_dir/build` before init.
- **D-06:** Do not hide missing preload data by fabricating placeholder schema or dictionary files. Missing assets should remain a deterministic setup/init failure so the recovery path can tell callers what is absent.
- **D-07:** Resource identifiers remain logical IDs. Browser helper code must not accept path-like schema or dictionary IDs that include traversal, absolute path syntax, or platform separators before joining virtual paths.

### Persistence Sync Policy
- **D-08:** Treat persistence sync as an explicit host-owned operation around the runtime: sync from persistent storage before init, and sync back after deploy, customize, cleanup when needed, and any userdb-changing flows that Phase 9 can observe or document.
- **D-09:** Use IDBFS as the primary documented Emscripten persistence target, but keep the helper abstraction narrow enough that tests can use a fake sync backend and docs can say “IDBFS or equivalent.”
- **D-10:** Sync failures should be surfaced as deterministic TypeScript errors, not swallowed. Callers need to know whether init is using fresh persistent data, stale in-memory data, or no persisted data.
- **D-11:** Phase 9 may provide convenience wrappers that call `runtime.deploy()` or `runtime.customize(...)` and then sync, but it should not change the underlying Phase 8 `TypeDuckRuntime` ownership/freeing contract.

### Failure And Recovery Behavior
- **D-12:** Missing assets, failed sync, and stale deployed config should have focused tests or documented repros. The expected behavior is visible failure plus actionable recovery instructions, not silent best-effort continuation.
- **D-13:** Stale deployed config recovery should prefer rerunning explicit preload/deploy/sync flows before init where possible. If recovery requires a live runtime, document the order clearly so callers do not initialize with incomplete state.
- **D-14:** Keep recovery paths local-first and deterministic. Do not add network fetch, remote asset discovery, or TypeDuck-Web-specific app policy in this phase.

### Claude's Discretion
- Choose exact TypeScript file names and helper API shape during planning, as long as the result stays small, package-local, and deterministic under Vitest.
- Prefer a fake Emscripten `FS`/`IDBFS` contract in tests over real browser automation. Real TypeDuck-Web/browser validation remains Phase 10.
- Prefer documenting userdb sync boundaries if the TypeScript wrapper cannot directly observe every native userdb mutation without widening the adapter contract.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope And Requirements
- `.planning/ROADMAP.md` — Phase 9 goal, success criteria, and planned slices `09-01` through `09-03` for filesystem layout/preload, persistence sync, and failure-mode docs/tests.
- `.planning/REQUIREMENTS.md` — `TYPEDUCK-FS-01` through `TYPEDUCK-FS-04`, defining browser setup, asset preload, sync timing, and recovery expectations.
- `.planning/PROJECT.md` — Compatibility, security, local-first, process-global lifecycle, and AI-native deferral constraints.
- `.planning/STATE.md` — Current project state; note this file may lag actual Phase 8 completion, so agents should prefer phase artifacts and git state when checking current progress.

### Phase 8 Handoff
- `.planning/phases/08-typescript-bridge-and-runtime-package/08-CONTEXT.md` — Locked TypeScript wrapper boundaries, injected Module interface, response ownership, key mapping, lifecycle constraints, and explicit deferral of browser filesystem work to Phase 9.
- `packages/yune-typeduck-runtime/src/typeduck.ts` — Current `TypeDuckRuntime.init`, lifecycle, deploy/customize, cleanup, and event-processing wrapper shape that Phase 9 helpers should compose with rather than replace.
- `packages/yune-typeduck-runtime/src/module.ts` — Narrow Emscripten Module binding surface; Phase 9 should extend host filesystem types separately instead of bloating adapter symbol binding.
- `packages/yune-typeduck-runtime/test/fake-module.ts` — Existing fake Module pattern for deterministic TypeScript tests.
- `packages/yune-typeduck-runtime/package.json` and `packages/yune-typeduck-runtime/tsconfig.json` — Package-local TypeScript/Vitest tooling that Phase 9 should reuse rather than adding root JS workspace scaffolding.

### Adapter And Browser Contract
- `docs/typeduck-web-adapter.md` — Current browser filesystem contract, TypeScript wrapper flow, IDBFS responsibility notes, response ownership, exported symbols, and Phase 9 deferrals.
- `crates/yune-rime-api/src/typeduck_web.rs` — Native adapter init/preload guard, path/schema expectations, deploy/customize behavior, process-global cleanup, and response generation.
- `crates/yune-rime-api/tests/typeduck_web.rs` — Native fallback tests for browser host layout constraints, missing assets, lifecycle, response copying/freeing, and process-global serialization.
- `scripts/typeduck-exports.txt` — Canonical `yune_typeduck_*` export list; Phase 9 should not require additional native exports unless a focused blocker proves it necessary.

### Runtime Filesystem And Persistence Internals
- `crates/yune-rime-api/src/runtime.rs` — Runtime path resolution, installation metadata, user data/shared data path handling, sync/log path derivation.
- `crates/yune-rime-api/src/deployment.rs` — Deploy, staging/build freshness, build-info behavior, maintenance, and sync-facing runtime operations.
- `crates/yune-rime-api/src/config_api.rs` and `crates/yune-rime-api/src/config_compiler.rs` — Deployed/user YAML config loading, patch/custom config behavior, and freshness metadata interactions.
- `crates/yune-rime-api/src/userdb.rs` and `crates/yune-rime-api/src/userdb/` — User dictionary storage, snapshots, recovery, sync, and mutation behavior relevant to persistence timing.

### Codebase Maps
- `.planning/codebase/STACK.md` — Rust workspace plus package-local TypeScript runtime package context.
- `.planning/codebase/INTEGRATIONS.md` — Runtime filesystem, userdb, deployment, config, and frontend integration points.
- `.planning/codebase/ARCHITECTURE.md` — Process-global runtime state, ABI boundary, runtime path flow, facade constraints, and integration-test patterns.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `packages/yune-typeduck-runtime`: The Phase 8 package is the natural home for browser host helpers because it already owns typed wrapper ergonomics and deterministic Vitest coverage.
- `packages/yune-typeduck-runtime/test/fake-module.ts`: Provides the pattern for fake Emscripten-like objects; Phase 9 can add a fake `FS`/sync backend beside it.
- `docs/typeduck-web-adapter.md`: Already names the expected browser paths and sync moments; Phase 9 should turn these from documentation-only responsibilities into tested helper behavior.
- `crates/yune-rime-api/tests/typeduck_web.rs`: Native tests already prove init fails on missing browser assets and succeeds when required shared/build files exist.

### Established Patterns
- Browser and frontend compatibility surfaces stay adapter-shaped and adjacent to `yune-rime-api`/TypeScript wrapper boundaries; `yune-core` should not learn about Emscripten, IDBFS, or browser assets.
- Process-global service state means helpers should guide one active runtime per Module instance and avoid multi-runtime path isolation promises.
- Deterministic local tests are preferred over mandatory browser tooling until Phase 10 performs real app E2E.
- Missing external/browser tooling is a blocker or fake-tested boundary, not a reason to silently skip required behavior.

### Integration Points
- Phase 9 helper code should run before `TypeDuckRuntime.init(...)` and compose with the returned `TypeDuckRuntime` for deploy/customize/sync-after flows.
- The helper must create/write the Emscripten virtual paths that the native adapter sees as `shared_data_dir`, `user_data_dir`, and `user_data_dir/build`.
- Persistence sync connects to Emscripten `FS.syncfs`/IDBFS or a narrow equivalent interface, not to native Rust filesystem APIs directly.
- Failure-mode docs/tests connect to adapter-visible init failures, TypeScript setup errors, sync callback errors, and stale deployed config recovery ordering.

</code_context>

<specifics>
## Specific Ideas

- A likely helper shape is `prepareTypeDuckFilesystem(moduleOrFs, options)` followed by `TypeDuckRuntime.init(module, options.runtime)`; planning may choose the exact names.
- Asset preload inputs should be explicit data objects or file descriptors, not implicit network fetches. Network/app-specific asset loading belongs to Phase 10 or the consuming app.
- Sync helpers should make ordering easy to read, for example `syncBeforeInit`, `syncAfterDeploy`, or a small host wrapper around `runtime.deploy()`/`runtime.customize()`.
- Stale config recovery should be documented as a sequence the browser host can repeat deterministically: sync from storage, ensure/preload assets, deploy if needed, sync back, then init/select/process.

</specifics>

<deferred>
## Deferred Ideas

- Upstream TypeDuck-Web clone, source seam identification, app patching, and real browser E2E remain Phase 10.
- Network asset fetching, CDN/cache policy, service worker integration, and TypeDuck-Web-specific application state remain out of scope unless Phase 10 requires them.
- Native adapter API expansion for richer userdb mutation notifications is deferred unless Phase 9 planning finds no safe way to document or wrap sync timing with the current exports.
- Multi-instance isolation beyond one active process-global Yune/RIME service remains out of scope for this milestone.
- AI-native provider, ranking, context, memory, privacy, and frontend exposure behavior remains deferred until TypeDuck-Web integration produces a go/no-go recommendation.

</deferred>

---

*Phase: 09-browser-filesystem-and-persistence*
*Context gathered: 2026-05-05*
