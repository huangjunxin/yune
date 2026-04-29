use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::{Mutex, MutexGuard, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};

use yune_rime_api::{rime_get_api, RimeTraits};

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

fn unique_temp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "yune-cli-frontend-surrogate-{label}-{}-{nanos}",
        std::process::id()
    ))
}

fn cli_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_yune-cli"))
}

fn run_cli(args: &[&str]) -> std::process::Output {
    Command::new(cli_binary())
        .args(args)
        .output()
        .expect("yune-cli binary should run")
}

fn write_runtime(root: &Path) -> (PathBuf, PathBuf) {
    let shared = root.join("shared");
    let user = root.join("user");
    let staging = user.join("build");
    fs::create_dir_all(&shared).expect("shared dir should be created");
    fs::create_dir_all(&staging).expect("staging dir should be created");
    fs::write(
        shared.join("default.yaml"),
        "config_version: test\nschema_list:\n  - schema: default\n",
    )
    .expect("default config should be written");
    fs::write(
        shared.join("default.schema.yaml"),
        "schema:\n  schema_id: default\n  name: Default\nmenu:\n  page_size: 5\n",
    )
    .expect("schema config should be written");
    (shared, user)
}

#[test]
fn frontend_command_rejects_missing_runtime_paths_before_abi_calls() {
    let _guard = test_guard();
    let output = run_cli(&[
        "frontend",
        "--schema",
        "default",
        "--sequence",
        "ni",
        "--user-data-dir",
        "user",
    ]);

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "error: missing --shared-data-dir. next: pass --shared-data-dir <path>.\n"
    );

    let api = unsafe { &mut *rime_get_api() };
    let create_session = api
        .create_session
        .expect("frontend requires create_session for validation check");
    let find_session = api
        .find_session
        .expect("frontend requires find_session for validation check");
    let cleanup_all_sessions = api
        .cleanup_all_sessions
        .expect("frontend requires cleanup_all_sessions for validation check");
    let session_id = create_session();
    assert_ne!(session_id, 0);
    assert_eq!(find_session(session_id), 1);
    cleanup_all_sessions();
}

#[test]
fn frontend_command_uses_explicit_runtime_schema_and_per_key_abi_events() {
    let _guard = test_guard();
    let root = unique_temp_dir("run");
    let (shared, user) = write_runtime(&root);

    let output = run_cli(&[
        "frontend",
        "--shared-data-dir",
        shared.to_str().expect("shared path should be UTF-8"),
        "--user-data-dir",
        user.to_str().expect("user path should be UTF-8"),
        "--schema",
        "default",
        "--sequence",
        "ni ",
    ]);

    assert!(
        output.status.success(),
        "frontend stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stderr).is_empty());
    let json = String::from_utf8(output.stdout).expect("frontend output should be UTF-8");
    assert!(json.contains("\"schema_id\": \"default\""));
    assert!(json.contains("\"sequence\": \"ni \""));
    assert!(json.contains("\"events\": ["));
    assert!(json.contains("\"key\": \"n\""));
    assert!(json.contains("\"key\": \"i\""));
    assert!(json.contains("\"key\": \"space\""));
    assert!(json.contains("\"handled\": true"));
    assert!(json.contains("\"commits\": [\"ni\"]"));
    assert!(json.contains("\"input\": \"ni\""));
    assert!(json.contains("\"preedit\": \"ni\""));
    assert!(json.contains("\"schema_name\": \"Default\""));

    fs::remove_dir_all(root).expect("temp dirs should be removed");
}

#[test]
fn frontend_command_can_render_human_transcript() {
    let _guard = test_guard();
    let root = unique_temp_dir("human");
    let (shared, user) = write_runtime(&root);

    let output = run_cli(&[
        "frontend",
        "--shared-data-dir",
        shared.to_str().expect("shared path should be UTF-8"),
        "--user-data-dir",
        user.to_str().expect("user path should be UTF-8"),
        "--schema",
        "default",
        "--sequence",
        "ni",
        "--output",
        "human",
    ]);

    assert!(
        output.status.success(),
        "frontend stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stderr).is_empty());
    let text = String::from_utf8(output.stdout).expect("frontend output should be UTF-8");
    assert!(text.contains("event: 0\n"));
    assert!(text.contains("key: n\n"));
    assert!(text.contains("handled: true\n"));
    assert!(text.contains("preedit: n\n"));
    assert!(text.contains("caret: 1\n"));
    assert!(text.contains("highlighted: 0\n"));
    assert!(text.contains("status: schema_id=default schema_name=Default"));
    assert!(!text.contains("{\n"));
    assert!(!text.contains(shared.to_str().expect("shared path should be UTF-8")));
    assert!(!text.contains(user.to_str().expect("user path should be UTF-8")));
    assert!(!text.contains('\u{1b}'));

    fs::remove_dir_all(root).expect("temp dirs should be removed");
}

#[test]
fn frontend_json_omits_environment_dependent_values() {
    let _guard = test_guard();
    let root = unique_temp_dir("determinism");
    let (shared, user) = write_runtime(&root);

    let output = run_cli(&[
        "frontend",
        "--shared-data-dir",
        shared.to_str().expect("shared path should be UTF-8"),
        "--user-data-dir",
        user.to_str().expect("user path should be UTF-8"),
        "--schema",
        "default",
        "--sequence",
        "ni",
    ]);

    assert!(
        output.status.success(),
        "frontend stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json = String::from_utf8(output.stdout).expect("frontend output should be UTF-8");
    assert!(!json.contains(shared.to_str().expect("shared path should be UTF-8")));
    assert!(!json.contains(user.to_str().expect("user path should be UTF-8")));
    assert!(!json.contains(&std::process::id().to_string()));
    assert!(!json.contains("timestamp"));
    assert!(!json.contains("duration"));
    assert!(!json.contains("0x"));
    assert!(!json.contains("CARGO"));
    assert!(!json.contains("HOME"));

    fs::remove_dir_all(root).expect("temp dirs should be removed");
}

#[test]
fn frontend_check_replays_expected_fixture_through_abi_transcript() {
    let _guard = test_guard();
    let root = unique_temp_dir("fixture");
    let (shared, user) = write_runtime(&root);

    let run = run_cli(&[
        "frontend",
        "--shared-data-dir",
        shared.to_str().expect("shared path should be UTF-8"),
        "--user-data-dir",
        user.to_str().expect("user path should be UTF-8"),
        "--schema",
        "default",
        "--sequence",
        "ni",
    ]);
    assert!(
        run.status.success(),
        "frontend stderr: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    let fixture = root.join("frontend.json");
    fs::write(&fixture, run.stdout).expect("frontend fixture should be written");

    let check = run_cli(&[
        "frontend-check",
        fixture.to_str().expect("fixture path should be UTF-8"),
        "--shared-data-dir",
        shared.to_str().expect("shared path should be UTF-8"),
        "--user-data-dir",
        user.to_str().expect("user path should be UTF-8"),
    ]);

    assert!(
        check.status.success(),
        "frontend-check stderr: {}",
        String::from_utf8_lossy(&check.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&check.stdout),
        format!("ok {}\n", fixture.display())
    );
    assert!(String::from_utf8_lossy(&check.stderr).is_empty());

    fs::remove_dir_all(root).expect("temp dirs should be removed");
}
