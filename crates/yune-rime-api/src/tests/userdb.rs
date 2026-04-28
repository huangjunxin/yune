use super::*;

#[test]
fn sync_user_data_merges_plain_userdb_snapshots_and_backs_up_current_state() {
    let _guard = test_guard();
    let root = unique_temp_dir("rime-sync-user-data");
    let user = root.join("user");
    let peer_sync = user.join("sync").join("peer");
    fs::create_dir_all(&user).expect("user dir should be created");
    fs::create_dir_all(&peer_sync).expect("peer sync dir should be created");
    struct CurrentDirGuard(PathBuf);
    impl Drop for CurrentDirGuard {
        fn drop(&mut self) {
            let _ = env::set_current_dir(&self.0);
        }
    }
    let current_dir_guard =
        CurrentDirGuard(env::current_dir().expect("current dir should be available"));
    env::set_current_dir(&root).expect("test cwd should move under temp root");

    fs::write(user.join("luna_pinyin.userdb"), "ni hao\t你好\t1\n")
        .expect("local user dict should be written");
    fs::write(user.join("default.yaml"), "config_version: '1.0'\n")
        .expect("user config should be written");
    fs::write(user.join("notes.txt"), "local notes\n").expect("text file should be written");
    fs::write(
        user.join("generated.yaml"),
        "customization: abc123\nschema:\n  name: Generated\n",
    )
    .expect("generated customized copy should be written");
    fs::write(
        peer_sync.join("luna_pinyin.userdb.txt"),
        "ni hao\t你好\t1\nzhong guo\t中国\t2\n",
    )
    .expect("peer snapshot should be written");

    let user_c = CString::new(user.to_string_lossy().as_ref()).expect("path is valid");
    let mut traits = empty_traits();
    traits.user_data_dir = user_c.as_ptr();
    // SAFETY: traits points to valid storage and strings live for the call.
    unsafe { RimeSetup(&traits) };

    let session_id = RimeCreateSession();
    assert_eq!(RimeFindSession(session_id), TRUE);
    assert_eq!(RimeSyncUserData(), TRUE);
    assert_eq!(RimeFindSession(session_id), FALSE);

    let merged =
        fs::read_to_string(user.join("luna_pinyin.userdb")).expect("dict should be readable");
    assert_eq!(merged, "ni hao\t你好\t1\nzhong guo\t中国\t2\n");

    let installation_metadata = fs::read_to_string(user.join("installation.yaml"))
        .expect("installation metadata should be written during sync");
    let installation_metadata: Value =
        serde_yaml::from_str(&installation_metadata).expect("installation metadata should parse");
    let installation_id = find_config_value(&installation_metadata, "installation_id")
        .and_then(Value::as_str)
        .expect("installation id should be available");
    let sync_user_dir = user.join("sync").join(installation_id);
    let backup = fs::read_to_string(sync_user_dir.join("luna_pinyin.userdb.txt"))
        .expect("current user snapshot should be written");
    assert_eq!(backup, merged);

    assert_eq!(
        fs::read_to_string(sync_user_dir.join("default.yaml"))
            .expect("user config backup should be readable"),
        "config_version: '1.0'\n"
    );
    assert_eq!(
        fs::read_to_string(sync_user_dir.join("notes.txt"))
            .expect("text backup should be readable"),
        "local notes\n"
    );
    assert!(!sync_user_dir.join("generated.yaml").exists());

    let task_name = CString::new("user_dict_sync").expect("task name should be valid");
    assert_eq!(RimeRunTask(task_name.as_ptr()), TRUE);
    fs::remove_file(sync_user_dir.join("default.yaml")).expect("config backup should be removable");
    let backup_config_task =
        CString::new("backup_config_files").expect("task name should be valid");
    assert_eq!(RimeRunTask(backup_config_task.as_ptr()), TRUE);
    assert_eq!(
        fs::read_to_string(sync_user_dir.join("default.yaml"))
            .expect("task should recreate config backup"),
        "config_version: '1.0'\n"
    );

    let reset_traits = empty_traits();
    // SAFETY: reset traits points to valid storage.
    unsafe { RimeSetup(&reset_traits) };
    drop(current_dir_guard);
    fs::remove_dir_all(root).expect("temp dirs should be removed");
}
