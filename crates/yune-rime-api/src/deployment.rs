use std::{
    collections::HashSet,
    fs,
    os::raw::c_char,
    path::{Path, PathBuf},
    sync::atomic::Ordering,
    time::{SystemTime, UNIX_EPOCH},
};

use serde_yaml::Value;

use crate::{
    bool_from, cstring_from_lossless_str, deploy_config_file, deploy_schema_file,
    find_config_value, optional_c_string, path_join, prebuild_all_schemas,
    run_workspace_maintenance_tasks, runtime_paths, runtime_user_data_sync_dir, service_started,
    sync_all_user_dicts, user_dict_upgrade, workspace_update, Bool, RimeCleanupAllSessions,
    RimeSetup, RimeTraits, FALSE, RIME_VERSION_BYTES, TRUE,
};

/// Initializes the runtime using the same trait handling as `RimeSetup`.
///
/// # Safety
///
/// `traits` follows the same preconditions as `RimeSetup`.
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
pub extern "C" fn RimeStartMaintenanceOnWorkspaceChange() -> Bool {
    RimeStartMaintenance(FALSE)
}

#[no_mangle]
pub extern "C" fn RimeIsMaintenancing() -> Bool {
    FALSE
}

#[no_mangle]
pub extern "C" fn RimeJoinMaintenanceThread() {}

/// Initializes deployer state using the same trait handling as `RimeSetup`.
///
/// # Safety
///
/// `traits` follows the same preconditions as `RimeSetup`.
#[no_mangle]
pub unsafe extern "C" fn RimeDeployerInitialize(traits: *const RimeTraits) {
    // SAFETY: forwarded preconditions are identical to `RimeSetup`.
    unsafe { RimeSetup(traits) };
}

#[no_mangle]
pub extern "C" fn RimePrebuildAllSchemas() -> Bool {
    bool_from(prebuild_all_schemas())
}

#[no_mangle]
pub extern "C" fn RimeDeployWorkspace() -> Bool {
    if !run_installation_update() {
        return FALSE;
    }
    if !run_workspace_maintenance_tasks() {
        return FALSE;
    }
    TRUE
}

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

pub(crate) fn run_installation_update() -> bool {
    let (
        user_data_dir,
        current_sync_dir,
        distribution_name,
        distribution_code_name,
        distribution_version,
    ) = {
        let paths = runtime_paths()
            .lock()
            .expect("runtime paths should not be poisoned");
        (
            PathBuf::from(paths.user_data_dir.to_string_lossy().into_owned()),
            paths.sync_dir.to_string_lossy().into_owned(),
            paths.distribution_name.to_string_lossy().into_owned(),
            paths.distribution_code_name.to_string_lossy().into_owned(),
            paths.distribution_version.to_string_lossy().into_owned(),
        )
    };

    if fs::create_dir_all(&user_data_dir).is_err() {
        return false;
    }

    let installation_path = user_data_dir.join("installation.yaml");
    let mut root = fs::read_to_string(&installation_path)
        .ok()
        .and_then(|text| serde_yaml::from_str::<Value>(&text).ok())
        .and_then(|value| match value {
            Value::Mapping(root) => Some(root),
            _ => None,
        })
        .unwrap_or_default();

    let installation_key = Value::String("installation_id".to_owned());
    let sync_key = Value::String("sync_dir".to_owned());
    let backup_key = Value::String("backup_config_files".to_owned());

    let existing_installation_id = root
        .get(&installation_key)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned);
    let installation_id = existing_installation_id.unwrap_or_else(|| {
        let generated = generate_installation_id();
        root.insert(installation_key.clone(), Value::String(generated.clone()));
        root.insert(
            Value::String("install_time".to_owned()),
            Value::String(current_unix_time_string()),
        );
        generated
    });

    let sync_dir = root
        .get(&sync_key)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| {
            if current_sync_dir != "sync" {
                current_sync_dir
            } else {
                user_data_dir.join("sync").to_string_lossy().into_owned()
            }
        });
    let backup_config_files = root
        .get(&backup_key)
        .and_then(Value::as_bool)
        .unwrap_or(true);

    if root.contains_key(Value::String("install_time".to_owned())) {
        root.insert(
            Value::String("update_time".to_owned()),
            Value::String(current_unix_time_string()),
        );
    }
    if !distribution_name.is_empty() {
        root.insert(
            Value::String("distribution_name".to_owned()),
            Value::String(distribution_name.clone()),
        );
    }
    if !distribution_code_name.is_empty() {
        root.insert(
            Value::String("distribution_code_name".to_owned()),
            Value::String(distribution_code_name.clone()),
        );
    }
    if !distribution_version.is_empty() {
        root.insert(
            Value::String("distribution_version".to_owned()),
            Value::String(distribution_version.clone()),
        );
    }
    root.insert(
        Value::String("rime_version".to_owned()),
        Value::String(
            String::from_utf8_lossy(&RIME_VERSION_BYTES[..RIME_VERSION_BYTES.len() - 1])
                .into_owned(),
        ),
    );

    let yaml = match serde_yaml::to_string(&Value::Mapping(root)) {
        Ok(yaml) => yaml,
        Err(_) => return false,
    };
    if fs::write(&installation_path, yaml).is_err() {
        return false;
    }

    let user_data_sync_dir = path_join(&sync_dir, &installation_id);
    let mut paths = runtime_paths()
        .lock()
        .expect("runtime paths should not be poisoned");
    paths.user_id = cstring_from_lossless_str(&installation_id);
    paths.sync_dir = cstring_from_lossless_str(&sync_dir);
    paths.user_data_sync_dir = cstring_from_lossless_str(&user_data_sync_dir);
    paths.backup_config_files = backup_config_files;
    true
}

pub(crate) fn backup_config_files() -> bool {
    let (user_data_dir, backup_enabled) = {
        let paths = runtime_paths()
            .lock()
            .expect("runtime paths should not be poisoned");
        (
            PathBuf::from(paths.user_data_dir.to_string_lossy().into_owned()),
            paths.backup_config_files,
        )
    };
    if !backup_enabled {
        return true;
    }

    let Ok(entries) = fs::read_dir(&user_data_dir) else {
        return false;
    };

    let backup_dir = runtime_user_data_sync_dir();
    let mut success = true;
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_file() || !should_backup_config_file(&path) {
            continue;
        }
        if fs::create_dir_all(&backup_dir).is_err() {
            return false;
        }
        let destination = backup_dir.join(entry.file_name());
        if fs::copy(&path, destination).is_err() {
            success = false;
        }
    }
    success
}

pub(crate) fn cleanup_trash() -> bool {
    let user_data_dir = {
        let paths = runtime_paths()
            .lock()
            .expect("runtime paths should not be poisoned");
        PathBuf::from(paths.user_data_dir.to_string_lossy().into_owned())
    };
    let Ok(entries) = fs::read_dir(&user_data_dir) else {
        return false;
    };

    let trash_dir = user_data_dir.join("trash");
    let mut success = true;
    let mut trash_created = false;
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_file() || !should_cleanup_trash_file(&path) {
            continue;
        }
        if !trash_created {
            if fs::create_dir_all(&trash_dir).is_err() {
                return false;
            }
            trash_created = true;
        }
        if fs::rename(&path, trash_dir.join(entry.file_name())).is_err() {
            success = false;
        }
    }
    success
}

pub(crate) fn clean_old_log_files() -> bool {
    let (app_name, log_dir) = {
        let paths = runtime_paths()
            .lock()
            .expect("runtime paths should not be poisoned");
        (
            paths.app_name.to_string_lossy().into_owned(),
            PathBuf::from(paths.log_dir.to_string_lossy().into_owned()),
        )
    };
    if app_name.is_empty() || log_dir.as_os_str().is_empty() {
        return true;
    }

    let Ok(entries) = fs::read_dir(&log_dir) else {
        return true;
    };
    let entries: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();
    let mut files_in_use = HashSet::new();
    for path in &entries {
        let Ok(metadata) = fs::symlink_metadata(path) else {
            continue;
        };
        if !metadata.file_type().is_symlink() {
            continue;
        }
        let Ok(target) = fs::read_link(path) else {
            continue;
        };
        let Some(target_file_name) = target.file_name().and_then(|file_name| file_name.to_str())
        else {
            continue;
        };
        if target_file_name.starts_with(&app_name) && target_file_name.ends_with(".log") {
            files_in_use.insert(target_file_name.to_owned());
        }
    }

    let today = current_log_date_marker();
    let mut success = true;
    for path in entries {
        let Ok(metadata) = fs::symlink_metadata(&path) else {
            success = false;
            continue;
        };
        if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
            continue;
        }
        let Some(file_name) = path.file_name().and_then(|file_name| file_name.to_str()) else {
            continue;
        };
        if !file_name.starts_with(&app_name) || !file_name.ends_with(".log") {
            continue;
        }
        if file_name.contains(&today) {
            continue;
        }
        if files_in_use.contains(file_name) {
            continue;
        }
        if fs::remove_file(&path).is_err() {
            success = false;
        }
    }
    success
}

pub(crate) fn detect_modifications() -> bool {
    let (shared_data_dir, user_data_dir) = {
        let paths = runtime_paths()
            .lock()
            .expect("runtime paths should not be poisoned");
        (
            PathBuf::from(paths.shared_data_dir.to_string_lossy().into_owned()),
            PathBuf::from(paths.user_data_dir.to_string_lossy().into_owned()),
        )
    };

    let Some(last_modified) =
        latest_workspace_modified_time([user_data_dir.as_path(), shared_data_dir.as_path()])
    else {
        return true;
    };
    last_modified > user_last_build_time(&user_data_dir)
}

fn latest_workspace_modified_time<const N: usize>(data_dirs: [&Path; N]) -> Option<u64> {
    let mut last_modified = 0;
    for data_dir in data_dirs {
        last_modified = last_modified.max(file_modified_secs(data_dir)?);
        if data_dir.is_dir() {
            let entries = fs::read_dir(data_dir).ok()?;
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if !path.is_file() || !is_workspace_yaml_file(&path) {
                    continue;
                }
                last_modified = last_modified.max(file_modified_secs(&path)?);
            }
        }
    }
    Some(last_modified)
}

fn file_modified_secs(path: &Path) -> Option<u64> {
    fs::metadata(path)
        .ok()?
        .modified()
        .ok()?
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|duration| duration.as_secs())
}

fn user_last_build_time(user_data_dir: &Path) -> u64 {
    let user_config_path = user_data_dir.join("user.yaml");
    fs::read_to_string(user_config_path)
        .ok()
        .and_then(|text| serde_yaml::from_str::<Value>(&text).ok())
        .and_then(|root| {
            find_config_value(&root, "var/last_build_time")
                .and_then(Value::as_i64)
                .and_then(|value| u64::try_from(value).ok())
        })
        .unwrap_or(0)
}

fn is_workspace_yaml_file(path: &Path) -> bool {
    path.extension().and_then(|extension| extension.to_str()) == Some("yaml")
        && path.file_name().and_then(|file_name| file_name.to_str()) != Some("user.yaml")
}

pub(crate) fn current_log_date_marker() -> String {
    let days_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs() / 86_400);
    let (year, month, day) = civil_from_days(days_since_epoch as i64);
    format!(".{year:04}{month:02}{day:02}")
}

fn civil_from_days(days_since_epoch: i64) -> (i64, i64, i64) {
    let days = days_since_epoch + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    (year, month, day)
}

fn should_cleanup_trash_file(path: &Path) -> bool {
    let Some(file_name) = path.file_name().and_then(|file_name| file_name.to_str()) else {
        return false;
    };
    file_name == "rime.log"
        || file_name.ends_with(".bin")
        || file_name.ends_with(".reverse.kct")
        || file_name.ends_with(".userdb.kct.old")
        || file_name.ends_with(".userdb.kct.snapshot")
}

fn should_backup_config_file(path: &Path) -> bool {
    let extension = path.extension().and_then(|extension| extension.to_str());
    match extension {
        Some("txt") => true,
        Some("yaml") => !is_generated_customized_copy(path),
        _ => false,
    }
}

fn is_generated_customized_copy(path: &Path) -> bool {
    let Some(file_name) = path.file_name().and_then(|file_name| file_name.to_str()) else {
        return false;
    };
    if file_name.ends_with(".custom.yaml") {
        return false;
    }
    let Ok(text) = fs::read_to_string(path) else {
        return false;
    };
    let Ok(Value::Mapping(root)) = serde_yaml::from_str::<Value>(&text) else {
        return false;
    };
    matches!(
        root.get(Value::String("customization".to_owned())),
        Some(Value::String(_))
    )
}

fn generate_installation_id() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    format!("yune-{nanos}-{}", std::process::id())
}

fn current_unix_time_string() -> String {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or_else(
        |_| "0".to_owned(),
        |duration| duration.as_secs().to_string(),
    )
}
