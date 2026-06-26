# M39 Phase 0 Cantonese Profile Gate

Date: 2026-06-25

Evidence root: `docs/reports/evidence/m39-long-input-engine-hardening/phase-0-baseline/`

Command:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\benchmark-native-rime-inprocess.ps1 -OutputRoot docs\reports\evidence\m39-long-input-engine-hardening\phase-0-baseline -Iterations 5 -SessionIterations 20 -KeyIterations 20 -TrackAInputs "ni,hao,zhongguo,ceshiyixiachangjushuruxingnengzenyang,zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong" -TrackBInputs "hai,ngohaig,jigaajiusihaa,loengjathau,caksijathaacoenggeoizi,neigojangingkeisatjinggoiziwunciucoenggeoizisyujapsinhojijung" -DeployProductBeforeBenchmark
```

## Required Row

| Track | Schema | Input | Median | p95 | Full-input median sample | Median working set | Max peak working set |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| Track B product | `jyut6ping3_mobile` | `neigojangingkeisatjinggoiziwunciucoenggeoizisyujapsinhojijung` | `189.207 us/op` | `202.084 us/op` | `230.832 ms` | `441,483,264 B` | `504,557,568 B` |

This row is present in `summary.csv`, `samples.csv`, and `m37_metrics.csv`.

## Phase 0 Owner Read

The Track B Cantonese long row does not share the same severity or storage shape as the Track A `luna_pinyin` long rows in this baseline.

| Signal | Track A 37-char row | Track A 59-char row | Track B 61-char row |
| --- | ---: | ---: | ---: |
| Median per-op latency | `452,200.116 us` | `1,240,080.937 us` | `189.207 us` |
| Median translator time | `452,168.792 us/op` | `1,240,040.378 us/op` | `185.979 us/op` |
| Full-list fallback | `0.730/op` | `0.881/op` | `0.934/op` |
| Exact lookup calls | `1.730/op` | `1.881/op` | `79.246/op` |
| Prefix lookup calls | `1.703/op` | `1.864/op` | `1.934/op` |
| `rsmarisa` exact/prefix calls | `1.730 / 1.703 op` | `1.881 / 1.864 op` | `0 / 0 op` |
| no-marisa compact exact/prefix calls | `0 / 0 op` | `0 / 0 op` | `79.246 / 1.934 op` |
| Bounded candidate requests | `1.000/op` | `1.000/op` | `1.000/op` |
| Unbounded candidate requests | `0.000/op` | `0.000/op` | `0.000/op` |

Track A remains a severe unsplit translator/long-composition stall with active `rsmarisa` and mmap-backed table/prism bytes. Track B is much faster at baseline and uses the product compiled `byte_backed` no-marisa path with mmap-backed table/prism bytes, zero selected table/prism heap mirror bytes, and a small per-op translator bucket that includes product exact lookup, prefix fallback, and full-list fallback work.

Task 1 must still add inner counters before any optimization, because the current M37 counters cannot split sentence composition, upstream sentence model, prefix fallback, and dynamic correction. However, Task 0 does not support treating the Cantonese row as the same latency owner as Track A.

## Native Profile Target

M39 must not claim a Track B TypeDuck-HK/librime ratio from this run, because the current harness measures Track B only with Yune product assets and does not include a same-run TypeDuck-HK/librime `v1.1.2` row for the 61-character input.

Until such an oracle row is added, the Track B native closeout target is a no-regression gate from this baseline:

- final Track B long-row median must not regress by more than `10%` from `189.207 us/op` unless a measured owner/no-go is recorded;
- final Track B long-row p95 must not regress by more than `10%` from `202.084 us/op` unless a measured owner/no-go is recorded;
- final Track B evidence must keep the row present in `summary.csv`, `samples.csv`, and `m37_metrics.csv`;
- final Track B evidence must report the inner owner counters added in Task 1, plus product storage status and memory rows.
