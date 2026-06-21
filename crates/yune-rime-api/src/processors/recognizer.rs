use serde_yaml::Value;
use yune_core::{KeyCode, KeyEvent};

use crate::{
    config_scalar_bool, find_config_value, load_runtime_config_root,
    load_schema_recognizer_patterns, recognizer_patterns_match, schema_component_prescription,
    ConfigOpenKind, RecognizerProcessor, SessionState,
};

pub(crate) fn install_schema_recognizer_processor(session: &mut SessionState, schema_id: &str) {
    let schema_config =
        load_runtime_config_root(&format!("{schema_id}.schema"), ConfigOpenKind::Deployed);
    let Some(Value::Sequence(processors)) = find_config_value(&schema_config, "engine/processors")
    else {
        return;
    };
    let Some(name_space) = processors
        .iter()
        .filter_map(Value::as_str)
        .map(schema_component_prescription)
        .find_map(|(component_name, name_space)| {
            (component_name == "recognizer")
                .then(|| {
                    let name_space = name_space.unwrap_or("recognizer");
                    if name_space == "processor" {
                        "recognizer"
                    } else {
                        name_space
                    }
                })
                .filter(|name_space| !name_space.is_empty())
        })
    else {
        return;
    };
    let patterns = load_schema_recognizer_patterns(&schema_config, name_space);
    if patterns.is_empty() {
        return;
    }
    let use_space = find_config_value(&schema_config, &format!("{name_space}/use_space"))
        .and_then(config_scalar_bool)
        .unwrap_or(false);
    session.recognizer_processor = Some(RecognizerProcessor {
        use_space,
        patterns,
    });
}

pub(crate) fn process_recognizer_processor(
    session: &mut SessionState,
    key_event: KeyEvent,
) -> bool {
    if key_event.modifiers.control
        || key_event.modifiers.alt
        || key_event.modifiers.super_key
        || key_event.modifiers.release
    {
        return false;
    }
    let KeyCode::Character(ch) = key_event.code else {
        return false;
    };
    if !((ch == ' '
        && session
            .recognizer_processor
            .as_ref()
            .is_some_and(|processor| processor.use_space))
        || (ch > '\u{20}' && ch < '\u{80}'))
    {
        return false;
    }
    let Some(processor) = &session.recognizer_processor else {
        return false;
    };

    let mut input = session.engine.context().composition.input.clone();
    input.push(ch);
    let affix_prefix_in_progress = session
        .affix_segmentors
        .iter()
        .any(|segmentor| segmentor.prefix.starts_with(&input));
    if !recognizer_patterns_match(&processor.patterns, &input) && !affix_prefix_in_progress {
        return false;
    }
    session.engine.set_input(input);
    true
}
