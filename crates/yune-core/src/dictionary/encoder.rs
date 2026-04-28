use regex::Regex;

#[derive(Clone, Debug, Default)]
pub struct TableEncoder {
    rules: Vec<TableEncodingRule>,
    exclude_pattern_sources: Vec<String>,
    exclude_patterns: Vec<Regex>,
    tail_anchor: String,
    max_phrase_length: usize,
}

impl PartialEq for TableEncoder {
    fn eq(&self, other: &Self) -> bool {
        self.rules == other.rules
            && self.exclude_pattern_sources == other.exclude_pattern_sources
            && self.tail_anchor == other.tail_anchor
            && self.max_phrase_length == other.max_phrase_length
    }
}

impl TableEncoder {
    const MAX_PHRASE_LENGTH: usize = 32;

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn loaded(&self) -> bool {
        !self.rules.is_empty()
    }

    #[must_use]
    pub fn rules(&self) -> &[TableEncodingRule] {
        &self.rules
    }

    #[must_use]
    pub fn max_phrase_length(&self) -> usize {
        self.max_phrase_length
    }

    pub fn add_length_equal_rule(
        &mut self,
        length: usize,
        formula: &str,
    ) -> Result<(), TableEncoderFormulaError> {
        let rule = TableEncodingRule::from_formula(length, length, formula)?;
        self.max_phrase_length = self
            .max_phrase_length
            .max(length)
            .min(Self::MAX_PHRASE_LENGTH);
        self.rules.push(rule);
        Ok(())
    }

    pub fn add_length_in_range_rule(
        &mut self,
        min_length: usize,
        max_length: usize,
        formula: &str,
    ) -> Result<(), TableEncoderFormulaError> {
        if min_length > max_length {
            return Err(TableEncoderFormulaError::new(
                "invalid encoder length range",
            ));
        }
        let rule = TableEncodingRule::from_formula(min_length, max_length, formula)?;
        self.max_phrase_length = self
            .max_phrase_length
            .max(max_length)
            .min(Self::MAX_PHRASE_LENGTH);
        self.rules.push(rule);
        Ok(())
    }

    pub fn set_exclude_patterns(
        &mut self,
        patterns: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<(), regex::Error> {
        let mut sources = Vec::new();
        let mut compiled = Vec::new();
        for pattern in patterns {
            let pattern = pattern.as_ref().to_owned();
            compiled.push(Regex::new(&pattern)?);
            sources.push(pattern);
        }
        self.exclude_pattern_sources = sources;
        self.exclude_patterns = compiled;
        Ok(())
    }

    pub(crate) fn add_exclude_pattern_lossy(&mut self, pattern: impl Into<String>) {
        let pattern = pattern.into();
        let Ok(compiled) = Regex::new(&pattern) else {
            return;
        };
        self.exclude_pattern_sources.push(pattern);
        self.exclude_patterns.push(compiled);
    }

    pub fn set_tail_anchor(&mut self, tail_anchor: impl Into<String>) {
        self.tail_anchor = tail_anchor.into();
    }

    #[must_use]
    pub fn is_code_excluded(&self, code: &str) -> bool {
        self.exclude_patterns.iter().any(|pattern| {
            pattern
                .find(code)
                .is_some_and(|matched| matched.start() == 0 && matched.end() == code.len())
        })
    }

    #[must_use]
    pub fn encode(&self, raw_code: &[impl AsRef<str>]) -> Option<String> {
        let num_syllables = raw_code.len();
        for rule in &self.rules {
            if num_syllables < rule.min_word_length || num_syllables > rule.max_word_length {
                continue;
            }

            let mut encoded = String::new();
            let mut previous = CodeCoords::default();
            let mut current_encoded = CodeCoords::default();
            for original in &rule.coords {
                let mut coords = *original;
                if coords.char_index < 0 {
                    coords.char_index += num_syllables as isize;
                }
                if coords.char_index >= num_syllables as isize || coords.char_index < 0 {
                    continue;
                }
                if original.char_index < 0 && coords.char_index < current_encoded.char_index {
                    continue;
                }

                let start_index = if coords.char_index == current_encoded.char_index {
                    current_encoded.code_index + 1
                } else {
                    0
                };
                let code = raw_code[coords.char_index as usize].as_ref();
                coords.code_index = self.calculate_code_index(code, coords.code_index, start_index);
                if coords.code_index >= code.len() as isize || coords.code_index < 0 {
                    continue;
                }
                if (original.char_index < 0 || original.code_index < 0)
                    && coords.char_index == current_encoded.char_index
                    && coords.code_index <= current_encoded.code_index
                    && (original.char_index != previous.char_index
                        || original.code_index != previous.code_index)
                {
                    continue;
                }

                encoded.push(code.as_bytes()[coords.code_index as usize] as char);
                previous = *original;
                current_encoded = coords;
            }
            if !encoded.is_empty() {
                return Some(encoded);
            }
        }
        None
    }

    fn calculate_code_index(&self, code: &str, mut index: isize, start: isize) -> isize {
        let bytes = code.as_bytes();
        let tail_anchor = self.tail_anchor.as_bytes();
        let mut byte_index = 0;
        if index < 0 {
            byte_index = bytes.len() as isize - 1;
            if let Some(tail) = bytes
                .iter()
                .enumerate()
                .skip((start + 1).max(0) as usize)
                .find_map(|(tail_index, byte)| tail_anchor.contains(byte).then_some(tail_index))
            {
                byte_index = tail as isize - 1;
            }
            while {
                index += 1;
                index < 0
            } {
                loop {
                    byte_index -= 1;
                    if byte_index < 0 || !tail_anchor.contains(&bytes[byte_index as usize]) {
                        break;
                    }
                }
            }
        } else {
            while index > 0 {
                index -= 1;
                loop {
                    byte_index += 1;
                    if byte_index >= bytes.len() as isize
                        || !tail_anchor.contains(&bytes[byte_index as usize])
                    {
                        break;
                    }
                }
            }
        }
        byte_index
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableEncodingRule {
    pub min_word_length: usize,
    pub max_word_length: usize,
    pub coords: Vec<CodeCoords>,
}

impl TableEncodingRule {
    fn from_formula(
        min_word_length: usize,
        max_word_length: usize,
        formula: &str,
    ) -> Result<Self, TableEncoderFormulaError> {
        if formula.len() % 2 != 0 {
            return Err(TableEncoderFormulaError::new(
                "encoder formula length is odd",
            ));
        }
        let mut coords = Vec::new();
        for pair in formula.as_bytes().chunks_exact(2) {
            let char_index = parse_encoder_formula_index(pair[0], b'A', b'Z')
                .ok_or_else(|| TableEncoderFormulaError::new("invalid character index"))?;
            let code_index = parse_encoder_formula_index(pair[1], b'a', b'z')
                .ok_or_else(|| TableEncoderFormulaError::new("invalid code index"))?;
            coords.push(CodeCoords {
                char_index,
                code_index,
            });
        }
        Ok(Self {
            min_word_length,
            max_word_length,
            coords,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CodeCoords {
    pub char_index: isize,
    pub code_index: isize,
}

fn parse_encoder_formula_index(byte: u8, lower: u8, upper: u8) -> Option<isize> {
    if !(lower..=upper).contains(&byte) {
        return None;
    }
    Some(if byte >= lower + 20 {
        byte as isize - upper as isize - 1
    } else {
        byte as isize - lower as isize
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableEncoderFormulaError {
    message: String,
}

impl TableEncoderFormulaError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for TableEncoderFormulaError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for TableEncoderFormulaError {}
