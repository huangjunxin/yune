use std::{
    ffi::{CStr, CString},
    mem,
    os::raw::c_int,
    path::PathBuf,
    ptr,
};

use yune_core::{parse_key_sequence, KeyCode, KeyEvent};
use yune_rime_api::{
    rime_get_api, RimeCandidate, RimeCommit, RimeComposition, RimeContext, RimeMenu, RimeSessionId,
    RimeStatus, RimeTraits, FALSE, TRUE,
};

use crate::transcript::FrontendTranscript;

// Owns the CLI surrogate's ABI lifecycle comparison target:
// librime frontend lifecycle: setup/initialize/deploy/select/create-session/process-key/read-state/destroy/finalize.
// librime remains the external behavior oracle; native frontend validation belongs to Phase 2.
const XK_BACKSPACE: c_int = 0xff08;
const XK_TAB: c_int = 0xff09;
const XK_ESCAPE: c_int = 0xff1b;
const XK_RETURN: c_int = 0xff0d;
const XK_DELETE: c_int = 0xffff;
const XK_HOME: c_int = 0xff50;
const XK_LEFT: c_int = 0xff51;
const XK_UP: c_int = 0xff52;
const XK_RIGHT: c_int = 0xff53;
const XK_DOWN: c_int = 0xff54;
const XK_PAGE_UP: c_int = 0xff55;
const XK_PAGE_DOWN: c_int = 0xff56;
const XK_END: c_int = 0xff57;
const XK_KP_ENTER: c_int = 0xff8d;
const XK_KP_LEFT: c_int = 0xff96;
const XK_KP_RIGHT: c_int = 0xff98;
const XK_KP_0: c_int = 0xffb0;
const K_SHIFT_MASK: c_int = 1 << 0;
const K_LOCK_MASK: c_int = 1 << 1;
const K_CONTROL_MASK: c_int = 1 << 2;
const K_ALT_MASK: c_int = 1 << 3;
const K_SUPER_MASK: c_int = 1 << 26;
const K_RELEASE_MASK: c_int = 1 << 30;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FrontendOptions {
    pub(crate) shared_data_dir: PathBuf,
    pub(crate) user_data_dir: PathBuf,
    pub(crate) schema_id: String,
    pub(crate) sequence: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FrontendRun {
    pub(crate) schema_id: String,
    pub(crate) sequence: String,
    pub(crate) events: Vec<FrontendEvent>,
    pub(crate) commits: Vec<String>,
    pub(crate) context: FrontendContext,
    pub(crate) status: FrontendStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FrontendEvent {
    pub(crate) key: String,
    pub(crate) keycode: c_int,
    pub(crate) mask: c_int,
    pub(crate) handled: bool,
    pub(crate) commits: Vec<String>,
    pub(crate) context: FrontendContext,
    pub(crate) status: FrontendStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FrontendContext {
    pub(crate) input: String,
    pub(crate) caret: usize,
    pub(crate) preedit: String,
    pub(crate) highlighted: usize,
    pub(crate) last_commit: Option<String>,
    pub(crate) candidates: Vec<FrontendCandidate>,
    pub(crate) page_size: usize,
    pub(crate) page_no: usize,
    pub(crate) is_last_page: bool,
    pub(crate) select_keys: Option<String>,
    pub(crate) select_labels: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FrontendCandidate {
    pub(crate) text: String,
    pub(crate) comment: String,
    pub(crate) source: String,
    pub(crate) quality: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FrontendStatus {
    pub(crate) schema_id: String,
    pub(crate) schema_name: String,
    pub(crate) is_disabled: bool,
    pub(crate) is_composing: bool,
    pub(crate) is_ascii_mode: bool,
    pub(crate) is_full_shape: bool,
    pub(crate) is_simplified: bool,
    pub(crate) is_traditional: bool,
    pub(crate) is_ascii_punct: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FrontendError {
    problem: String,
    next: String,
}

impl FrontendError {
    fn new(problem: impl Into<String>, next: impl Into<String>) -> Self {
        Self {
            problem: problem.into(),
            next: next.into(),
        }
    }
}

impl std::fmt::Display for FrontendError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "error: {}. next: {}.", self.problem, self.next)
    }
}

impl std::error::Error for FrontendError {}

impl FrontendRun {
    pub(crate) fn to_json(&self) -> String {
        FrontendTranscript::new(self).to_json()
    }
}

pub(crate) fn run_frontend(options: FrontendOptions) -> Result<FrontendRun, String> {
    run_frontend_inner(options).map_err(|error| error.to_string())
}

fn run_frontend_inner(options: FrontendOptions) -> Result<FrontendRun, FrontendError> {
    let shared_data_dir = cstring_from_path(&options.shared_data_dir, "shared_data_dir")?;
    let user_data_dir = cstring_from_path(&options.user_data_dir, "user_data_dir")?;
    let schema_id = cstring_from_value(&options.schema_id, "schema_id")?;
    let key_events = parse_key_sequence(&options.sequence).map_err(|error| {
        FrontendError::new(
            format!("invalid key sequence: {error}"),
            "pass --sequence with RIME key names or ASCII keys",
        )
    })?;

    // SAFETY: `rime_get_api` returns a static function table pointer owned by the
    // ABI crate for the duration of the process; null is checked before use.
    let api = unsafe {
        let api = rime_get_api();
        if api.is_null() {
            return Err(FrontendError::new(
                "RimeApi function table is unavailable",
                "ensure yune-rime-api is linked into yune-cli",
            ));
        }
        &*api
    };

    let setup = api.setup.ok_or_else(|| missing_api("setup"))?;
    let initialize = api.initialize.ok_or_else(|| missing_api("initialize"))?;
    let deploy = api.deploy.ok_or_else(|| missing_api("deploy"))?;
    let create_session = api
        .create_session
        .ok_or_else(|| missing_api("create_session"))?;
    let process_key = api.process_key.ok_or_else(|| missing_api("process_key"))?;
    let select_schema = api
        .select_schema
        .ok_or_else(|| missing_api("select_schema"))?;

    let mut traits = empty_traits();
    traits.shared_data_dir = shared_data_dir.as_ptr();
    traits.user_data_dir = user_data_dir.as_ptr();

    // SAFETY: the trait pointers reference CStrings kept alive for the whole run.
    unsafe { setup(&traits) };
    // SAFETY: same initialized trait object and live C strings as setup.
    unsafe { initialize(&traits) };
    let mut cleanup = CleanupGuard::new(api);
    cleanup.initialized = true;

    if deploy() != TRUE {
        return Err(FrontendError::new(
            "schema deployment failed",
            "verify shared_data_dir contains deployable RIME schema files",
        ));
    }

    let session_id = create_session();
    if session_id == 0 {
        return Err(FrontendError::new(
            "session creation failed",
            "verify RIME runtime initialized successfully",
        ));
    }
    cleanup.session_id = Some(session_id);

    // SAFETY: `schema_id` is a live NUL-terminated CString and session_id is valid.
    if unsafe { select_schema(session_id, schema_id.as_ptr()) } != TRUE {
        return Err(FrontendError::new(
            format!("schema selection failed for {}", options.schema_id),
            "verify --schema names a deployed schema id",
        ));
    }

    let mut events = Vec::with_capacity(key_events.len());
    let mut all_commits = Vec::new();
    let mut last_commit = None;
    for key_event in key_events {
        let key = key_label(key_event);
        let (keycode, mask) = key_event_to_rime(key_event)?;
        let handled = process_key(session_id, keycode, mask) == TRUE;
        let commits = capture_commits(api, session_id)?;
        if let Some(commit) = commits.last() {
            last_commit = Some(commit.clone());
        }
        all_commits.extend(commits.iter().cloned());
        let mut context = capture_context(api, session_id)?;
        context.last_commit = last_commit.clone();
        let status = capture_status(api, session_id)?;
        events.push(FrontendEvent {
            key,
            keycode,
            mask,
            handled,
            commits,
            context,
            status,
        });
    }

    let (context, status) = if let Some(event) = events.last() {
        (event.context.clone(), event.status.clone())
    } else {
        let mut context = capture_context(api, session_id)?;
        context.last_commit = last_commit;
        (context, capture_status(api, session_id)?)
    };

    Ok(FrontendRun {
        schema_id: options.schema_id,
        sequence: options.sequence,
        events,
        commits: all_commits,
        context,
        status,
    })
}

fn capture_commits(
    api: &yune_rime_api::RimeApi,
    session_id: RimeSessionId,
) -> Result<Vec<String>, FrontendError> {
    let get_commit = api.get_commit.ok_or_else(|| missing_api("get_commit"))?;
    let free_commit = api.free_commit.ok_or_else(|| missing_api("free_commit"))?;
    let mut commits = Vec::new();

    loop {
        let mut commit = empty_commit();
        // SAFETY: `commit` is initialized caller-owned storage for ABI population.
        if unsafe { get_commit(session_id, &mut commit) } != TRUE {
            break;
        }
        let text = c_string_from_mut(commit.text);
        // SAFETY: `commit` is exactly the populated struct returned by get_commit.
        let freed = unsafe { free_commit(&mut commit) };
        if freed != TRUE {
            return Err(FrontendError::new(
                "commit cleanup failed",
                "retry after checking ABI free_commit support",
            ));
        }
        commits.push(text);
    }

    Ok(commits)
}

fn capture_context(
    api: &yune_rime_api::RimeApi,
    session_id: RimeSessionId,
) -> Result<FrontendContext, FrontendError> {
    let get_input = api.get_input.ok_or_else(|| missing_api("get_input"))?;
    let get_context = api.get_context.ok_or_else(|| missing_api("get_context"))?;
    let free_context = api
        .free_context
        .ok_or_else(|| missing_api("free_context"))?;
    let input = c_string_from_const(get_input(session_id));
    let mut context = empty_context();
    // SAFETY: `context` is initialized caller-owned storage with a positive data_size.
    if unsafe { get_context(session_id, &mut context) } != TRUE {
        return Err(FrontendError::new(
            "context read failed",
            "verify the session is still active before reading context",
        ));
    }

    let captured = copy_context(&context, input);
    // SAFETY: `context` is exactly the populated struct returned by get_context.
    let freed = unsafe { free_context(&mut context) };
    if freed != TRUE {
        return Err(FrontendError::new(
            "context cleanup failed",
            "retry after checking ABI free_context support",
        ));
    }
    Ok(captured)
}

fn capture_status(
    api: &yune_rime_api::RimeApi,
    session_id: RimeSessionId,
) -> Result<FrontendStatus, FrontendError> {
    let get_status = api.get_status.ok_or_else(|| missing_api("get_status"))?;
    let free_status = api.free_status.ok_or_else(|| missing_api("free_status"))?;
    let mut status = empty_status();
    // SAFETY: `status` is initialized caller-owned storage with a positive data_size.
    if unsafe { get_status(session_id, &mut status) } != TRUE {
        return Err(FrontendError::new(
            "status read failed",
            "verify the session is still active before reading status",
        ));
    }

    let captured = FrontendStatus {
        schema_id: c_string_from_mut(status.schema_id),
        schema_name: c_string_from_mut(status.schema_name),
        is_disabled: status.is_disabled == TRUE,
        is_composing: status.is_composing == TRUE,
        is_ascii_mode: status.is_ascii_mode == TRUE,
        is_full_shape: status.is_full_shape == TRUE,
        is_simplified: status.is_simplified == TRUE,
        is_traditional: status.is_traditional == TRUE,
        is_ascii_punct: status.is_ascii_punct == TRUE,
    };
    // SAFETY: `status` is exactly the populated struct returned by get_status.
    let freed = unsafe { free_status(&mut status) };
    if freed != TRUE {
        return Err(FrontendError::new(
            "status cleanup failed",
            "retry after checking ABI free_status support",
        ));
    }
    Ok(captured)
}

fn copy_context(context: &RimeContext, input: String) -> FrontendContext {
    let mut candidates = Vec::new();
    if !context.menu.candidates.is_null() && context.menu.num_candidates > 0 {
        // SAFETY: RimeGetContext returned `num_candidates` initialized entries.
        let candidate_slice = unsafe {
            std::slice::from_raw_parts(
                context.menu.candidates,
                usize::try_from(context.menu.num_candidates).unwrap_or(0),
            )
        };
        candidates.extend(candidate_slice.iter().map(copy_candidate));
    }

    FrontendContext {
        input,
        caret: usize::try_from(context.composition.cursor_pos).unwrap_or(0),
        preedit: c_string_from_mut(context.composition.preedit),
        highlighted: usize::try_from(context.menu.highlighted_candidate_index).unwrap_or(0),
        last_commit: None,
        candidates,
        page_size: usize::try_from(context.menu.page_size).unwrap_or(0),
        page_no: usize::try_from(context.menu.page_no).unwrap_or(0),
        is_last_page: context.menu.is_last_page == TRUE,
        select_keys: if context.menu.select_keys.is_null() {
            None
        } else {
            Some(c_string_from_mut(context.menu.select_keys))
        },
        select_labels: copy_select_labels(context),
    }
}

fn copy_select_labels(context: &RimeContext) -> Vec<String> {
    if context.select_labels.is_null() || context.menu.page_size <= 0 {
        return Vec::new();
    }

    // SAFETY: RimeGetContext returns `page_size` select label pointers when
    // `select_labels` is non-null; they remain valid until free_context.
    let labels = unsafe {
        std::slice::from_raw_parts(
            context.select_labels,
            usize::try_from(context.menu.page_size).unwrap_or(0),
        )
    };
    labels
        .iter()
        .map(|label| c_string_from_mut(*label))
        .collect()
}

fn copy_candidate(candidate: &RimeCandidate) -> FrontendCandidate {
    FrontendCandidate {
        text: c_string_from_mut(candidate.text),
        comment: c_string_from_mut(candidate.comment),
        source: String::new(),
        quality: 0,
    }
}

fn key_event_to_rime(key_event: KeyEvent) -> Result<(c_int, c_int), FrontendError> {
    let keycode = match key_event.code {
        KeyCode::Character(ch) if ch.is_ascii() => ch as c_int,
        KeyCode::KeypadDigit(ch) if ch.is_ascii_digit() => {
            XK_KP_0 + c_int::try_from(ch as u32 - '0' as u32).expect("ASCII digit fits c_int")
        }
        KeyCode::Backspace => XK_BACKSPACE,
        KeyCode::Delete => XK_DELETE,
        KeyCode::Escape => XK_ESCAPE,
        KeyCode::MoveCaretLeft => XK_LEFT,
        KeyCode::MoveCaretRight => XK_RIGHT,
        KeyCode::MoveCaretLeftByChar => XK_KP_LEFT,
        KeyCode::MoveCaretRightByChar => XK_KP_RIGHT,
        KeyCode::MoveCaretLeftBySyllable | KeyCode::PreviousCandidate => XK_UP,
        KeyCode::MoveCaretRightBySyllable | KeyCode::NextCandidate => XK_DOWN,
        KeyCode::Home => XK_HOME,
        KeyCode::End => XK_END,
        KeyCode::PreviousPage => XK_PAGE_UP,
        KeyCode::NextPage => XK_PAGE_DOWN,
        KeyCode::Return => XK_RETURN,
        KeyCode::KeypadEnter => XK_KP_ENTER,
        KeyCode::Tab => XK_TAB,
        KeyCode::Ignored | KeyCode::FirstCandidate => {
            return Err(FrontendError::new(
                format!(
                    "unsupported key in frontend sequence: {}",
                    key_label(key_event)
                ),
                "use supported RIME key names or ASCII transcript keys",
            ));
        }
        KeyCode::Character(_) | KeyCode::KeypadDigit(_) => {
            return Err(FrontendError::new(
                format!(
                    "unsupported non-ASCII key in frontend sequence: {}",
                    key_label(key_event)
                ),
                "use ASCII transcript keys supported by this phase",
            ));
        }
    };

    let mut mask = 0;
    if key_event.modifiers.shift {
        mask |= K_SHIFT_MASK;
    }
    if key_event.modifiers.lock {
        mask |= K_LOCK_MASK;
    }
    if key_event.modifiers.control {
        mask |= K_CONTROL_MASK;
    }
    if key_event.modifiers.alt {
        mask |= K_ALT_MASK;
    }
    if key_event.modifiers.super_key {
        mask |= K_SUPER_MASK;
    }
    if key_event.modifiers.release {
        mask |= K_RELEASE_MASK;
    }
    if key_event.modifiers.hyper || key_event.modifiers.meta {
        return Err(FrontendError::new(
            format!(
                "unsupported key modifier in frontend sequence: {}",
                key_label(key_event)
            ),
            "use Shift, Lock, Control, Alt, Super, or Release modifiers",
        ));
    }

    Ok((keycode, mask))
}

fn key_label(key_event: KeyEvent) -> String {
    let base = match key_event.code {
        KeyCode::Character(' ') => "space".to_owned(),
        KeyCode::Character(ch) => ch.to_string(),
        KeyCode::KeypadDigit(ch) => format!("KP_{ch}"),
        KeyCode::Tab => "Tab".to_owned(),
        KeyCode::Ignored => "Ignored".to_owned(),
        KeyCode::Backspace => "BackSpace".to_owned(),
        KeyCode::Delete => "Delete".to_owned(),
        KeyCode::Escape => "Escape".to_owned(),
        KeyCode::MoveCaretLeft => "Left".to_owned(),
        KeyCode::MoveCaretRight => "Right".to_owned(),
        KeyCode::MoveCaretLeftByChar => "KP_Left".to_owned(),
        KeyCode::MoveCaretRightByChar => "KP_Right".to_owned(),
        KeyCode::MoveCaretLeftBySyllable => "Control+Up".to_owned(),
        KeyCode::MoveCaretRightBySyllable => "Control+Down".to_owned(),
        KeyCode::Home => "Home".to_owned(),
        KeyCode::End => "End".to_owned(),
        KeyCode::PreviousCandidate => "Up".to_owned(),
        KeyCode::NextCandidate => "Down".to_owned(),
        KeyCode::FirstCandidate => "FirstCandidate".to_owned(),
        KeyCode::PreviousPage => "Page_Up".to_owned(),
        KeyCode::NextPage => "Page_Down".to_owned(),
        KeyCode::Return => "Return".to_owned(),
        KeyCode::KeypadEnter => "KP_Enter".to_owned(),
    };

    let mut modifiers = Vec::new();
    if key_event.modifiers.shift {
        modifiers.push("Shift");
    }
    if key_event.modifiers.lock {
        modifiers.push("Lock");
    }
    if key_event.modifiers.control && !base.starts_with("Control+") {
        modifiers.push("Control");
    }
    if key_event.modifiers.alt {
        modifiers.push("Alt");
    }
    if key_event.modifiers.super_key {
        modifiers.push("Super");
    }
    if key_event.modifiers.hyper {
        modifiers.push("Hyper");
    }
    if key_event.modifiers.meta {
        modifiers.push("Meta");
    }
    if key_event.modifiers.release {
        modifiers.push("Release");
    }
    if modifiers.is_empty() {
        base
    } else {
        format!("{}+{base}", modifiers.join("+"))
    }
}

fn cstring_from_path(path: &std::path::Path, name: &str) -> Result<CString, FrontendError> {
    CString::new(path.to_string_lossy().as_ref()).map_err(|_| {
        FrontendError::new(
            format!("{name} contains an unsupported NUL byte"),
            format!("pass --{name} without embedded NUL bytes"),
        )
    })
}

fn cstring_from_value(value: &str, name: &str) -> Result<CString, FrontendError> {
    CString::new(value).map_err(|_| {
        FrontendError::new(
            format!("{name} contains an unsupported NUL byte"),
            format!("pass --{name} without embedded NUL bytes"),
        )
    })
}

fn c_string_from_mut(value: *mut std::os::raw::c_char) -> String {
    if value.is_null() {
        return String::new();
    }
    // SAFETY: ABI-populated pointers are NUL-terminated and valid until the
    // corresponding free_* function is called by this module.
    unsafe { CStr::from_ptr(value) }
        .to_string_lossy()
        .into_owned()
}

fn c_string_from_const(value: *const std::os::raw::c_char) -> String {
    if value.is_null() {
        return String::new();
    }
    // SAFETY: ABI-borrowed input pointers are NUL-terminated and valid while the
    // session remains alive.
    unsafe { CStr::from_ptr(value) }
        .to_string_lossy()
        .into_owned()
}

fn missing_api(name: &str) -> FrontendError {
    FrontendError::new(
        format!("RimeApi missing {name}"),
        "ensure yune-rime-api exposes the required frontend function table entry",
    )
}

fn empty_traits() -> RimeTraits {
    RimeTraits {
        data_size: mem::size_of::<RimeTraits>() as i32,
        shared_data_dir: ptr::null(),
        user_data_dir: ptr::null(),
        distribution_name: ptr::null(),
        distribution_code_name: ptr::null(),
        distribution_version: ptr::null(),
        app_name: ptr::null(),
        modules: ptr::null(),
        min_log_level: 0,
        log_dir: ptr::null(),
        prebuilt_data_dir: ptr::null(),
        staging_dir: ptr::null(),
    }
}

fn empty_commit() -> RimeCommit {
    RimeCommit {
        data_size: (mem::size_of::<RimeCommit>() - mem::size_of::<i32>()) as i32,
        text: ptr::null_mut(),
    }
}

fn empty_context() -> RimeContext {
    RimeContext {
        data_size: (mem::size_of::<RimeContext>() - mem::size_of::<i32>()) as i32,
        composition: RimeComposition {
            length: 0,
            cursor_pos: 0,
            sel_start: 0,
            sel_end: 0,
            preedit: ptr::null_mut(),
        },
        menu: RimeMenu {
            page_size: 0,
            page_no: 0,
            is_last_page: FALSE,
            highlighted_candidate_index: 0,
            num_candidates: 0,
            candidates: ptr::null_mut(),
            select_keys: ptr::null_mut(),
        },
        commit_text_preview: ptr::null_mut(),
        select_labels: ptr::null_mut(),
    }
}

fn empty_status() -> RimeStatus {
    RimeStatus {
        data_size: (mem::size_of::<RimeStatus>() - mem::size_of::<i32>()) as i32,
        schema_id: ptr::null_mut(),
        schema_name: ptr::null_mut(),
        is_disabled: FALSE,
        is_composing: FALSE,
        is_ascii_mode: FALSE,
        is_full_shape: FALSE,
        is_simplified: FALSE,
        is_traditional: FALSE,
        is_ascii_punct: FALSE,
    }
}

struct CleanupGuard<'api> {
    api: &'api yune_rime_api::RimeApi,
    session_id: Option<RimeSessionId>,
    initialized: bool,
}

#[cfg(test)]
pub(crate) fn frontend_test_guard() -> std::sync::MutexGuard<'static, ()> {
    static TEST_LOCK: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
    TEST_LOCK
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
        .expect("test lock should not be poisoned")
}

impl<'api> CleanupGuard<'api> {
    fn new(api: &'api yune_rime_api::RimeApi) -> Self {
        Self {
            api,
            session_id: None,
            initialized: false,
        }
    }
}

impl Drop for CleanupGuard<'_> {
    fn drop(&mut self) {
        if let Some(session_id) = self.session_id.take() {
            if let Some(destroy_session) = self.api.destroy_session {
                destroy_session(session_id);
            }
        }
        if let Some(cleanup_all_sessions) = self.api.cleanup_all_sessions {
            cleanup_all_sessions();
        }
        if self.initialized {
            if let Some(finalize) = self.api.finalize {
                finalize();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        frontend_test_guard, key_event_to_rime, run_frontend, FrontendOptions, K_CONTROL_MASK,
        XK_TAB,
    };
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };
    use yune_core::parse_key_sequence;

    fn unique_temp_dir(label: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "yune-cli-frontend-{label}-{}-{nanos}",
            std::process::id()
        ))
    }

    fn write_runtime(root: &std::path::Path) -> (std::path::PathBuf, std::path::PathBuf) {
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
            "schema:\n  schema_id: default\n  name: Default\n",
        )
        .expect("schema config should be written");
        (shared, user)
    }

    #[test]
    fn maps_ascii_and_control_keys_to_rime_keycode_mask() {
        let keys = parse_key_sequence("a{Control+Return}").expect("sequence should parse");

        assert_eq!(key_event_to_rime(keys[0]), Ok(('a' as i32, 0)));
        assert_eq!(key_event_to_rime(keys[1]), Ok((0xff0d, K_CONTROL_MASK)));
    }

    #[test]
    fn maps_tab_key_name_to_rime_keycode() {
        let key = parse_key_sequence("{Tab}").expect("sequence should parse")[0];

        assert_eq!(key_event_to_rime(key), Ok((XK_TAB, 0)));
    }

    #[test]
    fn rejects_non_ascii_frontend_keys_with_corrective_error() {
        let key = parse_key_sequence("你").expect("sequence should parse")[0];

        assert_eq!(
            key_event_to_rime(key)
                .expect_err("non-ASCII key should fail")
                .to_string(),
            "error: unsupported non-ASCII key in frontend sequence: 你. next: use ASCII transcript keys supported by this phase."
        );
    }

    #[test]
    fn run_frontend_drives_lifecycle_and_captures_per_key_events() {
        let _guard = frontend_test_guard();
        let root = unique_temp_dir("basic");
        let (shared, user) = write_runtime(&root);

        let output = run_frontend(FrontendOptions {
            shared_data_dir: shared,
            user_data_dir: user,
            schema_id: "default".to_owned(),
            sequence: "ni".to_owned(),
        })
        .expect("frontend run should succeed");

        assert_eq!(output.schema_id, "default");
        assert_eq!(output.sequence, "ni");
        assert_eq!(output.events.len(), 2);
        assert_eq!(output.events[0].key, "n");
        assert!(output.events[0].handled);
        assert_eq!(output.events[1].key, "i");
        assert_eq!(output.context.input, "ni");
        assert_eq!(output.status.schema_id, "default");
        assert!(output.status.is_composing);

        fs::remove_dir_all(root).expect("temp dirs should be removed");
    }

    #[test]
    fn run_frontend_cleans_sessions_after_invalid_setup_error() {
        let _guard = frontend_test_guard();
        let root = unique_temp_dir("cleanup");
        let (shared, user) = write_runtime(&root);

        let error = run_frontend(FrontendOptions {
            shared_data_dir: shared,
            user_data_dir: user,
            schema_id: "default".to_owned(),
            sequence: "你".to_owned(),
        })
        .expect_err("unsupported key should fail before session setup");

        assert_eq!(
            error,
            "error: unsupported non-ASCII key in frontend sequence: 你. next: use ASCII transcript keys supported by this phase."
        );
        let api = unsafe { &*yune_rime_api::rime_get_api() };
        let create_session = api
            .create_session
            .expect("frontend requires create_session");
        assert_eq!(create_session(), 0);

        fs::remove_dir_all(root).expect("temp dirs should be removed");
    }
}
