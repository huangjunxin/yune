# Current Performance Dashboard Evidence

Date: 2026-06-29

This bundle normalizes the current benchmark rows used by the dashboard-style
reports. The **native lane was freshly re-measured on 2026-06-29** after the
completed M47 byte-backing work. The **browser lane is carried forward from the
2026-06-28 Playwright run** (it requires the WASM/Playwright pipeline against a
live My RIME and was not re-run this pass).

## Source Inputs

- Native Track A + Track B rows: `native-current-benchmark/summary.csv`, freshly
  captured for this dashboard pass with `-DeployProductBeforeBenchmark`
  (`environment.txt` records `deploy_product_before_benchmark=True`,
  `yune_git_head=3625e7bb`). Track A is `luna_pinyin` Yune versus same-run
  upstream librime `1.17.0`; Track B is the `jyut6ping3_mobile` product path
  (no fair librime peer — My RIME's Jyutping uses a different dictionary).
- Browser peer rows (carried forward): the 2026-06-28
  `apps/yune-web/e2e/results/yune-web-vs-my-rime-baseline/current-dashboard/`
  Playwright capture.
- Browser input-latency suite (carried forward): the 2026-06-28 rebuilt
  public-demo WEB-03 latency bundle.

`native-current-benchmark/` keeps the combined top-level run artifacts
(`summary.csv` and the other combined CSVs, `environment.txt`, `commands.txt`,
`README.md`). The harness also emits per-track working subdirectories
(`track-a-yune/`, `track-a-librime-1.17.0/`, `track-b-yune-product/`) plus a
`.marisa` probe binary and cargo logs; those are regenerable intermediates that
no dashboard row references, so they are omitted from this bundle. The combined
`summary.csv` already contains every track row.

## Normalized Tables

- `current-native-track-a.csv` — fresh `luna_pinyin` peer rows.
- `current-native-track-b.csv` — `jyut6ping3_mobile` product path, 2026-06-28
  vs 2026-06-29, showing the M47 owned-heap reduction (private bytes
  `420.0 MB → 181.4 MB`).
- `current-root-cause-gaps.csv` — ranked remaining gaps.
- `current-browser-peer-comparator.csv` — carried forward (2026-06-28).
- `current-yune-browser-input-latency.csv` — carried forward (2026-06-28).

## Visuals

- `visuals/current-native-latency-ratios.svg` — re-generated 2026-06-29.
- `visuals/current-memory-peaks.svg` — re-generated 2026-06-29 (native rows
  fresh; browser rows carried).
- `visuals/current-root-cause-gaps.svg` — re-generated 2026-06-29.
- `visuals/current-browser-peer-latency.svg` — carried forward (2026-06-28).
- `visuals/current-browser-memory-payload.svg` — carried forward (2026-06-28).
- `visuals/current-yune-browser-input-latency.svg` — carried forward (2026-06-28).

## Measurement Hygiene Notes

- Native MB values are decimal megabytes (bytes / 1e6), matching the prior
  dashboard convention. The native Track A peak (`105.9 MB`) is the process
  high-water working set across the whole benchmark, not a steady resident row;
  the median private-bytes proxy is `54.8 MB` and the session-create row is
  `48.4 MB`.
- The Track B product figures here come from the **heavy in-process benchmark
  harness** (deploy + compile of both `jyut6ping3` and `jyut6ping3_scolar`
  dictionaries, then 60 session and 80×8 key operations). They are **not** the
  lean iOS-proxy `native_memory_probe` numbers (one schema, prebuilt assets,
  measured right after `create_session`): the iOS-dirty private-bytes proxy for
  the comments-intact keyboard profile is `~22 MB`, reported separately in
  `docs/reports/ios-memory-budget.md`. Every figure names its harness.
- `luna_pinyin` is the fair cross-engine lane. Browser Jyutping rows are guard
  evidence only because My RIME uses a Cantonese-only Jyutping dictionary while
  Yune ships TypeDuck's larger multilingual `jyut6ping3` profile.
