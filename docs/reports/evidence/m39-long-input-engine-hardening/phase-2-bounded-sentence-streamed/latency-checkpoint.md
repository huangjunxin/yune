# M39 Task 2 Bounded Sentence Checkpoint

Command:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\benchmark-native-rime-inprocess.ps1 -OutputRoot docs\reports\evidence\m39-long-input-engine-hardening\phase-2-bounded-sentence-streamed -Iterations 3 -SessionIterations 10 -KeyIterations 5 -TrackAInputs "ni,hao,zhongguo,ceshiyixiachangjushuruxingnengzenyang,zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong" -TrackBInputs "hai,ngohaig,jigaajiusihaa,loengjathau,caksijathaacoenggeoizisyujapsinhojijung,neigojangingkeisatjinggoiziwunciucoenggeoizisyujapsinhojijung" -DeployProductBeforeBenchmark
```

This checkpoint supersedes the earlier `phase-2-bounded-sentence/` run for
Task 2 memory reporting because it adds the streamed upstream sentence-model
builder. The final closeout benchmark must still use the exact Task 4 input
set from the active plan.

## Track A

| Row | Yune median | librime median | Ratio | Full-list fallback | Bounded calls | Upstream model us/op |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `ceshiyixiachangjushuruxingnengzenyang` | `506.227 us` | `295.170 us` | `1.715x` | `0` | `37/op` | `437.814 us` |
| `zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong` | `916.183 us` | `689.490 us` | `1.329x` | `0` | `59/op` | `823.147 us` |

The Track A severe owner moved from full upstream sentence-model scans per
suffix to bounded indexed word-graph construction. The native rows are inside
the `5x` Task 2 gate in this checkpoint.

## Track B

| Row | Phase 0 median | Phase 0 p95 | Checkpoint median | Checkpoint p95 | Owner |
| --- | ---: | ---: | ---: | ---: | --- |
| `neigojangingkeisatjinggoiziwunciucoenggeoizisyujapsinhojijung` | `189.207 us` | `202.084 us` | `202.693 us` | `204.539 us` | no-marisa exact/prefix plus full-list fallback, no upstream model |

The required Cantonese row remains a protected no-regression/profile row, not
the Track A owner. This low-sample checkpoint is inside median +10% and p95
+10% versus Phase 0.

## Storage And Memory

Track A preserved `selected_storage=rsmarisa_byte_backed`, table/prism `mmap`,
`rsmarisa_status=ok`, positive `rsmarisa` exact/prefix counters, and zero
selected table/prism heap mirror bytes. Track B preserved product
`byte_backed` compact storage with mmap-backed table/prism bytes and zero
selected table/prism heap mirror bytes.

Track A max peak fell from `163,598,336` bytes in Phase 0 to `123,891,712`
bytes after streaming sentence-model construction. Track B max peak fell from
`504,557,568` bytes to `504,057,856` bytes.
