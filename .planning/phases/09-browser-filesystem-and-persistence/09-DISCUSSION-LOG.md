# Phase 9: Browser Filesystem And Persistence - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-05-05
**Phase:** 09-browser-filesystem-and-persistence
**Areas discussed:** Browser filesystem host shape, Virtual filesystem layout and asset preload, Persistence sync policy, Failure and recovery behavior

---

## Browser Filesystem Host Shape

| Option | Description | Selected |
|--------|-------------|----------|
| Package-local TypeScript helper | Add a small helper layer beside the Phase 8 runtime package, tested with fake Emscripten FS/module in Node. | ✓ |
| Documentation-only responsibility | Leave all filesystem setup to docs and the eventual TypeDuck-Web app integration. | |
| Rust adapter expansion | Move browser layout orchestration into native Rust adapter behavior. | |

**User's choice:** Auto-selected package-local TypeScript helper.
**Notes:** This matches Phase 8 package-local tooling and keeps Phase 10 app integration separate.

---

## Virtual Filesystem Layout And Asset Preload

| Option | Description | Selected |
|--------|-------------|----------|
| Explicit schema-scoped preload | Require caller-provided assets for `default.yaml`, selected schema YAML, selected dictionary YAML, and deployed build YAML before init. | ✓ |
| Lazy placeholder generation | Fabricate missing config/dictionary files so init can proceed. | |
| App-specific fetch policy | Build network/CDN asset fetching into the runtime package. | |

**User's choice:** Auto-selected explicit schema-scoped preload.
**Notes:** This preserves native adapter missing-asset failures and keeps network/app policy out of Phase 9.

---

## Persistence Sync Policy

| Option | Description | Selected |
|--------|-------------|----------|
| Explicit sync hooks | Sync persistent storage before init and after deploy/customize/userdb-changing flows; surface sync errors. | ✓ |
| Best-effort background sync | Attempt sync silently without blocking wrapper operations. | |
| No sync helper | Keep IDBFS entirely as documentation until Phase 10. | |

**User's choice:** Auto-selected explicit sync hooks.
**Notes:** The helper should support IDBFS or equivalent through a narrow fake-testable interface.

---

## Failure And Recovery Behavior

| Option | Description | Selected |
|--------|-------------|----------|
| Visible failure with recovery sequence | Test/document missing assets, failed sync, and stale deployed config as deterministic failures with actionable recovery order. | ✓ |
| Silent fallback to stale data | Continue with whatever virtual files exist and rely on later app behavior. | |
| Browser E2E-only validation | Defer recovery behavior until upstream TypeDuck-Web is patched. | |

**User's choice:** Auto-selected visible failure with recovery sequence.
**Notes:** Phase 9 should not require real TypeDuck-Web E2E, but should produce deterministic tests/docs that Phase 10 can reuse.

---

## Claude's Discretion

- Exact helper API names and file names are left to planning.
- Exact fake FS/sync interface shape is left to planning.
- Userdb sync observability may be documented if current adapter exports cannot expose every mutation boundary cleanly.

## Deferred Ideas

- Upstream TypeDuck-Web clone/patch/browser E2E remains Phase 10.
- Network asset fetching and app-specific cache/service-worker policy remain out of Phase 9.
- AI-native frontend behavior remains deferred until TypeDuck-Web integration produces a go/no-go recommendation.
