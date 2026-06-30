# M50 Sentence Row Reduction

Scope: native Track A `luna_pinyin` only. This run is not evidence for browser,
frontend, product package, deployment, public demo, TypeDuck keyboard-profile
memory, or iOS-device claims.

## Command

`powershell -ExecutionPolicy Bypass -File scripts\benchmark-native-rime-inprocess.ps1 -OutputRoot docs\reports\evidence\m50-track-a-launch-readiness\sentence-row -Iterations 9 -SessionIterations 60 -KeyIterations 80 -TrackAInputs n,ni,hao,zhongguo,ceshiyixiachangjushuruxingnengzenyang,zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong,cszysmsrsd,zybfshmsru -TrackBInputs neigojangingkeisatjinggoiziwunciucoenggeoizisyujapsinhojijung -DeployProductBeforeBenchmark`

## Result

The 37-character Track A row moved under the M50 latency gate:

| Input | Yune median | librime median | Ratio |
| --- | ---: | ---: | ---: |
| `n` | `59.400 us` | `20.900 us` | `2.842x` |
| `ni` | `44.800 us` | `14.550 us` | `3.079x` |
| `ceshiyixiachangjushuruxingnengzenyang` | `860.011 us` | `294.551 us` | `2.920x` |
| `zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong` | `1630.029 us` | `664.893 us` | `2.452x` |

The remaining latency blocker after this slice is `ni` at `3.079x` in the same
run. `n` remains inside the gate.

## Memory Attribution

Track A `luna_pinyin` owner profile now reports normal sentence vocabulary:

| Owner | Class | Retained estimate | Items | Notes |
| --- | --- | ---: | ---: | --- |
| `poet.entries_by_code` | `heap_owned_reducible` | `18,694,662 B` | `513,353` | sentence model entries cloned from table rows |
| `poet.lookup_index` | `heap_owned_guarded` | `2,660,848 B` | `332,604` | sorted code-range index used by M40 sentence lookup |
| `poet.vocabulary` | `heap_owned_reducible` | `53,644,752 B` | `421,966` | normal preset vocabulary used by upstream sentence graph |
| `poet.abbreviation_vocabulary` | `heap_owned_reducible` | `1,433 B` | `11` | abbreviation-only vocabulary used by M42 guard rows |
| `process.after_ready_working_set_unclassified_lower_bound` | `unclassified` | `106,063,759 B` | `1` | process lower-bound proxy after non-overlapping reducible owners |

This slice does not add a retained vocabulary prefix index. Focused test
`upstream_sentence_model_memory_profile_accounts_normal_vocabulary_without_prefix_index`
guards that the owner report has no hidden `prefix_index` row.

## Focused Checks

- `cargo test -p yune-core upstream_sentence_model_memory_profile_accounts_normal_vocabulary_without_prefix_index`
- `cargo test -p yune-core upstream_sentence_model_prefilters_irrelevant_vocabulary_codes`
- `cargo test -p yune-core poet`
- `cargo test -p yune-core bounded_compact_translator_uses_prism_abbreviation_spans_for_sentence_model`
