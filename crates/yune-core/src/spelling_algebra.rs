use super::Candidate;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Default)]
pub(crate) struct SpellingAlgebra {
    formulas: Vec<SpellingAlgebraFormula>,
}

const SPELLING_ALGEBRA_FUZZY_PENALTY: f32 = -std::f32::consts::LN_2;
const SPELLING_ALGEBRA_ABBREVIATION_PENALTY: f32 = -std::f32::consts::LN_2;
const SPELLING_ALGEBRA_CORRECTION_PENALTY: f32 = -std::f32::consts::LN_10 * 2.0;

impl SpellingAlgebra {
    pub(crate) fn parse(formulas: &[String]) -> Self {
        let mut parsed = Vec::new();
        for formula in formulas {
            if let Some(parsed_formula) = SpellingAlgebraFormula::parse(formula) {
                parsed.push(parsed_formula);
            }
        }
        Self { formulas: parsed }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.formulas.is_empty()
    }

    pub(crate) fn expand_entries_with_normal_codes(
        &self,
        entries: Vec<(String, Candidate)>,
    ) -> (Vec<(String, Candidate)>, HashSet<String>, bool) {
        let mut entries = entries
            .into_iter()
            .map(|(code, candidate)| SpellingAlgebraEntry {
                code,
                candidate,
                normal: true,
                abbreviation: false,
            })
            .collect::<Vec<_>>();
        for formula in &self.formulas {
            let mut next = Vec::new();
            for entry in entries {
                let mut transformed = entry.code.clone();
                let applied = formula.apply(&mut transformed);
                if applied {
                    if formula.keep_original() {
                        next.push(entry.clone());
                    }
                    if formula.add_transformed() && !transformed.is_empty() {
                        let mut candidate = entry.candidate;
                        candidate.quality += formula.quality_penalty();
                        next.push(SpellingAlgebraEntry {
                            code: transformed,
                            candidate,
                            normal: entry.normal && formula.transformed_is_normal(),
                            abbreviation: entry.abbreviation || formula.is_abbreviation(),
                        });
                    }
                } else {
                    next.push(entry);
                }
            }
            entries = dedupe_spelling_algebra_entries(next);
        }
        let normal_codes = entries
            .iter()
            .filter(|entry| entry.normal)
            .map(|entry| entry.code.clone())
            .collect::<HashSet<_>>();
        let has_single_letter_abbreviations = entries
            .iter()
            .any(|entry| entry.abbreviation && !entry.normal && entry.code.len() == 1);
        let entries = entries
            .into_iter()
            .map(|entry| (entry.code, entry.candidate))
            .collect();
        (entries, normal_codes, has_single_letter_abbreviations)
    }
}

#[derive(Clone)]
struct SpellingAlgebraEntry {
    code: String,
    candidate: Candidate,
    normal: bool,
    abbreviation: bool,
}

#[derive(Clone)]
enum SpellingAlgebraFormula {
    Transliterate(Vec<(char, char)>),
    Transform {
        pattern: Regex,
        replacement: String,
        keep_original: bool,
        add_transformed: bool,
        quality_penalty: f32,
        abbreviation: bool,
    },
    LookAheadTransform {
        prefix: Regex,
        lookahead: Regex,
        positive: bool,
        replacement: String,
        keep_original: bool,
        add_transformed: bool,
        quality_penalty: f32,
        abbreviation: bool,
    },
    Erase(Regex),
}

impl SpellingAlgebraFormula {
    pub(crate) fn parse(definition: &str) -> Option<Self> {
        let separator = definition.chars().find(|ch| !ch.is_ascii_lowercase())?;
        let args = definition.split(separator).collect::<Vec<_>>();
        match args.first().copied()? {
            "xlit" => Self::parse_xlit(&args),
            "xform" => Self::parse_transform(&args, false, true, 0.0, false),
            "derive" => Self::parse_derivation(&args),
            "fuzz" => {
                Self::parse_transform(&args, true, true, SPELLING_ALGEBRA_FUZZY_PENALTY, false)
            }
            "abbrev" => Self::parse_transform(
                &args,
                true,
                true,
                SPELLING_ALGEBRA_ABBREVIATION_PENALTY,
                true,
            ),
            "erase" => Self::parse_erase(&args),
            _ => None,
        }
    }

    fn parse_xlit(args: &[&str]) -> Option<Self> {
        if args.len() < 3 {
            return None;
        }
        let left = args[1].chars().collect::<Vec<_>>();
        let right = args[2].chars().collect::<Vec<_>>();
        if left.len() != right.len() {
            return None;
        }
        Some(Self::Transliterate(left.into_iter().zip(right).collect()))
    }

    fn parse_transform(
        args: &[&str],
        keep_original: bool,
        add_transformed: bool,
        quality_penalty: f32,
        abbreviation: bool,
    ) -> Option<Self> {
        if args.len() < 3 || args[1].is_empty() {
            return None;
        }
        if let Some((prefix, lookahead, positive)) = parse_lookahead_pattern(args[1]) {
            return Some(Self::LookAheadTransform {
                prefix,
                lookahead,
                positive,
                replacement: args[2].to_owned(),
                keep_original,
                add_transformed,
                quality_penalty,
                abbreviation,
            });
        }
        Some(Self::Transform {
            pattern: Regex::new(args[1]).ok()?,
            replacement: args[2].to_owned(),
            keep_original,
            add_transformed,
            quality_penalty,
            abbreviation,
        })
    }

    fn parse_derivation(args: &[&str]) -> Option<Self> {
        let (quality_penalty, abbreviation) = match args.get(3).copied() {
            Some("abbrev") => (SPELLING_ALGEBRA_ABBREVIATION_PENALTY, true),
            Some("fuzz") => (SPELLING_ALGEBRA_FUZZY_PENALTY, false),
            Some("correction") => (SPELLING_ALGEBRA_CORRECTION_PENALTY, false),
            _ => (0.0, false),
        };
        Self::parse_transform(args, true, true, quality_penalty, abbreviation)
    }

    fn parse_erase(args: &[&str]) -> Option<Self> {
        if args.len() < 2 || args[1].is_empty() {
            return None;
        }
        Some(Self::Erase(Regex::new(args[1]).ok()?))
    }

    fn keep_original(&self) -> bool {
        match self {
            Self::Transform { keep_original, .. }
            | Self::LookAheadTransform { keep_original, .. } => *keep_original,
            _ => false,
        }
    }

    fn quality_penalty(&self) -> f32 {
        match self {
            Self::Transform {
                quality_penalty, ..
            }
            | Self::LookAheadTransform {
                quality_penalty, ..
            } => *quality_penalty,
            _ => 0.0,
        }
    }

    fn is_abbreviation(&self) -> bool {
        match self {
            Self::Transform { abbreviation, .. }
            | Self::LookAheadTransform { abbreviation, .. } => *abbreviation,
            _ => false,
        }
    }

    fn add_transformed(&self) -> bool {
        !matches!(self, Self::Erase(_))
    }

    fn transformed_is_normal(&self) -> bool {
        match self {
            Self::Transliterate(_) => true,
            Self::Transform {
                quality_penalty, ..
            }
            | Self::LookAheadTransform {
                quality_penalty, ..
            } => *quality_penalty == 0.0,
            Self::Erase(_) => false,
        }
    }

    fn apply(&self, value: &mut String) -> bool {
        match self {
            Self::Transliterate(char_map) => {
                let mut modified = false;
                let transformed = value
                    .chars()
                    .map(|ch| {
                        if let Some((_, replacement)) =
                            char_map.iter().find(|(source, _)| *source == ch)
                        {
                            modified = true;
                            *replacement
                        } else {
                            ch
                        }
                    })
                    .collect::<String>();
                if modified {
                    *value = transformed;
                }
                modified
            }
            Self::Transform {
                pattern,
                replacement,
                add_transformed,
                ..
            } => {
                let transformed = pattern
                    .replace_all(value, replacement.as_str())
                    .into_owned();
                let modified = transformed != *value;
                if modified && *add_transformed {
                    *value = transformed;
                }
                modified
            }
            Self::LookAheadTransform {
                prefix,
                lookahead,
                positive,
                replacement,
                add_transformed,
                ..
            } => {
                let transformed =
                    replace_lookahead_matches(value, prefix, lookahead, *positive, replacement);
                let modified = transformed
                    .as_ref()
                    .is_some_and(|transformed| transformed != value);
                if modified && *add_transformed {
                    *value = transformed.expect("modified lookahead transform should have output");
                }
                modified
            }
            Self::Erase(pattern) => {
                let should_erase = pattern
                    .find(value)
                    .is_some_and(|matched| matched.start() == 0 && matched.end() == value.len());
                if should_erase {
                    value.clear();
                }
                should_erase
            }
        }
    }
}

fn parse_lookahead_pattern(pattern: &str) -> Option<(Regex, Regex, bool)> {
    let positive_index = pattern.find("(?=");
    let negative_index = pattern.find("(?!");
    let (lookahead_start, positive) = match (positive_index, negative_index) {
        (Some(positive), Some(negative)) if positive < negative => (positive, true),
        (Some(_), Some(negative)) => (negative, false),
        (Some(positive), None) => (positive, true),
        (None, Some(negative)) => (negative, false),
        (None, None) => return None,
    };
    let prefix = &pattern[..lookahead_start];
    let lookahead_body = &pattern[lookahead_start + 3..];
    let lookahead_end = lookahead_body.find(')')?;
    if !lookahead_body[lookahead_end + 1..].is_empty() {
        return None;
    }
    let lookahead = &lookahead_body[..lookahead_end];
    Some((
        Regex::new(prefix).ok()?,
        Regex::new(&format!("^(?:{lookahead})")).ok()?,
        positive,
    ))
}

fn replace_lookahead_matches(
    value: &str,
    prefix: &Regex,
    lookahead: &Regex,
    positive: bool,
    replacement: &str,
) -> Option<String> {
    let mut output = String::with_capacity(value.len());
    let mut last_end = 0;
    let mut modified = false;

    for captures in prefix.captures_iter(value) {
        let whole_match = captures.get(0)?;
        let predicate_matches = lookahead.is_match(&value[whole_match.end()..]);
        if predicate_matches != positive {
            continue;
        }
        output.push_str(&value[last_end..whole_match.start()]);
        captures.expand(replacement, &mut output);
        last_end = whole_match.end();
        modified = true;
    }

    if modified {
        output.push_str(&value[last_end..]);
        Some(output)
    } else {
        None
    }
}

fn dedupe_spelling_algebra_entries(
    entries: Vec<SpellingAlgebraEntry>,
) -> Vec<SpellingAlgebraEntry> {
    let mut deduped: Vec<SpellingAlgebraEntry> = Vec::new();
    let mut indexes = HashMap::<(String, String, String), usize>::new();
    for entry in entries {
        let key = (
            entry.code.clone(),
            entry.candidate.text.clone(),
            entry.candidate.comment.clone(),
        );
        if let Some(index) = indexes.get(&key).copied() {
            let existing_entry = &mut deduped[index];
            existing_entry.normal |= entry.normal;
            existing_entry.abbreviation |= entry.abbreviation;
            if entry.candidate.quality > existing_entry.candidate.quality {
                existing_entry.candidate = entry.candidate;
            }
        } else {
            indexes.insert(key, deduped.len());
            deduped.push(entry);
        }
    }
    deduped
}
