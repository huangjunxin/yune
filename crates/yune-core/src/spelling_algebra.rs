use super::Candidate;
use regex::Regex;
use std::collections::HashMap;

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
    },
    Erase(Regex),
}

impl SpellingAlgebraFormula {
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
    ) -> Option<Self> {
        if args.len() < 3 || args[1].is_empty() {
            return None;
        }
        Some(Self::Transform {
            pattern: Regex::new(args[1]).ok()?,
            replacement: args[2].to_owned(),
            keep_original,
            add_transformed,
            quality_penalty,
        })
    }

    fn parse_derivation(args: &[&str]) -> Option<Self> {
        let quality_penalty = match args.get(3).copied() {
            Some("abbrev") => SPELLING_ALGEBRA_ABBREVIATION_PENALTY,
            Some("fuzz") => SPELLING_ALGEBRA_FUZZY_PENALTY,
            Some("correction") => SPELLING_ALGEBRA_CORRECTION_PENALTY,
            _ => 0.0,
        };
        Self::parse_transform(args, true, true, quality_penalty)
    }

    fn parse_erase(args: &[&str]) -> Option<Self> {
        if args.len() < 2 || args[1].is_empty() {
            return None;
        }
        Some(Self::Erase(Regex::new(args[1]).ok()?))
    }

    fn keep_original(&self) -> bool {
        match self {
            Self::Transform { keep_original, .. } => *keep_original,
            _ => false,
        }
    }

    fn quality_penalty(&self) -> f32 {
        match self {
            Self::Transform {
                quality_penalty, ..
            } => *quality_penalty,
            _ => 0.0,
        }
    }

    fn add_transformed(&self) -> bool {
        !matches!(self, Self::Erase(_))
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

fn dedupe_spelling_algebra_entries(entries: Vec<(String, Candidate)>) -> Vec<(String, Candidate)> {
    let mut deduped: Vec<(String, Candidate)> = Vec::new();
    let mut indexes = HashMap::<(String, String, String), usize>::new();
    for (code, candidate) in entries {
        let key = (
            code.clone(),
            candidate.text.clone(),
            candidate.comment.clone(),
        );
        if let Some(index) = indexes.get(&key).copied() {
            let (_, existing_candidate) = &mut deduped[index];
            if candidate.quality > existing_candidate.quality {
                *existing_candidate = candidate;
            }
        } else {
            indexes.insert(key, deduped.len());
            deduped.push((code, candidate));
        }
    }
    deduped
}
