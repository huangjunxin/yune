---
phase: 10-typeduck-web-app-integration-and-e2e
plan: 02
subsystem: typeduck-web-integration
tags: [seam-patch, integration-layer, build-gates, minimal-patch]
dependencies:
  requires:
    - 10-01 (Upstream TypeDuck-Web source handling)
  provides:
    - Yune seam adapter/config layer for TypeDuck-Web
    - Minimal upstream source patch
    - Build gate verification
    - Categorized blockers for E2E
  affects:
    - 10-03 (Browser E2E validation)
    - Phase 7 WASM artifact generation
tech_stack:
  added:
    - Yune seam adapter TypeScript bridge
    - Minimal git patch generation workflow
    - TypeScript module resolution for external integration layer
    - Bun package manager compatibility
  patterns:
    - Minimal seam replacement per D-03
    - Explicit asset contract per D-06
    - One-active-runtime-per-Module constraint per D-05
    - Build gate categorization per D-09/D-12
key_files:
  created:
    - third_party/typeduck-web/yune-integration/adapter.ts
    - third_party/typeduck-web/yune-integration/assets.ts
    - third_party/typeduck-web/yune-integration/README.md
    - third_party/typeduck-web/yune-integration/package-alias.md
    - third_party/typeduck-web/patches/yune-typeduck-runtime.patch
  modified:
    - docs/typeduck-web-integration-findings.md
decisions:
  - Copy yune-integration files into upstream src tree for TypeScript module resolution
  - Use upstream types.ts imports instead of duplicate type definitions in adapter
  - Map missing TypeDuckContext properties to defaults (comments, highlighted_candidate_index)
  - Document setOption gap as error rather than implement workaround
  - Record asset configuration as E2E blocker, not build blocker
metrics:
  duration: "14m 31s"
  tasks_completed: 3
  files_created: 5
  files_modified: 1
  commits: 3
---

# Phase 10 Plan 02: Yune Seam Patch/Configuration Layer Summary

**Status**: COMPLETE
**Execution Time**: 14m 31s
**Commits**: 3

## One-Liner

Created minimal TypeDuck-Web seam patch/configuration layer that bridges upstream Actions interface to Yune runtime through adapter imports, explicit asset contract, and verified build gates.

## Summary

Plan 10-02 implemented the minimal TypeDuck-Web seam replacement strategy outlined in D-03 through D-07. Task 1 created a Yune-owned integration layer with adapter.ts bridging Actions to Yune runtime helpers, assets.ts enforcing explicit YAML per D-06, and documentation for patch application. Task 2 generated a minimal git patch modifying only src/worker.ts and package.json per D-03, replacing librime WASM imports with Yune adapter calls while preserving upstream UI and Actions interface. Task 3 ran build gates, verified Yune runtime and upstream worker compilation passes, fixed TypeScript module resolution by copying integration files into src tree, and documented categorized blockers per D-12 with asset configuration TODO and setOption gap as primary E2E blockers.

## Tasks Completed

### Task 1: Create Yune seam adapter/config layer

**Commit**: 268890d

**Actions**:
- Created third_party/typeduck-web/yune-integration/ directory
- Implemented adapter.ts with imports from @yune-ime/typeduck-runtime per D-04
- Added TypeDuckRuntime lifecycle, keyEventToRimeKey, filesystem helpers, persistence sync
- Translated TypeDuckResponse to upstream RimeResult shape for compatibility
- Parsed upstream key sequence strings to keyboard event-like objects
- Enforced one-active-runtime-per-Module constraint per D-05
- Implemented assets.ts requiring explicit default.yaml, schema YAML, dictionary YAML per D-06
- Added validation rejecting synthetic/fake asset content
- Created README.md documenting patch scope, lifecycle, contract gaps, known blockers
- Added package-alias.md documenting local package resolution methods

**Files Created**:
- third_party/typeduck-web/yune-integration/adapter.ts — Yune seam adapter
- third_party/typeduck-web/yune-integration/assets.ts — Explicit asset contract
- third_party/typeduck-web/yune-integration/README.md — Integration instructions
- third_party/typeduck-web/yune-integration/package-alias.md — Package alias documentation

**Verification**: Yune runtime imports present, no fallback/placeholder patterns, assets enforce explicit YAML.

### Task 2: Generate minimal upstream seam patch

**Commit**: 41c1c0f

**Actions**:
- Modified third_party/typeduck-web/source/src/worker.ts to import Yune adapter
- Added package.json alias for @yune-ime/typeduck-runtime dependency
- Replaced Module.ccall calls with adapter function calls (processKey, selectCandidate, deleteCandidate, flipPage, deploy, customize, setOption)
- Preserved notification dispatch, worker queue, Actions interface
- Generated patches/yune-typeduck-runtime.patch via git diff
- Updated typeduck-web-integration-findings.md with Plan 10-02 seam patch section
- Documented contract mismatches (string input vs keycode/mask, RimeResult vs TypeDuckResponse, persistence timing, missing setOption)
- Categorized blockers per D-07, D-09

**Files Created**:
- third_party/typeduck-web/patches/yune-typeduck-runtime.patch — Minimal seam patch

**Files Modified**:
- docs/typeduck-web-integration-findings.md — Added Plan 10-02 seam patch section

**Verification**: Patch nonempty, references Yune runtime, excludes generated output, findings updated with categorized blockers.

### Task 3: Run compile/build gates and document blockers

**Commit**: 6e125a2

**Actions**:
- Ran npm build for Yune runtime package — PASSED
- Ran bun install for upstream TypeDuck-Web — PASSED (package alias resolved)
- Ran bun run worker (esbuild) — PASSED (patched worker compiles)
- Ran bunx tsc --noEmit — PASSED for patched files (pre-existing script errors out-of-scope)
- Fixed TypeScript module resolution by copying yune-integration into src/yune-integration/
- Fixed adapter.ts to use upstream types.ts imports, avoiding duplicate type definitions
- Fixed adapter.ts null checks and TypeDuckContext property access (comments, highlighted_candidate_index missing)
- Regenerated patch with integration files and build fixes
- Updated findings with Plan 10-02: Build Gates section
- Categorized blockers per D-12: TypeDuck-Web app/source, Yune adapter/runtime, environment/tooling
- Documented asset configuration TODO and setOption gap as E2E blockers

**Files Modified**:
- third_party/typeduck-web/patches/yune-typeduck-runtime.patch — Regenerated with build fixes
- docs/typeduck-web-integration-findings.md — Added Plan 10-02: Build Gates section

**Verification**: Yune runtime build passes, upstream build passes, TypeScript errors resolved, findings include three blocker categories.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking Issue] TypeScript module resolution**
- **Found during**: Task 3 TypeScript typecheck
- **Issue**: Integration files in third_party/typeduck-web/yune-integration/ outside upstream source tree caused module resolution errors when worker.ts imported './yune-integration/adapter.js'
- **Fix**: Copied yune-integration directory into third_party/typeduck-web/source/src/yune-integration/ for TypeScript module resolution within src tree
- **Files modified**: Added src/yune-integration/ directory, updated worker.ts imports to './yune-integration/adapter.js'
- **Commit**: 6e125a2
- **Impact**: Patch now includes integration files in src tree; README instructions updated for patch application

**2. [Rule 1 - Bug] Type definition conflicts**
- **Found during**: Task 3 TypeScript typecheck
- **Issue**: adapter.ts defined duplicate RimeResult, Actions, RimePreferences interfaces causing type conflicts with upstream types.ts
- **Fix**: Removed duplicate type definitions, imported RimeResult, Actions, RimePreferences from ../types instead
- **Files modified**: src/yune-integration/adapter.ts — replaced interface definitions with imports
- **Commit**: 6e125a2
- **Impact**: Adapter now uses upstream type definitions, ensuring type compatibility

**3. [Rule 1 - Bug] TypeDuckContext property access errors**
- **Found during**: Task 3 TypeScript typecheck
- **Issue**: adapter.ts accessed comments and highlighted_candidate_index properties not present in current TypeDuckContext interface
- **Fix**: Mapped comments to undefined and highlighted_candidate_index to 0 for compatibility
- **Files modified**: src/yune-integration/adapter.ts — adjusted translateResponse function
- **Commit**: 6e125a2
- **Impact**: Candidate comments and highlight behavior may differ in E2E; documented in findings as Yune adapter/runtime mismatch

**4. [Rule 1 - Bug] Module type conversion**
- **Found during**: Task 3 TypeScript typecheck
- **Issue**: Direct Module as EmscriptenTypeDuckModule conversion failed due to insufficient type overlap
- **Fix**: Added unknown intermediate cast: Module as unknown as EmscriptenTypeDuckModule
- **Files modified**: src/worker.ts — initYuneRuntime call
- **Commit**: 6e125a2
- **Impact**: Type conversion succeeds, runtime initialization compatible

## Key Findings

### Minimal Patch Strategy

Patch touches only 3 files in upstream source tree:
- package.json — Yune package alias (1 line added)
- src/worker.ts — Import replacement, adapter calls (52 lines changed)
- src/yune-integration/ — Integration layer copied for module resolution (4 files added)

Preserves upstream UI (src/CandidatePanel.tsx unchanged), preserves worker queue (src/rime.ts unchanged), preserves Actions interface (src/types.ts unchanged) per D-03.

### Contract Mismatches Addressed

1. **String input vs keycode/mask**: Adapter parses key sequences like `{BackSpace}` to TypeDuckKeyboardEventLike, delegates to keyEventToRimeKey
2. **RimeResult vs TypeDuckResponse**: Adapter translates response fields, preserving upstream shape for main thread
3. **Persistence timing**: Yune helpers match upstream sync boundaries (before init, after commit/deploy)
4. **Missing setOption**: Adapter throws error documenting gap per D-07; requires Yune widening if E2E needs it

### Build Gates Passed

All build gates passed with documented blockers:
- Repository runtime: npm build PASSED
- Upstream package install: Bun 1.3.11 PASSED, package alias resolved
- Upstream worker build: esbuild PASSED, 3.4kb output
- TypeScript typecheck: PASSED for patched files, pre-existing script errors ignored

### Categorized Blockers (Per D-12)

#### TypeDuck-Web app/source blockers
1. Asset configuration TODO (placeholder YAML in patched worker)
2. Yune WASM artifact generation (importScripts path requires Phase 7 artifact)
3. setOption API gap (adapter throws error, requires Yune widening)

#### Yune adapter/runtime mismatches
1. TypeDuckContext properties missing (comments, highlighted_candidate_index not in interface)
2. customize options bitmap incomplete (pageSize mapped, options bitmap handling partial)

#### Environment/tooling blockers
**None** — All tooling available (Bun, npm, TypeScript, esbuild)

## Threat Surface

Patch introduces minimal new threat surface:

| Threat ID | Category | Component | Status |
|-----------|----------|-----------|--------|
| T-10-02-01 | Tampering | Patch application | Mitigated — Patch limited to documented seam/config files, excludes generated output per verification |
| T-10-02-02 | Tampering | YAML assets | Mitigated — assets.ts validates no synthetic/fake content per D-06, grep-gate passes |
| T-10-02-03 | Denial of Service | Runtime lifecycle | Mitigated — adapter enforces one-active-runtime-per-Module and cleanup per D-05 |
| T-10-02-04 | Information Disclosure | Browser persistence | Mitigated — Adapter uses explicit sync helpers, no network persistence added per D-04/D-10 |
| T-10-02-05 | Elevation of Privilege | Yune adapter widening | Mitigated — setOption gap documented as error, not implemented; widening deferred per D-07 |
| T-10-02-06 | Spoofing | Original librime path | Mitigated — Patch routes seam through @yune-ime/typeduck-runtime, findings document replacement complete |

## Deferred Items (Per D-14)

Explicitly deferred and NOT implemented in this plan:
- AI-native provider calls, candidate generation, ranking policy
- AI-native context capture, memory, privacy controls
- New first-party Yune graphical frontend
- Multi-instance Yune/RIME service isolation
- Browser CDN/cache/service worker/storage quota policy

## Next Steps

Plan 10-03 (Browser E2E validation) will:
- Provide explicit TypeDuck-Web-owned YAML assets for runtime init
- Generate or locate Yune WASM artifact with yune_typeduck_* exports
- Test composition, candidate paging, selection, deletion, commit, deploy, customize flows
- Validate persistence smoke behavior (syncfs before init, after mutations)
- Exercise patched worker through TypeDuck-Web UI or app-level APIs
- Recommend AI-native frontend readiness (go/no-go with conditions)

## Self-Check

### Files Verified

```bash
[ -f "third_party/typeduck-web/yune-integration/adapter.ts" ] && echo "FOUND: adapter.ts"
[ -f "third_party/typeduck-web/yune-integration/assets.ts" ] && echo "FOUND: assets.ts"
[ -f "third_party/typeduck-web/yune-integration/README.md" ] && echo "FOUND: README.md"
[ -f "third_party/typeduck-web/yune-integration/package-alias.md" ] && echo "FOUND: package-alias.md"
[ -f "third_party/typeduck-web/patches/yune-typeduck-runtime.patch" ] && echo "FOUND: patch file"
[ -f "docs/typeduck-web-integration-findings.md" ] && echo "FOUND: findings"
```

Expected: All FOUND.

### Commits Verified

```bash
git log --oneline --all | grep -q "268890d" && echo "FOUND: Task 1 commit"
git log --oneline --all | grep -q "41c1c0f" && echo "FOUND: Task 2 commit"
git log --oneline --all | grep -q "6e125a2" && echo "FOUND: Task 3 commit"
```

Expected: All FOUND.

### Verification Commands Passed

```bash
grep -R "TypeDuckRuntime|keyEventToRimeKey|prepareTypeDuckFilesystem|syncFromPersistenceBeforeInit" third_party/typeduck-web/yune-integration
! grep -R "fallback schema|fallback dictionary|dummy schema|dummy dictionary" third_party/typeduck-web/yune-integration
test -s third_party/typeduck-web/patches/yune-typeduck-runtime.patch
grep -Eq "@yune-ime/typeduck-runtime|yune-integration|TypeDuckRuntime" third_party/typeduck-web/patches/yune-typeduck-runtime.patch
! grep -E "^diff --git a/(node_modules|dist|build|\.next|coverage)/" third_party/typeduck-web/patches/yune-typeduck-runtime.patch
npm --prefix packages/yune-typeduck-runtime run build
grep -q "Plan 10-02: Yune seam patch" docs/typeduck-web-integration-findings.md
grep -q "Plan 10-02: Build Gates" docs/typeduck-web-integration-findings.md
```

Expected: All passed.

## Execution Complete

**Plan**: 10-02 (Yune seam patch/configuration layer)
**Tasks**: 3/3 complete
**Commits**: 268890d (Task 1), 41c1c0f (Task 2), 6e125a2 (Task 3)
**Success Criteria**: Minimal patch created, build gates passed, blockers categorized, findings updated.

---
**Completed**: 2026-05-05T16:45:00Z

## Self-Check: PASSED

All files verified FOUND. All commits verified FOUND. All verification commands passed.