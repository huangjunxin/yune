#[cfg(test)]
mod tests {
    use crate::{
        BackdatedScanPolicy, CandidateSource, Engine, StaticTableTranslator, UserDb,
        UserDbLookupRequest,
    };

    #[test]
    fn userdb_learning_commit_records_metadata_before_clear() {
        let mut engine = Engine::new();
        engine.add_translator(StaticTableTranslator::new([("ni", "你")]));
        engine.set_input("ni");

        assert_eq!(engine.commit_composition(), Some("你".to_owned()));
        assert!(engine.context().composition.input.is_empty());

        let event = engine
            .take_pending_userdb_learning()
            .expect("commit should expose a pending learning event");
        assert_eq!(event.input, "ni");
        assert_eq!(event.selected_text, "你");
        assert_eq!(event.candidate_type, "table");
        assert_eq!(event.candidate_source, CandidateSource::Table);
        assert_eq!(event.segment_start, 0);
        assert_eq!(event.segment_end, 2);
        assert_eq!(event.tick, 1);
    }

    #[test]
    fn userdb_learning_repeated_commits_increase_quality_and_emit_updates() {
        let mut db = UserDb::default();
        let mut engine = Engine::new();
        engine.add_translator(StaticTableTranslator::new([("ni", "你")]));

        for _ in 0..2 {
            engine.set_input("ni");
            assert_eq!(engine.commit_composition(), Some("你".to_owned()));
            let event = engine
                .take_pending_userdb_learning()
                .expect("commit should expose learning metadata");
            let update = db.record_commit(&event);
            assert_eq!(update.input, "ni");
            assert_eq!(update.selected_text, "你");
        }

        let request = UserDbLookupRequest::new("ni");
        let learned = db.lookup(&request);
        assert_eq!(learned[0].text, "你");
        assert_eq!(learned[0].value.commits, 2);
        assert!(learned[0].quality > 1.5);
    }

    #[test]
    fn predictive_userdb_lookup_returns_prefix_matches_before_optional_rankers() {
        let mut db = UserDb::default();
        db.learn_entry("ni hao", "你好", 2, 2.0, 2);
        db.learn_entry("ni", "你", 1, 1.0, 1);

        let predictive = db.lookup(&UserDbLookupRequest::new("ni").with_predictive(true));
        assert_eq!(predictive[0].text, "你");
        assert_eq!(predictive[0].source, CandidateSource::UserTable);
        assert_eq!(predictive[1].text, "你好");
        assert_eq!(predictive[1].comment, "~hao");

        let mut engine = Engine::new();
        engine.add_translator(StaticTableTranslator::new([("ni", "尼")]).with_initial_quality(0.0));
        engine.set_userdb(db);
        engine.set_input("ni");

        assert_eq!(engine.context().candidates[0].source, CandidateSource::UserTable);
        assert_eq!(engine.context().candidates[0].text, "你");
    }

    #[test]
    fn backdated_scan_scope_is_explicit_and_excludes_history_or_ai_memory() {
        let policy = BackdatedScanPolicy::normal_runtime_context_only();
        assert!(policy.scans_commit_records);
        assert!(policy.scans_current_composition);
        assert!(!policy.scans_history_translator);
        assert!(!policy.scans_ai_ranker_memory);

        let mut engine = Engine::new();
        engine.add_translator(StaticTableTranslator::new([("hao", "好")]));
        engine.set_input("hao");
        assert_eq!(engine.commit_composition(), Some("好".to_owned()));
        let event = engine
            .take_pending_userdb_learning()
            .expect("normal commit context should be scanable");
        let scanned = policy.scan_commit_event(&event);
        assert_eq!(scanned.input, "hao");
        assert_eq!(scanned.selected_text, "好");
    }
}
