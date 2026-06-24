# M35 Reader Audit

Command:

```powershell
rg -n "entries_by_code|TableLookup|exact_candidates|prefix_candidates|all_codes|Vec<Candidate>|CandidateRequest|TranslationResult|refresh_candidates|candidate_list_complete|prediction_never_first|sentence_over_completion|prefix_fallback|assign_ordered_candidate_qualities" crates/yune-core/src crates/yune-rime-api/src
```

## Classification

| Reader | Classification | M35 result |
| --- | --- | --- |
| `StaticTableTranslator::bounded_candidates_for_lookup_codes` | candidate-view-safe | now ranks borrowed `LookupCandidate` views and materializes only selected rows |
| `StaticTableTranslator::candidates_for_lookup_codes` | selected-materialization/full-list compatibility | iterates storage views, then materializes owned `Candidate` rows for eager callers |
| exact lookup probes | candidate-view-safe | use `TableStorage::exact_candidates` |
| prefix completion probes | candidate-view-safe | use `TableStorage::prefix_candidates` |
| dynamic correction all-code scan | heap-fallback-required for TypeDuck | compact upstream path does not enable TypeDuck dynamic correction; heap storage remains |
| `prefix_fallback_candidates` | selected-materialization | uses storage views and materializes accepted partial candidates |
| `sentence_candidate` | selected/full-list compatibility | uses storage views, materializes only sentence path pieces |
| `prediction_never_first` | heap/full-list compatibility | still runs after owned candidate materialization |
| filters/rankers/userdb/AI staging | full-list compatibility | remain downstream of owned candidates and do not force compact schemas to retain heap maps |
| ABI candidate-list iterators | full-list compatibility | still receive complete owned lists when requested |
| schema install | storage-choice owner | compiled upstream `luna_pinyin` now constructs compact storage with prism payload; unsupported/source/TypeDuck paths keep heap |

No unclassified heap reader remains in the compact-active upstream path. The
string `entries_by_code` remains only in heap constructors/helpers and TypeDuck
benchmark notes, not as retained storage for compact `luna_pinyin`.
