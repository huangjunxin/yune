use std::{
    ffi::{CStr, CString},
    fs,
    path::PathBuf,
    ptr,
    sync::{Mutex, MutexGuard, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::Value;
use yune_rime_api::{
    rime_get_api, yune_typeduck_cleanup, yune_typeduck_customize, yune_typeduck_delete_candidate,
    yune_typeduck_deploy, yune_typeduck_flip_page, yune_typeduck_free_response, yune_typeduck_init,
    yune_typeduck_process_key, yune_typeduck_response_handled, yune_typeduck_response_json,
    yune_typeduck_select_candidate, Bool, FALSE, TRUE,
};

const SCHEMA_ID: &str = "typeduck_luna";

fn test_guard() -> MutexGuard<'static, ()> {
    static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    TEST_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[test]
fn typeduck_adapter_processes_keys_and_returns_json_state() {
    let _guard = test_guard();
    let runtime = TypeDuckRuntime::create("process-json-state");
    runtime.write_schema();

    let state = unsafe {
        yune_typeduck_init(
            runtime.shared_c.as_ptr(),
            runtime.user_c.as_ptr(),
            runtime.schema_id_c.as_ptr(),
        )
    };
    assert!(!state.is_null());

    let first = response_json(unsafe { yune_typeduck_process_key(state, 'b' as i32, 0) });
    assert_eq!(first["handled"], Value::Bool(true));
    assert_eq!(first["context"]["input"], Value::String("b".to_owned()));
    assert_eq!(
        first["status"]["schema_id"],
        Value::String(SCHEMA_ID.to_owned())
    );
    assert_eq!(first["status"]["is_composing"], Value::Bool(true));

    let second = response_json(unsafe { yune_typeduck_process_key(state, 'a' as i32, 0) });
    assert_eq!(second["handled"], Value::Bool(true));
    assert_eq!(second["context"]["input"], Value::String("ba".to_owned()));
    assert_eq!(second["context"]["page_size"], Value::from(2));
    assert_eq!(
        second["context"]["select_keys"],
        Value::String("AB".to_owned())
    );
    assert_eq!(
        second["context"]["select_labels"][0],
        Value::String("Alpha".to_owned())
    );
    assert_eq!(
        second["context"]["candidates"][0]["text"],
        Value::String("八".to_owned())
    );
    assert_eq!(
        second["context"]["candidates"][1]["text"],
        Value::String("吧".to_owned())
    );

    unsafe { yune_typeduck_cleanup(state) };
    runtime.remove();
}

#[test]
fn typeduck_adapter_supports_page_candidate_actions_and_commits() {
    let _guard = test_guard();
    let runtime = TypeDuckRuntime::create("candidate-actions");
    runtime.write_schema();
    let state = unsafe {
        yune_typeduck_init(
            runtime.shared_c.as_ptr(),
            runtime.user_c.as_ptr(),
            runtime.schema_id_c.as_ptr(),
        )
    };
    assert!(!state.is_null());

    drop(response_json(unsafe {
        yune_typeduck_process_key(state, 'b' as i32, 0)
    }));
    let composing = response_json(unsafe { yune_typeduck_process_key(state, 'a' as i32, 0) });
    assert_eq!(
        composing["context"]["candidates"][0]["text"],
        Value::String("八".to_owned())
    );

    let next_page = response_json(unsafe { yune_typeduck_flip_page(state, FALSE) });
    assert_eq!(next_page["handled"], Value::Bool(true));
    assert_eq!(next_page["context"]["page_no"], Value::from(1));
    assert_eq!(
        next_page["context"]["candidates"][0]["text"],
        Value::String("爸".to_owned())
    );

    let previous_page = response_json(unsafe { yune_typeduck_flip_page(state, TRUE) });
    assert_eq!(previous_page["handled"], Value::Bool(true));
    assert_eq!(previous_page["context"]["page_no"], Value::from(0));

    let deleted = response_json(unsafe { yune_typeduck_delete_candidate(state, 0) });
    assert_eq!(deleted["handled"], Value::Bool(true));
    assert_eq!(
        deleted["context"]["candidates"][0]["text"],
        Value::String("吧".to_owned())
    );

    let selected = response_json(unsafe { yune_typeduck_select_candidate(state, 0) });
    assert_eq!(selected["handled"], Value::Bool(true));
    assert_eq!(
        selected["commits"],
        Value::Array(vec![Value::String("吧".to_owned())])
    );
    assert_eq!(selected["status"]["is_composing"], Value::Bool(false));

    unsafe { yune_typeduck_cleanup(state) };
    runtime.remove();
}

#[test]
fn typeduck_adapter_documents_browser_host_layout_constraints() {
    let _guard = test_guard();
    let runtime = TypeDuckRuntime::create("browser-host-layout");

    assert!(runtime.shared.exists());
    assert!(runtime.user.exists());
    assert!(
        runtime.user.join("build").exists(),
        "browser host fixture must create user_data_dir/build before init"
    );

    let state_without_preloaded_assets = unsafe {
        yune_typeduck_init(
            runtime.shared_c.as_ptr(),
            runtime.user_c.as_ptr(),
            runtime.schema_id_c.as_ptr(),
        )
    };
    assert!(
        state_without_preloaded_assets.is_null(),
        "init without preloaded schema/dictionary assets must fail deterministically"
    );

    runtime.write_schema();
    let state = unsafe {
        yune_typeduck_init(
            runtime.shared_c.as_ptr(),
            runtime.user_c.as_ptr(),
            runtime.schema_id_c.as_ptr(),
        )
    };
    assert!(!state.is_null());

    let response = unsafe { yune_typeduck_process_key(state, 'b' as i32, 0) };
    let json = unsafe { yune_typeduck_response_json(response) };
    assert!(!json.is_null());
    let text = unsafe { CStr::from_ptr(json) }
        .to_str()
        .expect("adapter JSON should be valid UTF-8")
        .to_owned();
    unsafe { yune_typeduck_free_response(response) };
    let value: Value = serde_json::from_str(&text).expect("copied response should parse as JSON");
    assert_eq!(value["handled"], Value::Bool(true));

    unsafe { yune_typeduck_cleanup(state) };
    runtime.remove();
}

#[test]
fn typeduck_adapter_deploy_and_customize_are_explicit() {
    let _guard = test_guard();
    let runtime = TypeDuckRuntime::create("deploy-customize");
    runtime.write_schema();
    let state = unsafe {
        yune_typeduck_init(
            runtime.shared_c.as_ptr(),
            runtime.user_c.as_ptr(),
            runtime.schema_id_c.as_ptr(),
        )
    };
    assert!(!state.is_null());

    assert_eq!(unsafe { yune_typeduck_deploy(state) }, TRUE);
    let config_id = CString::new("typeduck_luna.schema").expect("config id should be valid");
    let key = CString::new("schema/name").expect("custom key should be valid");
    let value = CString::new("TypeDuck Luna Web").expect("custom value should be valid");
    assert_eq!(
        unsafe { yune_typeduck_customize(state, config_id.as_ptr(), key.as_ptr(), value.as_ptr()) },
        TRUE
    );
    let saved = fs::read_to_string(runtime.user.join("typeduck_luna.custom.yaml"))
        .expect("customized schema patch should be saved");
    assert!(saved.contains("schema/name"));
    assert!(saved.contains("TypeDuck Luna Web"));

    unsafe { yune_typeduck_cleanup(state) };
    runtime.remove();
}

#[test]
fn typeduck_adapter_handles_null_inputs_and_response_freeing() {
    let _guard = test_guard();
    assert!(unsafe { yune_typeduck_init(ptr::null(), ptr::null(), ptr::null()) }.is_null());
    assert_eq!(unsafe { yune_typeduck_deploy(ptr::null_mut()) }, FALSE);
    assert_eq!(
        unsafe { yune_typeduck_customize(ptr::null_mut(), ptr::null(), ptr::null(), ptr::null()) },
        FALSE
    );
    assert!(unsafe { yune_typeduck_process_key(ptr::null_mut(), 'a' as i32, 0) }.is_null());
    assert!(unsafe { yune_typeduck_response_json(ptr::null()) }.is_null());
    assert_eq!(
        unsafe { yune_typeduck_response_handled(ptr::null()) },
        FALSE
    );
    unsafe { yune_typeduck_free_response(ptr::null_mut()) };
}

fn response_json(response: *mut yune_rime_api::YuneTypeDuckResponse) -> Value {
    assert!(!response.is_null());
    let handled: Bool = unsafe { yune_typeduck_response_handled(response) };
    let json = unsafe { yune_typeduck_response_json(response) };
    assert!(!json.is_null());
    let text = unsafe { CStr::from_ptr(json) }
        .to_str()
        .expect("adapter JSON should be valid UTF-8")
        .to_owned();
    unsafe { yune_typeduck_free_response(response) };
    let value: Value = serde_json::from_str(&text).expect("adapter response should parse as JSON");
    assert_eq!(value["handled"].as_bool(), Some(handled == TRUE));
    value
}

struct TypeDuckRuntime {
    root: PathBuf,
    shared: PathBuf,
    user: PathBuf,
    shared_c: CString,
    user_c: CString,
    schema_id_c: CString,
}

impl TypeDuckRuntime {
    fn create(label: &str) -> Self {
        let root = unique_temp_dir(label);
        let shared = root.join("shared");
        let user = root.join("user");
        fs::create_dir_all(&shared).expect("shared dir should be created");
        fs::create_dir_all(user.join("build")).expect("staging dir should be created");
        let shared_c =
            CString::new(shared.to_string_lossy().as_ref()).expect("path should be valid");
        let user_c = CString::new(user.to_string_lossy().as_ref()).expect("path should be valid");
        let schema_id_c = CString::new(SCHEMA_ID).expect("schema id should be valid");
        Self {
            root,
            shared,
            user,
            shared_c,
            user_c,
            schema_id_c,
        }
    }

    fn write_schema(&self) {
        let default_config =
            "config_version: typeduck-web\nschema_list:\n  - schema: typeduck_luna\n";
        let schema_config = "\
schema:\n  schema_id: typeduck_luna\n  name: TypeDuck Luna\nmenu:\n  page_size: 2\n  alternative_select_keys: AB\n  alternative_select_labels: [Alpha, Beta]\nswitches:\n  - name: ascii_mode\n    reset: 0\nengine:\n  translators:\n    - table_translator\ntranslator:\n  dictionary: typeduck\n";
        let staging = self.user.join("build");
        fs::write(staging.join("default.yaml"), default_config)
            .expect("staging default config should be written");
        fs::write(staging.join("typeduck_luna.schema.yaml"), schema_config)
            .expect("staging schema config should be written");
        fs::write(self.shared.join("default.yaml"), default_config)
            .expect("shared default config should be written");
        fs::write(self.shared.join("typeduck_luna.schema.yaml"), schema_config)
            .expect("shared schema config should be written");
        fs::write(
            self.shared.join("typeduck.dict.yaml"),
            "\
---\nname: typeduck\nversion: '1'\nsort: original\ncolumns: [code, text, weight]\n...\nba\t八\t10\nba\t吧\t9\nba\t爸\t8\nba\t巴\t7\nba\t把\t6\nba\t拔\t5\n",
        )
        .expect("dictionary should be written");
    }

    fn remove(self) {
        reset_rime();
        fs::remove_dir_all(self.root).expect("temp dir should be removed");
    }
}

fn reset_rime() {
    let api = rime_get_api();
    if api.is_null() {
        return;
    }
    let api = unsafe { &*api };
    if let Some(cleanup_all_sessions) = api.cleanup_all_sessions {
        cleanup_all_sessions();
    }
    if let Some(finalize) = api.finalize {
        finalize();
    }
}

fn unique_temp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "yune-rime-api-typeduck-web-{label}-{}-{nanos}",
        std::process::id()
    ))
}
