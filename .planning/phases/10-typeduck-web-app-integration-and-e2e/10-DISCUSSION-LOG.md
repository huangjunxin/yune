# Phase 10: TypeDuck-Web App Integration And E2E - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-05-05
**Phase:** 10-typeduck-web-app-integration-and-e2e
**Areas discussed:** Upstream TypeDuck-Web source handling, Yune bridge integration shape, browser E2E validation, findings and AI-native recommendation

---

## Upstream TypeDuck-Web Source Handling

| Option | Description | Selected |
|--------|-------------|----------|
| Reproducible local integration checkout | Clone/vendor/script upstream TypeDuck-Web in an auditable location and record the exact source revision before patching. | ✓ |
| Inline app source into core repo | Copy app code directly into Yune crates or runtime package before the seam is understood. | |
| Defer source inspection | Plan against assumptions about TypeDuck-Web’s bridge shape. | |

**User's choice:** Auto-selected recommended default.
**Notes:** The roadmap requires a reproducible TypeDuck-Web source location and current bridge seam identification before replacement work.

---

## Yune Bridge Integration Shape

| Option | Description | Selected |
|--------|-------------|----------|
| Use repository-owned TypeScript runtime package | Route TypeDuck-Web engine calls through `@yune-ime/typeduck-runtime` and Phase 9 filesystem helpers wherever possible. | ✓ |
| Use raw C/WASM exports directly from app code | Bypass wrapper ownership, key mapping, response parsing, and filesystem helpers. | |
| Expand native adapter first | Add new `yune_typeduck_*` exports before proving the real TypeDuck-Web seam requires them. | |

**User's choice:** Auto-selected recommended default.
**Notes:** Prior phases locked the wrapper, lifecycle, response ownership, and filesystem contract; Phase 10 should compose with those surfaces and widen them only for proven blockers.

---

## Browser E2E Validation

| Option | Description | Selected |
|--------|-------------|----------|
| Real TypeDuck-Web browser smoke/E2E flows | Exercise composition, candidate paging, selection, deletion, commit, deploy, customize, and persistence through the app seam. | ✓ |
| Package-local fake tests only | Continue testing the wrapper and fake FS without loading the real app. | |
| Visual-only screenshot validation | Validate appearance without proving engine and persistence behavior. | |

**User's choice:** Auto-selected recommended default.
**Notes:** Fake tests remain useful fallback diagnostics, but the Phase 10 requirement is real browser validation against TypeDuck-Web.

---

## Findings And AI-Native Recommendation

| Option | Description | Selected |
|--------|-------------|----------|
| Evidence-based go/no-go recommendation | Separate app blockers, Yune mismatches, and tooling blockers, then recommend GO, GO WITH CONDITIONS, or NO-GO for AI-native frontend exposure. | ✓ |
| Start AI-native features immediately | Add providers/ranking/context/memory before real frontend readiness is known. | |
| End with raw test output only | Omit an explicit recommendation for the next milestone. | |

**User's choice:** Auto-selected recommended default.
**Notes:** AI-native behavior remains deferred until TypeDuck-Web integration shows the frontend path is stable enough.

---

## Claude's Discretion

- Choose exact clone/vendor/patch layout after inspecting upstream TypeDuck-Web.
- Reuse TypeDuck-Web’s existing browser test runner if one exists; otherwise choose the smallest reliable E2E harness.
- Record fetch/build/tooling blockers reproducibly rather than silently skipping real browser validation.

## Deferred Ideas

- AI-native provider calls, candidate generation/ranking policy, context capture, memory, and privacy controls remain deferred.
- A new first-party Yune graphical frontend remains deferred.
- Multi-instance browser service isolation remains deferred unless TypeDuck-Web proves it is a blocker.
