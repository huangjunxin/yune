use std::collections::HashMap;

use serde_yaml::Value;
use yune_core::{KeyCode, KeyEvent};

use crate::{
    config_scalar_string, find_config_value, load_runtime_config_root,
    parse_single_key_binding_event, ConfigOpenKind, NavigatorAction, NavigatorBindingAction,
    NavigatorSyllableJumpPosition, SessionState,
};

pub(crate) fn process_navigator_configured_key(
    session: &mut SessionState,
    key_event: KeyEvent,
) -> Option<bool> {
    if session.engine.context().composition.input.is_empty() || key_event.modifiers.release {
        return None;
    }
    let is_vertical = session.engine.get_option("_vertical");
    let action = navigator_configured_action(session, is_vertical, key_event)?;
    match action {
        NavigatorBindingAction::Noop => Some(false),
        NavigatorBindingAction::Action(action) => {
            apply_navigator_action(session, action);
            Some(true)
        }
    }
}

pub(crate) fn process_navigator_delimiter_key(
    session: &mut SessionState,
    key_event: KeyEvent,
) -> Option<bool> {
    if session.engine.context().composition.input.is_empty() || key_event.modifiers.release {
        return None;
    }
    let input = &session.engine.context().composition.input;
    if !input
        .chars()
        .any(|ch| session.navigator_delimiters.contains(ch))
    {
        return None;
    }
    let action = default_navigator_syllable_action(key_event)?;
    apply_navigator_action(session, action);
    Some(true)
}

fn default_navigator_syllable_action(key_event: KeyEvent) -> Option<NavigatorAction> {
    let exact_control_or_shift = (key_event.modifiers.control ^ key_event.modifiers.shift)
        && !key_event.modifiers.lock
        && !key_event.modifiers.alt
        && !key_event.modifiers.super_key
        && !key_event.modifiers.hyper
        && !key_event.modifiers.meta
        && !key_event.modifiers.release;
    if !exact_control_or_shift {
        return None;
    }

    match key_event.code {
        KeyCode::MoveCaretLeft | KeyCode::MoveCaretLeftBySyllable => {
            Some(NavigatorAction::LeftBySyllable)
        }
        KeyCode::MoveCaretRight | KeyCode::MoveCaretRightBySyllable => {
            Some(NavigatorAction::RightBySyllable)
        }
        _ => None,
    }
}

pub(crate) fn navigator_configured_action(
    session: &SessionState,
    is_vertical: bool,
    key_event: KeyEvent,
) -> Option<NavigatorBindingAction> {
    let bindings = if is_vertical {
        &session.navigator_bindings.vertical
    } else {
        &session.navigator_bindings.horizontal
    };
    bindings.get(&key_event).copied()
}

fn apply_navigator_action(session: &mut SessionState, action: NavigatorAction) {
    match action {
        NavigatorAction::Rewind => {
            session.engine.move_caret_left();
        }
        NavigatorAction::Forward => {
            session.engine.move_caret_right();
        }
        NavigatorAction::LeftByChar => {
            session.engine.move_caret_left_by_char();
        }
        NavigatorAction::RightByChar => {
            session.engine.move_caret_right_by_char();
        }
        NavigatorAction::LeftBySyllable | NavigatorAction::LeftBySyllableNoLoop => {
            let loop_at_boundary = matches!(action, NavigatorAction::LeftBySyllable);
            if !move_caret_left_by_delimited_syllable(session, loop_at_boundary) {
                session.engine.move_caret_left_by_syllable();
            }
        }
        NavigatorAction::RightBySyllable | NavigatorAction::RightBySyllableNoLoop => {
            let loop_at_boundary = matches!(action, NavigatorAction::RightBySyllable);
            if !move_caret_right_by_delimited_syllable(session, loop_at_boundary) {
                session.engine.move_caret_right_by_syllable();
            }
        }
        NavigatorAction::LeftByCharNoLoop => {
            session.engine.move_caret_left();
        }
        NavigatorAction::RightByCharNoLoop => {
            session.engine.move_caret_right();
        }
        NavigatorAction::Home => {
            session.engine.move_caret_home();
        }
        NavigatorAction::End => {
            session.engine.move_caret_end();
        }
    }
}

fn move_caret_left_by_delimited_syllable(
    session: &mut SessionState,
    loop_at_boundary: bool,
) -> bool {
    let context = session.engine.context();
    let input = &context.composition.input;
    let caret = context.composition.caret.min(input.len());
    if input.is_empty() || !input.is_ascii() {
        return false;
    }

    let stops = navigator_syllable_stops(
        input,
        &session.navigator_delimiters,
        session.navigator_syllable_jump_position,
    );
    let next_caret = stops
        .iter()
        .rev()
        .copied()
        .find(|stop| *stop < caret)
        .or_else(|| {
            loop_at_boundary
                .then(|| stops.iter().rev().copied().find(|stop| *stop < input.len()))
                .flatten()
        });
    let Some(next_caret) = next_caret else {
        return false;
    };
    if next_caret == caret {
        return false;
    }
    session.engine.set_caret_pos(next_caret);
    true
}

fn move_caret_right_by_delimited_syllable(
    session: &mut SessionState,
    loop_at_boundary: bool,
) -> bool {
    let context = session.engine.context();
    let input = &context.composition.input;
    let caret = context.composition.caret.min(input.len());
    if input.is_empty() || !input.is_ascii() {
        return false;
    }

    let stops = navigator_syllable_stops(
        input,
        &session.navigator_delimiters,
        session.navigator_syllable_jump_position,
    );
    let next_caret = stops
        .iter()
        .copied()
        .find(|stop| *stop > caret)
        .or_else(|| {
            loop_at_boundary
                .then(|| stops.iter().copied().find(|stop| *stop > 0))
                .flatten()
        });
    let Some(next_caret) = next_caret else {
        return false;
    };
    if next_caret == caret {
        return false;
    }
    session.engine.set_caret_pos(next_caret);
    true
}

fn navigator_syllable_stops(
    input: &str,
    delimiters: &str,
    jump_position: NavigatorSyllableJumpPosition,
) -> Vec<usize> {
    let mut stops = vec![0, input.len()];
    let mut delimiter_run_start = None;
    for (index, ch) in input.char_indices() {
        if delimiters.contains(ch) {
            delimiter_run_start.get_or_insert(index);
            continue;
        }

        if let Some(start) = delimiter_run_start.take() {
            stops.push(match jump_position {
                NavigatorSyllableJumpPosition::AfterDelimiter => index,
                NavigatorSyllableJumpPosition::BeforeDelimiter => start,
            });
        }
    }
    if let Some(start) = delimiter_run_start {
        stops.push(match jump_position {
            NavigatorSyllableJumpPosition::AfterDelimiter => input.len(),
            NavigatorSyllableJumpPosition::BeforeDelimiter => start,
        });
    }
    stops.sort_unstable();
    stops.dedup();
    stops
}

pub(crate) fn install_schema_navigator_bindings(session: &mut SessionState, schema_id: &str) {
    let schema_config =
        load_runtime_config_root(&format!("{schema_id}.schema"), ConfigOpenKind::Deployed);
    session.navigator_delimiters = find_config_value(&schema_config, "speller/delimiter")
        .and_then(config_scalar_string)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| " ".to_owned());
    session.navigator_syllable_jump_position =
        match find_config_value(&schema_config, "navigator/syllable_jump_position")
            .and_then(config_scalar_string)
            .as_deref()
        {
            Some("before_delimiter") => NavigatorSyllableJumpPosition::BeforeDelimiter,
            _ => NavigatorSyllableJumpPosition::AfterDelimiter,
        };
    load_navigator_binding_section(
        &schema_config,
        "navigator",
        &mut session.navigator_bindings.horizontal,
    );
    load_navigator_binding_section(
        &schema_config,
        "navigator/vertical",
        &mut session.navigator_bindings.vertical,
    );
}

fn load_navigator_binding_section(
    schema_config: &Value,
    section: &str,
    bindings: &mut HashMap<KeyEvent, NavigatorBindingAction>,
) {
    let Some(Value::Mapping(config_bindings)) =
        find_config_value(schema_config, &format!("{section}/bindings"))
    else {
        return;
    };

    for (key, action) in config_bindings {
        let Some(key) = config_scalar_string(key) else {
            continue;
        };
        let Some(key_event) = parse_single_key_binding_event(&key) else {
            continue;
        };
        let Some(action) = action.as_str().and_then(navigator_binding_action_from_name) else {
            continue;
        };
        bindings.insert(key_event, action);
    }
}

fn navigator_binding_action_from_name(action: &str) -> Option<NavigatorBindingAction> {
    let action = match action {
        "noop" => NavigatorBindingAction::Noop,
        "rewind" => NavigatorBindingAction::Action(NavigatorAction::Rewind),
        "forward" => NavigatorBindingAction::Action(NavigatorAction::Forward),
        "left_by_char" => NavigatorBindingAction::Action(NavigatorAction::LeftByChar),
        "right_by_char" => NavigatorBindingAction::Action(NavigatorAction::RightByChar),
        "left_by_syllable" => NavigatorBindingAction::Action(NavigatorAction::LeftBySyllable),
        "right_by_syllable" => NavigatorBindingAction::Action(NavigatorAction::RightBySyllable),
        "left_by_char_no_loop" => NavigatorBindingAction::Action(NavigatorAction::LeftByCharNoLoop),
        "right_by_char_no_loop" => {
            NavigatorBindingAction::Action(NavigatorAction::RightByCharNoLoop)
        }
        "left_by_syllable_no_loop" => {
            NavigatorBindingAction::Action(NavigatorAction::LeftBySyllableNoLoop)
        }
        "right_by_syllable_no_loop" => {
            NavigatorBindingAction::Action(NavigatorAction::RightBySyllableNoLoop)
        }
        "home" => NavigatorBindingAction::Action(NavigatorAction::Home),
        "end" => NavigatorBindingAction::Action(NavigatorAction::End),
        _ => return None,
    };
    Some(action)
}
