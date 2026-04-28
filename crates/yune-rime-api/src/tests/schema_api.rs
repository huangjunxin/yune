use super::*;

#[test]
fn gets_and_frees_available_schema_list() {
    let _guard = test_guard();
    RimeCleanupAllSessions();
    let root = unique_temp_dir("schema-list");
    let shared = root.join("shared");
    let user = root.join("user");
    let prebuilt = shared.join("build");
    let staging = user.join("build");
    fs::create_dir_all(&prebuilt).expect("prebuilt dir should be created");
    fs::create_dir_all(&staging).expect("staging dir should be created");
    fs::write(
        prebuilt.join("default.yaml"),
        "\
schema_list:
  - schema: prebuilt_only
",
    )
    .expect("prebuilt default config should be written");
    fs::write(
        staging.join("default.yaml"),
        "\
schema_list:
  - schema: luna_pinyin
  - schema: cangjie5
    case: [conditions/include_cangjie]
  - schema: hidden
    case: [conditions/include_hidden]
  - schema: missing_name
  - not_schema: ignored
conditions:
  include_cangjie: true
  include_hidden: false
",
    )
    .expect("staging default config should be written");
    fs::write(
        staging.join("luna_pinyin.schema.yaml"),
        "schema:\n  schema_id: luna_pinyin\n  name: Luna Pinyin\n",
    )
    .expect("luna schema config should be written");
    fs::write(
        prebuilt.join("cangjie5.schema.yaml"),
        "schema:\n  schema_id: cangjie5\n  name: Cangjie 5\n",
    )
    .expect("cangjie schema config should be written");

    let shared_c = CString::new(shared.to_string_lossy().as_ref()).expect("path is valid");
    let user_c = CString::new(user.to_string_lossy().as_ref()).expect("path is valid");
    let mut traits = empty_traits();
    traits.shared_data_dir = shared_c.as_ptr();
    traits.user_data_dir = user_c.as_ptr();
    // SAFETY: traits points to valid storage and strings live for the call.
    unsafe { RimeSetup(&traits) };

    let mut schema_list = empty_schema_list();

    // SAFETY: schema_list points to valid writable storage.
    assert_eq!(unsafe { RimeGetSchemaList(&mut schema_list) }, TRUE);
    assert_eq!(schema_list.size, 4);
    assert!(!schema_list.list.is_null());

    let mut actual = Vec::new();
    for index in 0..schema_list.size {
        // SAFETY: `RimeGetSchemaList` returned true and populated `size`
        // initialized schema-list items.
        let item = unsafe { *schema_list.list.add(index) };
        // SAFETY: schema strings are valid NUL-terminated strings owned by
        // the schema-list object.
        let schema_id = unsafe { CStr::from_ptr(item.schema_id) };
        // SAFETY: schema strings are valid NUL-terminated strings owned by
        // the schema-list object.
        let name = unsafe { CStr::from_ptr(item.name) };
        actual.push((
            schema_id.to_string_lossy().into_owned(),
            name.to_string_lossy().into_owned(),
        ));
        assert!(item.reserved.is_null());
    }
    assert_eq!(
        actual,
        vec![
            ("luna_pinyin".to_owned(), "Luna Pinyin".to_owned()),
            ("cangjie5".to_owned(), "Cangjie 5".to_owned()),
            ("hidden".to_owned(), "hidden".to_owned()),
            ("missing_name".to_owned(), "missing_name".to_owned()),
        ]
    );

    // SAFETY: nested pointers were allocated by `RimeGetSchemaList` above.
    unsafe { crate::RimeFreeSchemaList(&mut schema_list) };
    assert_eq!(schema_list.size, 0);
    assert!(schema_list.list.is_null());

    // SAFETY: null pointers are explicitly rejected/no-oped.
    assert_eq!(unsafe { RimeGetSchemaList(std::ptr::null_mut()) }, FALSE);
    unsafe { crate::RimeFreeSchemaList(std::ptr::null_mut()) };

    let reset_traits = empty_traits();
    // SAFETY: reset traits points to valid storage.
    unsafe { RimeSetup(&reset_traits) };
    fs::remove_dir_all(root).expect("temp dirs should be removed");
}

#[test]
fn schema_list_returns_false_when_default_config_has_no_schema_list() {
    let _guard = test_guard();
    let root = unique_temp_dir("schema-list-empty");
    let shared = root.join("shared");
    let user = root.join("user");
    let staging = user.join("build");
    fs::create_dir_all(&shared).expect("shared dir should be created");
    fs::create_dir_all(&staging).expect("staging dir should be created");
    fs::write(staging.join("default.yaml"), "config_version: test\n")
        .expect("default config should be written");

    let shared_c = CString::new(shared.to_string_lossy().as_ref()).expect("path is valid");
    let user_c = CString::new(user.to_string_lossy().as_ref()).expect("path is valid");
    let mut traits = empty_traits();
    traits.shared_data_dir = shared_c.as_ptr();
    traits.user_data_dir = user_c.as_ptr();
    // SAFETY: traits points to valid storage and strings live for the call.
    unsafe { RimeSetup(&traits) };

    let mut schema_list = empty_schema_list();
    // SAFETY: schema_list points to valid writable storage.
    assert_eq!(unsafe { RimeGetSchemaList(&mut schema_list) }, FALSE);
    assert_eq!(schema_list.size, 0);
    assert!(schema_list.list.is_null());

    let reset_traits = empty_traits();
    // SAFETY: reset traits points to valid storage.
    unsafe { RimeSetup(&reset_traits) };
    fs::remove_dir_all(root).expect("temp dirs should be removed");
}
