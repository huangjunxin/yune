# M34 full-list reader audit

Date: 2026-06-23

Audit command shape:

```powershell
rg -n "entries_by_code|entries_from_entries_by_code|entries_by_code_from_entries|BTreeMap<String, Vec<Candidate>>|fn translate\(|refresh_candidates|sort_by|apply_with_context|try_rerank|prediction_never_first|sentence_over_completion|prefix_fallback|assign_ordered_candidate_qualities" crates/yune-core/src crates/yune-rime-api/src
```

## Translator readers

| Reader | Classification | M34 treatment |
| --- | --- | --- |
| Exact `entries_by_code` lookup | Exact lookup | Added `TableLookup::exact_candidates`; eager path retained. |
| Prefix `entries_by_code.range(...)` completion | Prefix/range lookup | Added `TableLookup::prefix_candidates`; bounded path enumerates complete prefix range but keeps references. |
| `entries_by_code.keys()` dynamic correction | Full-code scan | Eager fallback; this remains a storage-swap blocker. |
| Sentence segmentation and sentence candidates | Full-list/DP behavior | Eager fallback; no bounded sentence path landed. |
| `prefix_fallback` | Global fallback behavior | Eager fallback. |
| `sentence_over_completion` | Global ordering behavior | Eager fallback. |
| `prediction_never_first` / prediction limit | Global first-candidate behavior | Eager fallback. |
| `assign_ordered_candidate_qualities` | Construction/order assignment | Retained; storage swap must preserve this. |
| Filter/ranker integration | Full-list caller | Bounded engine refresh allowed only when rankers are absent and filters are bounded-safe. |

## Engine/API readers

| Reader | Classification | M34 treatment |
| --- | --- | --- |
| `Translator::translate` | Eager compatibility API | Retained as default wrapper. |
| `Translator::translate_with_context` | Eager context API | Retained as fallback. |
| `Engine::refresh_candidates` | Full-list sort/filter/store owner | Added bounded request path with lazy expansion to full list on out-of-window access. |
| `CandidateFilter::apply_with_context` | Full-list mutator by default | Bounded path only with `charset_filter`; other filters fallback eager. |
| `CandidateRanker::try_rerank` | Full-list ranker | Any ranker disables bounded refresh. |
| `RimeGetContext` | Paged ABI reader with full snapshot clone | Uses bounded window and `candidate_list_complete` for honest `is_last_page`. |
| Candidate-list iterator APIs | Full-list ABI reader | Force complete candidate list before iteration. |
| TypeDuck inspector/source attachment | Debug/inspector reader | Uses current candidate snapshot; TypeDuck is not on bounded path. |

## Result

No retained table state was deleted. `entries_by_code` remains the canonical
storage for eager, correction, sentence, TypeDuck, and full-list fallback paths.
M34 only inserted an internal abstraction and a narrow bounded first-page path.
