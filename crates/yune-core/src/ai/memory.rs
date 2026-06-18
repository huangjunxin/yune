use crate::{AiConfidence, AiPrivacyPolicy, CandidateSource, Context, UserDbCommitMetadata};

pub const MEMORY_STORE_FILE_SUFFIX: &str = ".ai-memory";
pub const MEMORY_STORE_SNAPSHOT_SUFFIX: &str = ".ai-memory.txt";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiMemoryEntry {
    pub input: String,
    pub selected_text: String,
    pub provider: String,
    pub confidence: AiConfidence,
    pub app_id: Option<String>,
    pub field_id: Option<String>,
    pub preceding_text: Option<String>,
    pub commits: u32,
    pub last_tick: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AiMemorySkipReason {
    NonAiCandidate,
    Disabled,
    Privacy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AiMemoryRecordResult {
    Recorded(AiMemoryEntry),
    Skipped(AiMemorySkipReason),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiMemorySnapshotError {
    message: &'static str,
}

impl AiMemorySnapshotError {
    const fn new(message: &'static str) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for AiMemorySnapshotError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.message)
    }
}

impl std::error::Error for AiMemorySnapshotError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryStore {
    enabled: bool,
    entries: Vec<AiMemoryEntry>,
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self {
            enabled: true,
            entries: Vec::new(),
        }
    }
}

impl MemoryStore {
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    #[must_use]
    pub fn entries(&self) -> &[AiMemoryEntry] {
        &self.entries
    }

    #[must_use]
    pub fn into_entries(self) -> Vec<AiMemoryEntry> {
        self.entries
    }

    pub fn record_commit(
        &mut self,
        context: &Context,
        metadata: &UserDbCommitMetadata,
    ) -> AiMemoryRecordResult {
        let CandidateSource::Ai {
            provider,
            confidence,
        } = &metadata.candidate_source
        else {
            return AiMemoryRecordResult::Skipped(AiMemorySkipReason::NonAiCandidate);
        };

        if !self.enabled {
            return AiMemoryRecordResult::Skipped(AiMemorySkipReason::Disabled);
        }
        if !AiPrivacyPolicy.allows_learning(context) {
            return AiMemoryRecordResult::Skipped(AiMemorySkipReason::Privacy);
        }

        let key = AiMemoryKey {
            input: metadata.input.as_str(),
            selected_text: metadata.selected_text.as_str(),
            provider: provider.as_str(),
            app_id: context.ai_context.app_id.as_deref(),
            field_id: context.ai_context.field_id.as_deref(),
        };
        let entry = if let Some(entry) = self.entries.iter_mut().find(|entry| key.matches(entry)) {
            entry.commits = entry.commits.saturating_add(1);
            entry.confidence = (*confidence).max(entry.confidence);
            entry.preceding_text = context.ai_context.preceding_text.clone();
            entry.last_tick = entry.last_tick.max(metadata.tick);
            entry.clone()
        } else {
            let entry = AiMemoryEntry {
                input: metadata.input.clone(),
                selected_text: metadata.selected_text.clone(),
                provider: provider.clone(),
                confidence: *confidence,
                app_id: context.ai_context.app_id.clone(),
                field_id: context.ai_context.field_id.clone(),
                preceding_text: context.ai_context.preceding_text.clone(),
                commits: 1,
                last_tick: metadata.tick,
            };
            self.entries.push(entry.clone());
            entry
        };
        AiMemoryRecordResult::Recorded(entry)
    }

    #[must_use]
    pub fn export_snapshot(&self) -> String {
        let mut output = String::from("# yune ai memory\n/db_type\tai_memory\n/version\t1\n");
        output.push_str(&format!("/enabled\t{}\n", self.enabled));
        for entry in &self.entries {
            let confidence = entry.confidence.basis_points().to_string();
            let commits = entry.commits.to_string();
            let last_tick = entry.last_tick.to_string();
            let fields = [
                entry.input.as_str(),
                entry.selected_text.as_str(),
                entry.provider.as_str(),
                confidence.as_str(),
                commits.as_str(),
                last_tick.as_str(),
                entry.app_id.as_deref().unwrap_or_default(),
                entry.field_id.as_deref().unwrap_or_default(),
                entry.preceding_text.as_deref().unwrap_or_default(),
            ];
            output.push_str(
                &fields
                    .iter()
                    .map(|field| escape_snapshot_field(field))
                    .collect::<Vec<_>>()
                    .join("\t"),
            );
            output.push('\n');
        }
        output
    }

    pub fn import_snapshot(input: &str) -> Result<Self, AiMemorySnapshotError> {
        let mut store = Self::default();
        let mut saw_type = false;
        for line in input.lines() {
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line
                .split_once('\t')
                .filter(|(key, _)| key.starts_with('/'))
            {
                match key {
                    "/db_type" if value == "ai_memory" => saw_type = true,
                    "/db_type" => {
                        return Err(AiMemorySnapshotError::new(
                            "invalid ai memory snapshot type",
                        ));
                    }
                    "/version" if value == "1" => {}
                    "/version" => {
                        return Err(AiMemorySnapshotError::new(
                            "invalid ai memory snapshot version",
                        ));
                    }
                    "/enabled" => {
                        store.enabled = value
                            .parse()
                            .map_err(|_| AiMemorySnapshotError::new("invalid enabled flag"))?;
                    }
                    _ => {}
                }
                continue;
            }

            let fields = line
                .split('\t')
                .map(unescape_snapshot_field)
                .collect::<Result<Vec<_>, _>>()?;
            if fields.len() != 9 {
                return Err(AiMemorySnapshotError::new("invalid ai memory entry"));
            }
            let confidence = fields[3]
                .parse()
                .map(AiConfidence::from_basis_points)
                .map_err(|_| AiMemorySnapshotError::new("invalid ai memory confidence"))?;
            let commits = fields[4]
                .parse()
                .map_err(|_| AiMemorySnapshotError::new("invalid ai memory commit count"))?;
            let last_tick = fields[5]
                .parse()
                .map_err(|_| AiMemorySnapshotError::new("invalid ai memory tick"))?;
            store.entries.push(AiMemoryEntry {
                input: fields[0].clone(),
                selected_text: fields[1].clone(),
                provider: fields[2].clone(),
                confidence,
                commits,
                last_tick,
                app_id: optional_snapshot_field(&fields[6]),
                field_id: optional_snapshot_field(&fields[7]),
                preceding_text: optional_snapshot_field(&fields[8]),
            });
        }
        if !saw_type {
            return Err(AiMemorySnapshotError::new(
                "missing ai memory snapshot type",
            ));
        }
        Ok(store)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AiMemoryKey<'a> {
    input: &'a str,
    selected_text: &'a str,
    provider: &'a str,
    app_id: Option<&'a str>,
    field_id: Option<&'a str>,
}

impl AiMemoryKey<'_> {
    fn matches(self, entry: &AiMemoryEntry) -> bool {
        entry.input == self.input
            && entry.selected_text == self.selected_text
            && entry.provider == self.provider
            && entry.app_id.as_deref() == self.app_id
            && entry.field_id.as_deref() == self.field_id
    }
}

#[must_use]
pub fn memory_store_file_name(logical_id: &str) -> Option<String> {
    validate_memory_store_id(logical_id).map(|id| format!("{id}{MEMORY_STORE_FILE_SUFFIX}"))
}

#[must_use]
pub fn memory_store_snapshot_file_name(logical_id: &str) -> Option<String> {
    validate_memory_store_id(logical_id).map(|id| format!("{id}{MEMORY_STORE_SNAPSHOT_SUFFIX}"))
}

#[must_use]
pub fn validate_memory_store_id(id: &str) -> Option<String> {
    let normalized = id
        .strip_suffix(MEMORY_STORE_SNAPSHOT_SUFFIX)
        .or_else(|| id.strip_suffix(MEMORY_STORE_FILE_SUFFIX))
        .unwrap_or(id);
    if normalized.ends_with(".userdb") || normalized.ends_with(".userdb.txt") {
        return None;
    }
    validate_logical_id(normalized)
}

fn validate_logical_id(id: &str) -> Option<String> {
    if id.is_empty()
        || id == "."
        || id == ".."
        || id.starts_with('~')
        || id.contains('\0')
        || id.contains('/')
        || id.contains('\\')
        || has_windows_drive_prefix(id)
    {
        return None;
    }

    Some(id.to_owned())
}

fn has_windows_drive_prefix(id: &str) -> bool {
    let bytes = id.as_bytes();
    bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':'
}

fn optional_snapshot_field(input: &str) -> Option<String> {
    (!input.is_empty()).then(|| input.to_owned())
}

fn escape_snapshot_field(input: &str) -> String {
    let mut escaped = String::new();
    for ch in input.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '\t' => escaped.push_str("\\t"),
            '\r' => escaped.push_str("\\r"),
            '\n' => escaped.push_str("\\n"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn unescape_snapshot_field(input: &str) -> Result<String, AiMemorySnapshotError> {
    let mut output = String::new();
    let mut chars = input.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            output.push(ch);
            continue;
        }
        let Some(escaped) = chars.next() else {
            return Err(AiMemorySnapshotError::new(
                "invalid ai memory snapshot escape",
            ));
        };
        match escaped {
            '\\' => output.push('\\'),
            't' => output.push('\t'),
            'r' => output.push('\r'),
            'n' => output.push('\n'),
            _ => {
                return Err(AiMemorySnapshotError::new(
                    "invalid ai memory snapshot escape",
                ));
            }
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use crate::{
        memory_store_file_name, memory_store_snapshot_file_name, validate_memory_store_id,
        AiConfidence, AiContext, CandidateSource, Context, MemoryStore, UserDbCommitMetadata,
    };

    fn ai_commit(input: &str, text: &str, tick: u64) -> UserDbCommitMetadata {
        UserDbCommitMetadata::new(
            input,
            text,
            CandidateSource::ai("mock", AiConfidence::from_score(0.72)),
            0,
            input.len(),
            tick,
        )
    }

    #[test]
    fn records_standard_ai_commits_and_aggregates_repeated_selection() {
        let mut store = MemoryStore::default();
        let mut context = Context {
            ai_context: AiContext::standard()
                .with_app_id("sample_cli")
                .with_field_id("search")
                .with_preceding_text("hello"),
            ..Context::default()
        };

        let first =
            store.record_commit(&context, &ai_commit("nihao", "\u{4f60}\u{597d}\u{5440}", 1));
        context.ai_context = context
            .ai_context
            .clone()
            .with_preceding_text("hello again");
        let second =
            store.record_commit(&context, &ai_commit("nihao", "\u{4f60}\u{597d}\u{5440}", 3));

        assert!(matches!(first, crate::AiMemoryRecordResult::Recorded(_)));
        assert!(matches!(second, crate::AiMemoryRecordResult::Recorded(_)));
        assert_eq!(store.entries().len(), 1);
        let entry = &store.entries()[0];
        assert_eq!(entry.input, "nihao");
        assert_eq!(entry.selected_text, "\u{4f60}\u{597d}\u{5440}");
        assert_eq!(entry.provider, "mock");
        assert_eq!(entry.commits, 2);
        assert_eq!(entry.last_tick, 3);
        assert_eq!(entry.app_id.as_deref(), Some("sample_cli"));
        assert_eq!(entry.field_id.as_deref(), Some("search"));
        assert_eq!(entry.preceding_text.as_deref(), Some("hello again"));
    }

    #[test]
    fn sensitive_context_skips_ai_memory_writes() {
        let mut store = MemoryStore::default();
        let context = Context::default();

        let result =
            store.record_commit(&context, &ai_commit("nihao", "\u{4f60}\u{597d}\u{5440}", 1));

        assert_eq!(
            result,
            crate::AiMemoryRecordResult::Skipped(crate::AiMemorySkipReason::Privacy)
        );
        assert!(store.entries().is_empty());
    }

    #[test]
    fn disabled_store_skips_writes_and_clear_removes_entries() {
        let mut store = MemoryStore::default();
        let context = Context {
            ai_context: AiContext::standard(),
            ..Context::default()
        };
        assert!(store.is_enabled());

        store.set_enabled(false);
        let result =
            store.record_commit(&context, &ai_commit("nihao", "\u{4f60}\u{597d}\u{5440}", 1));
        assert_eq!(
            result,
            crate::AiMemoryRecordResult::Skipped(crate::AiMemorySkipReason::Disabled)
        );
        assert!(store.entries().is_empty());

        store.set_enabled(true);
        store.record_commit(&context, &ai_commit("nihao", "\u{4f60}\u{597d}\u{5440}", 2));
        assert_eq!(store.entries().len(), 1);
        store.clear();
        assert!(store.entries().is_empty());
    }

    #[test]
    fn memory_store_names_use_ai_memory_namespace_not_userdb() {
        assert_eq!(
            memory_store_file_name("jyut6ping3").as_deref(),
            Some("jyut6ping3.ai-memory")
        );
        assert_eq!(
            memory_store_snapshot_file_name("jyut6ping3").as_deref(),
            Some("jyut6ping3.ai-memory.txt")
        );
        assert!(memory_store_file_name("jyut6ping3")
            .expect("name should be valid")
            .ends_with(".ai-memory"));
        assert!(!memory_store_file_name("jyut6ping3")
            .expect("name should be valid")
            .ends_with(".userdb"));
        assert!(validate_memory_store_id("../jyut6ping3").is_none());
        assert!(validate_memory_store_id("C:\\jyut6ping3").is_none());
        assert!(validate_memory_store_id("jyut6ping3.userdb").is_none());
        assert!(validate_memory_store_id("jyut6ping3.userdb.txt").is_none());
    }

    #[test]
    fn memory_store_snapshot_round_trips_escaped_fields() {
        let mut store = MemoryStore::default();
        let context = Context {
            ai_context: AiContext::standard()
                .with_app_id("sample\tcli")
                .with_field_id("message")
                .with_preceding_text("hello\nthere"),
            ..Context::default()
        };
        store.record_commit(&context, &ai_commit("ni\thao", "\u{4f60}\\\u{597d}", 1));

        let snapshot = store.export_snapshot();
        let round_tripped =
            MemoryStore::import_snapshot(&snapshot).expect("snapshot should round-trip");

        assert_eq!(round_tripped, store);
    }
}
