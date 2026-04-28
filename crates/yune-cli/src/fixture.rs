use std::{fs, path::Path};

use crate::sample_core::run_sequence;

pub(crate) fn check_fixture(path: &Path) -> Result<(), String> {
    let expected = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    let sequence = sequence_from_fixture(&expected)?;
    let actual = run_sequence(&sequence)?.to_json();
    if normalize_json(&expected) == normalize_json(&actual) {
        println!("ok {}", path.display());
        return Ok(());
    }

    Err(format!(
        "fixture mismatch: {}\n\nexpected:\n{}\n\nactual:\n{}",
        path.display(),
        expected.trim(),
        actual
    ))
}

pub(crate) fn sequence_from_fixture(json: &str) -> Result<String, String> {
    let key = "\"sequence\"";
    let key_start = json
        .find(key)
        .ok_or_else(|| "fixture does not contain a top-level sequence field".to_owned())?;
    let after_key = &json[key_start + key.len()..];
    let colon = after_key
        .find(':')
        .ok_or_else(|| "fixture sequence field is missing ':'".to_owned())?;
    let after_colon = after_key[colon + 1..].trim_start();
    parse_json_string(after_colon)
        .map(|(sequence, _)| sequence)
        .map_err(|error| format!("invalid fixture sequence: {error}"))
}

fn parse_json_string(input: &str) -> Result<(String, usize), String> {
    let mut chars = input.char_indices();
    match chars.next() {
        Some((_, '"')) => {}
        _ => return Err("expected string".to_owned()),
    }

    let mut value = String::new();
    let mut escaped = false;
    for (index, ch) in chars {
        if escaped {
            match ch {
                '"' => value.push('"'),
                '\\' => value.push('\\'),
                '/' => value.push('/'),
                'b' => value.push('\u{8}'),
                'f' => value.push('\u{c}'),
                'n' => value.push('\n'),
                'r' => value.push('\r'),
                't' => value.push('\t'),
                'u' => {
                    return Err(
                        "unicode escapes are not supported in fixture sequence strings".to_owned(),
                    );
                }
                other => return Err(format!("unsupported escape: \\{other}")),
            }
            escaped = false;
            continue;
        }

        match ch {
            '\\' => escaped = true,
            '"' => return Ok((value, index + ch.len_utf8())),
            other => value.push(other),
        }
    }

    Err("unterminated string".to_owned())
}

fn normalize_json(input: &str) -> String {
    input.chars().filter(|ch| !ch.is_whitespace()).collect()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{check_fixture, sequence_from_fixture};

    #[test]
    fn reads_sequence_from_fixture() {
        let fixture = "{ \"schema_id\": \"sample\", \"sequence\": \"nihao \" }";

        assert_eq!(sequence_from_fixture(fixture).as_deref(), Ok("nihao "));
    }

    #[test]
    fn checked_in_fixtures_match_cli_output() {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let fixtures_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .expect("CLI crate should live under workspace crates")
            .join("fixtures");
        let mut fixtures = std::fs::read_dir(&fixtures_dir)
            .expect("fixtures directory should be readable")
            .map(|entry| entry.expect("fixture entry should be readable").path())
            .filter(|path| {
                path.extension()
                    .is_some_and(|extension| extension == "json")
            })
            .collect::<Vec<_>>();
        fixtures.sort();

        assert!(!fixtures.is_empty());
        for fixture in fixtures {
            check_fixture(&fixture).unwrap_or_else(|error| panic!("{error}"));
        }
    }
}
