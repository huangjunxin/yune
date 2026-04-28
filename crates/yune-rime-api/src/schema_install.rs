use std::fs;

use serde_yaml::Value;
use yune_core::{
    CharsetFilter, HistoryTranslator, ReverseLookupFilter, ReverseLookupTranslator,
    SchemaListTranslator, SimplifierFilter, SingleCharFilter, StaticTableTranslator,
    SwitchTranslator, TableDictionary, TaggedFilter, UniquifierFilter,
};

use crate::{
    config_scalar_bool, config_scalar_double, config_scalar_int, config_scalar_string,
    find_config_value, install_schema_punctuation_translator_from_config, load_runtime_config_root,
    schema_folded_switch_options, schema_list_translator_entries_for_current,
    schema_switch_translator_switches, selected_runtime_data_path, ConfigOpenKind, SessionState,
};

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
            "punct_translator" if !punctuation_translator_installed => {
                install_schema_punctuation_translator_from_config(session, &schema_config);
                punctuation_translator_installed = true;
            }
            "table_translator" | "script_translator" | "r10n_translator" => {
                install_schema_dictionary_translator_from_config(
                    session,
                    &schema_config,
                    component_name,
                    name_space.unwrap_or("translator"),
                );
            }
            "reverse_lookup_translator" => install_schema_reverse_lookup_translator_from_config(
                session,
                &schema_config,
                match name_space {
                    Some("translator") | None => "reverse_lookup",
                    Some(name_space) => name_space,
                },
            ),
            "history_translator" => install_schema_history_translator_from_config(
                session,
                &schema_config,
                match name_space {
                    Some("translator") | None => "history",
                    Some(name_space) => name_space,
                },
            ),
            "switch_translator" => {
                install_schema_switch_translator_from_config(session, &schema_config);
            }
            "schema_list_translator" => {
                let entries = schema_list_translator_entries_for_current(
                    session.engine.status().schema_id.as_str(),
                    &schema_config,
                );
                session
                    .engine
                    .add_translator(SchemaListTranslator::new(entries));
            }
            _ => {}
        }
    }
}

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

fn install_schema_dictionary_translator_from_config(
    session: &mut SessionState,
    schema_config: &Value,
    component_name: &str,
    name_space: &str,
) {
    let Some(dictionary) = load_schema_table_dictionary(schema_config, name_space) else {
        return;
    };
    let enable_charset_filter = find_config_value(
        schema_config,
        &format!("{name_space}/enable_charset_filter"),
    )
    .and_then(config_scalar_bool)
    .unwrap_or(false);
    let enable_sentence =
        find_config_value(schema_config, &format!("{name_space}/enable_sentence"))
            .and_then(config_scalar_bool)
            .unwrap_or(true);
    let sentence_over_completion = find_config_value(
        schema_config,
        &format!("{name_space}/sentence_over_completion"),
    )
    .and_then(config_scalar_bool)
    .unwrap_or(false);
    let mut enable_completion =
        find_config_value(schema_config, &format!("{name_space}/enable_completion"))
            .and_then(config_scalar_bool)
            .unwrap_or(true);
    if matches!(component_name, "script_translator" | "r10n_translator") {
        if let Some(enable_word_completion) = find_config_value(
            schema_config,
            &format!("{name_space}/enable_word_completion"),
        )
        .and_then(config_scalar_bool)
        {
            enable_completion = enable_word_completion;
        }
    }
    let delimiters = find_config_value(schema_config, &format!("{name_space}/delimiter"))
        .or_else(|| find_config_value(schema_config, "speller/delimiter"))
        .and_then(config_scalar_string)
        .unwrap_or_else(|| " ".to_owned());
    let tags = schema_translator_tags(schema_config, name_space);
    let initial_quality =
        find_config_value(schema_config, &format!("{name_space}/initial_quality"))
            .and_then(config_scalar_f32)
            .unwrap_or(0.0);
    let comment_format = schema_comment_format(schema_config, name_space);
    let dictionary_exclude =
        schema_string_list(schema_config, &format!("{name_space}/dictionary_exclude"));
    let spelling_algebra = schema_string_list(schema_config, "speller/algebra");
    session.engine.add_translator(
        StaticTableTranslator::from_dictionary(dictionary)
            .with_spelling_algebra(&spelling_algebra)
            .with_completion(enable_completion)
            .with_charset_filter(enable_charset_filter)
            .with_sentence(enable_sentence)
            .with_sentence_over_completion(sentence_over_completion)
            .with_delimiters(delimiters)
            .with_tags(tags)
            .with_initial_quality(initial_quality)
            .with_comment_format(&comment_format)
            .with_dictionary_exclude(dictionary_exclude),
    );
}

fn install_schema_reverse_lookup_translator_from_config(
    session: &mut SessionState,
    schema_config: &Value,
    name_space: &str,
) {
    let Some(dictionary) = load_schema_table_dictionary(schema_config, name_space) else {
        return;
    };
    let target_namespace = find_config_value(schema_config, &format!("{name_space}/target"))
        .and_then(config_scalar_string)
        .filter(|target| !target.is_empty())
        .unwrap_or_else(|| "translator".to_owned());
    let reverse_dictionary = load_schema_table_dictionary(schema_config, &target_namespace);
    let prefix = find_config_value(schema_config, &format!("{name_space}/prefix"))
        .and_then(config_scalar_string)
        .unwrap_or_default();
    let suffix = find_config_value(schema_config, &format!("{name_space}/suffix"))
        .and_then(config_scalar_string)
        .unwrap_or_default();
    let tag = find_config_value(schema_config, &format!("{name_space}/tag"))
        .and_then(config_scalar_string)
        .unwrap_or_else(|| "reverse_lookup".to_owned());
    let enable_completion =
        find_config_value(schema_config, &format!("{name_space}/enable_completion"))
            .and_then(config_scalar_bool)
            .unwrap_or(false);
    let comment_format = schema_comment_format(schema_config, name_space);

    session.engine.add_translator(
        ReverseLookupTranslator::new(dictionary, reverse_dictionary, prefix, suffix)
            .with_tag(tag)
            .with_completion(enable_completion)
            .with_comment_format(&comment_format),
    );
}

fn install_schema_history_translator_from_config(
    session: &mut SessionState,
    schema_config: &Value,
    name_space: &str,
) {
    let input = find_config_value(schema_config, &format!("{name_space}/input"))
        .and_then(config_scalar_string)
        .unwrap_or_default();
    let size = find_config_value(schema_config, &format!("{name_space}/size"))
        .and_then(config_scalar_int)
        .and_then(|size| usize::try_from(size).ok())
        .unwrap_or(1);
    let initial_quality =
        find_config_value(schema_config, &format!("{name_space}/initial_quality"))
            .and_then(config_scalar_double)
            .map(|quality| quality as f32)
            .unwrap_or(1000.0);
    let tag = find_config_value(schema_config, &format!("{name_space}/tag"))
        .and_then(config_scalar_string)
        .unwrap_or_else(|| "abc".to_owned());

    session.engine.add_translator(
        HistoryTranslator::new(input)
            .with_size(size)
            .with_initial_quality(initial_quality)
            .with_tag(tag),
    );
}

fn install_schema_switch_translator_from_config(session: &mut SessionState, schema_config: &Value) {
    let switches = schema_switch_translator_switches(schema_config);
    if switches.is_empty() {
        return;
    }
    let fold_options = find_config_value(schema_config, "switcher/fold_options")
        .and_then(config_scalar_bool)
        .unwrap_or(false);
    session.engine.set_option("_fold_options", fold_options);
    session.engine.add_translator(
        SwitchTranslator::new(switches)
            .with_folded_options(schema_folded_switch_options(schema_config)),
    );
}

pub(crate) fn install_schema_filter_chain(session: &mut SessionState, schema_id: &str) {
    let schema_config =
        load_runtime_config_root(&format!("{schema_id}.schema"), ConfigOpenKind::Deployed);
    let Some(Value::Sequence(filters)) = find_config_value(&schema_config, "engine/filters") else {
        return;
    };
    for filter in filters.iter().filter_map(Value::as_str) {
        let (filter_name, name_space) = schema_component_prescription(filter);
        match filter_name {
            "reverse_lookup_filter" => install_schema_reverse_lookup_filter_from_config(
                session,
                &schema_config,
                match name_space {
                    Some("filter") | None => "reverse_lookup",
                    Some(name_space) => name_space,
                },
            ),
            "simplifier" => install_schema_simplifier_filter_from_config(
                session,
                &schema_config,
                match name_space {
                    Some("filter") | None => "simplifier",
                    Some(name_space) => name_space,
                },
            ),
            "uniquifier" => session.engine.add_filter(UniquifierFilter),
            "single_char_filter" => session.engine.add_filter(SingleCharFilter),
            "charset_filter" | "cjk_minifier" => {
                let tags = schema_filter_tags(&schema_config, name_space.unwrap_or(filter_name));
                session
                    .engine
                    .add_filter(TaggedFilter::new(CharsetFilter, tags));
            }
            _ => {}
        }
    }
}

fn schema_filter_tags(schema_config: &Value, name_space: &str) -> Vec<String> {
    schema_string_list(schema_config, &format!("{name_space}/tags"))
}

fn install_schema_reverse_lookup_filter_from_config(
    session: &mut SessionState,
    schema_config: &Value,
    name_space: &str,
) {
    let Some(reverse_dictionary) = load_schema_table_dictionary(schema_config, name_space) else {
        return;
    };

    let overwrite_comment =
        find_config_value(schema_config, &format!("{name_space}/overwrite_comment"))
            .and_then(config_scalar_bool)
            .unwrap_or(false);
    let append_comment = find_config_value(schema_config, &format!("{name_space}/append_comment"))
        .and_then(config_scalar_bool)
        .unwrap_or(false);
    let comment_format = schema_comment_format(schema_config, name_space);

    let tags = schema_filter_tags(schema_config, name_space);
    session.engine.add_filter(TaggedFilter::new(
        ReverseLookupFilter::new(reverse_dictionary)
            .with_overwrite_comment(overwrite_comment)
            .with_append_comment(append_comment)
            .with_comment_format(&comment_format),
        tags,
    ));
}

fn install_schema_simplifier_filter_from_config(
    session: &mut SessionState,
    schema_config: &Value,
    name_space: &str,
) {
    let option_name = find_config_value(schema_config, &format!("{name_space}/option_name"))
        .and_then(config_scalar_string)
        .filter(|option_name| !option_name.is_empty())
        .unwrap_or_else(|| "simplification".to_owned());
    let tips = find_config_value(schema_config, &format!("{name_space}/tips"))
        .or_else(|| find_config_value(schema_config, &format!("{name_space}/tip")))
        .and_then(config_scalar_string)
        .unwrap_or_default();
    let opencc_config = find_config_value(schema_config, &format!("{name_space}/opencc_config"))
        .and_then(config_scalar_string)
        .unwrap_or_default();
    let show_in_comment =
        find_config_value(schema_config, &format!("{name_space}/show_in_comment"))
            .and_then(config_scalar_bool)
            .unwrap_or(false);
    let inherit_comment =
        find_config_value(schema_config, &format!("{name_space}/inherit_comment"))
            .and_then(config_scalar_bool)
            .unwrap_or(true);
    let comment_format = schema_comment_format(schema_config, name_space);
    let excluded_types = schema_string_list(schema_config, &format!("{name_space}/excluded_types"));

    let tags = schema_filter_tags(schema_config, name_space);
    session.engine.add_filter(TaggedFilter::new(
        SimplifierFilter::new()
            .with_option_name(option_name)
            .with_opencc_config(opencc_config)
            .with_tips(tips)
            .with_show_in_comment(show_in_comment)
            .with_inherit_comment(inherit_comment)
            .with_comment_format(&comment_format)
            .with_excluded_types(excluded_types),
        tags,
    ));
}

fn load_schema_table_dictionary(
    schema_config: &Value,
    name_space: &str,
) -> Option<TableDictionary> {
    let dictionary_name = find_config_value(schema_config, &format!("{name_space}/dictionary"))
        .and_then(config_scalar_string)
        .filter(|dictionary_name| !dictionary_name.is_empty())?;
    let dictionary_path = selected_runtime_data_path(&format!("{dictionary_name}.dict.yaml"))?;
    let dictionary_yaml = fs::read_to_string(dictionary_path).ok()?;
    let packs = schema_dictionary_packs(schema_config, name_space);
    TableDictionary::parse_rime_dict_yaml_with_imports_packs_and_vocabulary(
        &dictionary_yaml,
        packs,
        |import_table| {
            selected_runtime_data_path(&format!("{import_table}.dict.yaml"))
                .and_then(|path| fs::read_to_string(path).ok())
        },
        |vocabulary| {
            selected_runtime_data_path(&format!("{vocabulary}.txt"))
                .and_then(|path| fs::read_to_string(path).ok())
        },
    )
    .ok()
}

fn schema_dictionary_packs(schema_config: &Value, name_space: &str) -> Vec<String> {
    let Some(Value::Sequence(packs)) =
        find_config_value(schema_config, &format!("{name_space}/packs"))
    else {
        return Vec::new();
    };
    packs.iter().filter_map(config_scalar_string).collect()
}

fn schema_comment_format(schema_config: &Value, name_space: &str) -> Vec<String> {
    schema_string_list(schema_config, &format!("{name_space}/comment_format"))
}

fn schema_translator_tags(schema_config: &Value, name_space: &str) -> Vec<String> {
    let mut tags = Vec::new();
    if let Some(tag) = find_config_value(schema_config, &format!("{name_space}/tag"))
        .and_then(config_scalar_string)
    {
        tags.push(tag);
    }
    tags.extend(schema_string_list(
        schema_config,
        &format!("{name_space}/tags"),
    ));
    if tags.is_empty() {
        tags.push("abc".to_owned());
    }
    tags
}

pub(crate) fn schema_string_list(schema_config: &Value, key: &str) -> Vec<String> {
    let Some(Value::Sequence(formulas)) = find_config_value(schema_config, key) else {
        return Vec::new();
    };
    formulas.iter().filter_map(config_scalar_string).collect()
}

fn config_scalar_f32(value: &Value) -> Option<f32> {
    config_scalar_double(value).map(|number| number as f32)
}
