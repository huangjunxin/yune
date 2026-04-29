# Phase 2: Native ABI Validation And Runtime Safety - Research

**Researched:** 2026-04-29 [VERIFIED: currentDate]
**Domain:** Rust C ABI/FFI validation, dynamic library loading, process-global runtime safety, and filesystem resource-ID validation [VERIFIED: .planning/ROADMAP.md]
**Confidence:** HIGH for repository state and Rust/Cargo mechanics; MEDIUM for exact future harness wiring because Cargo test artifact orchestration must be implemented and verified on each target platform [VERIFIED: cargo metadata; CITED: Rust Reference linkage docs]

<user_constraints>
## User Constraints (from CONTEXT.md)

### Context File Status

`.planning/phases/02-native-abi-validation-and-runtime-safety/02-CONTEXT.md` is present in the main workspace and is the canonical source for the locked Phase 2 decisions below. [VERIFIED: .planning/phases/02-native-abi-validation-and-runtime-safety/02-CONTEXT.md]

### Locked Decisions

- Minimum validation target is a Rust dynamic-loader harness. [CITED: user prompt]
- Harness must load a real dynamic artifact for `yune-rime-api`. [CITED: user prompt]
- Harness must resolve `rime_get_api` and exercise the `RimeApi` function table first. [CITED: user prompt]
- Loader failure on the current platform is a Phase 2 blocker unless an equivalent platform-specific replacement validation path is documented. [CITED: user prompt]
- Observed ABI/frontend gaps should become focused failing regression tests first, then fixes when in Phase 2 scope. [CITED: user prompt]
- In-scope fixes are ABI/runtime safety only: layout, lifetime, loading, notification, deployment, session lifecycle, process-global determinism, and resource-ID safety. [CITED: user prompt]
- Out-of-scope schema semantics, compiled dictionary payloads, and userdb compatibility should be recorded as structured findings. [CITED: user prompt]
- Lifecycle stress should cover repeated setup/initialize/finalize, session create/destroy/cleanup-all, schema switching, deployment, and notification registration. [CITED: user prompt]
- Use small deterministic loop counts suitable for normal `cargo test`. [CITED: user prompt]
- Multi-threading is document-only unless the loader exposes a concrete issue. [CITED: user prompt]
- Notification validation should assert deterministic callback order around exercised deployment/schema lifecycle paths. [CITED: user prompt]

### Claude's Discretion

Phase 2 CONTEXT.md leaves exact dynamic-loader organization, helper naming, loop counts, structured finding file format, and resource-ID helper organization to planning/execution, provided the locked decisions remain true. [VERIFIED: .planning/phases/02-native-abi-validation-and-runtime-safety/02-CONTEXT.md]

### Deferred Ideas (OUT OF SCOPE)

- Product GUI frontend proof is not required by the Phase 2 minimum target because the user locked the minimum target to a Rust dynamic-loader harness. [CITED: user prompt]
- Schema semantics expansion, compiled dictionary payloads, and userdb compatibility fixes are out of scope except as structured findings. [CITED: user prompt]
- Multi-threading fixes are out of scope unless the loader exposes a concrete issue. [CITED: user prompt]
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| ABI-01 | Developer can run the current ABI against at least one real frontend client or native frontend-like loading path and record observed gaps. [VERIFIED: .planning/REQUIREMENTS.md] | Add a Rust `libloading` harness that loads a real `cdylib` artifact, resolves `rime_get_api`, and drives `RimeApi` first. [CITED: docs.rs/libloading; CITED: Rust Reference linkage docs] |
| ABI-02 | Struct layout, lifetime, notification, deployment, and session gaps found by frontend validation have focused regression coverage. [VERIFIED: .planning/REQUIREMENTS.md] | Keep the existing direct `frontend_client` integration test as baseline coverage, then add focused loader/lifecycle regressions for any observed ABI gaps. [VERIFIED: cargo test --test frontend_client -- --list; VERIFIED: codebase read] |
| ABI-03 | Runtime resource IDs from C APIs and schema YAML reject traversal, absolute paths, platform separators, and other filesystem syntax before joins. [VERIFIED: .planning/REQUIREMENTS.md] | Centralize logical resource-ID validation before config, deployment, dictionary, custom-settings, and userdb path joins; distinguish logical IDs from intentionally path-valued import/export arguments. [VERIFIED: codebase read; CITED: OWASP Path Traversal] |
| ABI-04 | Process-wide session, module, notification, switcher, and runtime behavior remains deterministic under repeated initialize/finalize and session lifecycle operations. [VERIFIED: .planning/REQUIREMENTS.md] | Add small-loop deterministic lifecycle tests using existing global test guard patterns and dynamic-loader API-table calls. [VERIFIED: codebase read; CITED: user prompt] |
</phase_requirements>

## Summary

Phase 2 should be planned as an ABI/runtime safety phase with three required work streams: produce/load a real `yune-rime-api` dynamic artifact, convert native-loader observations into regression tests and fixes, and close runtime resource-ID traversal gaps before filesystem joins. [VERIFIED: .planning/ROADMAP.md; CITED: user prompt] The current repository already has a direct Rust frontend-style integration test that exercises `rime_get_api` and the `RimeApi` table, but Cargo metadata shows `yune-rime-api` currently builds only a Rust library target, not a `cdylib`, so it does not yet satisfy the locked dynamic-loader requirement. [VERIFIED: cargo metadata; VERIFIED: cargo test --test frontend_client -- --list]

The standard Phase 2 stack should stay small: Cargo/Rust test harnesses, `libloading` for cross-platform dynamic loading, optional `tempfile` for runtime fixture directories, and platform tools (`file`, `nm`, `otool` on macOS) for artifact diagnostics. [VERIFIED: crates.io API; VERIFIED: environment probe; CITED: docs.rs/libloading] Avoid building a custom dynamic loader, custom temporary-directory cleanup, or ad hoc path sanitization; use `libloading`, RAII temporary directories, and a centralized allowlist validator instead. [CITED: docs.rs/libloading; CITED: tempfile docs; CITED: OWASP Path Traversal]

**Primary recommendation:** Add `crate-type = ["lib", "cdylib"]` for `yune-rime-api`, then add a `libloading`-based integration harness that loads the produced artifact, resolves `rime_get_api`, runs deterministic API-table lifecycle/deployment/schema/notification flows, and records any non-scope frontend gaps as structured findings. [CITED: Cargo Book crate-type docs; CITED: Rust Reference linkage docs; CITED: docs.rs/libloading; CITED: user prompt]

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Dynamic artifact production | Build / Package | ABI crate | Cargo owns output artifact type; `yune-rime-api` owns exported symbols. [CITED: Cargo Book crate-type docs; VERIFIED: codebase read] |
| Dynamic-loader validation | Test harness | ABI crate | The harness should load a real shared object and call the ABI as an external frontend-like client would. [CITED: docs.rs/libloading; CITED: user prompt] |
| `RimeApi` table validation | ABI crate | Test harness | `api_table.rs` owns `rime_get_api` and table construction; the harness verifies exported access and callability. [VERIFIED: codebase read] |
| Session/runtime lifecycle determinism | ABI crate | Tests | `yune-rime-api` owns process globals for sessions, service state, modules, notifications, and runtime paths; tests should verify deterministic behavior. [VERIFIED: .planning/codebase/ARCHITECTURE.md; VERIFIED: codebase read] |
| Notification order validation | ABI crate | Test harness | Notification registration and dispatch are ABI-layer responsibilities; tests should assert deployment/schema lifecycle callback order. [VERIFIED: codebase read; CITED: user prompt] |
| Resource-ID validation | ABI crate | Schema/runtime loaders | C ABI entry points and schema YAML loaders must reject path-like logical IDs before joining runtime roots. [VERIFIED: .planning/codebase/CONCERNS.md; CITED: OWASP Path Traversal] |
| Out-of-scope compatibility findings | Planning docs / fixtures | Tests | Userdb compatibility, compiled dictionary payloads, and schema semantics should be recorded as findings unless they expose ABI/runtime safety issues. [CITED: user prompt] |

## Project Constraints (from CLAUDE.md)

No `./CLAUDE.md` file was present in the working directory during research, so there are no additional CLAUDE.md directives to apply. [VERIFIED: Bash test]

## Standard Stack

### Core

| Library / Tool | Version | Purpose | Why Standard |
|----------------|---------|---------|--------------|
| Rust / Cargo | rustc 1.95.0, cargo 1.95.0 in environment; workspace `rust-version` is 1.76. [VERIFIED: environment probe; VERIFIED: Cargo.toml read] | Build and test the Rust workspace. | Existing project is a Cargo workspace with Rust crates `yune-core`, `yune-schema`, `yune-rime-api`, and `yune-cli`. [VERIFIED: Cargo.toml read] |
| `libloading` | 0.9.0, published/updated 2025-11-05. [VERIFIED: crates.io API] | Cross-platform dynamic library loading and symbol lookup. | Official docs show `Library::new` loads a shared library and `Library::get` retrieves typed symbols; symbol use is `unsafe`, matching FFI harness needs. [CITED: docs.rs/libloading/0.9.0] |
| Cargo `[lib] crate-type` | Cargo manifest feature. [CITED: Cargo Book] | Configure `yune-rime-api` to emit a dynamic artifact. | Cargo docs show `[lib] crate-type = ["cdylib"]`; Rust Reference says `cdylib` creates dynamic system libraries suitable for loading from other languages. [CITED: Cargo Book; CITED: Rust Reference linkage docs] |
| Rust `#[repr(C)]`/`extern "C"` ABI patterns | Stable Rust FFI pattern. [CITED: Rustonomicon FFI] | Keep C struct layout and callbacks compatible with librime-style frontends. | Rust FFI docs require `#[repr(C)]` for C-shared structs and correct extern calling conventions for callbacks/functions. [CITED: Rustonomicon FFI] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|----------------|---------|---------|-------------|
| `tempfile` | 3.27.0, updated 2026-03-11. [VERIFIED: crates.io API] | RAII temporary runtime directories for loader and resource-ID tests. | Use when tests need isolated shared/user/prebuilt/staging/sync dirs and automatic cleanup. [CITED: tempfile docs] |
| `object` | 0.39.1, updated 2026-04-21. [VERIFIED: crates.io API] | Optional cross-platform object-file/export inspection. | Use only if platform tools are insufficient or the harness needs portable symbol/export diagnostics. [VERIFIED: crates.io API; ASSUMED] |
| `libc` | 0.2.186, updated 2026-04-23; already locked in the workspace. [VERIFIED: crates.io API; VERIFIED: Cargo.lock read] | C primitive types and FFI bindings. | Keep using existing ABI type dependency where platform C types are needed. [VERIFIED: crates.io API; VERIFIED: Cargo.lock read] |
| `regex` | 1.12.3, updated 2026-02-03; already locked in the workspace. [VERIFIED: crates.io API; VERIFIED: Cargo.lock read] | Existing parsing/validation support. | Use only if existing code paths already rely on it; a simple resource-ID allowlist can be implemented without regex if clearer. [VERIFIED: Cargo.lock read; ASSUMED] |
| `serde_yaml` | 0.9.34+deprecated, updated 2024-03-25; already locked in the workspace. [VERIFIED: crates.io API; VERIFIED: Cargo.lock read] | Existing YAML config/schema parsing. | Do not expand Phase 2 around YAML parser changes; parser replacement is not required for ABI/runtime safety scope. [VERIFIED: Cargo.lock read; CITED: user prompt] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `libloading` | Manual `dlopen`/`dlsym`/`LoadLibrary` bindings | More platform branching and unsafe boilerplate; `libloading` already handles cross-platform library/symbol APIs. [CITED: docs.rs/libloading; ASSUMED] |
| `tempfile::TempDir` | Existing manual `unique_temp_dir` helper | Existing helper is dependency-free, but RAII cleanup reduces leftover test directories if assertions panic. [VERIFIED: codebase read; CITED: tempfile docs] |
| `object` crate | `nm`, `otool`, `file`, `lipo` | Platform tools are available on this macOS environment and sufficient for diagnostics; `object` is useful only if portable export assertions are needed. [VERIFIED: environment probe; VERIFIED: crates.io API] |
| `serial_test` | Existing `test_guard()` with `Mutex`/`OnceLock` | Existing tests already serialize process-global ABI state without another dependency; `serial_test` may help cross-module annotations but should not be required unless races appear. [VERIFIED: codebase read; ASSUMED] |

**Installation / manifest changes:**

```toml
# crates/yune-rime-api/Cargo.toml
[lib]
crate-type = ["lib", "cdylib"]

[dev-dependencies]
libloading = "0.9"
tempfile = "3.27"
```

The `lib` crate type keeps existing Rust integration tests importable, while `cdylib` emits a dynamic system library for the locked native-loader harness. [CITED: Rust Reference linkage docs; VERIFIED: cargo metadata]

**Version verification:** Recommended crate versions were verified against the crates.io API during research rather than inferred from training data. [VERIFIED: crates.io API]

| Package | Verified Version | Publish/Update Date |
|---------|------------------|---------------------|
| `libloading` | 0.9.0 | 2025-11-05T23:15:12.937093Z [VERIFIED: crates.io API] |
| `tempfile` | 3.27.0 | 2026-03-11T00:20:04.812840Z [VERIFIED: crates.io API] |
| `object` | 0.39.1 | 2026-04-21T08:41:10.871144Z [VERIFIED: crates.io API] |
| `libc` | 0.2.186 | 2026-04-23T20:00:00.943607Z [VERIFIED: crates.io API] |
| `regex` | 1.12.3 | 2026-02-03T13:48:35.405091Z [VERIFIED: crates.io API] |
| `serde_yaml` | 0.9.34+deprecated | 2024-03-25T00:50:19.759577Z [VERIFIED: crates.io API] |

## Architecture Patterns

### System Architecture Diagram

```text
Cargo build/test invocation
  |
  v
Build yune-rime-api as Rust lib + cdylib
  |                         |
  |                         v
  |                 Platform dynamic artifact
  |                 macOS: libyune_rime_api.dylib
  |                 Linux: libyune_rime_api.so
  |                 Windows: yune_rime_api.dll
  |                 [CITED: Rust Reference linkage docs]
  |
  v
Rust dynamic-loader integration harness
  |
  v
libloading::Library::new(artifact_path)
  |
  v
Library::get(b"rime_get_api\0")
  |
  v
unsafe extern "C" fn() -> *mut RimeApi
  |
  v
RimeApi table validation
  |-- data_size/layout checks
  |-- required function pointers present
  |-- setup/initialize/finalize lifecycle
  |-- notification callback registration
  |-- deployment/schema/session flows
  |
  v
Observed gap?
  |-- yes --> focused failing regression test --> ABI/runtime safety fix if in scope
  |-- no  --> record validation fixture/notes and keep regression test green

Resource-ID validation path:
C API input or schema YAML reference
  |
  v
C string/YAML scalar extraction
  |
  v
central logical resource-ID validator
  |-- reject empty, '.', '..', separators, absolute paths, drive prefixes, NUL-derived/path syntax
  |-- allow only expected logical identifier characters/classes
  |
  v
runtime root join with generated suffix only
  |
  v
config/schema/dictionary/custom-settings/userdb file operation
```

The diagram separates dynamic artifact validation from resource-ID validation because the first proves frontend-like ABI loading while the second closes runtime filesystem safety gaps discovered in existing path joins. [VERIFIED: .planning/ROADMAP.md; VERIFIED: codebase read]

### Recommended Project Structure

```text
crates/yune-rime-api/
├── Cargo.toml                  # add [lib] crate-type = ["lib", "cdylib"] and dev-deps [CITED: Cargo Book]
├── src/
│   ├── resource_id.rs           # centralized logical resource-ID validation [VERIFIED: codebase read; CITED: OWASP]
│   ├── lib.rs                   # facade exports module; keep path joins behind validated helpers [VERIFIED: codebase read]
│   ├── api_table.rs             # owns rime_get_api/RimeApi table [VERIFIED: codebase read]
│   ├── deployment.rs            # deployment lifecycle and deploy notifications [VERIFIED: codebase read]
│   ├── notifications.rs         # handler registration/dispatch [VERIFIED: codebase read]
│   ├── session.rs               # session registry and service lifecycle [VERIFIED: codebase read]
│   └── tests/                   # focused unit/regression coverage for ABI globals [VERIFIED: codebase read]
└── tests/
    ├── frontend_client.rs        # existing direct linked frontend-style API-table tests [VERIFIED: cargo test -- --list]
    └── dynamic_loader.rs         # new real-artifact loader harness [CITED: user prompt; CITED: docs.rs/libloading]
```

### Pattern 1: Real Dynamic Artifact Loader Harness

**What:** Build or locate the `yune-rime-api` `cdylib`, load it with `libloading::Library::new`, resolve `rime_get_api`, cast it to `unsafe extern "C" fn() -> *mut RimeApi`, and drive all validation through returned function pointers first. [CITED: docs.rs/libloading; CITED: user prompt]

**When to use:** Use for ABI-01 and any regression where frontend-like dynamic loading can expose symbol export, function table, layout, lifetime, or callback issues that direct Rust linking might hide. [CITED: user prompt; ASSUMED]

**Example:**

```rust
// Source: https://docs.rs/libloading/0.9.0/libloading/index.html
// Adapted for yune-rime-api's existing rime_get_api symbol. [VERIFIED: codebase read]
type RimeGetApi = unsafe extern "C" fn() -> *mut yune_rime_api::RimeApi;

unsafe {
    let lib = libloading::Library::new(artifact_path)?;
    let get_api: libloading::Symbol<RimeGetApi> = lib.get(b"rime_get_api\0")?;
    let api = get_api();
    assert!(!api.is_null());
    let api = &mut *api;
    assert!(api.data_size >= 0);
}
```

### Pattern 2: Versioned Struct Layout Checks

**What:** Validate `data_size` conventions for FFI structs and API tables using librime's convention that struct data size excludes the `data_size` field. [VERIFIED: /Users/trenton/Projects/librime/src/rime_api.h; VERIFIED: codebase read]

**When to use:** Use whenever the harness passes or receives `RimeTraits`, `RimeCommit`, `RimeContext`, `RimeStatus`, `RimeApi`, or levers structs across FFI. [VERIFIED: codebase read]

**Example:**

```rust
// Source: /Users/trenton/Projects/librime/src/rime_api.h RIME_STRUCT_INIT macro. [VERIFIED: local librime oracle]
let expected = (std::mem::size_of::<RimeApi>() - std::mem::size_of::<std::os::raw::c_int>()) as i32;
assert_eq!(unsafe { (*api).data_size }, expected);
```

### Pattern 3: Boundary-First Logical Resource-ID Validation

**What:** Validate C API and YAML-derived logical IDs before any path join, then append fixed suffixes like `.yaml`, `.schema.yaml`, `.dict.yaml`, `.userdb`, or `.custom.yaml` after validation. [VERIFIED: codebase read; CITED: OWASP Path Traversal]

**When to use:** Use for config IDs, schema IDs, deployment file IDs, dictionary names/imports/packs/vocabulary names, levers custom config IDs, and user dictionary names. [VERIFIED: .planning/codebase/CONCERNS.md; VERIFIED: codebase read]

**Example:**

```rust
// Source: OWASP recommends known-good allowlists for path traversal prevention.
// Phase-specific shape; planner should refine exact allowed chars with librime compatibility checks. [CITED: OWASP Path Traversal; ASSUMED]
pub(crate) fn validate_logical_resource_id(value: &str) -> bool {
    !value.is_empty()
        && value != "."
        && value != ".."
        && !value.contains('/')
        && !value.contains('\\')
        && !value.contains('\0')
        && !std::path::Path::new(value).is_absolute()
        && value.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-' | b'.'))
}
```

### Pattern 4: Existing Process-Global Test Guard

**What:** Use a single process-wide mutex guard for tests that mutate `RimeInitialize`, `RimeFinalize`, session registry, notification handler, runtime paths, modules, or levers state. [VERIFIED: codebase read]

**When to use:** Use for ABI-04 deterministic lifecycle and notification tests because existing `yune-rime-api` tests already use `Mutex`/`OnceLock` guards around process-global ABI state. [VERIFIED: codebase read]

**Example:**

```rust
// Source: crates/yune-rime-api/src/tests/mod.rs existing pattern. [VERIFIED: codebase read]
fn test_guard() -> MutexGuard<'static, ()> {
    static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}
```

### Anti-Patterns to Avoid

- **Direct-linked frontend test as the only ABI proof:** `frontend_client.rs` exercises `rime_get_api`, but it is linked through Rust/Cargo rather than loading a dynamic artifact, so it does not satisfy the locked Phase 2 target by itself. [VERIFIED: cargo test --test frontend_client -- --list; CITED: user prompt]
- **Changing schema semantics to make loader tests pass:** Schema semantics, compiled dictionary payloads, and userdb compatibility are explicitly out of scope unless the issue is ABI/runtime safety. [CITED: user prompt]
- **String replacement path sanitization:** Reject invalid logical IDs at boundaries with allowlists; do not try to strip `../` or normalize attacker-controlled paths into safety. [CITED: OWASP Path Traversal]
- **Holding notification locks while invoking callbacks:** Existing notification code drops the lock before invoking the callback; keep that property to avoid reentrancy deadlocks. [VERIFIED: codebase read; ASSUMED]
- **Large stress loops in unit tests:** User locked small deterministic loop counts suitable for normal `cargo test`; do not introduce long fuzz/stress tests in Phase 2. [CITED: user prompt]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Cross-platform dynamic loading | Custom `dlopen`/`dlsym`/`LoadLibrary` abstraction | `libloading` | Official docs provide `Library::new` and typed `Library::get`; symbols are tied to library lifetime. [CITED: docs.rs/libloading] |
| Dynamic artifact type | Ad hoc rustc flags hidden in scripts | Cargo `[lib] crate-type = ["lib", "cdylib"]` | Cargo and Rust Reference document crate-type outputs and platform extensions. [CITED: Cargo Book; CITED: Rust Reference linkage docs] |
| Temporary runtime fixture cleanup | Manual temp path deletion in every test | `tempfile::TempDir` or existing helper plus guaranteed cleanup | `TempDir` auto-deletes unless kept; this reduces leaked test dirs after panics. [CITED: tempfile docs] |
| Path traversal prevention | String stripping or canonicalization after joining | Central logical-ID allowlist before joining | OWASP recommends known-good validation and avoiding user-controlled filesystem names. [CITED: OWASP Path Traversal] |
| FFI memory ownership conventions | Rust references or borrowed strings exposed across C boundaries without lifetime control | `#[repr(C)]`, raw pointers, `CString`, explicit free APIs | Rust FFI guidance requires C layout, correct calling conventions, and explicit ownership/lifetime management. [CITED: Rustonomicon FFI] |
| Process-global test ordering | Assuming Cargo test order or thread scheduling | Existing `test_guard()` / explicit serialized harness | Existing ABI tests serialize process-global runtime mutations with a mutex guard. [VERIFIED: codebase read] |

**Key insight:** Phase 2 should validate ABI behavior from the outside in; if the harness cheats by linking directly, hand-building a loader, or sanitizing paths after joins, it will miss the same classes of native frontend and traversal failures the phase is meant to expose. [CITED: user prompt; CITED: OWASP Path Traversal; ASSUMED]

## Common Pitfalls

### Pitfall 1: Cargo test does not prove a dynamic artifact exists

**What goes wrong:** A Rust integration test can call `rime_get_api` through a normal Rust dependency even when no shared library artifact exists. [VERIFIED: cargo metadata; VERIFIED: cargo test --test frontend_client -- --list]

**Why it happens:** Cargo metadata currently reports `yune-rime-api` target kind `['lib']` and crate type `['lib']`; no `cdylib` target is configured. [VERIFIED: cargo metadata]

**How to avoid:** Add `[lib] crate-type = ["lib", "cdylib"]`, then make the new harness locate and load the platform artifact path. [CITED: Cargo Book; CITED: Rust Reference linkage docs]

**Warning signs:** Tests pass without any `target/debug/libyune_rime_api.dylib`, `target/debug/libyune_rime_api.so`, or `target/debug/yune_rime_api.dll` being produced. [CITED: Rust Reference linkage docs; ASSUMED]

### Pitfall 2: `libloading::Symbol` used after `Library` drop

**What goes wrong:** Function pointers or symbols can outlive the loaded library and become invalid. [CITED: docs.rs/libloading]

**Why it happens:** Dynamic library symbol lifetimes are tied to the loaded `Library`; docs model symbol retrieval through `lib.get` on a live library. [CITED: docs.rs/libloading]

**How to avoid:** Keep the `Library` object alive for the full test, and do not store raw function pointers beyond that lifetime unless the owning library also stays alive. [CITED: docs.rs/libloading; ASSUMED]

**Warning signs:** Harness extracts raw pointers into globals or returns `RimeApi` references from a helper after dropping `Library`. [ASSUMED]

### Pitfall 3: Over-validating intentionally path-valued API parameters

**What goes wrong:** The planner may reject legitimate export/import text paths or snapshot file paths while trying to validate logical resource IDs. [VERIFIED: codebase read]

**Why it happens:** `RimeLeversExportUserDict` and `RimeLeversImportUserDict` take both a logical dictionary name and a text file path, while `RimeLeversRestoreUserDict` takes a snapshot file path from which a dictionary name is derived. [VERIFIED: codebase read]

**How to avoid:** Validate `dict_name` as a logical ID, but treat `text_file` and `snapshot_file` as path-valued parameters with separate safety requirements. [VERIFIED: codebase read; CITED: OWASP Path Traversal]

**Warning signs:** Tests expect `RimeLeversImportUserDict("luna", "/tmp/luna.txt")` to fail solely because the second argument is an absolute path. [VERIFIED: codebase read; ASSUMED]

### Pitfall 4: Validating config IDs in one API but not lower-level joins

**What goes wrong:** A boundary validation fix can leave schema YAML dictionary imports, deployment filenames, or custom settings paths vulnerable because they join paths through different helper functions. [VERIFIED: codebase read; VERIFIED: .planning/codebase/CONCERNS.md]

**Why it happens:** Existing resource joins are spread across `lib.rs`, `deployment.rs`, `schema_install.rs`, `levers.rs`, and `userdb.rs`. [VERIFIED: codebase read]

**How to avoid:** Add a single `resource_id` module and call it at every boundary and before every resource-root join. [VERIFIED: codebase read; ASSUMED]

**Warning signs:** Tests cover `RimeConfigOpen("../x")` but not dictionary imports, custom settings, deploy config files, or userdb names. [VERIFIED: codebase read; ASSUMED]

### Pitfall 5: Callback assertions miss deterministic order

**What goes wrong:** Tests only assert that notifications occurred, not that they occurred in frontend-compatible order. [CITED: user prompt]

**Why it happens:** Deployment currently emits `deploy/start` followed by `deploy/success` or `deploy/failure`; schema selection emits schema notifications separately, so order matters to frontend clients. [VERIFIED: codebase read; VERIFIED: local librime oracle]

**How to avoid:** Capture `(context_object, session_id, message_type, message_value)` in a vector and compare exact ordered sequences for each lifecycle path. [VERIFIED: codebase read; CITED: user prompt]

**Warning signs:** Assertions use unordered containment checks for notification events. [ASSUMED]

### Pitfall 6: Panics crossing FFI boundaries

**What goes wrong:** A panic in exported `extern "C"` code can abort or cause undefined behavior depending on ABI/unwind configuration. [CITED: Rustonomicon FFI]

**Why it happens:** Rust FFI docs state foreign functions are unsafe because Rust cannot verify ABI correctness, pointer validity, thread safety, or unwinding behavior; panics must not cross non-unwinding FFI boundaries. [CITED: Rustonomicon FFI]

**How to avoid:** Prefer returning false/null/error codes at exported boundaries and add regression tests for invalid pointers/IDs where feasible. [CITED: Rustonomicon FFI; VERIFIED: existing tests read]

**Warning signs:** New `extern "C"` functions call `.unwrap()` or `.expect()` on user-controlled inputs. [ASSUMED]

## Code Examples

Verified patterns from official or repository sources:

### Dynamic symbol resolution

```rust
// Source: https://docs.rs/libloading/0.9.0/libloading/index.html
unsafe {
    let lib = libloading::Library::new("/path/to/liblibrary.so")?;
    let func: libloading::Symbol<unsafe extern "C" fn() -> u32> = lib.get(b"my_func")?;
    let value = func();
}
```

This should be adapted to `rime_get_api` and the `RimeApi` struct shape already defined by `yune-rime-api`. [CITED: docs.rs/libloading; VERIFIED: codebase read]

### Cargo dynamic artifact configuration

```toml
# Source: https://doc.rust-lang.org/cargo/reference/cargo-targets.html
[lib]
crate-type = ["cdylib"]
bench = false
```

For this project, use `crate-type = ["lib", "cdylib"]` rather than only `cdylib` so existing Rust integration tests can still import `yune_rime_api`. [CITED: Cargo Book; VERIFIED: cargo metadata; ASSUMED]

### Librime-style `data_size` convention

```c
// Source: /Users/trenton/Projects/librime/src/rime_api.h
#define RIME_STRUCT_INIT(Type, var) \
  ((var).data_size = sizeof(Type) - sizeof((var).data_size))
```

The Rust ABI tests should keep using this convention for `RimeTraits`, `RimeApi`, `RimeCommit`, `RimeContext`, and `RimeStatus`. [VERIFIED: local librime oracle; VERIFIED: codebase read]

### Resource-ID allowlist gate before path join

```rust
// Source: OWASP Path Traversal prevention guidance: validate known-good input. [CITED: OWASP]
fn checked_resource_path(root: &Path, id: &str, suffix: &str) -> Option<PathBuf> {
    validate_logical_resource_id(id).then(|| root.join(format!("{id}{suffix}")))
}
```

The exact allowed character set should be decided against existing fixture IDs and librime compatibility expectations before implementation. [VERIFIED: codebase read; ASSUMED]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Direct Rust-linked frontend surrogate only | Real dynamic-loader harness that loads a `cdylib` and resolves exported symbols | Phase 2 locked decision, 2026-04-29 [CITED: user prompt] | Planner must include build/artifact work before loader validation. [CITED: user prompt; VERIFIED: cargo metadata] |
| Custom platform loader calls | `libloading` crate | `libloading` 0.9.0 current as of 2025-11-05 update [VERIFIED: crates.io API] | Reduces platform-specific unsafe code in tests. [CITED: docs.rs/libloading; ASSUMED] |
| Path-like resource strings joined directly under runtime roots | Logical resource-ID allowlist before every join | Phase 2 requirement ABI-03 [VERIFIED: .planning/REQUIREMENTS.md] | Prevents traversal/absolute/separator-based escape from runtime roots. [CITED: OWASP Path Traversal] |
| Large stress or manual frontend validation | Small deterministic `cargo test` lifecycle loops plus structured findings | Phase 2 locked decision [CITED: user prompt] | Keeps validation runnable in normal developer workflow. [CITED: user prompt] |

**Deprecated/outdated:**

- Treating `yune-cli frontend` as native frontend proof is out of date for Phase 2; Phase 1 verification confirms it is an intermediate surrogate, while Phase 2 requires native/frontend-like dynamic loading. [VERIFIED: Phase 1 verification docs; CITED: user prompt]
- Expanding `serde_yaml` usage is not a Phase 2 goal; the crate is current in the lockfile as `0.9.34+deprecated`, and parser replacement is outside the locked ABI/runtime safety scope. [VERIFIED: crates.io API; VERIFIED: Cargo.lock read; CITED: user prompt]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `object` is only optional unless portable export assertions are required. | Standard Stack | Planner may omit a dependency that would have simplified cross-platform diagnostics. |
| A2 | A simple non-regex allowlist may be clearer than using existing `regex`. | Standard Stack | Implementation may choose regex and still be correct if tests cover edge cases. |
| A3 | Manual platform loader alternatives add unsafe boilerplate compared with `libloading`. | Alternatives / Don't Hand-Roll | If `libloading` fails on the target, planner needs platform-specific fallback work. |
| A4 | The new `resource_id` module is the best ownership boundary. | Architecture Patterns / Pitfalls | If maintainers prefer colocated validation, tasks may need refactoring but validation requirements remain. |
| A5 | Notification lock preservation prevents reentrancy deadlocks. | Anti-Patterns | If callback threading semantics differ from assumptions, additional lifecycle tests may be needed. |
| A6 | Dynamic loader tests may reveal symbol/lifetime issues that direct linking hides. | Architecture Patterns | If no such issues exist, the harness is still required by locked decisions but finds fewer gaps. |
| A7 | `crate-type = ["lib", "cdylib"]` is the best manifest shape for this crate. | Code Examples | If Cargo/build behavior conflicts with integration tests, planner must adjust artifact build orchestration. |
| A8 | Exact allowed resource-ID character set should be refined against fixtures and librime compatibility. | Code Examples | Too-strict validation can reject legitimate schema/dictionary IDs; too-loose validation can miss traversal syntax. |

## Open Questions

1. **How should the dynamic-loader integration test force or locate the `cdylib` artifact in all supported Cargo invocations?** [CITED: user prompt; CITED: Cargo Book]
   - What we know: Cargo can emit `cdylib` artifacts via `[lib] crate-type`, and Rust Reference defines platform extensions. [CITED: Cargo Book; CITED: Rust Reference linkage docs]
   - What's unclear: Whether the desired `cargo test -p yune-rime-api --test dynamic_loader` command will always build the `cdylib` before running the test on every platform. [ASSUMED]
   - Recommendation: Plan Wave 0 to add crate type and a loader test that first checks the expected artifact path; if missing, add a documented build step or test harness wrapper. [ASSUMED]

2. **What exact character set should logical resource IDs allow?** [VERIFIED: .planning/codebase/CONCERNS.md]
   - What we know: Phase 2 must reject traversal, absolute paths, separators, and filesystem syntax. [VERIFIED: .planning/REQUIREMENTS.md]
   - What's unclear: Whether existing legitimate RIME resource IDs require dots beyond normalized suffix forms like `.schema`/`.custom`, or other punctuation. [ASSUMED]
   - Recommendation: Start with tests for known project fixtures, allow ASCII alnum plus `_`, `-`, and controlled `.`, and explicitly reject `.`, `..`, `/`, `\`, NUL, absolute paths, and drive-like prefixes. [CITED: OWASP Path Traversal; ASSUMED]

3. **Should Phase 2 add `tempfile` or keep the existing manual `unique_temp_dir` helper?** [VERIFIED: codebase read; CITED: tempfile docs]
   - What we know: Existing tests use a manual unique temp path helper, while `tempfile::TempDir` auto-deletes on drop. [VERIFIED: codebase read; CITED: tempfile docs]
   - What's unclear: Whether maintainers prefer no new dev dependency for temporary directories. [ASSUMED]
   - Recommendation: Use `tempfile` if adding `libloading` dev-deps anyway; otherwise keep the helper and ensure cleanup is robust. [ASSUMED]

4. **Should export-symbol checks use platform tools or the `object` crate?** [VERIFIED: environment probe; VERIFIED: crates.io API]
   - What we know: macOS tools `file`, `nm`, and `otool` are available in this environment. [VERIFIED: environment probe]
   - What's unclear: Whether CI target platforms should require portable object-file inspection. [ASSUMED]
   - Recommendation: Keep symbol verification inside `libloading` first; add `object` only if diagnostics need cross-platform export introspection. [CITED: docs.rs/libloading; ASSUMED]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Cargo | Build/test workspace and `cdylib` artifact | yes | cargo 1.95.0 [VERIFIED: environment probe] | Use `/Users/trenton/.cargo/bin/cargo` if shell PATH omits Cargo. [VERIFIED: environment probe] |
| rustc | Compile Rust crates | yes | rustc 1.95.0 [VERIFIED: environment probe] | Use `/Users/trenton/.cargo/bin/rustc` if shell PATH omits rustc. [VERIFIED: environment probe] |
| `libloading` crate | Dynamic-loader harness | not yet in manifest | 0.9.0 available on crates.io [VERIFIED: crates.io API] | Manual platform loader, not recommended. [CITED: docs.rs/libloading; ASSUMED] |
| Dynamic artifact (`yune-rime-api` cdylib) | ABI-01 loader proof | no | Cargo metadata shows only `lib` crate type [VERIFIED: cargo metadata] | Add `[lib] crate-type = ["lib", "cdylib"]`; loader failure is blocker unless equivalent path documented. [CITED: Cargo Book; CITED: user prompt] |
| `file` | Artifact diagnostics | yes | file-5.41 [VERIFIED: environment probe] | `object` crate or skip diagnostics. [VERIFIED: crates.io API; ASSUMED] |
| `nm` / `llvm-nm` | Export diagnostics | yes | llvm-nm compatible with GNU nm [VERIFIED: environment probe] | `libloading::Library::get` as functional symbol proof. [CITED: docs.rs/libloading] |
| `otool` | macOS dylib diagnostics | yes | Apple command-line tool present [VERIFIED: environment probe] | `file`, `nm`, or `libloading` functional check. [VERIFIED: environment probe; CITED: docs.rs/libloading] |
| Miri | Optional undefined-behavior exploration | no | unavailable for stable aarch64-apple-darwin during earlier probe [VERIFIED: environment probe from session summary] | Do not plan Phase 2 around Miri. [ASSUMED] |

**Missing dependencies with no fallback:**

- Real `yune-rime-api` dynamic artifact is currently missing; Phase 2 must create it or document an equivalent platform-specific replacement validation path. [VERIFIED: cargo metadata; CITED: user prompt]

**Missing dependencies with fallback:**

- `tempfile` is not currently required by the manifest; fallback is the existing manual `unique_temp_dir` helper. [VERIFIED: codebase read; CITED: tempfile docs]
- `object` is not currently required by the manifest; fallback is functional `libloading` symbol resolution plus platform tools. [VERIFIED: crates.io API; VERIFIED: environment probe]

## Security Domain

Security enforcement is enabled in `.planning/config.json`, so Phase 2 must include security research and ASVS mapping. [VERIFIED: .planning/config.json]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | no | Phase 2 does not introduce user authentication. [VERIFIED: .planning/ROADMAP.md] |
| V3 Session Management | yes, but application-session semantics do not apply; ABI session IDs do apply | Deterministic `RimeSessionId` lifecycle tests for create/find/destroy/cleanup/finalize. [VERIFIED: codebase read; VERIFIED: .planning/REQUIREMENTS.md] |
| V4 Access Control | limited | Runtime file access must be constrained to application-controlled runtime roots via logical IDs. [VERIFIED: .planning/codebase/CONCERNS.md; CITED: OWASP Path Traversal] |
| V5 Input Validation | yes | Central allowlist validation for C API and YAML-derived resource IDs before filesystem joins. [VERIFIED: .planning/REQUIREMENTS.md; CITED: OWASP Path Traversal] |
| V6 Cryptography | no | Phase 2 does not introduce cryptographic operations. [VERIFIED: .planning/ROADMAP.md] |
| V8 Data Protection | limited | Prevent runtime resource IDs from escaping shared/user/prebuilt/staging/sync directories. [VERIFIED: .planning/codebase/CONCERNS.md; CITED: OWASP Path Traversal] |
| V12 File and Resources | yes | Reject traversal, absolute paths, separators, drive prefixes, and non-logical filesystem syntax before path joins. [VERIFIED: .planning/REQUIREMENTS.md; CITED: OWASP Path Traversal] |
| V14 Configuration | yes | Keep runtime path setup deterministic across repeated initialize/finalize and setup flows. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: codebase read] |

### Known Threat Patterns for Rust C ABI / Runtime Resource Stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Path traversal through config/schema/dictionary/userdb IDs | Tampering / Information Disclosure | Validate known-good logical IDs before joining runtime roots; reject `../`, `..\`, absolute paths, separators, and drive prefixes. [CITED: OWASP Path Traversal; VERIFIED: .planning/REQUIREMENTS.md] |
| ABI callback lifetime misuse | Denial of Service / Tampering | Keep callback context alive for as long as registered; unregister before destruction; synchronize shared state. [CITED: Rustonomicon FFI] |
| Invalid raw pointers at FFI boundaries | Denial of Service | Null-check caller pointers and return false/null/error codes as existing APIs do. [VERIFIED: codebase read; CITED: Rustonomicon FFI] |
| Panic crossing `extern "C"` boundary | Denial of Service | Avoid panics in exported functions; catch or convert errors to C-style return values where needed. [CITED: Rustonomicon FFI] |
| Process-global state pollution across tests | Tampering / Denial of Service | Serialize tests with `test_guard()` and explicitly cleanup sessions/handlers/runtime state between loops. [VERIFIED: codebase read] |
| Dynamic library symbol mismatch | Denial of Service | Resolve `rime_get_api` by exact unmangled symbol name and validate non-null table/function pointers before use. [CITED: docs.rs/libloading; VERIFIED: codebase read] |

## Validation Architecture

Validation Architecture is omitted because `.planning/config.json` explicitly sets `workflow.nyquist_validation` to `false`. [VERIFIED: .planning/config.json]

## Sources

### Primary (HIGH confidence)

- `.planning/REQUIREMENTS.md` — Phase 2 requirement IDs ABI-01 through ABI-04. [VERIFIED: file read]
- `.planning/ROADMAP.md` — Phase 2 description, success criteria, and plan slices 02-01 through 02-03. [VERIFIED: file read]
- `.planning/PROJECT.md` — project constraints including librime oracle and resource-ID security posture. [VERIFIED: file read]
- `.planning/codebase/ARCHITECTURE.md` — ABI architecture and process-global state map. [VERIFIED: file read]
- `.planning/codebase/INTEGRATIONS.md` — ABI entry points and current missing dynamic crate type concern. [VERIFIED: file read]
- `.planning/codebase/CONCERNS.md` — native frontend validation and path traversal concerns. [VERIFIED: file read]
- `.planning/phases/01-cli-frontend-surrogate/VERIFICATION.md` — prior frontend surrogate verification and limits. [VERIFIED: file read]
- `crates/yune-rime-api/src/abi.rs` — FFI structs and `RimeApi` function table shape. [VERIFIED: codebase read]
- `crates/yune-rime-api/src/api_table.rs` — `rime_get_api`, `rime_levers_get_api`, and table construction. [VERIFIED: codebase read]
- `crates/yune-rime-api/src/runtime.rs`, `session.rs`, `deployment.rs`, `notifications.rs`, `modules.rs`, `config_api.rs`, `schema_install.rs`, `levers.rs`, `userdb.rs` — runtime and resource path ownership. [VERIFIED: codebase read]
- `Cargo.toml`, `crates/yune-rime-api/Cargo.toml`, `Cargo.lock`, and Cargo metadata — current workspace targets and dependency versions. [VERIFIED: file read; VERIFIED: cargo metadata]
- `/Users/trenton/Projects/librime/src/rime_api.h` — local librime ABI oracle for session ID, `data_size`, notifications, and `rime_get_api`. [VERIFIED: local file read]
- Context7 `/websites/rs_libloading_0_9_0_libloading` — `Library::new`, `Library::get`, symbol typing. [CITED: docs.rs/libloading/0.9.0]
- Context7 `/rust-lang/reference` — `cdylib` dynamic system library and platform extensions. [CITED: Rust Reference linkage docs]
- Context7 `/websites/doc_rust-lang_cargo` — Cargo `[lib] crate-type = ["cdylib"]`. [CITED: Cargo Book]
- Rustonomicon FFI page — `extern "C"`, `#[repr(C)]`, callbacks, ownership, and unwinding guidance. [CITED: https://doc.rust-lang.org/nomicon/ffi.html]
- OWASP Path Traversal page — traversal patterns and known-good allowlist mitigation. [CITED: https://owasp.org/www-community/attacks/Path_Traversal]

### Secondary (MEDIUM confidence)

- crates.io API — current crate versions and update dates for `libloading`, `tempfile`, `object`, `libc`, `regex`, and `serde_yaml`. [VERIFIED: crates.io API]
- Environment probes — Cargo/rustc/platform tool availability on the current macOS host. [VERIFIED: environment probe]
- Context7 `/stebalien/tempfile` — `TempDir::keep` and cleanup semantics. [CITED: tempfile docs]

### Tertiary (LOW confidence)

- Assumptions about exact resource-ID character allowlist, optional `object` usage, and the best Cargo test orchestration for always producing `cdylib` artifacts. [ASSUMED]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH — crate versions were verified against crates.io API, and dynamic loading/crate-type behavior was checked against official docs. [VERIFIED: crates.io API; CITED: docs.rs/libloading; CITED: Cargo Book; CITED: Rust Reference linkage docs]
- Architecture: HIGH — codebase files and planning docs identify ABI table ownership, runtime globals, frontend surrogate limits, and resource path joins. [VERIFIED: codebase read; VERIFIED: planning docs]
- Pitfalls: MEDIUM-HIGH — direct repository evidence confirms missing `cdylib` and resource join risks; exact harness artifact orchestration remains to be validated during implementation. [VERIFIED: cargo metadata; VERIFIED: codebase read; ASSUMED]
- Security: HIGH for path traversal requirement and OWASP mitigation; MEDIUM for exact allowed ID grammar pending fixture/librime compatibility checks. [VERIFIED: .planning/REQUIREMENTS.md; CITED: OWASP Path Traversal; ASSUMED]

**Research date:** 2026-04-29 [VERIFIED: currentDate]
**Valid until:** 2026-05-29 for repository-specific findings; 2026-05-06 for fast-moving crate version recommendations. [ASSUMED]
