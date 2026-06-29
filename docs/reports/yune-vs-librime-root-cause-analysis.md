# Current Yune Root-Cause Dashboard

Date: 2026-06-29

This report keeps only the current root-cause read. Older milestone narratives,
WEB-01/WEB-02/WEB-03 closeout detail, and superseded measurements have been
archived at
[`history/2026-06-28-yune-vs-librime-root-cause-analysis-pre-current-dashboard.md`](./history/2026-06-28-yune-vs-librime-root-cause-analysis-pre-current-dashboard.md).

The native lane was re-measured 2026-06-29; the browser lane is carried forward
from the 2026-06-28 Playwright run.

## Technical Summary

- **Current native latency owner**: `ni` (`3.514x`) and `hao` (`2.134x`) are the
  only remaining Track A latency misses, and both are microsecond-scale
  short-prefix translator/prefix lookup constant-factor problems — not
  sentence-model or abbreviation-path regressions. The 37-character pinyin row is
  no longer a watch row; it is now faster than librime at `0.940x`. This run did
  not measure the single-character `n` row.
- **Current native memory owner**: Track A peak working set narrowed to
  `105.9 MB` (from `128.5 MB`), but it remains well above librime's `18.8 MB` max
  peer peak. Existing retained-owner rows do not explain a safe further large
  reduction; the next owner is allocator/transient/private high-water
  attribution. The `jyut6ping3_mobile` product path saw the larger win: owned
  heap (private bytes) fell from `420.0 MB` to `181.4 MB` after the M47
  lookup-record/comment byte-backing.
- **Current browser fair memory owner**: the fair `luna_pinyin` browser gap is
  `64.0 MiB` Yune public demo versus `16.0 MiB` My RIME (carried 2026-06-28).
  This is the clean browser memory target because it uses the same schema and
  avoids Jyutping dictionary confounds.
- **Current Jyutping launch state**: the shipping public-demo Jyutping path is
  byte-backed at `160.0 MiB`, not the old `893.1 MiB` source-fallback shape.
  Long-input latency and phrase composition are guarded by WEB-03 follow-up
  tests.

## Current Gap Map

![Current remaining performance gaps](./evidence/current-performance-dashboard-2026-06-29/visuals/current-root-cause-gaps.svg)

| Area | Current root cause | Evidence | Current status |
| --- | --- | --- | --- |
| Native `ni` | Short-prefix translator/prefix lookup constant factor | `50.250 us` vs librime `14.300 us`; `3.514x` | blocker |
| Native `hao` | Same short-prefix path | `25.033 us` vs librime `11.733 us`; `2.134x` | watch |
| Native Track A peak memory | High-water peak not explained by easy retained owners | Yune peak `105.9 MB`; librime max peer peak `18.8 MB` | blocker |
| Browser `luna_pinyin` memory | Yune WASM/runtime floor still larger than My RIME | `64.0 MiB` vs `16.0 MiB`; same schema (carried) | blocker |
| Browser `luna_pinyin` startup | Yune public-demo startup still slower | `1000 ms` vs My RIME `634 ms` (carried) | watch |
| Browser Jyutping | Larger TypeDuck profile; not a peer-comparable lane | Yune `160.0 MiB`, My RIME Jyutping `68.0 MiB` on different dictionary (carried) | guard only |

## Native Track A Cause

![Current native Track A latency ratios](./evidence/current-performance-dashboard-2026-06-29/visuals/current-native-latency-ratios.svg)

The current native latency problem is narrow. Startup, session create/select,
`zhongguo`, the 37-character row, the 59-character row, and both abbreviation
rows are at or faster than same-run upstream librime. The only misses are the two
short-prefix rows, and both are microsecond-scale:

| Row | Yune median | librime median | Ratio | Current cause |
| --- | ---: | ---: | ---: | --- |
| `ni` | `50.250 us` | `14.300 us` | `3.514x` | short-prefix translator/prefix constant factor |
| `hao` | `25.033 us` | `11.733 us` | `2.134x` | same owner |
| 37-char pinyin | `285.103 us` | `303.241 us` | `0.940x` | now faster than librime; no longer a watch row |

The current evidence keeps the sentence paths out of this diagnosis. The
short-key owner counters show no upstream sentence model calls for the short
rows, and the long-input and abbreviation rows remain green. The next native
latency diagnostic should isolate prefix lookup and translator dispatch cost
without widening the full-sentence or TypeDuck-profile behavior surfaces.

## Native Memory Cause

![Current memory high-water by lane](./evidence/current-performance-dashboard-2026-06-29/visuals/current-memory-peaks.svg)

Native Track A memory is not solved because the peak remains high, even though it
narrowed and the steady rows are lower:

| Measurement | Current value | Read |
| --- | ---: | --- |
| Yune Track A max peak working set | `105.9 MB` | standing high-water blocker (was `128.5 MB`) |
| librime Track A max peer peak | `18.8 MB` | same-run peer scale |
| Yune Track A median private-bytes proxy | `54.8 MB` | lower than the peak |
| Yune Track A session-create row | `48.4 MB` | lowest measured row |
| Largest retained reducible row | `poet.entries_by_code`, `18.7 MB` | not enough to explain the process peak |

The current root cause is therefore not another broad structural-owner rewrite of
Track A. The next useful step is peak attribution: allocator behavior, transient
buffers, private bytes, mapped residency, and startup high-water timing.

The `jyut6ping3_mobile` product path is the place the M47 byte-backing landed:
owned heap (private bytes) fell from `420.0 MB` (2026-06-28) to `181.4 MB`
(2026-06-29) in the heavy benchmark harness, with the peak working set flat at
`~505 MB` because the in-process deploy/compile transient dominates the
high-water. The lean iOS-proxy probe (separate harness) reports `~22 MB` private
for the comments-intact keyboard profile; see
[`ios-memory-budget.md`](./ios-memory-budget.md).

## Browser Root Cause

Carried forward from the 2026-06-28 Playwright run (not re-measured this pass).

![Current browser memory and payload](./evidence/current-performance-dashboard-2026-06-29/visuals/current-browser-memory-payload.svg)

The fair browser target is `luna_pinyin`, not Jyutping. Current browser peer
evidence shows:

| Scenario | Ready | Input -> candidate | Commit | WASM peak | Resource payload | Read |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| Yune public demo `luna_pinyin` | `1000 ms` | `74 ms` | `107 ms` | `64.0 MiB` | `29.5 MiB` | fair Yune row |
| My RIME live `luna_pinyin` | `634 ms` | `95 ms` | `119 ms` | `16.0 MiB` | `8.5 MiB` | fair peer row |

The current fair gap is `4.0x`, not the earlier `10x` `160 MiB` Luna row. Yune's
first-input and commit latencies are competitive in this comparator, while
startup and WASM memory remain the browser-side blockers.

## Jyutping Guard State

Jyutping is currently a guard lane, not a peer lane. The Yune public demo ships
TypeDuck's larger multilingual `jyut6ping3` profile; My RIME's Jyutping row uses
a Cantonese-only dictionary. The current Yune state is still important because it
is the launch path (browser figures carried from 2026-06-28):

| Guard | Current value | Read |
| --- | ---: | --- |
| Public-demo Jyutping WASM peak | `160.0 MiB` | byte-backed launch path; old `893.1 MiB` source-fallback row is historical |
| Ready-to-input | `1347 ms` | current public-demo comparator |
| First input -> candidate | `103 ms` | current public-demo comparator |
| Long row `sihaacoenggeoisyujapgecukdou` | `130 ms` exact keydown-to-paint | WEB-03 latency guard |
| Long row `taihaajyugwodaahoucoenggegeoizigosingnangwuidimjoeng` | `74 ms` exact keydown-to-paint | WEB-03 latency guard |

The root cause of the old `893.1 MiB` path was stale public-demo compiled assets
causing source fallback. That root cause is fixed on the shipping path. It should
stay in history, not in the current blocker list.

## Current Evidence

The dashboard evidence bundle is
[`evidence/current-performance-dashboard-2026-06-29/`](./evidence/current-performance-dashboard-2026-06-29/).

Key normalized tables:

- [`current-native-track-a.csv`](./evidence/current-performance-dashboard-2026-06-29/current-native-track-a.csv)
- [`current-native-track-b.csv`](./evidence/current-performance-dashboard-2026-06-29/current-native-track-b.csv)
- [`current-root-cause-gaps.csv`](./evidence/current-performance-dashboard-2026-06-29/current-root-cause-gaps.csv)
- [`current-browser-peer-comparator.csv`](./evidence/current-performance-dashboard-2026-06-29/current-browser-peer-comparator.csv) (carried 2026-06-28)
- [`current-yune-browser-input-latency.csv`](./evidence/current-performance-dashboard-2026-06-29/current-yune-browser-input-latency.csv) (carried 2026-06-28)

## Next Diagnostic Order

| Rank | Work | Why this is next |
| ---: | --- | --- |
| 1 | Browser fair-lane memory floor on `luna_pinyin` | Same-schema browser gap is the cleanest current memory target: `64.0 MiB` vs `16.0 MiB`. |
| 2 | Native Track A peak attribution | Track A peak is still `105.9 MB`; existing retained-owner rows do not explain it. |
| 3 | Native short-prefix constant factor | `ni` and `hao` are the only current native latency misses. |
| 4 | Browser startup phases | Yune public-demo `luna_pinyin` ready-to-input is `1000 ms` vs My RIME `634 ms`. |

## History

Archived milestone-style report:
[`history/2026-06-28-yune-vs-librime-root-cause-analysis-pre-current-dashboard.md`](./history/2026-06-28-yune-vs-librime-root-cause-analysis-pre-current-dashboard.md).
