use std::{collections::HashMap, ffi::CString, os::raw::c_int};

use serde_yaml::Value;
use yune_core::{KeyCode, KeyEvent};

use crate::{
    config_scalar_string, find_config_value, load_runtime_config_root, schema_component_prescription,
    AsciiComposerProcessResult, AsciiModeSwitchStyle, ConfigOpenKind, RimeGetKeycodeByName,
    SessionState, K_RELEASE_MASK, XK_ALT_L, XK_ALT_R, XK_CAPS_LOCK, XK_CONTROL_L, XK_CONTROL_R,
    XK_SHIFT_L, XK_SHIFT_R, XK_SUPER_L, XK_SUPER_R,
};

pub(crate) fn install_schema_ascii_composer_processor(session: &mut SessionState, schema_id: &str) {
    let schema_config =
        load_runtime_config_root(&format!("{schema_id}.schema"), ConfigOpenKind::Deployed);
    let Some(Value::Sequence(processors)) = find_config_value(&schema_config, "engine/processors")
    else {
        return;
    };
    session.ascii_composer_enabled = processors
        .iter()
        .filter_map(Value::as_str)
        .map(schema_component_prescription)
        .any(|(component_name, _)| component_name == "ascii_composer");
    if !session.ascii_composer_enabled {
        return;
    }

    session.ascii_composer_switch_bindings = load_ascii_composer_switch_bindings(&schema_config);
    if session.ascii_composer_switch_bindings.is_empty() {
        let default_config = load_runtime_config_root("default", ConfigOpenKind::Deployed);
        session.ascii_composer_switch_bindings =
            load_ascii_composer_switch_bindings(&default_config);
    }
}

fn load_ascii_composer_switch_bindings(
    schema_config: &Value,
) -> HashMap<c_int, AsciiModeSwitchStyle> {
    let Some(Value::Mapping(bindings)) =
        find_config_value(schema_config, "ascii_composer/switch_key")
    else {
        return HashMap::new();
    };

    bindings
        .iter()
        .filter_map(|(key, style)| {
            let key = config_scalar_string(key)?;
            let style =
                config_scalar_string(style).and_then(|style| ascii_mode_switch_style(&style))?;
            let key_c = CString::new(key).ok()?;
            // SAFETY: `key_c` is a valid NUL-terminated key-name string.
            let keycode = unsafe { RimeGetKeycodeByName(key_c.as_ptr()) };
            (keycode != 0x00ff_ffff).then_some((keycode, style))
        })
        .collect()
}

fn ascii_mode_switch_style(style: &str) -> Option<AsciiModeSwitchStyle> {
    match style {
        "inline_ascii" => Some(AsciiModeSwitchStyle::InlineAscii),
        "commit_text" => Some(AsciiModeSwitchStyle::CommitText),
        "commit_code" => Some(AsciiModeSwitchStyle::CommitCode),
        "clear" => Some(AsciiModeSwitchStyle::Clear),
        "set_ascii_mode" => Some(AsciiModeSwitchStyle::SetAsciiMode),
        "unset_ascii_mode" => Some(AsciiModeSwitchStyle::UnsetAsciiMode),
        _ => None,
    }
}

pub(crate) fn process_ascii_composer_processor(
    session: &mut SessionState,
    key_event: KeyEvent,
) -> AsciiComposerProcessResult {
    if !session.ascii_composer_enabled
        || key_event.modifiers.control
        || key_event.modifiers.alt
        || key_event.modifiers.super_key
        || key_event.modifiers.hyper
        || key_event.modifiers.meta
        || key_event.modifiers.release
    {
        return AsciiComposerProcessResult::Noop;
    }
    if key_event.modifiers.shift && matches!(key_event.code, KeyCode::Character(' ')) {
        return AsciiComposerProcessResult::Noop;
    }
    if !session.engine.status().is_ascii_mode {
        return AsciiComposerProcessResult::Noop;
    }
    let KeyCode::Character(ch) = key_event.code else {
        return AsciiComposerProcessResult::Noop;
    };
    if !(('\u{20}'..'\u{80}').contains(&ch)) {
        return AsciiComposerProcessResult::Noop;
    }
    if session.engine.context().composition.input.is_empty() {
        return AsciiComposerProcessResult::Rejected;
    }

    let mut input = session.engine.context().composition.input.clone();
    input.push(ch);
    session.engine.set_input(input);
    AsciiComposerProcessResult::Accepted(None)
}

pub(crate) fn is_ascii_composer_modifier_key(keycode: c_int) -> bool {
    matches!(
        keycode,
        XK_SHIFT_L
            | XK_SHIFT_R
            | XK_CONTROL_L
            | XK_CONTROL_R
            | XK_ALT_L
            | XK_ALT_R
            | XK_SUPER_L
            | XK_SUPER_R
    )
}

pub(crate) fn process_ascii_composer_modifier_switch_key(
    session: &mut SessionState,
    keycode: c_int,
    mask: c_int,
) -> Option<String> {
    if !session.ascii_composer_enabled {
        return None;
    }
    if mask == K_RELEASE_MASK {
        let pressed = session.ascii_composer_pressed_switch_key.take();
        if pressed == Some(keycode) {
            return switch_ascii_mode_with_key(session, keycode);
        }
        return None;
    }

    match session.ascii_composer_pressed_switch_key {
        None => session.ascii_composer_pressed_switch_key = Some(keycode),
        Some(pressed) if pressed != keycode => session.ascii_composer_pressed_switch_key = None,
        Some(_) => {}
    }
    None
}

pub(crate) fn process_ascii_composer_switch_key(
    session: &mut SessionState,
    keycode: c_int,
) -> Option<Option<String>> {
    if !session.ascii_composer_enabled {
        return None;
    }
    Some(switch_ascii_mode_with_key(session, keycode))
}

fn switch_ascii_mode_with_key(session: &mut SessionState, keycode: c_int) -> Option<String> {
    let style = *session.ascii_composer_switch_bindings.get(&keycode)?;
    let old_mode = session.engine.status().is_ascii_mode;
    let new_mode = match style {
        AsciiModeSwitchStyle::SetAsciiMode => true,
        AsciiModeSwitchStyle::UnsetAsciiMode => false,
        AsciiModeSwitchStyle::InlineAscii
        | AsciiModeSwitchStyle::CommitText
        | AsciiModeSwitchStyle::CommitCode
        | AsciiModeSwitchStyle::Clear => !old_mode,
    };
    if old_mode == new_mode {
        return None;
    }
    switch_ascii_mode(session, new_mode, style)
}

pub(crate) fn process_ascii_composer_caps_lock_switch_key(
    session: &mut SessionState,
) -> Option<Option<String>> {
    if !session.ascii_composer_enabled {
        return None;
    }
    let mut style = *session.ascii_composer_switch_bindings.get(&XK_CAPS_LOCK)?;
    if matches!(
        style,
        AsciiModeSwitchStyle::InlineAscii
            | AsciiModeSwitchStyle::SetAsciiMode
            | AsciiModeSwitchStyle::UnsetAsciiMode
    ) {
        style = AsciiModeSwitchStyle::Clear;
    }
    Some(switch_ascii_mode(session, true, style))
}

fn switch_ascii_mode(
    session: &mut SessionState,
    ascii_mode: bool,
    style: AsciiModeSwitchStyle,
) -> Option<String> {
    let mut commit = None;
    let was_composing = !session.engine.context().composition.input.is_empty();
    if was_composing {
        match style {
            AsciiModeSwitchStyle::InlineAscii => {}
            AsciiModeSwitchStyle::CommitText => {
                commit = session.engine.commit_composition();
            }
            AsciiModeSwitchStyle::CommitCode => {
                commit = session.engine.commit_raw_input();
            }
            AsciiModeSwitchStyle::Clear
            | AsciiModeSwitchStyle::SetAsciiMode
            | AsciiModeSwitchStyle::UnsetAsciiMode => {
                session.engine.clear_composition();
            }
        }
    }
    session.ascii_composer_inline_ascii =
        was_composing && ascii_mode && style == AsciiModeSwitchStyle::InlineAscii;
    session.engine.set_option("ascii_mode", ascii_mode);
    commit
}
