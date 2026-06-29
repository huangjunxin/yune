# Current Performance Dashboard Evidence

Date: 2026-06-29

This bundle normalizes the current benchmark rows used by the dashboard-style
reports. The native lane was refreshed on 2026-06-29 by the M49 Track A
follow-up. The browser lane is carried forward from the 2026-06-28 Playwright run
because it requires the WASM/Playwright pipeline against a live My RIME and was
not re-run in this pass.

## Source Inputs

- Native Track A + Track B rows: refreshed by
  `../m49-track-a-short-key-latency/final-native-benchmark/summary.csv`,
  captured with `-DeployProductBeforeBenchmark` after the Track A follow-up.
  Track A is `luna_pinyin` Yune versus same-run upstream librime `1.17.0`;
  Track B is the `jyut6ping3_mobile` product path (no fair librime peer; My
  RIME's Jyutping uses a different dictionary).
- Browser peer rows (carried forward): the 2026-06-28
  `apps/yune-web/e2e/results/yune-web-vs-my-rime-baseline/current-dashboard/`
  Playwright capture.
- Browser input-latency suite (carried forward): the 2026-06-28 rebuilt
  public-demo WEB-03 latency bundle.

`native-current-benchmark/` remains the previous dashboard run. The current
native source of truth is the M49 bundle above; its combined `summary.csv`,
`m37_metrics.csv`, `memory-owner-profile.csv`, and per-track subdirectories are
kept intact because this pass ended as a measured partial, not a clean success.

## Normalized Tables

- `current-native-track-a.csv` - refreshed `luna_pinyin` peer rows. Track A is
  improved but still partial: `n` is `3.074x`, `ni` is `3.269x`, and the
  37-character pinyin row is `3.094x`.
- `current-native-track-b.csv` - `jyut6ping3_mobile` product path, 2026-06-28 vs
  2026-06-29, showing the M47 owned-heap reduction (private bytes
  `420.0 MB -> 179.0 MB`).
- `current-root-cause-gaps.csv` - ranked remaining gaps.
- `current-browser-peer-comparator.csv` - carried forward (2026-06-28).
- `current-yune-browser-input-latency.csv` - carried forward (2026-06-28).

## Visuals

- `visuals/current-native-latency-ratios.svg` - re-generated 2026-06-29 after the
  M49 partial Track A run.
- `visuals/current-memory-peaks.svg` - re-generated 2026-06-29 after the M49
  partial Track A run (native rows fresh; browser rows carried).
- `visuals/current-root-cause-gaps.svg` - re-generated 2026-06-29 after the M49
  partial Track A run.
- `visuals/current-browser-peer-latency.svg` - carried forward (2026-06-28).
- `visuals/current-browser-memory-payload.svg` - carried forward (2026-06-28).
- `visuals/current-yune-browser-input-latency.svg` - carried forward
  (2026-06-28).

## Measurement Hygiene Notes

- Native MB values are decimal megabytes (bytes / 1e6), matching the prior
  dashboard convention. The native Track A peak (`188.3 MB`) is the process
  high-water working set across the whole benchmark, not a steady resident row;
  the average private-bytes proxy across Track A rows is `193.1 MB` and the
  session-create working-set row is `175.8 MB`. This is the post-M48 full
  `luna_pinyin` preset-vocabulary shape, not the M47 iOS-keyboard profile.
- The Track B product figures here come from the heavy in-process benchmark
  harness (deploy + compile of both `jyut6ping3` and `jyut6ping3_scolar`
  dictionaries, then 60 session and 80x8 key operations). They are not the lean
  iOS-proxy `native_memory_probe` numbers (one schema, prebuilt assets, measured
  right after `create_session`): the iOS-dirty private-bytes proxy for the
  comments-intact keyboard profile is `~22 MB`, reported separately in
  `docs/reports/ios-memory-budget.md`. Every figure names its harness.
- `luna_pinyin` is the fair cross-engine lane. Browser Jyutping rows are guard
  evidence only because My RIME uses a Cantonese-only Jyutping dictionary while
  Yune ships TypeDuck's larger multilingual `jyut6ping3` profile.
