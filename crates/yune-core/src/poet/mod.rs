use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};

use crate::{Candidate, CandidateSource, PresetVocabularyEntry, TableDictionary, TableEntry};

/// Upstream `grammar.h` null-grammar penalty (`ln(1e-6)`) used when no `.gram`
/// language model is configured.
pub const UPSTREAM_NO_GRAMMAR_PENALTY: f64 = -13.815510557964274;

const CODE_LENGTH_QUALITY_BAND: f32 = 10_000_000.0;
const MAX_WORD_GRAPH_ENTRIES_PER_SPAN: usize = 7;

pub trait Grammar {
    fn query(&self, context: &str, word: &str, is_rear: bool) -> f64;
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NullGrammar;

impl Grammar for NullGrammar {
    fn query(&self, _context: &str, _word: &str, _is_rear: bool) -> f64 {
        UPSTREAM_NO_GRAMMAR_PENALTY
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WordGraphEntry {
    pub text: String,
    pub code: String,
    pub weight: f64,
}

impl WordGraphEntry {
    #[must_use]
    pub fn new(text: impl Into<String>, code: impl Into<String>, weight: f64) -> Self {
        Self {
            text: text.into(),
            code: code.into(),
            weight,
        }
    }
}

pub type WordGraph = BTreeMap<usize, BTreeMap<usize, Vec<WordGraphEntry>>>;

#[derive(Clone, Debug, PartialEq)]
pub struct SentencePath {
    pub text: String,
    pub weight: f64,
    pub word_lengths: Vec<usize>,
}

#[must_use]
pub fn null_grammar_score(entry_weight: f64) -> f64 {
    entry_weight + NullGrammar.query("", "", false)
}

#[must_use]
pub fn make_sentence(graph: &WordGraph, total_length: usize) -> Option<SentencePath> {
    make_sentences(graph, total_length, 1).into_iter().next()
}

#[must_use]
pub fn make_sentences(
    graph: &WordGraph,
    total_length: usize,
    max_sentences: usize,
) -> Vec<SentencePath> {
    if max_sentences == 0 {
        return Vec::new();
    }

    make_sentences_by_end(graph, max_sentences)
        .remove(&total_length)
        .unwrap_or_default()
}

fn make_sentences_by_end(
    graph: &WordGraph,
    max_sentences: usize,
) -> BTreeMap<usize, Vec<SentencePath>> {
    if max_sentences == 0 {
        return BTreeMap::new();
    }

    collect_sentence_states(graph, max_sentences)
        .into_iter()
        .filter(|(end, _)| *end > 0)
        .map(|(end, states)| (end, sentence_paths_from_states(states, max_sentences)))
        .collect()
}

fn collect_sentence_states(
    graph: &WordGraph,
    max_sentences: usize,
) -> BTreeMap<usize, Vec<PathState>> {
    let mut states: BTreeMap<usize, Vec<PathState>> = BTreeMap::new();
    states.insert(0, vec![PathState::default()]);
    for (start, edges) in graph {
        let Some(source_states) = states.get(start).cloned() else {
            continue;
        };
        for (end, entries) in edges {
            for source in &source_states {
                for entry in entries {
                    let mut next = source.clone();
                    next.weight += null_grammar_score(entry.weight);
                    next.text.push_str(&entry.text);
                    next.word_lengths.push(end - start);
                    insert_state(states.entry(*end).or_default(), next, max_sentences * 3);
                }
            }
        }
    }

    states
}

fn sentence_paths_from_states(
    mut states: Vec<PathState>,
    max_sentences: usize,
) -> Vec<SentencePath> {
    states.sort_by(compare_path_state);
    states
        .into_iter()
        .take(max_sentences)
        .map(|state| SentencePath {
            text: state.text,
            weight: state.weight,
            word_lengths: state.word_lengths,
        })
        .collect()
}

#[derive(Clone, Debug, Default)]
struct PathState {
    text: String,
    weight: f64,
    word_lengths: Vec<usize>,
}

fn insert_state(states: &mut Vec<PathState>, candidate: PathState, beam_width: usize) {
    if let Some(existing_index) = states
        .iter()
        .position(|existing| existing.text == candidate.text)
    {
        if compare_path_state(&candidate, &states[existing_index]) == Ordering::Less {
            states.remove(existing_index);
        } else {
            return;
        }
    }
    let index = states
        .binary_search_by(|existing| compare_path_state(existing, &candidate))
        .unwrap_or_else(|index| index);
    states.insert(index, candidate);
    if states.len() > beam_width {
        states.pop();
    }
}

fn compare_path_state(left: &PathState, right: &PathState) -> Ordering {
    right
        .weight
        .partial_cmp(&left.weight)
        .unwrap_or(Ordering::Equal)
        .then_with(|| left.word_lengths.len().cmp(&right.word_lengths.len()))
        .then_with(|| right.word_lengths.cmp(&left.word_lengths))
        .then_with(|| left.text.cmp(&right.text))
}

#[derive(Clone, Debug, Default)]
pub struct UpstreamSentenceModel {
    entries_by_code: Vec<ModelEntry>,
    vocabulary: Vec<ModelVocabularyEntry>,
    vocabulary_first_codes: Vec<(String, usize)>,
    character_codes: HashMap<char, Vec<String>>,
    max_candidates: usize,
}

impl UpstreamSentenceModel {
    #[must_use]
    pub fn from_dictionary(dictionary: &TableDictionary, max_candidates: usize) -> Self {
        Self::from_entries(
            dictionary.entries(),
            dictionary.preset_vocabulary_entries(),
            max_candidates,
        )
    }

    #[must_use]
    pub fn from_entries(
        entries: &[TableEntry],
        vocabulary: &[PresetVocabularyEntry],
        max_candidates: usize,
    ) -> Self {
        Self::from_model_entries(
            entries.iter().map(ModelEntry::from_table_entry),
            vocabulary,
            max_candidates,
        )
    }

    #[must_use]
    pub fn from_table_entries(
        entries: impl IntoIterator<Item = TableEntry>,
        vocabulary: &[PresetVocabularyEntry],
        max_candidates: usize,
    ) -> Self {
        Self::from_model_entries(
            entries.into_iter().map(ModelEntry::from_owned_table_entry),
            vocabulary,
            max_candidates,
        )
    }

    fn from_model_entries(
        entries: impl IntoIterator<Item = ModelEntry>,
        vocabulary: &[PresetVocabularyEntry],
        max_candidates: usize,
    ) -> Self {
        let mut entries_by_code = Vec::new();
        let mut character_codes: HashMap<char, Vec<String>> = HashMap::new();
        for entry in entries {
            if entry.code.is_empty() {
                continue;
            }
            let mut chars = entry.text.chars();
            if let Some(ch) = chars.next() {
                if chars.next().is_none() {
                    character_codes
                        .entry(ch)
                        .or_default()
                        .push(entry.code.clone());
                }
            }
            entries_by_code.push(entry);
        }
        for codes in character_codes.values_mut() {
            codes.sort();
            codes.dedup();
        }
        entries_by_code.sort_by(compare_model_entry_by_code);
        let vocabulary = vocabulary
            .iter()
            .filter_map(|entry| {
                let chars = entry.text.chars().collect::<Vec<_>>();
                (chars.len() > 1).then(|| ModelVocabularyEntry {
                    text: entry.text.clone(),
                    chars,
                    weight: entry.weight,
                })
            })
            .collect::<Vec<_>>();
        let mut vocabulary_first_codes = Vec::new();
        for (index, entry) in vocabulary.iter().enumerate() {
            let Some(first_char) = entry.chars.first() else {
                continue;
            };
            let Some(first_codes) = character_codes.get(first_char) else {
                continue;
            };
            for code in first_codes {
                vocabulary_first_codes.push((code.clone(), index));
            }
        }
        vocabulary_first_codes.sort();
        vocabulary_first_codes.dedup();
        Self {
            entries_by_code,
            vocabulary,
            vocabulary_first_codes,
            character_codes,
            max_candidates: max_candidates.max(1),
        }
    }

    #[must_use]
    pub fn candidates_for_input(&self, input: &str) -> Vec<Candidate> {
        self.candidates_for_input_with_limit(input, self.max_candidates)
    }

    #[must_use]
    pub fn candidates_for_input_with_limit(
        &self,
        input: &str,
        max_candidates: usize,
    ) -> Vec<Candidate> {
        if input.is_empty() {
            return Vec::new();
        }

        let max_candidates = max_candidates.max(1).min(self.max_candidates);
        let graph = self.word_graph_for_input(input);
        let sentences_by_end = make_sentences_by_end(&graph, max_candidates);
        let mut candidates = HashMap::new();
        for end in input
            .char_indices()
            .map(|(index, _)| index)
            .filter(|index| *index > 0)
            .chain(std::iter::once(input.len()))
        {
            let Some(sentences) = sentences_by_end.get(&end) else {
                continue;
            };
            for sentence in sentences {
                let candidate = Candidate {
                    text: sentence.text.clone(),
                    comment: String::new(),
                    preedit: None,
                    source: if end < input.len() {
                        CandidateSource::PartialTable {
                            consumed: end,
                            recompose_on_default: false,
                        }
                    } else {
                        CandidateSource::Sentence
                    },
                    quality: end as f32 * CODE_LENGTH_QUALITY_BAND + sentence.weight as f32,
                };
                match candidates.get(&candidate.text) {
                    Some(existing)
                        if compare_sentence_candidate(&candidate, existing) != Ordering::Less => {}
                    _ => {
                        candidates.insert(candidate.text.clone(), candidate);
                    }
                }
            }
        }
        let mut candidates = candidates.into_values().collect::<Vec<_>>();
        candidates.sort_by(compare_sentence_candidate);
        candidates.truncate(max_candidates);
        candidates
    }

    fn word_graph_for_input(&self, input: &str) -> WordGraph {
        let mut graph = WordGraph::new();
        let boundaries = input
            .char_indices()
            .map(|(index, _)| index)
            .chain(std::iter::once(input.len()))
            .collect::<Vec<_>>();
        let mut code_prefix_checks = 0usize;
        let mut table_entries_considered = 0usize;
        let mut vocabulary_entries_considered = 0usize;
        let mut graph_edges = 0usize;
        for (start_index, start) in boundaries.iter().copied().enumerate() {
            if start >= input.len() {
                continue;
            }
            let suffix = &input[start..];
            for end in boundaries.iter().copied().skip(start_index + 1) {
                let code = &input[start..end];
                code_prefix_checks += 1;
                let entries = self.entries_for_code(code);
                table_entries_considered += entries.len();
                for entry in entries {
                    graph
                        .entry(start)
                        .or_default()
                        .entry(end)
                        .or_default()
                        .push(WordGraphEntry::new(
                            entry.text.clone(),
                            entry.code.clone(),
                            f64::from(entry.weight),
                        ));
                    graph_edges += 1;
                }
                let vocabulary_entries = self.vocabulary_indices_for_first_code(code);
                for (_, index) in vocabulary_entries {
                    let vocabulary_entry = &self.vocabulary[*index];
                    vocabulary_entries_considered += 1;
                    for phrase_code in
                        self.derive_matching_phrase_codes(vocabulary_entry, suffix, code)
                    {
                        let end = start + phrase_code.len();
                        graph
                            .entry(start)
                            .or_default()
                            .entry(end)
                            .or_default()
                            .push(WordGraphEntry::new(
                                vocabulary_entry.text.clone(),
                                phrase_code,
                                f64::from(vocabulary_entry.weight),
                            ));
                        graph_edges += 1;
                    }
                }
            }
        }
        for edges in graph.values_mut() {
            for entries in edges.values_mut() {
                entries.sort_by(compare_word_graph_entry);
                entries.truncate(MAX_WORD_GRAPH_ENTRIES_PER_SPAN);
            }
        }
        crate::m37_record_upstream_sentence_model_scan(
            code_prefix_checks,
            table_entries_considered,
            vocabulary_entries_considered,
            graph_edges,
        );
        graph
    }

    fn entries_for_code(&self, code: &str) -> &[ModelEntry] {
        let start = self
            .entries_by_code
            .partition_point(|entry| entry.code.as_str() < code);
        let end = self.entries_by_code[start..]
            .partition_point(|entry| entry.code.as_str() == code)
            + start;
        &self.entries_by_code[start..end]
    }

    fn vocabulary_indices_for_first_code(&self, code: &str) -> &[(String, usize)] {
        let start = self
            .vocabulary_first_codes
            .partition_point(|(entry_code, _)| entry_code.as_str() < code);
        let end = self.vocabulary_first_codes[start..]
            .partition_point(|(entry_code, _)| entry_code.as_str() == code)
            + start;
        &self.vocabulary_first_codes[start..end]
    }

    fn derive_matching_phrase_codes(
        &self,
        entry: &ModelVocabularyEntry,
        input: &str,
        first_code: &str,
    ) -> Vec<String> {
        let mut codes = Vec::new();
        self.derive_matching_phrase_codes_from(
            &entry.chars,
            input,
            1,
            first_code.to_owned(),
            &mut codes,
        );
        codes.sort();
        codes.dedup();
        codes
    }

    fn derive_matching_phrase_codes_from(
        &self,
        chars: &[char],
        input: &str,
        index: usize,
        current: String,
        codes: &mut Vec<String>,
    ) {
        if index == chars.len() {
            if input.starts_with(&current) {
                codes.push(current);
            }
            return;
        }
        let Some(next_codes) = self.character_codes.get(&chars[index]) else {
            return;
        };
        for next_code in next_codes {
            let next = format!("{current}{next_code}");
            if input.starts_with(&next) {
                self.derive_matching_phrase_codes_from(chars, input, index + 1, next, codes);
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ModelVocabularyEntry {
    text: String,
    chars: Vec<char>,
    weight: f32,
}

#[derive(Clone, Debug, PartialEq)]
struct ModelEntry {
    text: String,
    code: String,
    weight: f32,
}

impl ModelEntry {
    fn from_table_entry(entry: &TableEntry) -> Self {
        Self {
            text: entry.text.clone(),
            code: entry.code.clone(),
            weight: entry.weight,
        }
    }

    fn from_owned_table_entry(entry: TableEntry) -> Self {
        Self {
            text: entry.text,
            code: entry.code,
            weight: entry.weight,
        }
    }
}

fn compare_sentence_candidate(left: &Candidate, right: &Candidate) -> Ordering {
    right
        .quality
        .partial_cmp(&left.quality)
        .unwrap_or(Ordering::Equal)
        .then_with(|| left.text.cmp(&right.text))
}

fn compare_word_graph_entry(left: &WordGraphEntry, right: &WordGraphEntry) -> Ordering {
    right
        .weight
        .partial_cmp(&left.weight)
        .unwrap_or(Ordering::Equal)
        .then_with(|| left.text.cmp(&right.text))
}

fn compare_model_entry_by_code(left: &ModelEntry, right: &ModelEntry) -> Ordering {
    left.code
        .cmp(&right.code)
        .then_with(|| compare_model_entry(left, right))
}

fn compare_model_entry(left: &ModelEntry, right: &ModelEntry) -> Ordering {
    right
        .weight
        .partial_cmp(&left.weight)
        .unwrap_or(Ordering::Equal)
        .then_with(|| left.text.cmp(&right.text))
}
