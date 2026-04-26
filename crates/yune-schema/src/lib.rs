#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Schema {
    pub id: String,
    pub name: String,
    pub engine: EngineSpec,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EngineSpec {
    pub processors: Vec<String>,
    pub segmentors: Vec<String>,
    pub translators: Vec<String>,
    pub filters: Vec<String>,
}

impl Schema {
    #[must_use]
    pub fn minimal(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            engine: EngineSpec {
                processors: vec!["speller".to_owned(), "selector".to_owned()],
                segmentors: vec!["abc_segmentor".to_owned(), "fallback_segmentor".to_owned()],
                translators: vec!["echo_translator".to_owned()],
                filters: Vec::new(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Schema;

    #[test]
    fn creates_minimal_schema() {
        let schema = Schema::minimal("sample", "Sample");

        assert_eq!(schema.id, "sample");
        assert_eq!(schema.engine.translators, ["echo_translator"]);
    }
}
