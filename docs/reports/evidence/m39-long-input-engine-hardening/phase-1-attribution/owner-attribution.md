# M39 Phase 1 Owner Attribution

Command:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\benchmark-native-rime-inprocess.ps1 -OutputRoot docs\reports\evidence\m39-long-input-engine-hardening\phase-1-attribution -Iterations 1 -SessionIterations 5 -KeyIterations 1 -TrackAInputs "ni,hao,zhongguo,ceshiyixiachangjushuruxingnengzenyang,zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong" -TrackBInputs "hai,ngohaig,jigaajiusihaa,loengjathau,caksijathaacoenggeoizi,neigojangingkeisatjinggoiziwunciucoenggeoizisyujapsinhojijung" -DeployProductBeforeBenchmark
```

## Result

Track A `luna_pinyin` long rows are dominated by the upstream sentence model
counter, not by `StaticTableTranslator::sentence_candidate`.

| Track | Input | Median us/op | Dominant owner |
| --- | --- | ---: | --- |
| A | `ceshiyixiachangjushuruxingnengzenyang` | 437,076.884 | `upstream_sentence_model_ns` at 436,917.530 us/op |
| A | `zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong` | 1,228,783.612 | `upstream_sentence_model_ns` at 1,228,565.656 us/op |
| B | `neigojangingkeisatjinggoiziwunciucoenggeoizisyujapsinhojijung` | 225.259 | no shared Track A owner; small `sentence_candidate` and no-marisa lookup costs |

Track B does not share the Track A owner:

- `upstream_sentence_model_calls_per_op=0`.
- `sentence_candidate_ns` averages 7.414 us per call and 57 calls for 61 keys.
- `prefix_fallback_ns` is 3.841 us/op.
- no-marisa compact exact lookup remains the larger measured Track B bucket at
  79.246 calls/op and 34.243 us/op, but the total long row remains close to the
  Phase 0 no-regression target.

Storage gates stayed intact in this attribution run:

- Track A `luna_pinyin`: `selected_storage=rsmarisa_byte_backed`,
  `table_mapping_mode=mmap`, `prism_mapping_mode=mmap`,
  `rsmarisa_status=ok`, `rsmarisa_mapping_mode=mmap`,
  `table_heap_mirror_bytes=0`, `prism_heap_mirror_bytes=0`.
- Track B `jyut6ping3_mobile`: `selected_storage=byte_backed`,
  `table_mapping_mode=mmap`, `prism_mapping_mode=mmap`,
  `rsmarisa_status=missing_string_table`, `table_heap_mirror_bytes=0`,
  `prism_heap_mirror_bytes=0`.

## Coding Decision

Task 2 must optimize `UpstreamSentenceModel::candidates_for_input` and its word
graph construction for Track A. Track B should be treated as a protected
no-regression row unless later evidence shows the no-marisa compact lookup path
needs separate work.
