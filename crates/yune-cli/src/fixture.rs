use std::{fs, path::Path};

use crate::{
    args::validate_schema_id,
    rime_frontend::{run_frontend, FrontendOptions},
    sample_core::run_sequence,
};

// Owns fixture replay comparison for both retained core JSON and ABI-backed
// frontend transcripts; expected-vs-actual bodies are the librime comparison seam.
pub(crate) fn check_fixture(path: &Path) -> Result<(), String> {
    let expected = read_fixture(path)?;
    let sequence = sequence_from_fixture(&expected)?;
    let actual = run_sequence(&sequence)?.to_json();
    compare_fixture(path, &expected, &actual)
}

pub(crate) fn check_frontend_fixture(
    path: &Path,
    shared_data_dir: &Path,
    user_data_dir: &Path,
) -> Result<(), String> {
    let expected = read_fixture(path)?;
    let schema_id = validate_schema_id(string_field_from_fixture(&expected, "schema_id")?)?;
    let sequence = string_field_from_fixture(&expected, "sequence")?;
    let actual = run_frontend(FrontendOptions {
        shared_data_dir: shared_data_dir.to_path_buf(),
        user_data_dir: user_data_dir.to_path_buf(),
        schema_id,
        sequence,
    })?
    .to_json();

    compare_fixture(path, &expected, &actual)
}

fn read_fixture(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("failed to read {}: {error}", path.display()))
}

fn compare_fixture(path: &Path, expected: &str, actual: &str) -> Result<(), String> {
    if normalize_json(expected) == normalize_json(actual) {
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
    string_field_from_fixture(json, "sequence")
}

fn string_field_from_fixture(json: &str, field: &str) -> Result<String, String> {
    for (key, value_start) in root_field_values(json)? {
        if key == field {
            return parse_json_string(&json[value_start..])
                .map(|(value, _)| value)
                .map_err(|error| format!("invalid fixture {field}: {error}"));
        }
    }

    Err(format!(
        "fixture does not contain a top-level {field} string field"
    ))
}

fn root_field_values(json: &str) -> Result<Vec<(String, usize)>, String> {
    let mut fields = Vec::new();
    let mut index = skip_whitespace(json, 0);
    if !json[index..].starts_with('{') {
        return Err("fixture root must be a JSON object".to_owned());
    }
    index += 1;

    loop {
        index = skip_whitespace(json, index);
        if json[index..].starts_with('}') {
            return Ok(fields);
        }

        let (key, consumed) = parse_json_string(&json[index..])
            .map_err(|error| format!("invalid fixture object key: {error}"))?;
        index += consumed;
        index = skip_whitespace(json, index);
        if !json[index..].starts_with(':') {
            return Err("fixture object field is missing ':'".to_owned());
        }
        index += 1;
        index = skip_whitespace(json, index);
        fields.push((key, index));
        index = skip_json_value(json, index)?;
        index = skip_whitespace(json, index);
        match json[index..].chars().next() {
            Some(',') => index += 1,
            Some('}') => return Ok(fields),
            _ => return Err("fixture object field is missing ',' or '}'".to_owned()),
        }
    }
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

fn skip_whitespace(input: &str, mut index: usize) -> usize {
    while let Some(ch) = input[index..].chars().next() {
        if !ch.is_whitespace() {
            break;
        }
        index += ch.len_utf8();
    }
    index
}

fn skip_json_value(input: &str, index: usize) -> Result<usize, String> {
    match input[index..].chars().next() {
        Some('"') => parse_json_string(&input[index..]).map(|(_, consumed)| index + consumed),
        Some('{') => skip_balanced(input, index, '{', '}'),
        Some('[') => skip_balanced(input, index, '[', ']'),
        Some(_) => Ok(skip_json_literal(input, index)),
        None => Err("fixture field value is missing".to_owned()),
    }
}

fn skip_balanced(input: &str, mut index: usize, open: char, close: char) -> Result<usize, String> {
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    while let Some(ch) = input[index..].chars().next() {
        index += ch.len_utf8();
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        if ch == '"' {
            in_string = true;
        } else if ch == open {
            depth += 1;
        } else if ch == close {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Ok(index);
            }
        }
    }
    Err("unterminated fixture JSON container".to_owned())
}

fn skip_json_literal(input: &str, mut index: usize) -> usize {
    while let Some(ch) = input[index..].chars().next() {
        if ch == ',' || ch == '}' || ch == ']' || ch.is_whitespace() {
            break;
        }
        index += ch.len_utf8();
    }
    index
}

fn normalize_json(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut in_string = false;
    let mut escaped = false;

    for ch in input.chars() {
        if in_string {
            output.push(ch);
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
        } else if ch == '"' {
            in_string = true;
            output.push(ch);
        } else if !ch.is_whitespace() {
            output.push(ch);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::Path,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::{
        check_fixture, check_frontend_fixture, compare_fixture, sequence_from_fixture,
        string_field_from_fixture,
    };
    use crate::rime_frontend::{frontend_test_guard, run_frontend, FrontendOptions};

    fn unique_temp_dir(label: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "yune-cli-frontend-fixture-{label}-{}-{nanos}",
            std::process::id()
        ))
    }

    fn write_runtime(root: &std::path::Path) -> (std::path::PathBuf, std::path::PathBuf) {
        let shared = root.join("shared");
        let user = root.join("user");
        let staging = user.join("build");
        fs::create_dir_all(&shared).expect("shared dir should be created");
        fs::create_dir_all(&staging).expect("staging dir should be created");
        fs::write(
            shared.join("default.yaml"),
            "config_version: test\nschema_list:\n  - schema: default\n",
        )
        .expect("default config should be written");
        fs::write(
            shared.join("default.schema.yaml"),
            "schema:\n  schema_id: default\n  name: Default\n",
        )
        .expect("schema config should be written");
        (shared, user)
    }

    #[test]
    fn reads_sequence_from_fixture() {
        let fixture = "{ \"schema_id\": \"sample\", \"sequence\": \"nihao \" }";

        assert_eq!(sequence_from_fixture(fixture).as_deref(), Ok("nihao "));
        assert_eq!(
            string_field_from_fixture(fixture, "schema_id").as_deref(),
            Ok("sample")
        );
    }

    #[test]
    fn fixture_field_reader_only_accepts_top_level_fields() {
        let fixture = "{ \"events\": [{ \"schema_id\": \"nested\" }], \"sequence\": \"ni\" }";

        assert_eq!(
            string_field_from_fixture(fixture, "schema_id"),
            Err("fixture does not contain a top-level schema_id string field".to_owned())
        );
    }

    #[test]
    fn fixture_comparison_preserves_whitespace_inside_strings() {
        let expected = "{\n  \"schema_id\": \"default\",\n  \"sequence\": \"ni \"\n}\n";
        let actual = "{\"schema_id\":\"default\",\"sequence\":\"ni\"}\n";

        let error = compare_fixture(Path::new("fixture.json"), expected, actual)
            .expect_err("string whitespace should be semantic");

        assert!(error.contains("fixture mismatch:"));
    }

    #[test]
    fn frontend_fixture_rejects_invalid_schema_id() {
        let _guard = frontend_test_guard();
        let root = unique_temp_dir("invalid-schema");
        let (shared, user) = write_runtime(&root);
        let fixture = root.join("frontend.json");
        fs::write(
            &fixture,
            "{\n  \"schema_id\": \"../default\",\n  \"sequence\": \"ni\",\n  \"events\": [],\n  \"commits\": [],\n  \"context\": {},\n  \"status\": {}\n}\n",
        )
        .expect("fixture should be written");

        let error = check_frontend_fixture(&fixture, &shared, &user)
            .expect_err("invalid fixture schema should fail");

        assert_eq!(
            error,
            "error: invalid --schema. next: pass a logical schema id such as luna_pinyin."
        );

        fs::remove_dir_all(root).expect("temp dirs should be removed");
    }

    #[test]
    fn checks_frontend_fixture_against_abi_replay() {
        let _guard = frontend_test_guard();
        let root = unique_temp_dir("match");
        let (shared, user) = write_runtime(&root);
        let fixture = root.join("frontend.json");
        let expected = run_frontend(FrontendOptions {
            shared_data_dir: shared.clone(),
            user_data_dir: user.clone(),
            schema_id: "default".to_owned(),
            sequence: "ni".to_owned(),
        })
        .expect("frontend run should succeed")
        .to_json();
        fs::write(&fixture, expected).expect("fixture should be written");

        check_frontend_fixture(&fixture, &shared, &user).unwrap_or_else(|error| panic!("{error}"));

        fs::remove_dir_all(root).expect("temp dirs should be removed");
    }

    #[test]
    fn frontend_fixture_mismatch_reports_expected_and_actual() {
        let _guard = frontend_test_guard();
        let root = unique_temp_dir("mismatch");
        let (shared, user) = write_runtime(&root);
        let fixture = root.join("frontend.json");
        fs::write(
            &fixture,
            "{\n  \"schema_id\": \"default\",\n  \"sequence\": \"n\",\n  \"events\": [],\n  \"commits\": [],\n  \"context\": {},\n  \"status\": {}\n}\n",
        )
        .expect("fixture should be written");

        let error =
            check_frontend_fixture(&fixture, &shared, &user).expect_err("fixture should mismatch");

        assert!(error.contains("fixture mismatch:"));
        assert!(error.contains("expected:"));
        assert!(error.contains("actual:"));
        assert!(!error.contains("0x"));

        fs::remove_dir_all(root).expect("temp dirs should be removed");
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
