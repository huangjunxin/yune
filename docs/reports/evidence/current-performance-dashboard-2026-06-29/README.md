# Current Performance Dashboard Evidence - 2026-06-29

This folder contains normalized CSVs and SVGs used by the current performance
and root-cause reports. Native rows were refreshed from the M50 final native
benchmark; browser rows are carried from the 2026-06-28 Playwright dashboard.

Fresh native source:
`../m50-track-a-launch-readiness/final-native-benchmark/`.

## Files

- `current-native-track-a.csv` - refreshed `luna_pinyin` peer rows. Track A is
  still measured partial: `n` passes, while `ni`, the 37-character row, and
  peak memory remain blockers.
- `current-native-track-b.csv` - refreshed product-path guard rows for
  `jyut6ping3_mobile`; not a fair librime peer comparison.
- `current-root-cause-gaps.csv` - ranked remaining gaps after M50 final.
- `current-browser-peer-comparator.csv` - carried forward from 2026-06-28.
- `current-yune-browser-input-latency.csv` - carried forward from 2026-06-28.
- `visuals/current-native-latency-ratios.svg` - regenerated from M50 final.
- `visuals/current-memory-peaks.svg` - regenerated from M50 final native memory
  plus carried browser memory rows.
- `visuals/current-root-cause-gaps.svg` - regenerated from M50 final gap ranks.
- `visuals/current-browser-peer-latency.svg`,
  `visuals/current-browser-memory-payload.svg`, and
  `visuals/current-yune-browser-input-latency.svg` - carried forward from
  2026-06-28.

## Scope Notes

Native Track A values are for upstream `luna_pinyin` against same-run upstream
librime `1.17.0`. They are not browser, product package, deployment, TypeDuck
keyboard-profile memory, or iOS-device claims.
