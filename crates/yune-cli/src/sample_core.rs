use yune_core::{Engine, PunctuationTranslator, StaticTableTranslator};

use crate::transcript::FixtureOutput;

pub(crate) const DEFAULT_SEQUENCE: &str = "nihao ";

const SAMPLE_DICT: &str = r#"
---
name: sample
version: "0.1"
sort: by_weight
...

你	ni	10
好	hao	10
你好	ni hao	100
"#;

pub(crate) fn run_sequence(sequence: &str) -> Result<FixtureOutput, String> {
    let mut engine = Engine::new();
    engine.set_schema("sample", "Sample");
    engine.add_translator(PunctuationTranslator::default_half_shape());
    engine.add_translator(
        StaticTableTranslator::parse_rime_dict_yaml(SAMPLE_DICT)
            .map_err(|error| format!("invalid sample dictionary: {error}"))?,
    );
    let commits = engine
        .process_key_sequence(sequence)
        .map_err(|error| format!("invalid key sequence: {error}"))?;

    Ok(FixtureOutput {
        schema_id: "sample".to_owned(),
        sequence: sequence.to_owned(),
        commits,
        snapshot: engine.snapshot(),
    })
}
