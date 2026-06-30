# M52 Final Gates

Date: 2026-06-30

Scope: native Track A `luna_pinyin` guardrails and blocker disposition. No web,
frontend, WASM, package, deployment, iOS-device, or ABI claim is made.

## Native Benchmark And Regression Gate

Command:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\benchmark-native-rime-inprocess.ps1 -OutputRoot docs\reports\evidence\m52-track-a-guardrails-and-disposition\final-native-benchmark -Iterations 9 -SessionIterations 60 -KeyIterations 80 -TrackAInputs n,ni,hao,ceshiyixiachangjushuruxingnengzenyang,zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong -SkipTrackB -TrackAThresholds docs\reports\evidence\m52-track-a-guardrails-and-disposition\track-a-thresholds.csv -FailOnRegression
```

Result: pass. The generated
[`final-native-benchmark/threshold-check.csv`](./final-native-benchmark/threshold-check.csv)
records all rows as `pass`.

| Guard | Observed | Ceiling | Status |
| --- | ---: | ---: | --- |
| `n` latency ratio | `2.818x` | `3.050x` | pass |
| `ni` latency ratio | `3.143x` | `3.223x` | pass |
| `hao` latency ratio | `2.146x` | `2.287x` | pass |
| 37-char latency ratio | `3.053x` | `3.267x` | pass |
| 59-char latency ratio | `2.247x` | `2.447x` | pass |
| Track A peak working set | `188,383,232 B` | `198,000,000 B` | pass |

## Rust Gates

| Command | Result |
| --- | --- |
| `cargo fmt --check` | pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass |
| `cargo test -p yune-core short_key` | pass: 2 passed, 0 failed |
| `cargo test -p yune-core poet` | pass: 14 passed, 0 failed |
| `cargo test -p yune-core --test upstream_luna_pinyin_parity` | pass: 12 passed, 0 failed |
| `cargo test -p yune-core --test cantonese_parity` | pass: 37 passed, 0 failed |

Broader `cargo test --workspace` was not run because M52 changed the benchmark
script and docs/evidence only; shared ABI/runtime code was not changed.
