# M52 Track A Guardrails And Disposition Evidence

Date: 2026-06-30

Scope: native Track A `luna_pinyin` only, same-run Yune versus upstream
librime 1.17.0. Track B was intentionally skipped for these M52 runs. No web,
frontend, WASM, public-demo, package, deployment, iOS-device, or ABI claim is
made from this evidence.

## Commands

Baseline:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\benchmark-native-rime-inprocess.ps1 -OutputRoot docs\reports\evidence\m52-track-a-guardrails-and-disposition\phase-0-baseline -Iterations 9 -SessionIterations 60 -KeyIterations 80 -TrackAInputs n,ni,hao,ceshiyixiachangjushuruxingnengzenyang,zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong -SkipTrackB
```

Final plus regression gate:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\benchmark-native-rime-inprocess.ps1 -OutputRoot docs\reports\evidence\m52-track-a-guardrails-and-disposition\final-native-benchmark -Iterations 9 -SessionIterations 60 -KeyIterations 80 -TrackAInputs n,ni,hao,ceshiyixiachangjushuruxingnengzenyang,zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong -SkipTrackB -TrackAThresholds docs\reports\evidence\m52-track-a-guardrails-and-disposition\track-a-thresholds.csv -FailOnRegression
```

## Key Artifacts

- [`track-a-thresholds.csv`](./track-a-thresholds.csv) - committed Track A
  latency and peak-memory ceilings.
- [`phase-0-baseline/summary-comparison.csv`](./phase-0-baseline/summary-comparison.csv)
  - fresh M52 starting same-run comparison.
- [`final-native-benchmark/summary-comparison.csv`](./final-native-benchmark/summary-comparison.csv)
  - final same-run comparison.
- [`final-native-benchmark/threshold-check.csv`](./final-native-benchmark/threshold-check.csv)
  - final regression-gate result.
- [`final-native-benchmark/raw_lookup_microbench.csv`](./final-native-benchmark/raw_lookup_microbench.csv)
  - final raw lookup and translator owner diagnostics.
- [`final-native-benchmark/memory-owner-profile.csv`](./final-native-benchmark/memory-owner-profile.csv)
  - final memory owner attribution.

## Final Native Rows

| Row | Yune median | librime median | Ratio | Abs gap | M52 read |
| --- | ---: | ---: | ---: | ---: | --- |
| startup | `24,139.200 us` | `21,686.900 us` | `1.113x` | `2,452.300 us` | near parity |
| session | `23,404.000 us` | `23,390.700 us` | `1.001x` | `13.300 us` | parity |
| `n` | `60.300 us` | `21.400 us` | `2.818x` | `38.900 us` | strict ratio pass |
| `ni` | `44.950 us` | `14.300 us` | `3.143x` | `30.650 us` | bounded-microsecond ceiling |
| `hao` | `24.967 us` | `11.633 us` | `2.146x` | `13.334 us` | strict ratio pass |
| 37-char pinyin | `895.178 us` | `293.211 us` | `3.053x` | `601.967 us` | bounded-microsecond ceiling |
| 59-char pinyin | `1,545.754 us` | `687.795 us` | `2.247x` | `857.959 us` | strict ratio pass |

## Final Guardrail Result

All final threshold rows pass:

| Guard | Observed | Ceiling | Status |
| --- | ---: | ---: | --- |
| `n` latency ratio | `2.818x` | `3.050x` | pass |
| `ni` latency ratio | `3.143x` | `3.223x` | pass |
| `hao` latency ratio | `2.146x` | `2.287x` | pass |
| 37-char latency ratio | `3.053x` | `3.267x` | pass |
| 59-char latency ratio | `2.247x` | `2.447x` | pass |
| Track A peak working set | `188,383,232 B` | `198,000,000 B` | pass |

## Memory Disposition

Final Track A peak working set is `188,383,232 B`; the largest same-run librime
peer peak is `17,276,928 B`. The final maximum private-bytes proxy is
`194,252,800 B`.

Named Yune owners:

| Owner | Bytes | Read |
| --- | ---: | --- |
| `poet.vocabulary` | `53,644,752 B` | full upstream Luna preset vocabulary retained on heap |
| `poet.entries_by_code` | `18,694,662 B` | upstream sentence-model entries retained on heap |
| `poet.lookup_index` | `2,660,848 B` | guarded M40 sentence lookup index |
| process unclassified lower bound | `105,600,911 B` | measured process proxy after named owner accounting |

Native Track A `luna_pinyin` is the upstream comparison lane. The current native
product target remains the TypeDuck/Jyutping profile; M47's `jyut6ping3_mobile`
keyboard-profile memory result is a separate lane and is not used as a Track A
claim here. M52 therefore closes full-Luna memory as a guardrailed
comparison-lane watch, not as a reduction.

## Verdict

M52 is complete if the final Rust gates recorded in `final-gates.md` pass.
Performance disposition is complete: the regression guardrail is committed and
passes, `ni` and the 37-character row are closed as bounded-microsecond ceilings,
and full-Luna memory is closed as a precise product-profile-relevance watch.
