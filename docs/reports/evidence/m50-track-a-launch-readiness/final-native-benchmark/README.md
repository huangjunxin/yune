# M50 Final Native Benchmark

Scope: native Track A `luna_pinyin` launch-readiness closeout only. The Track B
product-path rows are emitted by the shared benchmark harness but are not used
for M50 product, browser, frontend, package, deployment, or iOS-device claims.

Command:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\benchmark-native-rime-inprocess.ps1 -OutputRoot docs\reports\evidence\m50-track-a-launch-readiness\final-native-benchmark -Iterations 9 -SessionIterations 60 -KeyIterations 80 -TrackAInputs n,ni,hao,zhongguo,ceshiyixiachangjushuruxingnengzenyang,zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong,cszysmsrsd,zybfshmsru -TrackBInputs neigojangingkeisatjinggoiziwunciucoenggeoizisyujapsinhojijung -DeployProductBeforeBenchmark
```

## Verdict

M50 closes as measured partial:

- `n`: `61.000 us` vs librime `21.200 us`, `2.877x`, pass.
- `ni`: `45.450 us` vs librime `14.400 us`, `3.156x`, measured blocker.
- 37-character row: `890.689 us` vs librime `289.773 us`, `3.074x`, measured
  blocker.
- Track A peak working set: Yune `188,432,384 B` versus librime `17,137,664 B`,
  measured blocker.
- Track A max summary median private: Yune `197,189,632 B`, measured blocker.

Named memory blockers:

- `poet.vocabulary`: `53,644,752 B`.
- `poet.entries_by_code`: `18,694,662 B`.
- `process.after_ready_working_set_unclassified_lower_bound`: `106,190,735 B`.

No retained heap prefix index was added.

## Gate Evidence

Passed before this final benchmark:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p yune-core short_key`
- `cargo test -p yune-core poet`
- `cargo test -p yune-core --test upstream_luna_pinyin_parity`
- `cargo test -p yune-core --test cantonese_parity`
