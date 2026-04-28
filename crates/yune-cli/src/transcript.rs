use yune_core::{Candidate, Context, Snapshot, Status};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FixtureOutput {
    pub(crate) schema_id: String,
    pub(crate) sequence: String,
    pub(crate) commits: Vec<String>,
    pub(crate) snapshot: Snapshot,
}

impl FixtureOutput {
    pub(crate) fn to_json(&self) -> String {
        let mut json = String::new();
        json.push_str("{\n");
        push_field(
            &mut json,
            1,
            "schema_id",
            &json_string(&self.schema_id),
            true,
        );
        push_field(&mut json, 1, "sequence", &json_string(&self.sequence), true);
        push_field(
            &mut json,
            1,
            "commits",
            &json_string_array(&self.commits),
            true,
        );
        push_field(
            &mut json,
            1,
            "context",
            &context_json(&self.snapshot.context, 1),
            true,
        );
        push_field(
            &mut json,
            1,
            "status",
            &status_json(&self.snapshot.status, 1),
            false,
        );
        json.push_str("}\n");
        json
    }
}

fn context_json(context: &Context, depth: usize) -> String {
    let mut json = String::new();
    json.push_str("{\n");
    push_field(
        &mut json,
        depth + 1,
        "input",
        &json_string(&context.composition.input),
        true,
    );
    push_field(
        &mut json,
        depth + 1,
        "caret",
        &context.composition.caret.to_string(),
        true,
    );
    push_field(
        &mut json,
        depth + 1,
        "preedit",
        &json_string(&context.composition.preedit),
        true,
    );
    push_field(
        &mut json,
        depth + 1,
        "highlighted",
        &context.highlighted.to_string(),
        true,
    );
    push_field(
        &mut json,
        depth + 1,
        "last_commit",
        &optional_string_json(context.last_commit.as_deref()),
        true,
    );
    push_field(
        &mut json,
        depth + 1,
        "candidates",
        &candidates_json(&context.candidates, depth + 1),
        false,
    );
    push_indent(&mut json, depth);
    json.push('}');
    json
}

fn candidates_json(candidates: &[Candidate], depth: usize) -> String {
    if candidates.is_empty() {
        return "[]".to_owned();
    }

    let mut json = String::new();
    json.push_str("[\n");
    for (index, candidate) in candidates.iter().enumerate() {
        push_indent(&mut json, depth + 1);
        json.push_str("{\n");
        push_field(
            &mut json,
            depth + 2,
            "text",
            &json_string(&candidate.text),
            true,
        );
        push_field(
            &mut json,
            depth + 2,
            "comment",
            &json_string(&candidate.comment),
            true,
        );
        push_field(
            &mut json,
            depth + 2,
            "source",
            &json_string(candidate.source.as_str()),
            true,
        );
        push_field(
            &mut json,
            depth + 2,
            "quality",
            &candidate.quality.to_string(),
            false,
        );
        push_indent(&mut json, depth + 1);
        json.push('}');
        if index + 1 != candidates.len() {
            json.push(',');
        }
        json.push('\n');
    }
    push_indent(&mut json, depth);
    json.push(']');
    json
}

fn status_json(status: &Status, depth: usize) -> String {
    let mut json = String::new();
    json.push_str("{\n");
    push_field(
        &mut json,
        depth + 1,
        "schema_id",
        &json_string(&status.schema_id),
        true,
    );
    push_field(
        &mut json,
        depth + 1,
        "schema_name",
        &json_string(&status.schema_name),
        true,
    );
    push_field(
        &mut json,
        depth + 1,
        "is_disabled",
        &status.is_disabled.to_string(),
        true,
    );
    push_field(
        &mut json,
        depth + 1,
        "is_composing",
        &status.is_composing.to_string(),
        true,
    );
    push_field(
        &mut json,
        depth + 1,
        "is_ascii_mode",
        &status.is_ascii_mode.to_string(),
        true,
    );
    push_field(
        &mut json,
        depth + 1,
        "is_full_shape",
        &status.is_full_shape.to_string(),
        true,
    );
    push_field(
        &mut json,
        depth + 1,
        "is_simplified",
        &status.is_simplified.to_string(),
        true,
    );
    push_field(
        &mut json,
        depth + 1,
        "is_traditional",
        &status.is_traditional.to_string(),
        true,
    );
    push_field(
        &mut json,
        depth + 1,
        "is_ascii_punct",
        &status.is_ascii_punct.to_string(),
        false,
    );
    push_indent(&mut json, depth);
    json.push('}');
    json
}

fn push_field(json: &mut String, depth: usize, key: &str, value: &str, comma: bool) {
    push_indent(json, depth);
    json.push('"');
    json.push_str(key);
    json.push_str("\": ");
    json.push_str(value);
    if comma {
        json.push(',');
    }
    json.push('\n');
}

fn push_indent(json: &mut String, depth: usize) {
    for _ in 0..depth {
        json.push_str("  ");
    }
}

fn json_string_array(values: &[String]) -> String {
    let values = values
        .iter()
        .map(|value| json_string(value))
        .collect::<Vec<_>>();
    format!("[{}]", values.join(", "))
}

fn optional_string_json(value: Option<&str>) -> String {
    value.map_or_else(|| "null".to_owned(), json_string)
}

pub(crate) fn json_string(value: &str) -> String {
    let mut output = String::with_capacity(value.len() + 2);
    output.push('"');
    for ch in value.chars() {
        match ch {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\u{8}' => output.push_str("\\b"),
            '\u{c}' => output.push_str("\\f"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            control if control.is_control() => {
                output.push_str(&format!("\\u{:04x}", u32::from(control)));
            }
            other => output.push(other),
        }
    }
    output.push('"');
    output
}

#[cfg(test)]
mod tests {
    use super::json_string;

    #[test]
    fn escapes_json_strings() {
        assert_eq!(json_string("a\"b\\c\n"), "\"a\\\"b\\\\c\\n\"");
    }
}
