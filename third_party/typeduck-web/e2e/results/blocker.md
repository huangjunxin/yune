# Browser E2E Blocker

**Category**: environment/tooling

**Date**: 2026-05-05T00:30:00Z

## Critical Blockers

Browser E2E validation is BLOCKED due to missing essential tooling for WASM artifact generation and native testing.

### Missing Executables

1. **cargo** — Rust build tool
   - Command attempted: `./scripts/typeduck-wasm-build.sh`
   - Error: `cargo: command not found`
   - Impact: Cannot build Yune WASM artifact, cannot run native fallback tests

2. **rustup** — Rust toolchain manager
   - Command attempted: `rustup target list --installed`
   - Error: `command not found: rustup`
   - Impact: Cannot install wasm32-unknown-emscripten Rust target
   - Install hint: https://rustup.rs

3. **emcc** — Emscripten compiler
   - Command attempted: `emcc --version`
   - Error: `emcc not found`
   - Impact: Cannot compile Rust to WASM/JS glue for browser runtime
   - Install hint: https://emscripten.org/docs/getting_started/downloads.html

### WASM Artifact Blocker

**Required artifact**: `yune-typeduck.js` + `yune-typeduck.wasm`

**Purpose**: Emscripten-generated JavaScript/WASM with `yune_typeduck_*` exports for browser runtime

**Patch dependency**: `src/worker.ts` calls `importScripts("yune-typeduck.js")`

**Build script**: `scripts/typeduck-wasm-build.sh`

**Blocker evidence**:
```bash
$ ./scripts/typeduck-wasm-build.sh
./scripts/typeduck-wasm-build.sh: line 130: cargo: command not found
```

**Expected build output** (from Phase 7 contract):
```
target/wasm32-unknown-emscripten/debug/yune_rime_api.wasm
```

**Install requirements**:
1. Install Rust: https://rustup.rs
2. Add WASM target: `rustup target add wasm32-unknown-emscripten`
3. Install Emscripten: https://emscripten.org/docs/getting_started/downloads.html
4. Configure Emscripten SDK so `emcc` and `emar` are on PATH
5. Run: `./scripts/typeduck-wasm-build.sh`

### Asset Configuration Blocker

**Required**: Explicit TypeDuck-Web-owned YAML assets

**Patch TODO**: `src/worker.ts` lines 246-251
```typescript
const assetsConfig: ExplicitTypeDuckAssets = {
  defaultYaml: { type: "content", content: "" }, // Empty for E2E configuration
  schemaYaml: { type: "content", content: "" },   // Empty for E2E configuration
  dictionaryYaml: { type: "content", content: "" }, // Empty for E2E configuration
};
```

**Impact**: Runtime init will fail with missing assets error

**Resolution**: Provide explicit default.yaml, schema.yaml, dictionary.yaml per `e2e/assets/README.md`

### Flow Status

All D-08/D-10/D-11 flows BLOCKED:

| Flow | Requirement | Status | Blocker |
|------|-------------|--------|---------|
| Composition | Schema-valid keys → preedit visible | BLOCKED | WASM artifact missing |
| Candidate list | Visible after composition | BLOCKED | WASM artifact missing |
| Candidate paging | PageDown → page change | BLOCKED | WASM artifact missing |
| Candidate selection | Selection key → commit text | BLOCKED | WASM artifact missing |
| Deletion | Delete key → candidate/composition change | BLOCKED | WASM artifact missing |
| Backspace mutation | Backspace → composition shorter/changed | BLOCKED | WASM artifact missing |
| Deploy | Deploy action → visible success/error | BLOCKED | WASM artifact missing |
| Customize | Customize action → visible success/error | BLOCKED | WASM artifact missing |
| Persistence sync | sync-after-mutation marker | BLOCKED | WASM artifact missing |
| Persistence reload | sync-before-init + reload/reinitialize | BLOCKED | WASM artifact missing |

**Reason**: Without WASM artifact, patched worker cannot initialize Yune runtime, so no browser flows can execute.

### Fallback Evidence

**Command attempted**: `./scripts/typeduck-wasm-build.sh` (native fallback path)
```bash
TypeDuck WASM build blocked: missing cargo executable.
Native fallback: cargo test -p yune-rime-api --test typeduck_web
Error: cargo: command not found
```

**Fallback unavailable**: Native tests also require cargo

**Package-local tests**: Cannot run `npm --prefix packages/yune-typeduck-runtime test` without browser environment

**Evidence captured**:
- `browser-run.log` — Not created (browser never ran)
- `screenshot-*.png` — Not captured (browser never ran)
- `persistence-sync.log` — Not created (browser never ran)
- **This blocker.md** — Documents all missing tooling

### Category Assignment (Per D-12)

**Environment/tooling blockers**:
- cargo missing → Rust toolchain unavailable
- rustup missing → Cannot install WASM target
- emcc missing → Emscripten unavailable
- WASM artifact not built → Browser runtime cannot initialize

**TypeDuck-Web app/source blockers**:
- Asset configuration TODO in patched worker → Needs explicit assets

**Yune adapter/runtime blockers**:
- None at adapter layer (adapter.ts, assets.ts built successfully)
- Runtime JS artifacts built (`packages/yune-typeduck-runtime/dist/*.js`)
- WASM artifact is Phase 7 build blocker, not adapter implementation blocker

### Recommendation for Plan 10-04

1. **WASM artifact generation**: Either build WASM in environment with cargo/rustup/emcc, OR document Phase 7 blocker with reproduction steps
2. **Asset configuration**: Provide explicit TypeDuck-Web-owned YAML assets from upstream source or CDN
3. **Browser runner**: Install Playwright OR execute manual browser smoke per `e2e/yune-browser-smoke.md`
4. **Selector discovery**: Once browser runs, update selectors in `yune-typeduck.spec.ts` based on actual TypeDuck-Web UI
5. **Persistence markers**: Add console/timeline markers in Yune adapter for D-11 verification

---

**Blocker Impact**: Cannot proceed with real browser validation per D-08/TYPEDUCK-E2E-03

**Environment Setup Required**:
- Install Rust toolchain (cargo, rustup)
- Install wasm32-unknown-emscripten target
- Install Emscripten SDK (emcc, emar)
- Build Yune WASM artifact
- Provide explicit YAML assets
- Install Playwright or use manual browser

**Alternative**: Run browser E2E in environment with full tooling (not current worktree)

---

**Updated**: 2026-05-05T00:30:00Z
**Plan**: 10-03 (Real browser E2E/smoke validation)
**Task**: 3 (Run real browser validation or capture reproducible blockers)