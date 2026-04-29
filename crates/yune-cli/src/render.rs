use crate::{args::help_text, rime_frontend::FrontendRun};

// Owns plain operator-facing frontend output labels for future librime CLI
// transcript comparison; GUI/native frontend rendering remains Phase 2 scope.
pub(crate) fn print_help() {
    println!("{}", help_text());
}

pub(crate) fn render_frontend_human(run: &FrontendRun) -> String {
    let mut output = String::new();
    for (index, event) in run.events.iter().enumerate() {
        if index > 0 {
            output.push('\n');
        }
        push_line(&mut output, "event", &index.to_string());
        push_line(&mut output, "key", &event.key);
        push_line(&mut output, "handled", bool_text(event.handled));
        if event.commits.is_empty() {
            push_line(&mut output, "commit", "none");
        } else {
            for commit in &event.commits {
                push_line(&mut output, "commit", commit);
            }
        }
        push_line(&mut output, "preedit", &event.context.preedit);
        push_line(&mut output, "caret", &event.context.caret.to_string());
        push_line(
            &mut output,
            "highlighted",
            &event.context.highlighted.to_string(),
        );
        push_candidates(
            &mut output,
            event.context.highlighted,
            &event.context.candidates,
        );
        push_line(&mut output, "status", &status_summary(run, index));
    }
    output
}

fn push_candidates(
    output: &mut String,
    highlighted: usize,
    candidates: &[crate::rime_frontend::FrontendCandidate],
) {
    if candidates.is_empty() {
        push_line(output, "candidates", "none");
        return;
    }

    for (index, candidate) in candidates.iter().enumerate() {
        push_line(output, "candidate", &index.to_string());
        push_line(output, "text", &candidate.text);
        push_line(output, "comment", &candidate.comment);
        push_line(output, "source", &candidate.source);
        push_line(output, "quality", &candidate.quality.to_string());
        push_line(
            output,
            "selected",
            if index == highlighted { "yes" } else { "no" },
        );
    }
}

fn status_summary(run: &FrontendRun, index: usize) -> String {
    let status = &run.events[index].status;
    format!(
        "schema_id={} schema_name={} disabled={} composing={} ascii_mode={} full_shape={} simplified={} traditional={} ascii_punct={}",
        status.schema_id,
        status.schema_name,
        bool_text(status.is_disabled),
        bool_text(status.is_composing),
        bool_text(status.is_ascii_mode),
        bool_text(status.is_full_shape),
        bool_text(status.is_simplified),
        bool_text(status.is_traditional),
        bool_text(status.is_ascii_punct)
    )
}

fn push_line(output: &mut String, label: &str, value: &str) {
    output.push_str(label);
    output.push_str(": ");
    output.push_str(value);
    output.push('\n');
}

fn bool_text(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

#[cfg(test)]
mod tests {
    use super::render_frontend_human;
    use crate::rime_frontend::{
        FrontendCandidate, FrontendContext, FrontendEvent, FrontendRun, FrontendStatus,
    };

    fn status() -> FrontendStatus {
        FrontendStatus {
            schema_id: "luna".to_owned(),
            schema_name: "Luna".to_owned(),
            is_disabled: false,
            is_composing: true,
            is_ascii_mode: false,
            is_full_shape: false,
            is_simplified: true,
            is_traditional: false,
            is_ascii_punct: true,
        }
    }

    fn run_with_candidates(candidates: Vec<FrontendCandidate>) -> FrontendRun {
        FrontendRun {
            schema_id: "luna".to_owned(),
            sequence: "ni".to_owned(),
            events: vec![FrontendEvent {
                key: "n".to_owned(),
                keycode: 110,
                mask: 0,
                handled: true,
                commits: vec!["你".to_owned()],
                context: FrontendContext {
                    input: "ni".to_owned(),
                    caret: 2,
                    preedit: "ni".to_owned(),
                    highlighted: 0,
                    last_commit: None,
                    candidates,
                    page_size: 5,
                    page_no: 0,
                    is_last_page: true,
                    select_keys: None,
                    select_labels: vec![],
                },
                status: status(),
            }],
            commits: vec!["你".to_owned()],
            context: FrontendContext {
                input: "ni".to_owned(),
                caret: 2,
                preedit: "ni".to_owned(),
                highlighted: 0,
                last_commit: None,
                candidates: vec![],
                page_size: 5,
                page_no: 0,
                is_last_page: true,
                select_keys: None,
                select_labels: vec![],
            },
            status: status(),
        }
    }

    #[test]
    fn renders_frontend_events_as_plain_text() {
        let run = run_with_candidates(vec![FrontendCandidate {
            text: "你".to_owned(),
            comment: "ni".to_owned(),
            source: "table".to_owned(),
            quality: 10,
        }]);

        let output = render_frontend_human(&run);

        assert_eq!(
            output,
            "event: 0\nkey: n\nhandled: true\ncommit: 你\npreedit: ni\ncaret: 2\nhighlighted: 0\ncandidate: 0\ntext: 你\ncomment: ni\nsource: table\nquality: 10\nselected: yes\nstatus: schema_id=luna schema_name=Luna disabled=false composing=true ascii_mode=false full_shape=false simplified=true traditional=false ascii_punct=true\n"
        );
        assert!(!output.contains('\u{1b}'));
        assert!(!output.contains("0x"));
        assert!(!output.contains("/tmp/"));
    }

    #[test]
    fn renders_empty_candidate_pages_as_none() {
        let run = run_with_candidates(vec![]);

        let output = render_frontend_human(&run);

        assert!(output.contains("candidates: none\n"));
    }
}
