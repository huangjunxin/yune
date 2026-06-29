# M49 Track A Short-Key Latency Evidence

Date: 2026-06-29

Scope: native engine Track A only. No browser, frontend, packaging, deployment,
or broad product-speed claim is made from this bundle.

## Verdict

M49 is a measured partial.

- `n`: `71.300 us -> 62.400 us`; ratio `3.478x -> 3.074x`.
- `ni`: `51.000 us -> 46.250 us`; ratio `3.617x -> 3.269x`.
- `hao`: remains under gate at `2.248x`.
- 37-character pinyin: `2,789.897 us -> 894.400 us`; ratio `9.670x -> 3.094x`.
- 59-character pinyin: `5,064.307 us -> 1,543.742 us`; ratio `7.512x -> 2.280x`.

The strict `<=3.0x` launch-readiness gate is still missed by `n`, `ni`, and the
37-character row. The retained code is still useful because it reduces the
measured owners without adding retained heap.

## Code Changes Proven Here

- MARISA-backed compact-table prefix traversal now carries the current code on
  traversal frames and lazily yields node entry rows instead of materializing all
  rows for a matching node up front.
- The normal preset-vocabulary sentence path now performs a transient
  character-code prefilter before expensive phrase-code derivation. The rejected
  retained prefix-index attempt was not kept because it added about `35 MB` of
  retained heap.

## Evidence Folders

- `phase-0-baseline/` - baseline from `67bbd13f` before this Track A follow-up.
- `final-native-benchmark/` - final benchmark for the retained implementation.

Both benchmark runs used:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\benchmark-native-rime-inprocess.ps1 `
  -OutputRoot <evidence-folder> `
  -Iterations 9 -SessionIterations 60 -KeyIterations 80 `
  -TrackAInputs n,ni,hao,zhongguo,ceshiyixiachangjushuruxingnengzenyang,zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong,cszysmsrsd,zybfshmsru `
  -TrackBInputs neigojangingkeisatjinggoiziwunciucoenggeoizisyujapsinhojijung `
  -DeployProductBeforeBenchmark
```

## Remaining Measured Blockers

- `n` / `ni`: prefix traversal plus short-key translator filter/ranking remains
  just above the ratio gate.
- 37-character pinyin: preset-vocabulary sentence graph rebuild still considers
  `3,950` vocabulary entries on the median sample.
- Track A memory: current full `luna_pinyin` peer-comparison peak is `188.3 MB`
  versus librime `17.6 MB`. This is separate from the M47 TypeDuck keyboard
  profile and should not be conflated with the iOS-dirty proxy.
