# Phase 3: Schema Pipeline Depth - Pattern Map

**Mapped:** 2026-04-29
**Files analyzed:** 14
**Analogs found:** 14 / 14

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `crates/yune-rime-api/src/processors/speller.rs` | processor | event-driven | `crates/yune-rime-api/src/processors/speller.rs` | exact-self |
| `crates/yune-rime-api/src/processors/editor.rs` | processor | event-driven | `crates/yune-rime-api/src/processors/editor.rs` | exact-self |
| `crates/yune-rime-api/src/processors/navigator.rs` | processor | event-driven | `crates/yune-rime-api/src/processors/navigator.rs` | exact-self |
| `crates/yune-rime-api/src/processors/selector.rs` | processor | event-driven | `crates/yune-rime-api/src/processors/selector.rs` | exact-self |
| `crates/yune-rime-api/src/processors/chord_composer.rs` | processor | event-driven | `crates/yune-rime-api/src/processors/chord_composer.rs` | exact-self |
| `crates/yune-rime-api/src/processors/shape.rs` | processor | event-driven | `crates/yune-rime-api/src/processors/shape.rs` | exact-self |
| `crates/yune-rime-api/src/processors/punctuation.rs` | processor | event-driven | `crates/yune-rime-api/src/processors/punctuation.rs` | exact-self |
| `crates/yune-rime-api/src/schema_install.rs` | service/config | transform | `crates/yune-rime-api/src/schema_install.rs` | exact-self |
| `crates/yune-rime-api/src/schema_selection.rs` | service/config | request-response | `crates/yune-rime-api/src/schema_selection.rs` | exact-self |
| `crates/yune-rime-api/src/session.rs` | model/store | request-response | `crates/yune-rime-api/src/session.rs` | exact-self |
| `crates/yune-rime-api/src/lib.rs` | route/façade | event-driven | `crates/yune-rime-api/src/lib.rs` | exact-self |
| `crates/yune-core/src/spelling_algebra.rs` | utility | transform | `crates/yune-core/src/spelling_algebra.rs` | exact-self |
| `crates/yune-core/src/translator/mod.rs` | service | transform | `crates/yune-core/src/translator/mod.rs` | exact-self |
| `crates/yune-core/src/filter/mod.rs` | service | transform | `crates/yune-core/src/filter/mod.rs` | exact-self |
| `crates/yune-rime-api/src/tests/schema_processors.rs` | test | request-response | `crates/yune-rime-api/src/tests/schema_processors.rs` | exact-self |
| `crates/yune-rime-api/src/tests/schema_selection.rs` | test | request-response | `crates/yune-rime-api/src/tests/schema_selection.rs` | exact-self |

## Pattern Assignments

### `crates/yune-rime-api/src/processors/speller.rs` (processor, event-driven)

**Analog:** `crates/yune-rime-api/src/processors/speller.rs`

**Imports pattern** (lines 1-8):
```rust
use regex::Regex;
use yune_core::{Candidate, CandidateSource, KeyCode, KeyEvent};

use crate::{
    config_scalar_bool, config_scalar_int, config_scalar_string, find_config_value,
    load_runtime_config_root, schema_engine_processors_include, ConfigOpenKind, SessionState,
    SpellerAutoClear, SpellerProcessResult, SpellerProcessor,
};
```

**Install-from-schema pattern** (lines 10-55):
```rust
pub(crate) fn install_schema_speller_processor(session: &mut SessionState, schema_id: &str) {
    let schema_config =
        load_runtime_config_root(&format!("{schema_id}.schema"), ConfigOpenKind::Deployed);
    if !schema_engine_processors_include(&schema_config, "speller") {
        return;
    }

    let alphabet = find_config_value(&schema_config, "speller/alphabet")
        .and_then(config_scalar_string)
        .unwrap_or_else(|| "zyxwvutsrqponmlkjihgfedcba".to_owned());
    // ... populate SessionState.speller from config helper fields ...
}
```

**Core event pattern** (lines 58-171):
```rust
pub(crate) fn process_speller_processor(
    session: &mut SessionState,
    key_event: KeyEvent,
) -> Option<SpellerProcessResult> {
    if key_event.modifiers.control
        || key_event.modifiers.alt
        || key_event.modifiers.super_key
        || key_event.modifiers.release
    {
        return None;
    }
    let KeyCode::Character(ch) = key_event.code else {
        return None;
    };
    // validate character, consult installed processor, mutate engine input,
    // then return accepted/commit through SpellerProcessResult.
}
```

**Previous-match / auto-select pattern** (lines 219-282):
```rust
fn speller_previous_match_backup(
    session: &SessionState,
    auto_select: bool,
    max_code_length: usize,
    auto_select_pattern: Option<&Regex>,
) -> Option<(String, usize, Candidate)> {
    if !auto_select || max_code_length > 0 || auto_select_pattern.is_some() {
        return None;
    }
    let context = session.engine.context();
    if !speller_context_has_menu(context) {
        return None;
    }
    let candidate = context.candidates.get(context.highlighted)?;
    (candidate.source == CandidateSource::Table).then(|| {
        (
            context.composition.input.clone(),
            context.highlighted,
            candidate.clone(),
        )
    })
}
```

**Error/validation pattern:** no thrown errors; invalid config and non-matching key states return `None`/`Some(... accepted: false ...)`. Regex compilation is fallible and ignored safely with `Regex::new(&pattern).ok()` (lines 40-42).

**Apply to Phase 3:** extend this module for SCHEMA-01 previous-match splitting and non-auto-commit behavior. Keep behavior owned here and keep `lib.rs` changes limited to dispatch glue.

---

### `crates/yune-rime-api/src/processors/editor.rs` (processor, event-driven)

**Analog:** `crates/yune-rime-api/src/processors/editor.rs`

**Imports pattern** (lines 1-9):
```rust
use std::collections::HashMap;

use serde_yaml::Value;
use yune_core::{KeyCode, KeyEvent};

use crate::{
    config_scalar_string, find_config_value, load_runtime_config_root,
    parse_single_key_binding_event, schema_engine_processors_include, ConfigOpenKind, EditorAction,
    EditorBindingAction, EditorCharHandler, EditorProcessor, SessionKeyProcessResult, SessionState,
};
```

**Install/config binding pattern** (lines 12-34, 37-57):
```rust
pub(crate) fn install_schema_editor_processor(session: &mut SessionState, schema_id: &str) {
    let schema_config =
        load_runtime_config_root(&format!("{schema_id}.schema"), ConfigOpenKind::Deployed);
    if schema_engine_processors_include(&schema_config, "express_editor") {
        session.editor_processor = Some(EditorProcessor::Express);
        session.editor_char_handler = Some(EditorCharHandler::DirectCommit);
        session.engine.set_option("_auto_commit", true);
    } else if schema_engine_processors_include(&schema_config, "fluid_editor")
        || schema_engine_processors_include(&schema_config, "fluency_editor")
    {
        session.editor_processor = Some(EditorProcessor::Fluid);
        session.editor_char_handler = Some(EditorCharHandler::AddToInput);
        session.engine.set_option("_auto_commit", false);
    }
    if session.editor_processor.is_some() {
        load_editor_binding_section(&schema_config, &mut session.editor_bindings);
    }
}
```

**Core event pattern** (lines 89-138):
```rust
pub(crate) fn process_editor_processor(
    session: &mut SessionState,
    key_event: KeyEvent,
) -> Option<SessionKeyProcessResult> {
    if session.editor_processor.is_none() || key_event.modifiers.release {
        return None;
    }

    let is_composing = !session.engine.context().composition.input.is_empty();
    if is_composing {
        if let Some(action) = session.editor_bindings.get(&key_event).copied() {
            return match action {
                EditorBindingAction::Noop => Some(SessionKeyProcessResult::Accepted),
                EditorBindingAction::Action(action) => Some(apply_editor_action(session, action)),
            };
        }
    }
    // char-handler and Return fallbacks follow.
    None
}
```

**Action mutation pattern** (lines 178-215):
```rust
fn apply_editor_action(
    session: &mut SessionState,
    action: EditorAction,
) -> SessionKeyProcessResult {
    let commit = match action {
        EditorAction::Confirm | EditorAction::CommitComposition => {
            session.engine.commit_composition()
        }
        EditorAction::ToggleSelection => {
            session.engine.first_candidate();
            None
        }
        EditorAction::Cancel => {
            session.engine.clear_composition();
            None
        }
        // ... other engine actions ...
    };
    commit.map_or(
        SessionKeyProcessResult::Accepted,
        SessionKeyProcessResult::Commit,
    )
}
```

**Apply to Phase 3:** use this shape for deeper editor segment/selection semantics: gate on installed processor and composition state, resolve configured bindings first, then mutate `session.engine` via focused helper functions.

---

### `crates/yune-rime-api/src/processors/navigator.rs` (processor, event-driven)

**Analog:** `crates/yune-rime-api/src/processors/navigator.rs`

**Imports pattern** (lines 1-9):
```rust
use std::collections::HashMap;

use serde_yaml::Value;
use yune_core::{KeyCode, KeyEvent};

use crate::{
    config_scalar_string, find_config_value, load_runtime_config_root,
    parse_single_key_binding_event, ConfigOpenKind, NavigatorAction, NavigatorBindingAction,
    NavigatorSyllableJumpPosition, SessionState,
};
```

**Configured-key then delimiter-fallback pattern** (lines 12-46):
```rust
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
```

**Span/jump helper pattern** (lines 126-159, 197-225):
```rust
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
    // choose next stop, mutate caret, return bool consumed flag.
}
```

**Install binding pattern** (lines 228-252):
```rust
pub(crate) fn install_schema_navigator_bindings(session: &mut SessionState, schema_id: &str) {
    let schema_config =
        load_runtime_config_root(&format!("{schema_id}.schema"), ConfigOpenKind::Deployed);
    session.navigator_delimiters = find_config_value(&schema_config, "speller/delimiter")
        .and_then(config_scalar_string)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| " ".to_owned());
    // load horizontal and vertical binding sections.
}
```

**Apply to Phase 3:** preserve the `Option<bool>` contract: `None` means fall through to selector/session processing, `Some(false)` means explicit noop/reject, `Some(true)` means accepted. This is important for selector/navigator fallback parity.

---

### `crates/yune-rime-api/src/processors/selector.rs` (processor, event-driven)

**Analog:** `crates/yune-rime-api/src/processors/selector.rs`

**Imports pattern** (lines 1-10):
```rust
use std::{collections::HashMap, os::raw::c_int};

use serde_yaml::Value;
use yune_core::{KeyCode, KeyEvent};

use crate::{
    config_scalar_string, context_menu_settings, find_config_value, load_runtime_config_root,
    parse_single_key_binding_event, session_menu_page_size, ConfigOpenKind, SelectorBindingAction,
    SelectorLayoutAction, SessionState, XK_DOWN, XK_KP_DOWN, XK_KP_LEFT, XK_KP_PAGE_DOWN,
    XK_KP_PAGE_UP, XK_KP_RIGHT, XK_KP_UP, XK_LEFT, XK_PAGE_DOWN, XK_PAGE_UP, XK_RIGHT, XK_UP,
};
```

**Raw-tag exclusion and fallback pattern** (lines 13-49):
```rust
pub(crate) fn process_selector_layout_key(
    session: &mut SessionState,
    key_event: KeyEvent,
    keycode: c_int,
    mask: c_int,
) -> Option<bool> {
    if session.engine.context().composition.input.is_empty()
        || session.engine.context().candidates.is_empty()
        || session
            .engine
            .context()
            .segment_tags
            .iter()
            .any(|tag| tag == "raw")
    {
        return None;
    }
    // configured binding first; then default layout keymap if mask == 0.
}
```

**Linear-layout navigator handoff pattern** (lines 117-148):
```rust
fn selector_previous_candidate_like_librime(
    session: &mut SessionState,
    is_linear: bool,
) -> Option<bool> {
    let context = session.engine.context();
    if is_linear && context.composition.caret < context.composition.input.len() {
        return None;
    }
    let highlighted = context.highlighted;
    if highlighted == 0 {
        return (!is_linear).then_some(true);
    }
    session.engine.highlight_candidate(highlighted - 1);
    session.paging = true;
    Some(true)
}
```

**Alternative select pattern** (lines 254-288):
```rust
pub(crate) fn process_alternative_select_key(
    session: &mut SessionState,
    key_event: KeyEvent,
) -> Option<Option<String>> {
    if key_event.modifiers.control
        || key_event.modifiers.alt
        || key_event.modifiers.super_key
        || key_event.modifiers.release
        || session.engine.context().candidates.is_empty()
    {
        return None;
    }
    // map configured select_keys to current page and return optional commit.
}
```

**Apply to Phase 3:** copy the raw-tag exclusion and linear-layout `None` fallback behavior when adding selector/navigator span tests.

---

### `crates/yune-rime-api/src/processors/chord_composer.rs` (processor, event-driven)

**Analog:** `crates/yune-rime-api/src/processors/chord_composer.rs`

**Imports pattern** (lines 1-10):
```rust
use std::collections::{HashMap, HashSet};

use regex::Regex;
use serde_yaml::Value;
use yune_core::{parse_key_sequence, KeyCode, KeyEvent, KeyModifiers};

use crate::{
    config_scalar_bool, config_scalar_string, find_config_value, load_runtime_config_root,
    parse_single_key_binding_event, schema_engine_processors_include, schema_string_list,
    ConfigOpenKind, SessionKeyProcessResult, SessionState,
};
```

**Stateful processor model pattern** (lines 12-46):
```rust
pub(crate) struct ChordComposerProcessor {
    alphabet: Vec<char>,
    algebra: ChordProjection,
    output_format: ChordProjection,
    prompt_format: ChordProjection,
    bindings: HashMap<KeyEvent, ChordComposerBindingAction>,
    use_control: bool,
    use_alt: bool,
    use_shift: bool,
    use_super: bool,
    use_caps: bool,
    raw_sequence: String,
    pressed_keys: HashSet<char>,
    recognized_chord: HashSet<char>,
    prompt: Option<String>,
    finish_on_first_release: bool,
    was_composing: bool,
}
```

**Install pattern** (lines 176-234):
```rust
pub(crate) fn install_schema_chord_composer_processor(session: &mut SessionState, schema_id: &str) {
    let schema_config =
        load_runtime_config_root(&format!("{schema_id}.schema"), ConfigOpenKind::Deployed);
    if !schema_engine_processors_include(&schema_config, "chord_composer") {
        return;
    }

    let alphabet = find_config_value(&schema_config, "chord_composer/alphabet")
        .and_then(config_scalar_string)
        .unwrap_or_default()
        .chars()
        .collect::<Vec<_>>();
    if alphabet.is_empty() {
        return;
    }

    session.engine.set_option("_chord_typing", true);
    session.chord_composer = Some(ChordComposerProcessor { /* config fields */ });
}
```

**Lifecycle cleanup pattern** (lines 270-281):
```rust
pub(crate) fn sync_chord_composer_context_update(session: &mut SessionState) {
    let Some(composer) = session.chord_composer.as_mut() else {
        return;
    };
    let is_composing =
        !session.engine.context().composition.input.is_empty() || composer.prompt.is_some();
    if is_composing {
        composer.was_composing = true;
    } else if composer.was_composing {
        composer.was_composing = false;
        composer.raw_sequence.clear();
    }
}
```

**Core key/release pattern** (lines 284-353):
```rust
pub(crate) fn process_chord_composer_processor(
    session: &mut SessionState,
    key_event: KeyEvent,
) -> Option<SessionKeyProcessResult> {
    if session.engine.get_option("ascii_mode") {
        return None;
    }
    let composer = session.chord_composer.as_ref()?;

    if let Some(action) = composer.bindings.get(&key_event).copied() {
        return Some(apply_chord_composer_binding(session, action));
    }

    // clear on cancel/non-character, track press/release sets, serialize chord output.
}
```

**Apply to Phase 3:** use `sync_chord_composer_context_update` and `clear_chord_state` style for raw-sequence lifecycle cleanup tests/fixes.

---

### `crates/yune-rime-api/src/processors/shape.rs` (processor, event-driven)

**Analog:** `crates/yune-rime-api/src/processors/shape.rs`

**Imports and event pattern** (lines 1-23):
```rust
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
```

**Core transform pattern** (lines 25-38):
```rust
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
```

**Apply to Phase 3:** commit formatting should use this helper through `append_unread_commit` rather than duplicating shape conversion.

---

### `crates/yune-rime-api/src/processors/punctuation.rs` (processor, event-driven)

**Analog:** `crates/yune-rime-api/src/processors/punctuation.rs`

**Imports pattern** (lines 1-10):
```rust
use std::collections::HashMap;

use serde_yaml::Value;
use yune_core::{KeyCode, KeyEvent, PunctuationTranslator};

use crate::{
    config_scalar_bool, config_scalar_string, ends_with_ascii_digit, find_config_value,
    load_runtime_config_root, schema_engine_processors_include, schema_engine_translators_include,
    shape_formatted_ascii_text, ConfigOpenKind, PunctuationProcessResult, PunctuationProcessor,
    SessionState,
};
```

**Translator install pattern** (lines 13-33):
```rust
pub(crate) fn install_schema_punctuation_translator_from_config(
    session: &mut SessionState,
    schema_config: &Value,
) {
    let half_shape_entries = punctuation_entries_from_config(schema_config, "half_shape");
    let full_shape_entries = punctuation_entries_from_config(schema_config, "full_shape");
    let symbol_entries = punctuation_entries_from_config(schema_config, "symbols");
    if half_shape_entries.is_empty() && full_shape_entries.is_empty() && symbol_entries.is_empty() {
        return;
    }
    let translator = PunctuationTranslator::with_shape_and_symbol_entries(
        half_shape_entries,
        full_shape_entries,
        symbol_entries,
    );
    let translator = if session.punct_segmentor.is_some() {
        translator.with_required_tags(["punct", "punct_number"])
    } else {
        translator
    };
    session.engine.add_translator(translator);
}
```

**Processor install pattern** (lines 36-97):
```rust
pub(crate) fn install_schema_punctuation_processor(session: &mut SessionState, schema_id: &str) {
    let schema_config =
        load_runtime_config_root(&format!("{schema_id}.schema"), ConfigOpenKind::Deployed);
    if !schema_engine_processors_include(&schema_config, "punctuator")
        || !schema_engine_translators_include(&schema_config, "punct_translator")
    {
        return;
    }
    // load digit separator, alternating, unique, and pair maps; install only if non-empty.
}
```

**Core punctuation event pattern** (lines 249-305):
```rust
pub(crate) fn process_punctuation_processor(
    session: &mut SessionState,
    key_event: KeyEvent,
) -> Option<PunctuationProcessResult> {
    if key_event.modifiers.control
        || key_event.modifiers.alt
        || key_event.modifiers.super_key
        || key_event.modifiers.release
        || session.engine.get_option("ascii_punct")
    {
        return None;
    }

    let KeyCode::Character(ch) = key_event.code else {
        return None;
    };
    if !ch.is_ascii() || ch.is_ascii_control() {
        return None;
    }
    // pending digit separator, use_space, pair, alternating, commit lookup.
}
```

**Apply to Phase 3:** copy this split between translator installation and key-event processor for punctuation segment ordering and `punct_number`/fallback interactions.

---

### `crates/yune-rime-api/src/schema_install.rs` (service/config, transform)

**Analog:** `crates/yune-rime-api/src/schema_install.rs`

**Imports pattern** (lines 1-17):
```rust
use std::{collections::HashSet, fs, os::raw::c_int};

use regex::Regex;
use serde_yaml::{Mapping, Value};
use yune_core::{
    CharsetFilter, HistoryTranslator, ReverseLookupFilter, ReverseLookupTranslator,
    SchemaListTranslator, SimplifierFilter, SingleCharFilter, StaticTableTranslator,
    SwitchTranslator, TableDictionary, TaggedFilter, UniquifierFilter,
};

use crate::{
    config_scalar_bool, config_scalar_double, config_scalar_int, config_scalar_string,
    ends_with_ascii_digit, find_config_value, install_schema_punctuation_translator_from_config,
    load_runtime_config_root, resource_id::validate_data_resource_id, schema_folded_switch_options,
    schema_list_translator_entries_for_current, schema_switch_translator_switches,
    selected_runtime_data_path, switch_scalar_field, AffixSegmentor, ConfigOpenKind,
    MatcherPattern, MatcherSegmentor, PunctSegmentor, SessionState,
};
```

**Translator-chain recognition pattern** (lines 20-75):
```rust
pub(crate) fn install_schema_translator_chain(session: &mut SessionState, schema_id: &str) {
    let schema_config =
        load_runtime_config_root(&format!("{schema_id}.schema"), ConfigOpenKind::Deployed);
    let Some(Value::Sequence(translators)) =
        find_config_value(&schema_config, "engine/translators")
    else {
        return;
    };
    let mut punctuation_translator_installed = false;

    for translator in translators.iter().filter_map(Value::as_str) {
        let (component_name, name_space) = schema_component_prescription(translator);
        match component_name {
            "punct_translator" if !punctuation_translator_installed => { /* install once */ }
            "table_translator" | "script_translator" | "r10n_translator" => { /* table */ }
            "reverse_lookup_translator" => { /* reverse lookup */ }
            "history_translator" => { /* history */ }
            "switch_translator" => { /* switch */ }
            "schema_list_translator" => { /* schema list */ }
            _ => {}
        }
    }
}
```

**Component prescription pattern** (lines 78-86):
```rust
pub(crate) fn schema_component_prescription(component: &str) -> (&str, Option<&str>) {
    let Some((component_name, name_space)) = component.split_once('@') else {
        return (component, None);
    };
    if component_name.is_empty() || name_space.is_empty() {
        (component, None)
    } else {
        (component_name, Some(name_space))
    }
}
```

**Filter-chain pattern** (lines 236-271):
```rust
pub(crate) fn install_schema_filter_chain(session: &mut SessionState, schema_id: &str) {
    let schema_config =
        load_runtime_config_root(&format!("{schema_id}.schema"), ConfigOpenKind::Deployed);
    let Some(Value::Sequence(filters)) = find_config_value(&schema_config, "engine/filters") else {
        return;
    };
    for filter in filters.iter().filter_map(Value::as_str) {
        let (filter_name, name_space) = schema_component_prescription(filter);
        match filter_name {
            "reverse_lookup_filter" => install_schema_reverse_lookup_filter_from_config(...),
            "simplifier" => install_schema_simplifier_filter_from_config(...),
            "uniquifier" => session.engine.add_filter(UniquifierFilter),
            "single_char_filter" => session.engine.add_filter(SingleCharFilter),
            "charset_filter" | "cjk_minifier" => { /* tagged filter */ }
            _ => {}
        }
    }
}
```

**Segment-tag update pattern** (lines 458-506, 625-679):
```rust
pub(crate) fn install_schema_segment_tags(session: &mut SessionState, schema_id: &str) {
    let schema_config =
        load_runtime_config_root(&format!("{schema_id}.schema"), ConfigOpenKind::Deployed);
    let mut tags = vec!["abc".to_owned()];
    session.affix_segmentors.clear();
    session.matcher_segmentor = None;
    session.ascii_segmentor_enabled = false;
    session.punct_segmentor = None;
    session.fallback_segmentor_enabled = false;
    // parse engine/segmentors and update session fields.
    session.base_segment_tags = tags;
    update_session_segment_tags(session);
}
```

**Validation/error handling pattern** (lines 390-414, 602-622):
```rust
let dictionary_name = find_config_value(schema_config, &format!("{name_space}/dictionary"))
    .and_then(config_scalar_string)
    .and_then(|dictionary_name| validate_data_resource_id(&dictionary_name))?;
let dictionary_path = selected_runtime_data_path(&format!("{dictionary_name}.dict.yaml"))?;
let dictionary_yaml = fs::read_to_string(dictionary_path).ok()?;
```

**Apply to Phase 3:** remaining gears (`memory`, `poet`/`grammar`, `contextual_translation`, `unity_table_encoder`) should be recognized or explicitly deferred here, not silently hidden behind the `_ => {}` match arms. Keep resource IDs validated with `validate_data_resource_id` before filesystem access.

---

### `crates/yune-rime-api/src/schema_selection.rs` (service/config, request-response)

**Analog:** `crates/yune-rime-api/src/schema_selection.rs`

**Imports pattern** (lines 1-15):
```rust
use std::{ffi::CStr, fs, os::raw::c_char};

use serde_yaml::Value;

use crate::{
    apply_schema_switch_resets, config_scalar_bool, copy_c_string_with_strncpy_semantics,
    deployed_schema_name, find_config_value, install_schema_ascii_composer_processor,
    install_schema_chord_composer_processor, install_schema_editor_processor,
    install_schema_filter_chain, install_schema_key_binder_processor,
    install_schema_navigator_bindings, install_schema_punctuation_processor,
    install_schema_recognizer_processor, install_schema_segment_tags,
    install_schema_selector_bindings, install_schema_speller_processor,
    install_schema_translator_chain, load_runtime_config_root, notify, schema_string_list,
    selected_runtime_config_path, sessions, with_session, Bool, ConfigOpenKind, NavigatorBindings,
    NavigatorSyllableJumpPosition, RimeSessionId, SelectorBindings, SessionState, FALSE, TRUE,
};
```

**ABI safety pattern** (lines 24-38, 48-63):
```rust
#[no_mangle]
pub unsafe extern "C" fn RimeSelectSchema(
    session_id: RimeSessionId,
    schema_id: *const c_char,
) -> Bool {
    if schema_id.is_null() {
        return FALSE;
    }
    // SAFETY: callers promise that `schema_id` is a valid nul-terminated
    // string.
    let schema_id = unsafe { CStr::from_ptr(schema_id) }
        .to_string_lossy()
        .into_owned();

    let selected = with_session(session_id, |session| {
        apply_schema_to_session(session, &schema_id);
        true
    });
    // notify on success.
    selected
}
```

**Schema reset/install ordering pattern** (lines 83-125):
```rust
pub(crate) fn apply_schema_to_session(session: &mut SessionState, schema_id: &str) {
    let schema_name = deployed_schema_name(schema_id);
    session.engine.set_schema(schema_id.to_owned(), schema_name);
    session.engine.reset_translators();
    session.engine.reset_filters();
    session.key_binder = None;
    session.speller = None;
    session.editor_processor = None;
    session.editor_bindings.clear();
    session.editor_char_handler = None;
    session.chord_composer = None;
    session.engine.set_option("_auto_commit", false);
    // reset all session-local processors/segmentors/bindings.
    restore_switcher_saved_options(session, schema_id);
    apply_schema_switch_resets(session, schema_id);
    install_schema_segment_tags(session, schema_id);
    install_schema_editor_processor(session, schema_id);
    install_schema_chord_composer_processor(session, schema_id);
    install_schema_ascii_composer_processor(session, schema_id);
    install_schema_speller_processor(session, schema_id);
    install_schema_recognizer_processor(session, schema_id);
    install_schema_selector_bindings(session, schema_id);
    install_schema_navigator_bindings(session, schema_id);
    install_schema_key_binder_processor(session, schema_id);
    install_schema_punctuation_processor(session, schema_id);
    install_schema_translator_chain(session, schema_id);
    install_schema_filter_chain(session, schema_id);
    session.engine.clear_composition();
    session.input_buffer = None;
    session.unread_commit = None;
}
```

**Apply to Phase 3:** add new processor/gear install hooks only through this reset/install flow. Do not mutate installed components from unrelated ABI calls.

---

### `crates/yune-rime-api/src/session.rs` (model/store, request-response)

**Analog:** `crates/yune-rime-api/src/session.rs`

**Session state ownership pattern** (lines 66-93):
```rust
pub(crate) struct SessionState {
    pub(crate) engine: Engine,
    pub(crate) unread_commit: Option<String>,
    pub(crate) input_buffer: Option<CString>,
    pub(crate) key_binder: Option<KeyBinderProcessor>,
    pub(crate) speller: Option<SpellerProcessor>,
    pub(crate) editor_processor: Option<EditorProcessor>,
    pub(crate) editor_bindings: HashMap<KeyEvent, EditorBindingAction>,
    pub(crate) editor_char_handler: Option<EditorCharHandler>,
    pub(crate) chord_composer: Option<ChordComposerProcessor>,
    pub(crate) ascii_composer_enabled: bool,
    pub(crate) punctuation_processor: Option<PunctuationProcessor>,
    pub(crate) recognizer_processor: Option<RecognizerProcessor>,
    pub(crate) selector_bindings: SelectorBindings,
    pub(crate) navigator_bindings: NavigatorBindings,
    pub(crate) navigator_delimiters: String,
    pub(crate) navigator_syllable_jump_position: NavigatorSyllableJumpPosition,
    pub(crate) base_segment_tags: Vec<String>,
    pub(crate) punct_segmentor: Option<PunctSegmentor>,
    pub(crate) affix_segmentors: Vec<AffixSegmentor>,
    pub(crate) matcher_segmentor: Option<MatcherSegmentor>,
    pub(crate) fallback_segmentor_enabled: bool,
    pub(crate) paging: bool,
    pub(crate) last_active_time: u64,
}
```

**Default initialization pattern** (lines 96-126):
```rust
impl SessionState {
    fn new() -> Self {
        Self {
            engine: Engine::default(),
            unread_commit: None,
            input_buffer: None,
            key_binder: None,
            speller: None,
            editor_processor: None,
            editor_bindings: HashMap::new(),
            editor_char_handler: None,
            chord_composer: None,
            // set explicit defaults for every schema-installed component.
            base_segment_tags: vec!["abc".to_owned()],
            fallback_segmentor_enabled: false,
            paging: false,
            last_active_time: session_activity_now(),
        }
    }
}
```

**Registry access pattern** (lines 196-211):
```rust
pub(crate) fn with_session(
    session_id: RimeSessionId,
    action: impl FnOnce(&mut SessionState) -> bool,
) -> Bool {
    if session_id == 0 {
        return FALSE;
    }

    let mut registry = sessions()
        .lock()
        .expect("session registry should not be poisoned");
    let Some(session) = registry.get_session_mut(session_id) else {
        return FALSE;
    };

    bool_from(action(session))
}
```

**Apply to Phase 3:** if a remaining gear needs state, add explicit `SessionState` fields with defaults and reset them in `apply_schema_to_session`.

---

### `crates/yune-rime-api/src/lib.rs` (route/façade, event-driven)

**Analog:** `crates/yune-rime-api/src/lib.rs`

**ABI event validation pattern** (lines 322-416):
```rust
#[no_mangle]
pub extern "C" fn RimeProcessKey(session_id: RimeSessionId, keycode: c_int, mask: c_int) -> Bool {
    if session_id == 0
        || (mask != 0
            && !(/* allowed librime-style masks/keycodes */))
    {
        return FALSE;
    }
    let mut registry = sessions()
        .lock()
        .expect("session registry should not be poisoned");
    let Some(session) = registry.get_session_mut(session_id) else {
        return FALSE;
    };

    let Some(key_event) = key_event_from_rime_keycode(keycode, mask) else {
        return FALSE;
    };
    // dispatch continues below.
}
```

**Pre-dispatch selector/navigator pattern** (lines 437-456):
```rust
let was_composing = !session.engine.context().composition.input.is_empty();
let mut accepted = false;
if let Some(selector_accepted) = process_selector_layout_key(session, key_event, keycode, mask)
{
    accepted = selector_accepted;
} else if let Some(navigator_accepted) = process_navigator_configured_key(session, key_event) {
    accepted = navigator_accepted;
} else if let Some(navigator_accepted) = process_navigator_delimiter_key(session, key_event) {
    accepted = navigator_accepted;
} else {
    // page keys or process_session_key_event.
}
```

**Session key dispatch order pattern** (lines 1415-1489):
```rust
pub(crate) fn process_session_key_event(
    session_id: RimeSessionId,
    session: &mut SessionState,
    key_event: KeyEvent,
) -> SessionKeyProcessResult {
    if let Some(result) = process_chord_composer_processor(session, key_event) {
        update_session_segment_tags(session);
        sync_chord_composer_context_update(session);
        return result;
    }
    if let Some(commits) = process_key_binder_processor(session_id, session, key_event) {
        update_session_segment_tags(session);
        return if commits.is_empty() {
            SessionKeyProcessResult::Accepted
        } else {
            SessionKeyProcessResult::Commit(commits.concat())
        };
    }
    if key_event.modifiers.release {
        return SessionKeyProcessResult::Noop;
    }
    if process_recognizer_processor(session, key_event) {
        update_session_segment_tags(session);
        return SessionKeyProcessResult::Accepted;
    }
    if let Some(result) = process_punctuation_processor(session, key_event) { /* ... */ }
    if let Some(commit) = process_alternative_select_key(session, key_event) { /* ... */ }
    if let Some(result) = process_speller_processor(session, key_event) { /* ... */ }
    if let Some(result) = process_editor_processor(session, key_event) { /* ... */ }
    let commit = session.engine.process_key_event(key_event);
    update_session_segment_tags(session);
    // map commit/input/highlight changes to SessionKeyProcessResult.
}
```

**Commit formatting pattern** (lines 1132-1142):
```rust
fn append_unread_commit(session: &mut SessionState, commit: String) {
    let commit = shape_formatted_commit_text(session, &commit);
    match &mut session.unread_commit {
        Some(buffer) => buffer.push_str(&commit),
        None => session.unread_commit = Some(commit),
    }
}

fn shape_formatted_commit_text(session: &SessionState, text: &str) -> String {
    shape_formatted_ascii_text(text, session.engine.status().is_full_shape)
}
```

**Apply to Phase 3:** dispatch order is fragile. Any edit to this file needs a focused ABI test first. Owned behavior should remain in processor/schema/core modules.

---

### `crates/yune-core/src/spelling_algebra.rs` (utility, transform)

**Analog:** `crates/yune-core/src/spelling_algebra.rs`

**Imports and model pattern** (lines 1-10):
```rust
use super::Candidate;
use regex::Regex;

#[derive(Clone, Default)]
pub(crate) struct SpellingAlgebra {
    formulas: Vec<SpellingAlgebraFormula>,
}

const SPELLING_ALGEBRA_FUZZY_PENALTY: f32 = -std::f32::consts::LN_2;
const SPELLING_ALGEBRA_ABBREVIATION_PENALTY: f32 = -std::f32::consts::LN_2;
const SPELLING_ALGEBRA_CORRECTION_PENALTY: f32 = -std::f32::consts::LN_10 * 2.0;
```

**Parse-all-or-default pattern** (lines 13-23):
```rust
impl SpellingAlgebra {
    pub(crate) fn parse(formulas: &[String]) -> Self {
        let mut parsed = Vec::new();
        for formula in formulas {
            let Some(parsed_formula) = SpellingAlgebraFormula::parse(formula) else {
                return Self::default();
            };
            parsed.push(parsed_formula);
        }
        Self { formulas: parsed }
    }
}
```

**Expansion/dedupe pattern** (lines 29-53, 204-223):
```rust
pub(crate) fn expand_entries(
    &self,
    mut entries: Vec<(String, Candidate)>,
) -> Vec<(String, Candidate)> {
    for formula in &self.formulas {
        let mut next = Vec::new();
        for (code, candidate) in entries {
            let mut transformed = code.clone();
            let applied = formula.apply(&mut transformed);
            if applied {
                if formula.keep_original() {
                    next.push((code, candidate.clone()));
                }
                if formula.add_transformed() && !transformed.is_empty() {
                    let mut candidate = candidate;
                    candidate.quality += formula.quality_penalty();
                    next.push((transformed, candidate));
                }
            } else {
                next.push((code, candidate));
            }
        }
        entries = dedupe_spelling_algebra_entries(next);
    }
    entries
}
```

**Formula parse pattern** (lines 71-84, 117-124):
```rust
pub(crate) fn parse(definition: &str) -> Option<Self> {
    let separator = definition.chars().find(|ch| !ch.is_ascii_lowercase())?;
    let args = definition.split(separator).collect::<Vec<_>>();
    match args.first().copied()? {
        "xlit" => Self::parse_xlit(&args),
        "xform" => Self::parse_transform(&args, false, true, 0.0),
        "derive" => Self::parse_derivation(&args),
        "fuzz" => Self::parse_transform(&args, true, true, SPELLING_ALGEBRA_FUZZY_PENALTY),
        "abbrev" => {
            Self::parse_transform(&args, true, true, SPELLING_ALGEBRA_ABBREVIATION_PENALTY)
        }
        "erase" => Self::parse_erase(&args),
        _ => None,
    }
}
```

**Apply to Phase 3:** broaden spelling algebra only with focused schema-visible lookup/ranking tests. Preserve fallible regex parse via `Regex::new(...).ok()?` and dedupe highest-quality candidate behavior.

---

### `crates/yune-core/src/translator/mod.rs` (service, transform)

**Analog:** `crates/yune-core/src/translator/mod.rs`

**Imports pattern** (lines 1-10):
```rust
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use crate::comment_format::CommentFormat;
use crate::dictionary::normalize_table_code;
use crate::filter::contains_extended_cjk;
use crate::spelling_algebra::SpellingAlgebra;
use crate::{
    Candidate, CandidateSource, Context, Status, TableDictionary, TableDictionaryParseError,
    TableEntry, Translator,
};
```

**Builder pattern for translators** (lines 34-45, 109-178):
```rust
pub struct StaticTableTranslator {
    entries: Vec<(String, Candidate)>,
    enable_completion: bool,
    enable_charset_filter: bool,
    enable_sentence: bool,
    sentence_over_completion: bool,
    tags: Vec<String>,
    delimiters: String,
    initial_quality: f32,
    comment_format: CommentFormat,
    dictionary_exclude: HashSet<String>,
}

#[must_use]
pub fn with_spelling_algebra(mut self, formulas: &[String]) -> Self {
    let algebra = SpellingAlgebra::parse(formulas);
    if !algebra.is_empty() {
        self.entries = algebra.expand_entries(self.entries);
    }
    self
}
```

**Segment-tag gated lookup pattern** (lines 188-249):
```rust
fn accepts_segment_tags(&self, segment_tags: &[String]) -> bool {
    self.tags
        .iter()
        .any(|tag| segment_tags.iter().any(|segment_tag| segment_tag == tag))
}

fn translated_candidates_for_segment(
    &self,
    input: &str,
    filter_by_charset: bool,
    segment_tags: Option<&[String]>,
) -> Vec<Candidate> {
    let accepts_segment = segment_tags
        .map(|tags| self.accepts_segment_tags(tags))
        .unwrap_or_else(|| self.accepts_default_segment());
    if !accepts_segment {
        return Vec::new();
    }
    // lookup, completion, sentence candidate construction.
}
```

**Context-aware trait pattern** (lines 389-423):
```rust
impl Translator for StaticTableTranslator {
    fn name(&self) -> &'static str {
        "static_table_translator"
    }

    fn translate_with_context(
        &self,
        input: &str,
        _status: &Status,
        options: &HashMap<String, bool>,
        context: &Context,
    ) -> Vec<Candidate> {
        let filter_by_charset = self.enable_charset_filter
            && !options.get("extended_charset").copied().unwrap_or(false);
        self.translated_candidates_for_segment(
            input,
            filter_by_charset,
            Some(&context.segment_tags),
        )
    }
}
```

**No-op compatibility increment pattern** (lines 553-636):
```rust
impl Translator for HistoryTranslator {
    fn name(&self) -> &'static str {
        "history_translator"
    }

    fn translate(&self, _input: &str) -> Vec<Candidate> {
        Vec::new()
    }

    fn translate_with_context(
        &self,
        input: &str,
        _status: &Status,
        _options: &HashMap<String, bool>,
        context: &Context,
    ) -> Vec<Candidate> {
        if !self.accepts_segment_tags(&context.segment_tags)
            || self.input.is_empty()
            || self.input != input
        {
            return Vec::new();
        }
        // deterministic history candidates from context, no external storage.
    }
}
```

**Apply to Phase 3:** candidate weighting/annotation increments for remaining gears should follow existing trait/builders and return deterministic empty vectors when behavior is unsupported or out of scope.

---

### `crates/yune-core/src/filter/mod.rs` (service, transform)

**Analog:** `crates/yune-core/src/filter/mod.rs`

**Imports and filter trait implementation pattern** (lines 1-14):
```rust
use super::{Candidate, CandidateFilter, CandidateSource, CommentFormat, Context, TableDictionary};
use std::collections::{HashMap, HashSet};

pub struct UniquifierFilter;

impl CandidateFilter for UniquifierFilter {
    fn name(&self) -> &'static str {
        "uniquifier"
    }

    fn apply(&self, candidates: &mut Vec<Candidate>) {
        let mut seen = HashSet::new();
        candidates.retain(|candidate| seen.insert(candidate.text.clone()));
    }
}
```

**Tagged wrapper pattern** (lines 65-114):
```rust
pub struct TaggedFilter {
    filter: Box<dyn CandidateFilter>,
    tags: Vec<String>,
}

impl CandidateFilter for TaggedFilter {
    fn name(&self) -> &'static str {
        self.filter.name()
    }

    fn apply_with_context(
        &self,
        candidates: &mut Vec<Candidate>,
        options: &HashMap<String, bool>,
        context: &Context,
    ) {
        if self.accepts_segment_tags(&context.segment_tags) {
            self.filter.apply_with_context(candidates, options, context);
        }
    }
}
```

**Simplifier builder pattern** (lines 141-239):
```rust
pub struct SimplifierFilter {
    option_name: String,
    conversion: SimplifierConversion,
    tips_level: SimplifierTipsLevel,
    show_in_comment: bool,
    inherit_comment: bool,
    comment_format: CommentFormat,
    excluded_types: HashSet<String>,
}

impl SimplifierFilter {
    #[must_use]
    pub fn with_opencc_config(mut self, opencc_config: impl AsRef<str>) -> Self {
        self.conversion = SimplifierConversion::from_opencc_config(opencc_config.as_ref());
        self
    }
}
```

**OpenCC-config mapping and apply pattern** (lines 242-327):
```rust
fn from_opencc_config(opencc_config: &str) -> Self {
    let config_name = opencc_config
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(opencc_config)
        .to_ascii_lowercase();
    let config_stem = config_name.strip_suffix(".json").unwrap_or(&config_name);
    match config_stem {
        "" | "t2s" | "hk2s" => Self::TraditionalToSimplified,
        "s2t" => Self::SimplifiedToTraditional,
        "t2tw" => Self::TraditionalToTaiwan,
        // unknown conversion data is explicitly None.
        _ => Self::None,
    }
}
```

**Apply to Phase 3:** OpenCC work should be framed as filter-chain integration/focused conversion behavior, not full OpenCC data parity. Preserve `excluded_types`, `tips`, and comment formatting behavior.

---

### `crates/yune-rime-api/src/tests/schema_processors.rs` (test, request-response)

**Analog:** `crates/yune-rime-api/src/tests/schema_processors.rs`

**Shared imports/helper source** (from `src/tests/mod.rs` lines 1-10, 94-104, 330-339):
```rust
use std::env;
use std::ffi::{c_void, CStr, CString};
use std::fs;
use std::os::raw::{c_char, c_int};
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

fn test_guard() -> MutexGuard<'static, ()> {
    static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    let guard = TEST_LOCK.get_or_init(|| Mutex::new(())).lock().expect("test lock should not be poisoned");
    let traits = empty_traits();
    unsafe { RimeInitialize(&traits) };
    guard
}

fn unique_temp_dir(name: &str) -> std::path::PathBuf {
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH).expect("clock should be after epoch").as_nanos();
    env::temp_dir().join(format!("yune-rime-api-{name}-{}-{nonce}", std::process::id()))
}
```

**ABI-facing temp schema fixture pattern** (lines 5590-5661):
```rust
#[test]
fn schema_express_editor_return_commits_raw_input() {
    let _guard = test_guard();
    RimeCleanupAllSessions();
    let root = unique_temp_dir("schema-express-editor-return");
    let shared = root.join("shared");
    let user = root.join("user");
    let staging = user.join("build");
    fs::create_dir_all(&shared).expect("shared dir should be created");
    fs::create_dir_all(&staging).expect("staging dir should be created");
    fs::write(staging.join("fluid.schema.yaml"), "...schema yaml...")
        .expect("fluid schema config should be written");
    fs::write(shared.join("luna.dict.yaml"), "...dict yaml...")
        .expect("dictionary should be written");

    let shared_c = CString::new(shared.to_string_lossy().as_ref()).expect("path is valid");
    let user_c = CString::new(user.to_string_lossy().as_ref()).expect("path is valid");
    let mut traits = empty_traits();
    traits.shared_data_dir = shared_c.as_ptr();
    traits.user_data_dir = user_c.as_ptr();
    unsafe { RimeSetup(&traits) };
    // create session, select schema, drive RimeProcessKey, assert output.
}
```

**Context/candidate assertion pattern** (lines 4756-4781):
```rust
assert_eq!(RimeProcessKey(session_id, 'a' as i32, 0), TRUE);
let mut context = empty_context();
assert_eq!(unsafe { RimeGetContext(session_id, &mut context) }, TRUE);
let candidates = unsafe {
    std::slice::from_raw_parts(
        context.menu.candidates,
        context.menu.num_candidates as usize,
    )
};
let texts = candidates
    .iter()
    .map(|candidate| unsafe { CStr::from_ptr(candidate.text) }
        .to_str()
        .expect("candidate text should be valid UTF-8")
        .to_owned())
    .collect::<Vec<_>>();
assert_eq!(unsafe { RimeFreeContext(&mut context) }, TRUE);
assert_eq!(texts, ["a".to_owned()]);
```

**Navigator focused ABI pattern** (lines 2328-2363):
```rust
let left = CString::new("Left").expect("key name should be valid");
let left_keycode = unsafe { RimeGetKeycodeByName(left.as_ptr()) };
let session_id = RimeCreateSession();
let input = CString::new("nix").expect("input should be valid");
assert_eq!(unsafe { RimeSetInput(session_id, input.as_ptr()) }, TRUE);
assert_eq!(RimeGetCaretPos(session_id), 3);
assert_eq!(RimeProcessKey(session_id, left_keycode, 0), TRUE);
assert_eq!(RimeGetCaretPos(session_id), 2);
```

**Punctuation/segmentor fixture pattern** (lines 4411-4507, 4516-4602):
```rust
fn schema_punctuator_candidates_expose_librime_shape_comments() {
    // schema YAML installs punct_translator and echo_translator,
    // then candidate_comments() drives RimeProcessKey and reads context comments.
    assert_eq!(candidate_comments(), [
        Some("〔半角〕".to_owned()),
        Some("〔全角〕".to_owned()),
        None,
        Some("echo".to_owned())
    ]);
}
```

**Chord fixture pattern** (lines 4790-4889):
```rust
fn schema_chord_composer_serializes_chord_on_key_release() {
    // write chord.schema.yaml and chord.dict.yaml, select schema through ABI.
    assert_eq!(RimeProcessKey(session_id, 'a' as i32, 0), TRUE);
    assert_eq!(RimeProcessKey(session_id, 'b' as i32, 0), TRUE);
    assert_eq!(current_input(), "");
    assert_eq!(RimeProcessKey(session_id, 'b' as i32, K_RELEASE_MASK), TRUE);
    assert_eq!(current_input(), "");
    assert_eq!(RimeProcessKey(session_id, 'a' as i32, K_RELEASE_MASK), TRUE);
    assert_eq!(current_input(), "yx");
}
```

**Apply to Phase 3:** every processor/segmentor change should begin with this ABI fixture pattern: `test_guard()`, unique temp dirs, schema/dict YAML, `RimeSetup`, `RimeCreateSession`, `RimeSelectSchema`, `RimeProcessKey`, then context/status/commit assertions and cleanup.

---

### `crates/yune-rime-api/src/tests/schema_selection.rs` (test, request-response)

**Analog:** `crates/yune-rime-api/src/tests/schema_selection.rs`

**Simplifier/filter schema fixture pattern** (lines 3824-3953):
```rust
fn select_schema_loads_librime_simplifier_filter() {
    let _guard = test_guard();
    RimeCleanupAllSessions();
    let root = unique_temp_dir("schema-simplifier-filter");
    let shared = root.join("shared");
    let user = root.join("user");
    let staging = user.join("build");
    fs::create_dir_all(&shared).expect("shared dir should be created");
    fs::create_dir_all(&staging).expect("staging dir should be created");
    fs::write(
        staging.join("luna.schema.yaml"),
        "\
schema:
  schema_id: luna
  name: Luna
engine:
  translators:
    - table_translator
  filters:
    - simplifier@zh_simp
translator:
  dictionary: luna
zh_simp:
  option_name: zh_simp
  tips: all
  comment_format:
    - xform/^/〔/
    - xform/$/〕/
",
    ).expect("schema config should be written");
    // write dict, setup traits, select schema, drive keys, inspect candidates.
}
```

**Candidate text/comment pair assertion pattern** (lines 3890-3924, 3935-3946):
```rust
let candidate_pairs = || {
    let mut context = empty_context();
    assert_eq!(unsafe { RimeGetContext(session_id, &mut context) }, TRUE);
    let candidates = unsafe {
        std::slice::from_raw_parts(
            context.menu.candidates,
            context.menu.num_candidates as usize,
        )
    };
    let texts = candidates
        .iter()
        .map(|candidate| {
            let text = unsafe { CStr::from_ptr(candidate.text) }
                .to_str()
                .expect("candidate text should be valid UTF-8")
                .to_owned();
            let comment = if candidate.comment.is_null() { String::new() } else { /* read */ };
            (text, comment)
        })
        .collect::<Vec<_>>();
    assert_eq!(unsafe { RimeFreeContext(&mut context) }, TRUE);
    texts
};

let option = CString::new("zh_simp").expect("option name should be valid");
unsafe { RimeSetOption(session_id, option.as_ptr(), TRUE) };
assert_eq!(candidate_pairs(), [
    ("台湾".to_owned(), "〔臺灣〕".to_owned()),
    ("龙马".to_owned(), "〔龍馬〕".to_owned()),
    ("tw".to_owned(), "echo".to_owned())
]);
```

**Core spelling-algebra unit analog** (from `crates/yune-core/src/lib.rs` lines 4089-4160):
```rust
let formulas = vec![
    "xlit/zyx/abc/".to_owned(),
    "xform/^lue$/lve/".to_owned(),
    "derive/^nv$/nu/".to_owned(),
    "fuzz/^bing$/pin/".to_owned(),
    "abbrev/^chang$/c/".to_owned(),
    "derive/^cuo$/cu/correction".to_owned(),
    "erase/^gone$/".to_owned(),
];
let translator =
    StaticTableTranslator::from_dictionary(dictionary).with_spelling_algebra(&formulas);
assert_eq!(translator.translate("lue").len(), 0);
assert_eq!(translator.translate("cu")[0].text, "错");
```

**Apply to Phase 3:** schema selection tests should own chain installation/filter/translator behavior; core unit tests should own pure spelling algebra transformations. Prefer adding ABI/schema tests for schema-visible lookup/ranking before core changes.

## Shared Patterns

### Schema installation through `apply_schema_to_session`
**Source:** `crates/yune-rime-api/src/schema_selection.rs` lines 83-125  
**Apply to:** all schema-loaded processor, segmentor, translator, filter, and remaining-gear changes
```rust
restore_switcher_saved_options(session, schema_id);
apply_schema_switch_resets(session, schema_id);
install_schema_segment_tags(session, schema_id);
install_schema_editor_processor(session, schema_id);
install_schema_chord_composer_processor(session, schema_id);
install_schema_ascii_composer_processor(session, schema_id);
install_schema_speller_processor(session, schema_id);
install_schema_recognizer_processor(session, schema_id);
install_schema_selector_bindings(session, schema_id);
install_schema_navigator_bindings(session, schema_id);
install_schema_key_binder_processor(session, schema_id);
install_schema_punctuation_processor(session, schema_id);
install_schema_translator_chain(session, schema_id);
install_schema_filter_chain(session, schema_id);
```

### Event dispatch ownership and ordering
**Source:** `crates/yune-rime-api/src/lib.rs` lines 1415-1489  
**Apply to:** any changes touching `RimeProcessKey`, selector/navigator fallback, punctuation/speller/editor ordering, chord lifecycle, or segment-tag updates
```rust
if let Some(result) = process_chord_composer_processor(session, key_event) { /* first */ }
if let Some(commits) = process_key_binder_processor(session_id, session, key_event) { /* second */ }
if key_event.modifiers.release { return SessionKeyProcessResult::Noop; }
if process_recognizer_processor(session, key_event) { /* before punctuation */ }
if let Some(result) = process_punctuation_processor(session, key_event) { /* before selector/speller */ }
if let Some(commit) = process_alternative_select_key(session, key_event) { /* before speller */ }
if let Some(result) = process_speller_processor(session, key_event) { /* before editor */ }
if let Some(result) = process_editor_processor(session, key_event) { return result; }
let commit = session.engine.process_key_event(key_event);
```

### Config validation and safe fallbacks
**Source:** `crates/yune-rime-api/src/schema_install.rs` lines 390-414, 602-622  
**Apply to:** schema component recognition, dictionary/vocabulary loads, regex-based recognizer/spelling/chord formulas
```rust
let dictionary_name = find_config_value(schema_config, &format!("{name_space}/dictionary"))
    .and_then(config_scalar_string)
    .and_then(|dictionary_name| validate_data_resource_id(&dictionary_name))?;
let dictionary_path = selected_runtime_data_path(&format!("{dictionary_name}.dict.yaml"))?;
let dictionary_yaml = fs::read_to_string(dictionary_path).ok()?;
```

### ABI-facing test harness
**Source:** `crates/yune-rime-api/src/tests/mod.rs` lines 94-104 and 330-339; `schema_processors.rs` lines 5590-5719  
**Apply to:** all SCHEMA-01 through SCHEMA-05 compatibility slices
```rust
let _guard = test_guard();
RimeCleanupAllSessions();
let root = unique_temp_dir("schema-depth-case");
let shared = root.join("shared");
let user = root.join("user");
let staging = user.join("build");
fs::create_dir_all(&shared).expect("shared dir should be created");
fs::create_dir_all(&staging).expect("staging dir should be created");
// write deployed schema/dict YAML, RimeSetup, RimeCreateSession, RimeSelectSchema,
// RimeProcessKey, RimeGetContext/RimeGetStatus/RimeGetCommit, free allocated structs.
```

### Candidate assertion and memory-free pattern
**Source:** `crates/yune-rime-api/src/tests/schema_processors.rs` lines 4756-4781 and `schema_selection.rs` lines 3890-3924  
**Apply to:** tests asserting candidates, comments, segment tags, highlight/page state
```rust
let mut context = empty_context();
assert_eq!(unsafe { RimeGetContext(session_id, &mut context) }, TRUE);
let candidates = unsafe {
    std::slice::from_raw_parts(context.menu.candidates, context.menu.num_candidates as usize)
};
let texts = candidates.iter().map(|candidate| {
    unsafe { CStr::from_ptr(candidate.text) }
        .to_str()
        .expect("candidate text should be valid UTF-8")
        .to_owned()
}).collect::<Vec<_>>();
assert_eq!(unsafe { RimeFreeContext(&mut context) }, TRUE);
```

### Structured deferral / deterministic no-op for unsupported gears
**Source:** `crates/yune-core/src/translator/mod.rs` lines 599-636; `crates/yune-rime-api/src/schema_install.rs` lines 20-75 and 236-271  
**Apply to:** `memory`, `poet`/`grammar`, `contextual_translation`, `unity_table_encoder`
```rust
impl Translator for HistoryTranslator {
    fn name(&self) -> &'static str {
        "history_translator"
    }

    fn translate(&self, _input: &str) -> Vec<Candidate> {
        Vec::new()
    }

    fn translate_with_context(...) -> Vec<Candidate> {
        if !self.accepts_segment_tags(&context.segment_tags)
            || self.input.is_empty()
            || self.input != input
        {
            return Vec::new();
        }
        // only produce deterministic candidates from available in-memory context.
    }
}
```

## No Analog Found

No Phase 3 source/test file is without a close in-repository analog. Distribution comparison storage path is intentionally not fixed by CONTEXT.md; if the planner creates a new checked-in findings document or comparison utility, use the ABI-facing test harness and structured deferral patterns above, and avoid broad generated snapshots.

## Metadata

**Analog search scope:** `/Users/trenton/Projects/yune/.claude/worktrees/agent-af5dafb83714f8d79/crates/**/*.rs`, plus Phase 3 context/research from `/Users/trenton/Projects/yune/.planning/phases/03-schema-pipeline-depth/`  
**Files scanned:** 70 Rust source/test files listed under `crates/`; 14 primary analogs read/extracted  
**Pattern extraction date:** 2026-04-29
