# Phase 2: Native ABI Validation And Runtime Safety - Pattern Map

**Mapped:** 2026-04-29
**Files analyzed:** 17
**Analogs found:** 17 / 17

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `crates/yune-rime-api/Cargo.toml` | config | build/artifact | `crates/yune-rime-api/Cargo.toml` | exact-modify |
| `crates/yune-rime-api/src/resource_id.rs` | utility | transform/validation | `crates/yune-rime-api/src/runtime.rs` + path joins in `lib.rs`/`deployment.rs`/`userdb.rs` | role-match |
| `crates/yune-rime-api/src/lib.rs` | facade/runtime utility | request-response + file-I/O | `crates/yune-rime-api/src/lib.rs` selected runtime path helpers | exact-modify |
| `crates/yune-rime-api/src/config_api.rs` | boundary API | request-response + file-I/O | `crates/yune-rime-api/src/config_api.rs` open APIs | exact-modify |
| `crates/yune-rime-api/src/deployment.rs` | service/API boundary | batch + file-I/O + event-driven | `crates/yune-rime-api/src/deployment.rs` deploy/config/schema paths | exact-modify |
| `crates/yune-rime-api/src/schema_install.rs` | service | file-I/O + transform | `crates/yune-rime-api/src/schema_install.rs` dictionary loading | exact-modify |
| `crates/yune-rime-api/src/levers.rs` | service/API boundary | CRUD + file-I/O | `crates/yune-rime-api/src/levers.rs` custom settings/schema/userdb helpers | exact-modify |
| `crates/yune-rime-api/src/userdb.rs` | service/API boundary | CRUD + file-I/O | `crates/yune-rime-api/src/userdb.rs` user dict path helpers | exact-modify |
| `crates/yune-rime-api/src/runtime.rs` | runtime/service | process-global + file-I/O | `crates/yune-rime-api/src/runtime.rs` runtime paths/global state | exact-modify |
| `crates/yune-rime-api/src/session.rs` | service/model | process-global CRUD | `crates/yune-rime-api/src/session.rs` session registry | exact-modify |
| `crates/yune-rime-api/src/notifications.rs` | service | event-driven | `crates/yune-rime-api/src/notifications.rs` notification handler | exact-modify |
| `crates/yune-rime-api/src/api_table.rs` | ABI provider | request-response/function-table | `crates/yune-rime-api/src/api_table.rs` RimeApi construction/export | exact-modify |
| `crates/yune-rime-api/src/abi.rs` | model | FFI layout | `crates/yune-rime-api/src/abi.rs` repr(C) structs/table | exact-modify |
| `crates/yune-rime-api/tests/dynamic_loader.rs` | test | dynamic loading + request-response | `crates/yune-rime-api/tests/frontend_client.rs` | role-match |
| `crates/yune-rime-api/src/tests/mod.rs` | test support | process-global guard | `crates/yune-rime-api/src/tests/mod.rs` | exact-modify |
| `crates/yune-rime-api/src/tests/{abi,runtime,deployment,config_api,levers,userdb}.rs` | test | regression/file-I/O/event-driven | `crates/yune-rime-api/src/tests/mod.rs` + corresponding module tests | exact-modify |
| `.planning/phases/02-native-abi-validation-and-runtime-safety/findings/*` | structured finding | batch/documentation | `02-CONTEXT.md` finding requirements | partial |

## Pattern Assignments

### `crates/yune-rime-api/Cargo.toml` (config, build/artifact)

**Analog:** `crates/yune-rime-api/Cargo.toml`

**Current manifest pattern** (lines 0-12):
```toml
[package]
name = "yune-rime-api"
version = "0.1.0"
edition.workspace = true
license.workspace = true
repository.workspace = true
rust-version.workspace = true

[dependencies]
libc = "0.2"
regex = "1"
serde_yaml = "0.9"
yune-core = { path = "../yune-core" }
```

**Copy/extend pattern:** Keep workspace-inherited package keys and existing `[dependencies]` style. Add a `[lib]` table for `crate-type = ["lib", "cdylib"]` and add test-only loader dependencies under `[dev-dependencies]` so existing Rust integration tests can still import `yune_rime_api`.

---

### `crates/yune-rime-api/tests/dynamic_loader.rs` (test, dynamic loading + request-response)

**Analog:** `crates/yune-rime-api/tests/frontend_client.rs`

**Imports and frontend ABI types pattern** (lines 0-17):
```rust
use std::{
    ffi::{c_void, CStr, CString},
    fs, mem,
    os::raw::{c_char, c_int},
    path::PathBuf,
    ptr,
    sync::{Mutex, MutexGuard, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};

use yune_rime_api::{
    rime_get_api, RimeCandidate, RimeCandidateListIterator, RimeCommit, RimeComposition,
    RimeConfig, RimeConfigIterator, RimeContext, RimeCustomApi, RimeLeversApi, RimeMenu,
    RimeModule, RimeSchemaList, RimeSessionId, RimeStatus, RimeTraits, RimeUserDictIterator, FALSE,
    TRUE,
};

use serde_yaml::Value;
```

**Test guard/process-global setup pattern** (lines 136-149):
```rust
fn test_guard() -> MutexGuard<'static, ()> {
    static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    let guard = TEST_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("test lock should not be poisoned");
    let api = unsafe { &mut *rime_get_api() };
    let initialize = api
        .initialize
        .expect("frontend requires initialize for test setup");
    let traits = empty_traits();
    unsafe { initialize(&traits) };
    guard
}
```

**Notification capture pattern** (lines 151-180):
```rust
fn notification_events() -> &'static Mutex<Vec<NotificationEvent>> {
    static NOTIFICATION_EVENTS: OnceLock<Mutex<Vec<NotificationEvent>>> = OnceLock::new();
    NOTIFICATION_EVENTS.get_or_init(|| Mutex::new(Vec::new()))
}

extern "C" fn record_notification(
    context_object: *mut c_void,
    session_id: RimeSessionId,
    message_type: *const c_char,
    message_value: *const c_char,
) {
    // SAFETY: the shim invokes handlers with valid NUL-terminated message
    // strings for the duration of the callback.
    let message_type = unsafe { CStr::from_ptr(message_type) }
        .to_string_lossy()
        .into_owned();
    // SAFETY: same as above.
    let message_value = unsafe { CStr::from_ptr(message_value) }
        .to_string_lossy()
        .into_owned();
    notification_events()
        .lock()
        .expect("notification events should not be poisoned")
        .push(NotificationEvent {
            context_object: context_object as usize,
            session_id,
            message_type,
            message_value,
        });
}
```

**Temp runtime root pattern** (lines 182-191):
```rust
fn unique_temp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "yune-rime-api-frontend-{label}-{}-{nanos}",
        std::process::id()
    ))
}
```

**RimeApi table resolution/call pattern to adapt after `libloading` symbol lookup** (lines 201-219):
```rust
#[test]
fn frontend_style_api_table_can_read_schema_lists_and_modules() {
    let _guard = test_guard();
    let api = rime_get_api();
    assert!(!api.is_null());
    let api = unsafe { &*api };

    let setup = api.setup.expect("frontend requires setup");
    let get_schema_list = api
        .get_schema_list
        .expect("frontend requires get_schema_list");
    let free_schema_list = api
        .free_schema_list
        .expect("frontend requires free_schema_list");
    let register_module = api
        .register_module
        .expect("frontend requires register_module");
    let find_module = api.find_module.expect("frontend requires find_module");
```

**Exact notification-order assertion pattern** (lines 995-1038):
```rust
let events = notification_events()
    .lock()
    .expect("notification events should not be poisoned");
assert_eq!(
    *events,
    vec![
        NotificationEvent {
            context_object: 0x7b,
            session_id,
            message_type: "option".to_owned(),
            message_value: "ascii_mode".to_owned(),
        },
        NotificationEvent {
            context_object: 0x7b,
            session_id,
            message_type: "option".to_owned(),
            message_value: "!ascii_mode".to_owned(),
        },
        NotificationEvent {
            context_object: 0x7b,
            session_id,
            message_type: "property".to_owned(),
            message_value: "client_app=frontend_client".to_owned(),
        },
        NotificationEvent {
            context_object: 0x7b,
            session_id,
            message_type: "schema".to_owned(),
            message_value: "sample_schema/sample_schema".to_owned(),
        },
        NotificationEvent {
            context_object: 0x7b,
            session_id: 0,
            message_type: "deploy".to_owned(),
            message_value: "start".to_owned(),
        },
        NotificationEvent {
            context_object: 0x7b,
            session_id: 0,
            message_type: "deploy".to_owned(),
            message_value: "success".to_owned(),
        },
    ]
);
```

**Dynamic-loader-specific adaptation:** Replace direct `rime_get_api()` call with a `libloading::Library` kept alive for the full test, then `lib.get(b"rime_get_api\0")` typed as `unsafe extern "C" fn() -> *mut RimeApi`. After that, copy the same table pointer checks, `expect("frontend requires ...")` function-pointer extraction style, temp root setup, and cleanup patterns from `frontend_client.rs`.

---

### `crates/yune-rime-api/src/resource_id.rs` (utility, transform/validation)

**Analog:** Current scattered path helper patterns in `runtime.rs`, `lib.rs`, `deployment.rs`, `levers.rs`, and `userdb.rs`.

**Runtime path helper style** from `runtime.rs` (lines 315-317):
```rust
pub(crate) fn path_join(base: &str, child: &str) -> String {
    Path::new(base).join(child).to_string_lossy().into_owned()
}
```

**Config resource selection pattern that needs validation before join** from `lib.rs` (lines 1574-1588, 1604-1613):
```rust
pub(crate) fn selected_runtime_config_path(
    resource_id: &str,
    kind: ConfigOpenKind,
) -> Option<PathBuf> {
    let roots = runtime_config_roots(kind);
    roots
        .iter()
        .map(|root| config_file_path(root, resource_id))
        .find(|path| path.exists())
        .or_else(|| {
            roots
                .first()
                .map(|root| config_file_path(root, resource_id))
        })
}

fn normalize_config_resource_id(config_id: &str) -> String {
    config_id
        .strip_suffix(".yaml")
        .unwrap_or(config_id)
        .to_owned()
}

fn config_file_path(root: &str, resource_id: &str) -> PathBuf {
    Path::new(root).join(format!("{resource_id}.yaml"))
}
```

**User dictionary path pattern that needs validation before join** from `userdb.rs` (lines 250-256):
```rust
fn user_dict_path(dict_name: &str) -> PathBuf {
    runtime_user_data_dir().join(format!("{dict_name}.userdb"))
}

fn user_dict_snapshot_path(dict_name: &str) -> PathBuf {
    runtime_user_data_sync_dir().join(format!("{dict_name}.userdb.txt"))
}
```

**Custom config path pattern that needs validation before join** from `levers.rs` (lines 606-613):
```rust
fn custom_config_path(config_id: &str) -> PathBuf {
    let config_name = config_id.strip_suffix(".schema").unwrap_or(config_id);
    let paths = runtime_paths()
        .lock()
        .expect("runtime paths should not be poisoned");
    Path::new(paths.user_data_dir.to_string_lossy().as_ref())
        .join(format!("{config_name}.custom.yaml"))
}
```

**Planner instruction:** Implement this module as a small crate-private allowlist validator with helper functions that return `Option<PathBuf>`/`bool` before callers join roots. Do not normalize attacker input after joining. Preserve existing suffix behavior (`.yaml`, `.schema.yaml`, `.dict.yaml`, `.custom.yaml`, `.userdb`) by validating the logical ID first and appending controlled suffixes after validation.

---

### `crates/yune-rime-api/src/lib.rs` (facade/runtime utility, request-response + file-I/O)

**Analog:** `crates/yune-rime-api/src/lib.rs`

**Module facade/export pattern** (lines 16-63):
```rust
mod abi;
mod api_table;
mod candidate_api;
mod config;
mod config_api;
mod config_compiler;
mod context_api;
mod deployment;
mod ffi_memory;
mod key_table;
mod levers;
mod modules;
mod notifications;
mod processors;
mod runtime;
mod schema_api;
mod schema_install;
mod schema_selection;
mod session;
mod userdb;
pub use abi::*;
use api_table::state_label_cache;
pub use api_table::{rime_get_api, rime_levers_get_api};
pub use candidate_api::*;
use config::*;
pub use config_api::*;
use config_compiler::*;
pub use context_api::*;
pub use deployment::*;
pub use ffi_memory::*;
pub use key_table::*;
pub use levers::*;
pub use modules::*;
use notifications::notify;
pub use notifications::RimeSetNotificationHandler;
pub(crate) use processors::*;
pub use runtime::*;
pub use schema_api::*;
pub(crate) use schema_install::{
    apply_schema_switch_resets, install_schema_filter_chain, install_schema_segment_tags,
    install_schema_translator_chain, load_schema_recognizer_patterns, recognizer_patterns_match,
    schema_component_prescription, schema_string_list, switch_reset_value,
    update_session_segment_tags,
};
pub(crate) use schema_selection::apply_schema_to_session;
pub use schema_selection::{RimeGetCurrentSchema, RimeSelectSchema};
pub use session::*;
pub use userdb::*;
```

**Path selection pattern to keep but guard with resource validation** (lines 1551-1613):
```rust
pub(crate) fn load_runtime_config_root(config_id: &str, kind: ConfigOpenKind) -> Value {
    let resource_id = normalize_config_resource_id(config_id);
    let selected_path = selected_runtime_config_path(&resource_id, kind);

    selected_path
        .and_then(|path| fs::read_to_string(path).ok())
        .and_then(|yaml| serde_yaml::from_str::<Value>(&yaml).ok())
        .unwrap_or(Value::Null)
}

pub(crate) fn selected_runtime_data_path(file_name: &str) -> Option<PathBuf> {
    let paths = runtime_paths()
        .lock()
        .expect("runtime paths should not be poisoned");
    [
        paths.staging_dir.to_string_lossy().into_owned(),
        paths.prebuilt_data_dir.to_string_lossy().into_owned(),
        paths.shared_data_dir.to_string_lossy().into_owned(),
    ]
    .into_iter()
    .map(|root| Path::new(&root).join(file_name))
    .find(|path| path.is_file())
}
```

**Planner instruction:** Add `mod resource_id;` and crate-private re-exports if multiple modules call validators. Update `normalize_config_resource_id`, `selected_runtime_config_path`, `selected_runtime_data_path`, or add new typed helper variants so callers cannot pass path-like resource strings into `Path::join`.

---

### `crates/yune-rime-api/src/config_api.rs` (boundary API, request-response + file-I/O)

**Analog:** `crates/yune-rime-api/src/config_api.rs`

**Imports pattern** (lines 0-16):
```rust
use std::{
    ffi::{c_void, CStr, CString},
    os::raw::{c_char, c_int},
    ptr,
};

use serde_yaml::{Mapping, Value};

use crate::{
    bool_from, c_string_key, clear_schema_list, config_child_path, config_iterator_begin,
    config_lookup, config_lookup_key, config_scalar_bool, config_scalar_double, config_scalar_int,
    config_set, config_state_mut, config_string_value, copy_c_string_with_strncpy_semantics,
    find_config_value, free_schema_list_fields, librime_signature_modified_time,
    open_runtime_config, reset_config_iterator_for_begin, runtime_paths, set_config_value, Bool,
    ConfigIteratorState, ConfigOpenKind, ConfigState, RimeConfig, RimeConfigIterator,
    RimeSchemaList, FALSE, RIME_VERSION_BYTES, TRUE,
};
```

**Boundary C-string + false-on-invalid pattern** (lines 18-63):
```rust
#[no_mangle]
pub unsafe extern "C" fn RimeSchemaOpen(schema_id: *const c_char, config: *mut RimeConfig) -> Bool {
    let Some(schema_id) = (unsafe { c_string_key(schema_id) }) else {
        return FALSE;
    };
    let config_id = format!("{schema_id}.schema");
    open_runtime_config(&config_id, ConfigOpenKind::Deployed, config)
}

#[no_mangle]
pub unsafe extern "C" fn RimeConfigOpen(config_id: *const c_char, config: *mut RimeConfig) -> Bool {
    let Some(config_id) = (unsafe { c_string_key(config_id) }) else {
        return FALSE;
    };
    open_runtime_config(&config_id, ConfigOpenKind::Deployed, config)
}

#[no_mangle]
pub unsafe extern "C" fn RimeUserConfigOpen(
    config_id: *const c_char,
    config: *mut RimeConfig,
) -> Bool {
    let Some(config_id) = (unsafe { c_string_key(config_id) }) else {
        return FALSE;
    };
    open_runtime_config(&config_id, ConfigOpenKind::User, config)
}
```

**Memory ownership/error handling pattern** (lines 89-103, 148-163):
```rust
#[no_mangle]
pub unsafe extern "C" fn RimeConfigInit(config: *mut RimeConfig) -> Bool {
    if config.is_null() {
        return FALSE;
    }
    // SAFETY: `config` is non-null and points to caller-owned storage.
    if unsafe { !(*config).ptr.is_null() } {
        return FALSE;
    }

    let state = Box::new(ConfigState::default());
    // SAFETY: `config` is non-null and writable.
    unsafe {
        (*config).ptr = Box::into_raw(state).cast::<c_void>();
    }
    TRUE
}

#[no_mangle]
pub unsafe extern "C" fn RimeConfigClose(config: *mut RimeConfig) -> Bool {
    if config.is_null() {
        return FALSE;
    }
    // SAFETY: `config` is non-null and points to caller-owned storage.
    let ptr = unsafe { (*config).ptr };
    if ptr.is_null() {
        return FALSE;
    }
    // SAFETY: `ptr` was returned by `Box::into_raw` in `RimeConfigInit`.
    unsafe {
        drop(Box::from_raw(ptr.cast::<ConfigState>()));
        (*config).ptr = ptr::null_mut();
    }
    TRUE
}
```

**Planner instruction:** Add logical resource-ID validation immediately after `c_string_key` succeeds in `RimeSchemaOpen`, `RimeConfigOpen`, and `RimeUserConfigOpen`. Keep existing C ABI style: no panic, return `FALSE`, and preserve `// SAFETY:` comments around unsafe blocks.

---

### `crates/yune-rime-api/src/deployment.rs` (service/API boundary, batch + file-I/O + event-driven)

**Analog:** `crates/yune-rime-api/src/deployment.rs`

**Imports pattern** (lines 0-19):
```rust
use std::{
    collections::HashSet,
    fs,
    os::raw::{c_char, c_int},
    path::{Path, PathBuf},
    sync::atomic::Ordering,
    time::{SystemTime, UNIX_EPOCH},
};

use serde_yaml::{Mapping, Number, Value};

use crate::{
    apply_config_directives, apply_custom_patch, apply_legacy_preset_config_plugins, bool_from,
    cstring_from_lossless_str, find_config_value, load_runtime_config_root,
    normalize_config_resource_id, optional_c_string, path_join, runtime_paths,
    runtime_user_data_sync_dir, service_started, set_build_info, set_config_value,
    source_modified_secs, source_uses_auto_custom_patch, sync_all_user_dicts, user_dict_upgrade,
    Bool, ConfigOpenKind, RimeCleanupAllSessions, RimeSetup, RimeTraits, FALSE, RIME_VERSION_BYTES,
    TRUE,
};
```

**Extern boundary/error pattern** (lines 94-114, 128-158):
```rust
#[no_mangle]
pub extern "C" fn RimeDeploySchema(schema_file: *const c_char) -> Bool {
    let Some(schema_file) = optional_c_string(schema_file) else {
        return FALSE;
    };
    bool_from(deploy_schema_file(&schema_file))
}

#[no_mangle]
pub extern "C" fn RimeDeployConfigFile(
    file_name: *const c_char,
    version_key: *const c_char,
) -> Bool {
    let Some(file_name) = optional_c_string(file_name) else {
        return FALSE;
    };
    let Some(version_key) = optional_c_string(version_key) else {
        return FALSE;
    };
    bool_from(deploy_config_file(&file_name, &version_key))
}

#[no_mangle]
pub extern "C" fn RimeRunTask(task_name: *const c_char) -> Bool {
    let Some(task_name) = optional_c_string(task_name) else {
        return FALSE;
    };
    if task_name == "user_dict_sync" {
        return bool_from(sync_all_user_dicts());
    }
    if task_name == "backup_config_files" {
        return bool_from(backup_config_files());
    }
    if task_name == "installation_update" {
        return bool_from(run_installation_update());
    }
    if task_name == "clean_old_log_files" {
        return bool_from(clean_old_log_files());
    }
    if task_name == "cleanup_trash" {
        return bool_from(cleanup_trash());
    }
    if task_name == "workspace_update" {
        return bool_from(workspace_update());
    }
    if task_name == "user_dict_upgrade" {
        return bool_from(user_dict_upgrade());
    }
    if task_name == "prebuild_all_schemas" {
        return bool_from(prebuild_all_schemas());
    }
    FALSE
}
```

**Lifecycle + service state pattern** (lines 21-37):
```rust
#[no_mangle]
pub unsafe extern "C" fn RimeInitialize(traits: *const RimeTraits) {
    // SAFETY: forwarded preconditions are identical to `RimeSetup`.
    unsafe { RimeSetup(traits) };
    service_started().store(true, Ordering::SeqCst);
}

#[no_mangle]
pub extern "C" fn RimeFinalize() {
    RimeCleanupAllSessions();
    service_started().store(false, Ordering::SeqCst);
}
```

**Notification order pattern** (lines 39-52, 117-125):
```rust
#[no_mangle]
pub extern "C" fn RimeStartMaintenance(full_check: Bool) -> Bool {
    let _ = clean_old_log_files();
    if !run_installation_update() {
        return FALSE;
    }
    if full_check == FALSE && !detect_modifications() {
        return FALSE;
    }
    crate::notify(0, "deploy", "start");
    let success = run_workspace_maintenance_tasks();
    crate::notify(0, "deploy", if success { "success" } else { "failure" });
    bool_from(success)
}

#[no_mangle]
pub extern "C" fn RimeSyncUserData() -> Bool {
    RimeCleanupAllSessions();
    crate::notify(0, "deploy", "start");
    let installation_synced = run_installation_update();
    let configs_synced = backup_config_files();
    let user_dicts_synced = sync_all_user_dicts();
    let success = installation_synced && configs_synced && user_dicts_synced;
    crate::notify(0, "deploy", if success { "success" } else { "failure" });
    bool_from(success)
}
```

**Path joins requiring validation** (lines 730-769, 873-889, 975-1006):
```rust
pub(crate) fn deploy_config_file(file_name: &str, version_key: &str) -> bool {
    if file_name.is_empty() || version_key.is_empty() {
        return false;
    }

    let (shared_data_dir, user_data_dir, staging_dir) = {
        let paths = runtime_paths()
            .lock()
            .expect("runtime paths should not be poisoned");
        (
            PathBuf::from(paths.shared_data_dir.to_string_lossy().into_owned()),
            PathBuf::from(paths.user_data_dir.to_string_lossy().into_owned()),
            PathBuf::from(paths.staging_dir.to_string_lossy().into_owned()),
        )
    };
    let source = shared_data_dir.join(file_name);
    let destination = staging_dir.join(file_name);
    if !source.is_file() {
        return false;
    }
```

```rust
fn custom_patch_resource_id(resource_id: &str) -> String {
    let base = resource_id.strip_suffix(".schema").unwrap_or(resource_id);
    format!("{base}.custom")
}

fn config_resource_path(
    shared_data_dir: &Path,
    user_data_dir: &Path,
    resource_id: &str,
) -> PathBuf {
    let root = if resource_id.ends_with(".custom") {
        user_data_dir
    } else {
        shared_data_dir
    };
    root.join(format!("{resource_id}.yaml"))
}
```

```rust
pub(crate) fn deploy_schema_file(schema_file: &str) -> bool {
    if schema_file.is_empty() {
        return false;
    }

    let shared_data_dir = {
        let paths = runtime_paths()
            .lock()
            .expect("runtime paths should not be poisoned");
        PathBuf::from(paths.shared_data_dir.to_string_lossy().into_owned())
    };
    let source = shared_data_dir.join(schema_file);
    if !source.is_file() {
        return false;
    }

    let schema_config = match fs::read_to_string(source)
        .ok()
        .and_then(|yaml| serde_yaml::from_str::<Value>(&yaml).ok())
    {
        Some(schema_config) => schema_config,
        None => return false,
    };
    let Some(schema_id) = find_config_value(&schema_config, "schema/schema_id")
        .and_then(Value::as_str)
        .filter(|schema_id| !schema_id.is_empty())
    else {
        return false;
    };

    deploy_config_file(&format!("{schema_id}.schema.yaml"), "schema/version")
}
```

**Planner instruction:** Validate `file_name`, `schema_file`, schema IDs from YAML, dependency IDs, and custom resource IDs before these joins. Keep path-valued runtime roots and log directories as paths; only validate logical IDs/filenames that become resource filenames under controlled roots.

---

### `crates/yune-rime-api/src/schema_install.rs` (service, file-I/O + transform)

**Analog:** `crates/yune-rime-api/src/schema_install.rs`

**Dictionary resource join pattern needing validation** (lines 390-412):
```rust
fn load_schema_table_dictionary(
    schema_config: &Value,
    name_space: &str,
) -> Option<TableDictionary> {
    let dictionary_name = find_config_value(schema_config, &format!("{name_space}/dictionary"))
        .and_then(config_scalar_string)
        .filter(|dictionary_name| !dictionary_name.is_empty())?;
    let dictionary_path = selected_runtime_data_path(&format!("{dictionary_name}.dict.yaml"))?;
    let dictionary_yaml = fs::read_to_string(dictionary_path).ok()?;
    let packs = schema_dictionary_packs(schema_config, name_space);
    TableDictionary::parse_rime_dict_yaml_with_imports_packs_and_vocabulary(
        &dictionary_yaml,
        packs,
        |import_table| {
            selected_runtime_data_path(&format!("{import_table}.dict.yaml"))
                .and_then(|path| fs::read_to_string(path).ok())
        },
        |vocabulary| {
            selected_runtime_data_path(&format!("{vocabulary}.txt"))
                .and_then(|path| fs::read_to_string(path).ok())
        },
    )
    .ok()
}
```

**Planner instruction:** Validate `dictionary_name`, parser-provided `import_table`, packs, and `vocabulary` names as logical resource IDs before formatting `.dict.yaml` or `.txt` filenames. Preserve the current `Option` style and return `None` on invalid IDs.

---

### `crates/yune-rime-api/src/levers.rs` (service/API boundary, CRUD + file-I/O)

**Analog:** `crates/yune-rime-api/src/levers.rs`

**Custom settings path pattern needing validation** (lines 606-613):
```rust
fn custom_config_path(config_id: &str) -> PathBuf {
    let config_name = config_id.strip_suffix(".schema").unwrap_or(config_id);
    let paths = runtime_paths()
        .lock()
        .expect("runtime paths should not be poisoned");
    Path::new(paths.user_data_dir.to_string_lossy().as_ref())
        .join(format!("{config_name}.custom.yaml"))
}
```

**Schema info list path discovery pattern** (lines 705-746):
```rust
fn deployed_levers_schema_infos() -> Vec<LeverSchemaInfo> {
    let paths = runtime_paths()
        .lock()
        .expect("runtime paths should not be poisoned");
    let roots = [
        paths.shared_data_dir.to_string_lossy().into_owned(),
        paths.user_data_dir.to_string_lossy().into_owned(),
    ];
    drop(paths);

    let mut seen = HashSet::new();
    let mut infos = Vec::new();
    for root in roots {
        for path in schema_file_paths_in_dir(&root) {
            let Some((schema_id, schema_config)) = levers_schema_config_from_file(&path) else {
                continue;
            };
            if !seen.insert(schema_id.clone()) {
                continue;
            }
            infos.push(levers_schema_info(schema_id, schema_config, Some(path)));
        }
    }
    infos
}

fn schema_file_paths_in_dir(root: &str) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(root) else {
        return Vec::new();
    };
    let mut paths = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .map(|name| name.to_string_lossy().ends_with(".schema.yaml"))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    paths.sort();
    paths
}
```

**Allocated schema list pattern** (lines 675-703):
```rust
fn populate_schema_id_list(schema_list: *mut RimeSchemaList, schema_ids: Vec<String>) -> Bool {
    if schema_ids.is_empty() {
        return FALSE;
    }

    let mut list = Vec::with_capacity(schema_ids.len());
    for schema_id in schema_ids {
        let Ok(schema_id) = CString::new(schema_id) else {
            free_schema_list_items(&mut list);
            return FALSE;
        };
        list.push(RimeSchemaListItem {
            schema_id: schema_id.into_raw(),
            name: ptr::null_mut(),
            reserved: ptr::null_mut(),
        });
    }
    let size = list.len();
    let list_ptr = list.as_mut_ptr();
    std::mem::forget(list);

    // SAFETY: `schema_list` is non-null and points to caller-owned writable
    // storage; `list_ptr` owns `size` initialized schema-list items.
    unsafe {
        (*schema_list).size = size;
        (*schema_list).list = list_ptr;
    }
    TRUE
}
```

**Planner instruction:** Validate custom config IDs before `custom_config_path`. Validate selected schema IDs before writing selections. For user-dict functions exposed through levers, share `resource_id` validation with `userdb.rs` for `dict_name` but do not reject path-valued `text_file`/`snapshot_file` solely because they are paths.

---

### `crates/yune-rime-api/src/userdb.rs` (service/API boundary, CRUD + file-I/O)

**Analog:** `crates/yune-rime-api/src/userdb.rs`

**Imports pattern** (lines 0-12):
```rust
use std::{
    collections::HashSet,
    ffi::c_void,
    fs,
    os::raw::{c_char, c_int},
    path::{Path, PathBuf},
    ptr,
};

use crate::{
    bool_from, clear_user_dict_iterator, cstring_from_lossless_str, optional_c_string,
    runtime_paths, Bool, RimeUserDictIterator, UserDictListState, FALSE, TRUE,
};
```

**C API boundary + path-valued parameter distinction** (lines 102-132, 142-213):
```rust
#[no_mangle]
pub unsafe extern "C" fn RimeLeversBackupUserDict(dict_name: *const c_char) -> Bool {
    let Some(dict_name) = optional_c_string(dict_name) else {
        return FALSE;
    };
    bool_from(backup_plain_user_dict(&dict_name))
}

#[no_mangle]
pub unsafe extern "C" fn RimeLeversRestoreUserDict(snapshot_file: *const c_char) -> Bool {
    let Some(snapshot_file) = optional_c_string(snapshot_file) else {
        return FALSE;
    };
    let snapshot = PathBuf::from(snapshot_file);
    if !snapshot.is_file() {
        return FALSE;
    }
    let Some(dict_name) = snapshot_dict_name(&snapshot) else {
        return FALSE;
    };
    let destination = user_dict_path(&dict_name);
```

```rust
#[no_mangle]
pub unsafe extern "C" fn RimeLeversExportUserDict(
    dict_name: *const c_char,
    text_file: *const c_char,
) -> c_int {
    let Some(dict_name) = optional_c_string(dict_name) else {
        return -1;
    };
    let Some(text_file) = optional_c_string(text_file) else {
        return -1;
    };
    if dict_name.is_empty() || text_file.is_empty() {
        return -1;
    }

    let source = user_dict_path(&dict_name);
```

```rust
#[no_mangle]
pub unsafe extern "C" fn RimeLeversImportUserDict(
    dict_name: *const c_char,
    text_file: *const c_char,
) -> c_int {
    let Some(dict_name) = optional_c_string(dict_name) else {
        return -1;
    };
    let Some(text_file) = optional_c_string(text_file) else {
        return -1;
    };
    if dict_name.is_empty() || text_file.is_empty() {
        return -1;
    }

    let source = PathBuf::from(text_file);
```

**Userdb path join pattern needing validation** (lines 250-273):
```rust
fn user_dict_path(dict_name: &str) -> PathBuf {
    runtime_user_data_dir().join(format!("{dict_name}.userdb"))
}

fn user_dict_snapshot_path(dict_name: &str) -> PathBuf {
    runtime_user_data_sync_dir().join(format!("{dict_name}.userdb.txt"))
}

pub(crate) fn backup_plain_user_dict(dict_name: &str) -> bool {
    if dict_name.is_empty() {
        return false;
    }

    let source = user_dict_path(dict_name);
    if !source.is_file() {
        return false;
    }
    let snapshot = user_dict_snapshot_path(dict_name);
    if let Some(parent) = snapshot.parent() {
        if fs::create_dir_all(parent).is_err() {
            return false;
        }
    }
    fs::copy(source, snapshot).is_ok()
}
```

**Snapshot name extraction pattern** (lines 349-356):
```rust
fn snapshot_dict_name(snapshot_file: &Path) -> Option<String> {
    snapshot_file
        .file_name()
        .and_then(|file_name| file_name.to_str())
        .and_then(|file_name| file_name.strip_suffix(".userdb.txt"))
        .filter(|dict_name| !dict_name.is_empty())
        .map(ToOwned::to_owned)
}
```

**Planner instruction:** Validate `dict_name` before `user_dict_path`/`user_dict_snapshot_path`; validate the extracted `snapshot_dict_name`; keep `text_file` and `snapshot_file` as path-valued parameters with separate existence checks.

---

### `crates/yune-rime-api/src/runtime.rs` (runtime/service, process-global + file-I/O)

**Analog:** `crates/yune-rime-api/src/runtime.rs`

**Imports and global runtime state pattern** (lines 0-14, 187-190):
```rust
use std::{
    ffi::CString,
    fs,
    os::raw::c_char,
    path::Path,
    ptr,
    sync::{Mutex, OnceLock},
};

use serde_yaml::Value;

use crate::{
    copy_c_string_with_strncpy_semantics, cstring_from_lossless_str, optional_c_string,
    rime_struct_has_member, RimeTraits,
};
```

```rust
pub(crate) fn runtime_paths() -> &'static Mutex<RuntimePaths> {
    static RUNTIME_PATHS: OnceLock<Mutex<RuntimePaths>> = OnceLock::new();
    RUNTIME_PATHS.get_or_init(|| Mutex::new(RuntimePaths::default()))
}
```

**Trait data-size/lifetime pattern** (lines 90-159):
```rust
pub(crate) unsafe fn from_traits(traits: *const RimeTraits) -> Option<Self> {
    if traits.is_null() {
        return None;
    }

    // SAFETY: callers promise that `traits`, when non-null, points to at
    // least the leading `data_size` field of a `RimeTraits` object.
    let data_size = unsafe { (*traits).data_size };
    let provided_string = |member: *const *const c_char| {
        if rime_struct_has_member(traits, data_size, member) {
            // SAFETY: the field is covered by `data_size`; callers promise
            // that provided non-null strings are NUL-terminated.
            unsafe { optional_c_string(*member) }
        } else {
            None
        }
    };
```

**Setup boundary pattern** (lines 192-206):
```rust
/// Stores process-wide runtime traits for later path queries.
///
/// # Safety
///
/// `traits` must be either null or a valid pointer to a `RimeTraits` object.
/// Any non-null string pointers in the traits object must be valid
/// NUL-terminated C strings.
#[no_mangle]
pub unsafe extern "C" fn RimeSetup(traits: *const RimeTraits) {
    if let Some(paths) = unsafe { RuntimePaths::from_traits(traits) } {
        *runtime_paths()
            .lock()
            .expect("runtime paths should not be poisoned") = paths;
    }
}
```

**Secure buffer-copy pattern** (lines 326-340):
```rust
pub(crate) fn copy_runtime_path_to_buffer(
    select: impl FnOnce(&RuntimePaths) -> &CString,
    output: *mut c_char,
    buffer_size: usize,
) {
    if output.is_null() || buffer_size == 0 {
        return;
    }

    let paths = runtime_paths()
        .lock()
        .expect("runtime paths should not be poisoned");
    let value = select(&paths).to_string_lossy();
    copy_c_string_with_strncpy_semantics(&value, output, buffer_size);
}
```

**Planner instruction:** For installation-derived `installation_id`/`user_id` used in `path_join(sync_dir, user_id)`, treat it as a logical segment and validate before joining. Do not validate runtime root paths (`shared_data_dir`, `user_data_dir`, `sync_dir`) as logical IDs because they are intentionally path-valued.

---

### `crates/yune-rime-api/src/session.rs` (service/model, process-global CRUD)

**Analog:** `crates/yune-rime-api/src/session.rs`

**Imports and atomic service state pattern** (lines 0-18):
```rust
use std::{
    collections::HashMap,
    ffi::CString,
    os::raw::c_int,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex, OnceLock,
    },
    time::{SystemTime, UNIX_EPOCH},
};

use yune_core::{Engine, KeyEvent};
```

**Session registry CRUD pattern** (lines 22-63):
```rust
#[derive(Default)]
pub(crate) struct SessionRegistry {
    pub(crate) next_id: RimeSessionId,
    pub(crate) sessions: HashMap<RimeSessionId, SessionState>,
}

impl SessionRegistry {
    pub(crate) fn create_session(&mut self) -> RimeSessionId {
        if !service_started().load(Ordering::SeqCst) {
            return 0;
        }

        self.next_id = self.next_id.saturating_add(1).max(1);
        let session_id = self.next_id;
        self.sessions.insert(session_id, SessionState::new());
        session_id
    }

    pub(crate) fn get_session_mut(
        &mut self,
        session_id: RimeSessionId,
    ) -> Option<&mut SessionState> {
        if session_id == 0 || !service_started().load(Ordering::SeqCst) {
            return None;
        }

        let session = self.sessions.get_mut(&session_id)?;
        session.activate();
        Some(session)
    }
```

**Global registry and exported functions pattern** (lines 139-193):
```rust
pub(crate) fn sessions() -> &'static Mutex<SessionRegistry> {
    static SESSIONS: OnceLock<Mutex<SessionRegistry>> = OnceLock::new();
    SESSIONS.get_or_init(|| Mutex::new(SessionRegistry::default()))
}

pub(crate) fn service_started() -> &'static AtomicBool {
    static SERVICE_STARTED: AtomicBool = AtomicBool::new(false);
    &SERVICE_STARTED
}

#[no_mangle]
pub extern "C" fn RimeCreateSession() -> RimeSessionId {
    sessions()
        .lock()
        .expect("session registry should not be poisoned")
        .create_session()
}

#[no_mangle]
pub extern "C" fn RimeFindSession(session_id: RimeSessionId) -> Bool {
    let mut registry = sessions()
        .lock()
        .expect("session registry should not be poisoned");
    bool_from(registry.find_session(session_id))
}
```

**Planner instruction:** Lifecycle tests should assert behavior through exported API table functions, not by mutating `sessions()` directly except in crate-internal focused tests. Preserve `service_started()` semantics when adding repeated initialize/finalize coverage.

---

### `crates/yune-rime-api/src/notifications.rs` (service, event-driven)

**Analog:** `crates/yune-rime-api/src/notifications.rs`

**Notification state and callback invocation pattern** (lines 0-52):
```rust
use std::{
    ffi::{c_void, CString},
    sync::{Mutex, OnceLock},
};

use crate::{RimeNotificationHandler, RimeSessionId};

#[derive(Default)]
struct NotificationState {
    handler: Option<RimeNotificationHandler>,
    context_object: usize,
}

fn notification_state() -> &'static Mutex<NotificationState> {
    static NOTIFICATION_STATE: OnceLock<Mutex<NotificationState>> = OnceLock::new();
    NOTIFICATION_STATE.get_or_init(|| Mutex::new(NotificationState::default()))
}

#[no_mangle]
pub extern "C" fn RimeSetNotificationHandler(
    handler: Option<RimeNotificationHandler>,
    context_object: *mut c_void,
) {
    let mut state = notification_state()
        .lock()
        .expect("notification state should not be poisoned");
    state.handler = handler;
    state.context_object = context_object as usize;
}

pub(crate) fn notify(session_id: RimeSessionId, message_type: &str, message_value: &str) {
    let (handler, context_object) = {
        let state = notification_state()
            .lock()
            .expect("notification state should not be poisoned");
        let Some(handler) = state.handler else {
            return;
        };
        (handler, state.context_object)
    };
    let Ok(message_type) = CString::new(message_type) else {
        return;
    };
    let Ok(message_value) = CString::new(message_value) else {
        return;
    };
    handler(
        context_object as *mut c_void,
        session_id,
        message_type.as_ptr(),
        message_value.as_ptr(),
    );
}
```

**Planner instruction:** Keep the lock released before invoking callbacks. Tests should assert deterministic order at call sites (`deployment.rs`, `schema_selection.rs`, option/property paths) rather than relying on unordered event presence.

---

### `crates/yune-rime-api/src/api_table.rs` (ABI provider, request-response/function-table)

**Analog:** `crates/yune-rime-api/src/api_table.rs`

**Function-table singleton pattern** (lines 0-12, 57-64):
```rust
use std::{
    ffi::CString,
    os::raw::c_int,
    sync::{Mutex, OnceLock},
};

use crate::*;

fn levers_api_entry() -> *mut RimeLeversApi {
    static LEVERS_API: OnceLock<usize> = OnceLock::new();
    *LEVERS_API.get_or_init(|| Box::into_raw(Box::new(build_levers_api())) as usize)
        as *mut RimeLeversApi
}
```

```rust
fn api_entry() -> *mut RimeApi {
    static API: OnceLock<usize> = OnceLock::new();
    *API.get_or_init(|| Box::into_raw(Box::new(build_rime_api())) as usize) as *mut RimeApi
}

fn build_rime_api() -> RimeApi {
    RimeApi {
        data_size: (std::mem::size_of::<RimeApi>() - std::mem::size_of::<c_int>()) as c_int,
```

**Exported symbol pattern** (lines 166-174):
```rust
#[no_mangle]
pub extern "C" fn rime_get_api() -> *mut RimeApi {
    api_entry()
}

#[no_mangle]
pub extern "C" fn rime_levers_get_api() -> *mut RimeCustomApi {
    levers_api_entry().cast::<RimeCustomApi>()
}
```

**Planner instruction:** The dynamic loader test must resolve this exact unmangled `rime_get_api` symbol from the produced cdylib and then drive function pointers. Keep `data_size` convention and `OnceLock` leaked table pointer behavior unless a loader regression shows an ABI lifetime bug.

---

### `crates/yune-rime-api/src/abi.rs` (model, FFI layout)

**Analog:** `crates/yune-rime-api/src/abi.rs`

**Basic C ABI struct/callback pattern** (lines 0-18):
```rust
use std::{
    ffi::c_void,
    os::raw::{c_char, c_int},
};

pub type RimeSessionId = usize;
pub type Bool = c_int;
pub type RimeNotificationHandler = extern "C" fn(
    context_object: *mut c_void,
    session_id: RimeSessionId,
    message_type: *const c_char,
    message_value: *const c_char,
);

pub const FALSE: Bool = 0;
pub const TRUE: Bool = 1;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
```

**Struct layout pattern** (lines 17-32, 71-94):
```rust
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RimeTraits {
    pub data_size: c_int,
    pub shared_data_dir: *const c_char,
    pub user_data_dir: *const c_char,
    pub distribution_name: *const c_char,
    pub distribution_code_name: *const c_char,
    pub distribution_version: *const c_char,
    pub app_name: *const c_char,
    pub modules: *const *const c_char,
    pub min_log_level: c_int,
    pub log_dir: *const c_char,
    pub prebuilt_data_dir: *const c_char,
    pub staging_dir: *const c_char,
}
```

```rust
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RimeContext {
    pub data_size: c_int,
    pub composition: RimeComposition,
    pub menu: RimeMenu,
    pub commit_text_preview: *mut c_char,
    pub select_labels: *mut *mut c_char,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RimeStatus {
    pub data_size: c_int,
    pub schema_id: *mut c_char,
    pub schema_name: *mut c_char,
    pub is_disabled: Bool,
    pub is_composing: Bool,
    pub is_ascii_mode: Bool,
    pub is_full_shape: Bool,
    pub is_simplified: Bool,
    pub is_traditional: Bool,
    pub is_ascii_punct: Bool,
}
```

**Function pointer table pattern** (lines 261-300):
```rust
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RimeApi {
    pub data_size: c_int,
    pub setup: Option<SetupFn>,
    pub set_notification_handler: Option<SetNotificationHandlerFn>,
    pub initialize: Option<SetupFn>,
    pub finalize: Option<extern "C" fn()>,
    pub start_maintenance: Option<extern "C" fn(Bool) -> Bool>,
    pub is_maintenance_mode: Option<NoArgBoolFn>,
    pub join_maintenance_thread: Option<extern "C" fn()>,
    pub deployer_initialize: Option<SetupFn>,
    pub prebuild: Option<NoArgBoolFn>,
    pub deploy: Option<NoArgBoolFn>,
    pub deploy_schema: Option<extern "C" fn(*const c_char) -> Bool>,
    pub deploy_config_file: Option<extern "C" fn(*const c_char, *const c_char) -> Bool>,
    pub sync_user_data: Option<NoArgBoolFn>,
    pub create_session: Option<extern "C" fn() -> RimeSessionId>,
    pub find_session: Option<extern "C" fn(RimeSessionId) -> Bool>,
    pub destroy_session: Option<extern "C" fn(RimeSessionId) -> Bool>,
```

**Planner instruction:** ABI layout tests should compare `data_size` to `size_of::<T>() - size_of::<c_int>()` for structs where librime-style `data_size` excludes the field itself. Any new ABI struct/function pointer must be `#[repr(C)]`, raw-pointer based, and exported via `RimeApi` only after tests assert layout and table pointer presence.

---

### `crates/yune-rime-api/src/tests/mod.rs` (test support, process-global guard)

**Analog:** `crates/yune-rime-api/src/tests/mod.rs`

**Internal test imports and module registration pattern** (lines 0-23):
```rust
use std::env;
use std::ffi::{c_void, CStr, CString};
use std::fs;
use std::os::raw::{c_char, c_int};
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_yaml::Value;
use yune_core::{Candidate, CandidateSource, StaticTableTranslator, Translator};

mod abi;
mod candidate_api;
mod config_api;
mod context_status;
mod deployment;
mod levers;
mod runtime;
mod schema_api;
mod schema_processors;
mod schema_selection;
mod session_api;
mod userdb;
```

**Process-global guard pattern** (lines 92-102):
```rust
fn test_guard() -> MutexGuard<'static, ()> {
    static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    let guard = TEST_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("test lock should not be poisoned");
    let traits = empty_traits();
    // SAFETY: empty traits points to valid storage for the duration of the call.
    unsafe { RimeInitialize(&traits) };
    guard
}
```

**Helper structs/empty ABI object pattern** (lines 155-178, 263-278):
```rust
fn empty_context() -> RimeContext {
    RimeContext {
        data_size: (std::mem::size_of::<RimeContext>() - std::mem::size_of::<i32>()) as i32,
        composition: super::RimeComposition {
            length: 0,
            cursor_pos: 0,
            sel_start: 0,
            sel_end: 0,
            preedit: std::ptr::null_mut(),
        },
        menu: super::RimeMenu {
            page_size: 0,
            page_no: 0,
            is_last_page: FALSE,
            highlighted_candidate_index: 0,
            num_candidates: 0,
            candidates: std::ptr::null_mut(),
            select_keys: std::ptr::null_mut(),
        },
        commit_text_preview: std::ptr::null_mut(),
        select_labels: std::ptr::null_mut(),
    }
}
```

```rust
fn empty_traits() -> RimeTraits {
    RimeTraits {
        data_size: std::mem::size_of::<RimeTraits>() as i32,
        shared_data_dir: std::ptr::null(),
        user_data_dir: std::ptr::null(),
        distribution_name: std::ptr::null(),
        distribution_code_name: std::ptr::null(),
        distribution_version: std::ptr::null(),
        app_name: std::ptr::null(),
        modules: std::ptr::null(),
        min_log_level: 0,
        log_dir: std::ptr::null(),
        prebuilt_data_dir: std::ptr::null(),
        staging_dir: std::ptr::null(),
    }
}
```

**Temp dir helper pattern** (lines 328-337):
```rust
fn unique_temp_dir(name: &str) -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();
    env::temp_dir().join(format!(
        "yune-rime-api-{name}-{}-{nonce}",
        std::process::id()
    ))
}
```

**Planner instruction:** Add new crate-internal regression tests to existing module files when possible and share these helpers. Tests touching runtime/session/notifications/deployment must call `test_guard()` and reset with `RimeSetup`/`RimeFinalize` or cleanup functions as existing tests do.

---

### `crates/yune-rime-api/src/tests/{abi,runtime,deployment,config_api,levers,userdb}.rs` (test, regression/file-I/O/event-driven)

**Analog:** `crates/yune-rime-api/src/tests/mod.rs` plus `frontend_client.rs`

**Direct exported-call test pattern** from `runtime.rs` tests (lines 0-19):
```rust
use super::*;

#[test]
fn maps_bool_to_rime_bool() {
    assert_eq!(bool_from(true), TRUE);
    assert_eq!(bool_from(false), FALSE);
}

#[test]
fn key_table_exposes_librime_style_modifier_and_key_name_lookup() {
    let shift = CString::new("Shift").expect("modifier name should be valid");
    let control = CString::new("Control").expect("modifier name should be valid");
    let alt = CString::new("Alt").expect("modifier name should be valid");
    let unknown = CString::new("NoSuchModifier").expect("modifier name should be valid");

    assert_eq!(unsafe { RimeGetModifierByName(shift.as_ptr()) }, 1);
    assert_eq!(unsafe { RimeGetModifierByName(control.as_ptr()) }, 1 << 2);
    assert_eq!(unsafe { RimeGetModifierByName(alt.as_ptr()) }, 1 << 3);
    assert_eq!(unsafe { RimeGetModifierByName(unknown.as_ptr()) }, 0);
    assert_eq!(unsafe { RimeGetModifierByName(std::ptr::null()) }, 0);
```

**Frontend-style API-table lifecycle/deployment pattern** from `frontend_client.rs` (lines 1219-1317):
```rust
#[test]
fn frontend_style_api_table_can_run_deployment_and_maintenance() {
    let _guard = test_guard();
    let api = rime_get_api();
    assert!(!api.is_null());
    let api = unsafe { &*api };

    let deployer_initialize = api
        .deployer_initialize
        .expect("frontend requires deployer_initialize");
    let start_maintenance = api
        .start_maintenance
        .expect("frontend requires start_maintenance");
    let is_maintenance_mode = api
        .is_maintenance_mode
        .expect("frontend requires is_maintenance_mode");
    let join_maintenance_thread = api
        .join_maintenance_thread
        .expect("frontend requires join_maintenance_thread");
    let prebuild = api.prebuild.expect("frontend requires prebuild");
    let deploy = api.deploy.expect("frontend requires deploy");
    let deploy_schema = api.deploy_schema.expect("frontend requires deploy_schema");
    let deploy_config_file = api
        .deploy_config_file
        .expect("frontend requires deploy_config_file");
    let run_task = api.run_task.expect("frontend requires run_task");
    let sync_user_data = api
        .sync_user_data
        .expect("frontend requires sync_user_data");
```

**Resource-ID regression test shape:** For each unsafe C boundary, pass `CString::new("../escape")`, `CString::new("/absolute")`, `CString::new("a/b")`, `CString::new("a\\b")`, `CString::new(".")`, `CString::new("..")`, and on Windows-like syntax `CString::new("C:evil")`. Assert `FALSE`, null, or `-1` according to the existing function's error convention. Also assert legitimate fixture IDs such as `default`, `luna_pinyin`, `luna_pinyin.schema`, `essay`, and `frontend_imported` still pass.

---

### `.planning/phases/02-native-abi-validation-and-runtime-safety/findings/*` (structured finding, batch/documentation)

**Analog:** Phase context decisions, no code analog.

**Scope pattern:** CONTEXT.md lines 21-25 require observed gaps outside Phase 2 to be recorded as structured findings, not fixed. If planner creates findings, include observed behavior, expected librime/frontend behavior when known, scope decision, and target future phase. Do not mix findings with source code changes.

## Shared Patterns

### C ABI boundary error handling
**Source:** `crates/yune-rime-api/src/config_api.rs`, `deployment.rs`, `userdb.rs`
**Apply to:** All exported `extern "C"` functions and loader-driven regression tests

Use `optional_c_string`/`c_string_key`, pointer null checks, `FALSE`/`TRUE`, `-1`, or null pointer returns. Do not `unwrap` on caller-controlled pointers or strings. Preserve `// SAFETY:` comments before unsafe dereferences.

### Process-global runtime guard
**Source:** `crates/yune-rime-api/src/tests/mod.rs` lines 92-102 and `tests/frontend_client.rs` lines 136-149
**Apply to:** Dynamic loader tests, lifecycle tests, session/deployment/notification tests

All tests that mutate process-global RIME state must serialize with `OnceLock<Mutex<()>>`, initialize through `RimeInitialize`/API table initialize, and clean sessions/runtime state before/after assertions.

### ABI table and layout validation
**Source:** `crates/yune-rime-api/src/api_table.rs` lines 57-64, 166-174; `abi.rs` lines 261-300
**Apply to:** Dynamic loader harness, ABI regression tests, any table additions

The function table is a leaked singleton behind `OnceLock`; `rime_get_api` is `#[no_mangle] extern "C"`. Validate table pointer non-null, required function pointers `Some`, and `data_size == size_of::<RimeApi>() - size_of::<c_int>()`.

### Notification callback ordering
**Source:** `crates/yune-rime-api/src/notifications.rs` lines 30-52 and `frontend_client.rs` lines 995-1038
**Apply to:** Deployment/schema/session lifecycle tests

Capture callback tuples in a mutex-backed vector and assert exact ordered sequences. Keep callback invocation outside the notification-state lock.

### Logical resource-ID validation before joins
**Source:** Current joins in `lib.rs` lines 1574-1613, `deployment.rs` lines 730-769/873-889/975-1006, `schema_install.rs` lines 390-412, `levers.rs` lines 606-613, `userdb.rs` lines 250-256
**Apply to:** Config IDs, schema IDs, dictionary IDs, custom config IDs, userdb dict names, deployment resource filenames

Validate logical IDs before appending controlled suffixes and before `Path::join`. Reject traversal, absolute paths, separators, drive-like prefixes, empty, `.`, `..`, and NUL-derived filesystem syntax. Do not validate intentionally path-valued API parameters as logical IDs.

## No Analog Found

All planned files have at least a partial codebase analog. The dynamic-loader test has no existing `libloading` analog in the repository, so use `frontend_client.rs` for ABI-table/lifecycle behavior and the Phase 2 research recommendation for `libloading` mechanics.

| File | Role | Data Flow | Reason |
|------|------|-----------|--------|
| `crates/yune-rime-api/tests/dynamic_loader.rs` | test | dynamic loading | No existing `libloading` test; closest analog is direct-linked `frontend_client.rs`. |
| `.planning/phases/02-native-abi-validation-and-runtime-safety/findings/*` | structured finding | batch/documentation | No existing finding file pattern in phase 2; context specifies required fields only. |

## Metadata

**Project instructions:** No `/Users/trenton/Projects/yune/CLAUDE.md` was present. No `.claude/skills` or `.agents/skills` directories with skills were present in the main workspace.
**Analog search scope:** `/Users/trenton/Projects/yune/crates/yune-rime-api`, especially `src/{abi,api_table,runtime,deployment,notifications,session,config_api,schema_install,levers,userdb}.rs`, `src/tests/*`, and `tests/frontend_client.rs`.
**Files scanned:** 20+
**Pattern extraction date:** 2026-04-29
