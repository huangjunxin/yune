use yune_core::{KeyCode, KeyEvent};

use crate::SessionState;

pub(crate) fn process_shape_processor(
    session: &SessionState,
    key_event: KeyEvent,
) -> Option<String> {
    if !session.engine.status().is_full_shape
        || key_event.modifiers.control
        || key_event.modifiers.alt
        || key_event.modifiers.super_key
        || key_event.modifiers.release
    {
        return None;
    }
    let KeyCode::Character(ch) = key_event.code else {
        return None;
    };
    if !('\u{20}'..='\u{7e}').contains(&ch) {
        return None;
    }
    Some(shape_formatted_ascii_text(&ch.to_string(), true))
}

pub(crate) fn shape_formatted_ascii_text(text: &str, full_shape: bool) -> String {
    if !full_shape {
        return text.to_owned();
    }
    text.chars()
        .map(|ch| match ch {
            ' ' => '\u{3000}',
            '!'..='~' => char::from_u32(ch as u32 + 0xfee0)
                .expect("printable ASCII has a full-shape compatibility form"),
            _ => ch,
        })
        .collect()
}
