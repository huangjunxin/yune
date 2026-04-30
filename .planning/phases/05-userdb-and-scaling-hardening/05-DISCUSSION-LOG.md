# Phase 5: UserDB And Scaling Hardening - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-30
**Phase:** 05-userdb-and-scaling-hardening
**Areas discussed:** storage compatibility boundary, lifecycle and sync semantics, learning and ranking semantics, test and quality hardening

---

## Storage Compatibility Boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Focused Rust userdb abstraction | Model librime-observable LevelDB/userdb behavior behind an owned Rust abstraction while keeping core independent of storage engine and ABI details. | ✓ |
| Direct LevelDB clone first | Add a LevelDB dependency and mimic storage details before defining observable behavior seams. | |
| Keep plain text shim | Extend the current `.userdb` text-file shim and document differences later. | |

**User's choice:** Auto-selected focused Rust userdb abstraction.
**Notes:** The roadmap allows either librime-compatible LevelDB/userdb behavior or a documented compatible abstraction. Auto mode chose behavior compatibility first because Phase 5 should preserve external semantics without forcing librime internals into the Rust core.

---

## Lifecycle And Sync Semantics

| Option | Description | Selected |
|--------|-------------|----------|
| Explicit transaction boundaries | Treat backup, restore, import, export, sync, recovery, and rollback as deterministic local transaction boundaries with failure tests. | ✓ |
| Best-effort file operations | Keep current whole-file copy/merge behavior and harden only obvious errors. | |
| Defer recovery semantics | Implement storage first and leave rollback/recovery tests for later. | |

**User's choice:** Auto-selected explicit transaction boundaries.
**Notes:** USERDB-02 requires snapshot backup, restore, recovery, sync, and rollback behavior to match librime-observable semantics, so these cannot remain incidental file operations.

---

## Learning And Ranking Semantics

| Option | Description | Selected |
|--------|-------------|----------|
| Commit-driven classic personalization | Update userdb state from normal runtime commits and feed deterministic frequency/predictive effects back into candidate quality/order. | ✓ |
| Ranker-only personalization | Reuse `CandidateRanker` or mock AI ranking hooks for learned ordering. | |
| Parser-only userdb tests | Validate stored records but avoid session-level learning behavior until a future milestone. | |

**User's choice:** Auto-selected commit-driven classic personalization.
**Notes:** USERDB-03 requires learning, frequency updates, predictive lookup, and backdated scan behavior in runtime candidate ranking and persistence. Existing AI ranker hooks are explicitly not a substitute.

---

## Test And Quality Hardening

| Option | Description | Selected |
|--------|-------------|----------|
| Behavior-owned splits with full gates | Split oversized tests only along ownership boundaries and require focused tests, formatting, workspace tests, and clippy for closure. | ✓ |
| Broad mechanical test split | Move large tests wholesale before userdb behavior work. | |
| No split this phase | Leave large tests unchanged and focus only on userdb behavior. | |

**User's choice:** Auto-selected behavior-owned splits with full gates.
**Notes:** QUAL-03 and QUAL-04 make this part of Phase 5 scope, but prior decisions warn against mixing mechanical moves and semantic changes.

---

## Claude's Discretion

- Exact storage backend shape, fixture formats, module names, transaction-log format, and selected librime comparison schemas are left to research and planning.
- The planner may adjust roadmap slices if dependency order makes storage, learning, and quality hardening safer in a different sequence.

## Deferred Ideas

- AI-native memory, context providers, source-labeled AI candidates, and local/remote model policy remain a future milestone.
- Full librime C++ plugin ABI and Lua/octagram/predict/proto plugin ecosystems remain deferred.
- Real graphical frontend integration remains outside Phase 5 unless a focused userdb ABI fixture requires it.
