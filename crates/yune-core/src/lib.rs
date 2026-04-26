#[derive(Clone, Debug, PartialEq)]
pub struct Candidate {
    pub text: String,
    pub comment: String,
    pub source: CandidateSource,
    pub quality: f32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CandidateSource {
    Echo,
    Table,
    Ai,
}

impl CandidateSource {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Echo => "echo",
            Self::Table => "table",
            Self::Ai => "ai",
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Composition {
    pub input: String,
    pub caret: usize,
    pub preedit: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Context {
    pub composition: Composition,
    pub candidates: Vec<Candidate>,
    pub highlighted: usize,
    pub last_commit: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Status {
    pub schema_id: String,
    pub schema_name: String,
    pub is_disabled: bool,
    pub is_composing: bool,
    pub is_ascii_mode: bool,
    pub is_full_shape: bool,
    pub is_simplified: bool,
    pub is_traditional: bool,
    pub is_ascii_punct: bool,
}

impl Default for Status {
    fn default() -> Self {
        Self {
            schema_id: "default".to_owned(),
            schema_name: "Default".to_owned(),
            is_disabled: false,
            is_composing: false,
            is_ascii_mode: false,
            is_full_shape: false,
            is_simplified: false,
            is_traditional: false,
            is_ascii_punct: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot {
    pub context: Context,
    pub status: Status,
}

pub trait Translator: Send + Sync {
    fn name(&self) -> &'static str;

    fn translate(&self, input: &str) -> Vec<Candidate>;
}

pub trait CandidateRanker: Send + Sync {
    fn name(&self) -> &'static str;

    fn rerank(&self, context: &Context, candidates: &mut Vec<Candidate>);
}

#[derive(Default)]
pub struct EchoTranslator;

impl Translator for EchoTranslator {
    fn name(&self) -> &'static str {
        "echo_translator"
    }

    fn translate(&self, input: &str) -> Vec<Candidate> {
        if input.is_empty() {
            return Vec::new();
        }
        vec![Candidate {
            text: input.to_owned(),
            comment: "echo".to_owned(),
            source: CandidateSource::Echo,
            quality: 0.0,
        }]
    }
}

pub struct StaticTableTranslator {
    entries: Vec<(String, Candidate)>,
}

impl StaticTableTranslator {
    #[must_use]
    pub fn new(entries: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>) -> Self {
        let entries = entries
            .into_iter()
            .map(|(code, text)| {
                let code = code.into();
                let text = text.into();
                (
                    code.clone(),
                    Candidate {
                        text,
                        comment: code,
                        source: CandidateSource::Table,
                        quality: 1.0,
                    },
                )
            })
            .collect();
        Self { entries }
    }
}

impl Translator for StaticTableTranslator {
    fn name(&self) -> &'static str {
        "static_table_translator"
    }

    fn translate(&self, input: &str) -> Vec<Candidate> {
        self.entries
            .iter()
            .filter(|(code, _)| code == input)
            .map(|(_, candidate)| candidate.clone())
            .collect()
    }
}

pub struct Engine {
    context: Context,
    status: Status,
    translators: Vec<Box<dyn Translator>>,
    rankers: Vec<Box<dyn CandidateRanker>>,
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            context: Context::default(),
            status: Status::default(),
            translators: vec![Box::new(EchoTranslator)],
            rankers: Vec::new(),
        }
    }
}

impl Engine {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_translator(&mut self, translator: impl Translator + 'static) {
        let insert_at = self
            .translators
            .iter()
            .position(|existing| existing.name() == "echo_translator")
            .unwrap_or(self.translators.len());
        self.translators.insert(insert_at, Box::new(translator));
        self.refresh_candidates();
    }

    pub fn add_ranker(&mut self, ranker: impl CandidateRanker + 'static) {
        self.rankers.push(Box::new(ranker));
        self.refresh_candidates();
    }

    pub fn set_schema(&mut self, id: impl Into<String>, name: impl Into<String>) {
        self.status.schema_id = id.into();
        self.status.schema_name = name.into();
    }

    pub fn process_char(&mut self, ch: char) -> Option<String> {
        match ch {
            '\u{8}' | '\u{7f}' => self.backspace(),
            ' ' => self.commit_highlighted(),
            _ if !ch.is_control() => {
                self.context.composition.input.push(ch);
                self.context.composition.caret = self.context.composition.input.len();
                self.context.composition.preedit = self.context.composition.input.clone();
                self.refresh_candidates();
                None
            }
            _ => None,
        }
    }

    pub fn process_sequence(&mut self, input: &str) -> Vec<String> {
        input
            .chars()
            .filter_map(|ch| self.process_char(ch))
            .collect()
    }

    #[must_use]
    pub fn context(&self) -> &Context {
        &self.context
    }

    #[must_use]
    pub fn status(&self) -> Status {
        let mut status = self.status.clone();
        status.is_composing = !self.context.composition.input.is_empty();
        status
    }

    #[must_use]
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            context: self.context.clone(),
            status: self.status(),
        }
    }

    fn backspace(&mut self) -> Option<String> {
        self.context.composition.input.pop();
        self.context.composition.caret = self.context.composition.input.len();
        self.context.composition.preedit = self.context.composition.input.clone();
        self.refresh_candidates();
        None
    }

    fn commit_highlighted(&mut self) -> Option<String> {
        let text = self
            .context
            .candidates
            .get(self.context.highlighted)
            .map(|candidate| candidate.text.clone())?;
        self.context.last_commit = Some(text.clone());
        self.context.composition = Composition::default();
        self.context.candidates.clear();
        self.context.highlighted = 0;
        Some(text)
    }

    fn refresh_candidates(&mut self) {
        let input = self.context.composition.input.as_str();
        let mut candidates = self
            .translators
            .iter()
            .flat_map(|translator| translator.translate(input))
            .collect::<Vec<_>>();
        for ranker in &self.rankers {
            ranker.rerank(&self.context, &mut candidates);
        }
        self.context.candidates = candidates;
        self.context.highlighted = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::{CandidateSource, Engine, StaticTableTranslator};

    #[test]
    fn commits_table_candidate_before_echo_candidate() {
        let mut engine = Engine::new();
        engine.add_translator(StaticTableTranslator::new([("ni", "你")]));

        engine.process_char('n');
        engine.process_char('i');

        assert_eq!(engine.context().composition.preedit, "ni");
        assert_eq!(engine.context().candidates[0].text, "你");
        assert_eq!(engine.context().candidates[1].text, "ni");

        let commit = engine.process_char(' ');
        assert_eq!(commit.as_deref(), Some("你"));
    }

    #[test]
    fn backspace_rebuilds_candidates() {
        let mut engine = Engine::new();

        engine.process_char('a');
        engine.process_char('b');
        engine.process_char('\u{8}');

        assert_eq!(engine.context().composition.input, "a");
        assert_eq!(engine.context().candidates[0].source, CandidateSource::Echo);
    }

    #[test]
    fn sequence_collects_commits_and_snapshot_status() {
        let mut engine = Engine::new();
        engine.set_schema("sample", "Sample");
        engine.add_translator(StaticTableTranslator::new([("ni", "你")]));

        let commits = engine.process_sequence("ni ");
        let snapshot = engine.snapshot();

        assert_eq!(commits, ["你"]);
        assert_eq!(snapshot.context.last_commit.as_deref(), Some("你"));
        assert_eq!(snapshot.status.schema_id, "sample");
        assert!(!snapshot.status.is_composing);
    }
}
