# Current Yune Performance Dashboard

Date: 2026-06-29

This dashboard shows the current benchmark state only. Older milestone closeout
narrative and superseded benchmark rows remain in
[`history/2026-06-28-yune-vs-librime-performance-pre-current-dashboard.md`](./history/2026-06-28-yune-vs-librime-performance-pre-current-dashboard.md).

The native lane was refreshed by the M50 Track A final benchmark on
2026-06-29. The browser lane is carried forward from the 2026-06-28 Playwright
run and was not re-measured in this pass.

## Technical Summary

- **Native fair lane (`luna_pinyin`)**: Yune remains faster than same-run upstream
  librime 1.17.0 on startup (`0.799x`), session create/select (`0.934x`), `n`
  (`2.877x`), `zhongguo` (`0.277x`), `hao` (`2.161x`), the 59-character row
  (`2.277x`), and both abbreviation rows (`0.425x`, `0.637x`). The
  launch-readiness latency gate is still **partial** because `ni` is `3.156x`
  and the 37-character row is `3.074x`.
- **Native Track A memory**: current high-water is `188.4 MB` versus librime's
  `17.1 MB` max peer peak in the same run. This is the post-M48 full
  `luna_pinyin` preset-vocabulary shape and is separate from the M47
  `jyut6ping3_mobile` iOS-keyboard profile.
- **Native product path (`jyut6ping3_mobile`, non-peer)**: the heavy benchmark
  harness still shows the M47 owned-heap win. Private bytes on the key-load row
  are `183.9 MB` versus the old `420.0 MB`; peak working set is flat at
  `504.4 MB` because this harness includes deploy/compile transient work.
- **Browser fair lane (`luna_pinyin`, carried 2026-06-28)**: Yune public demo
  uses `64.0 MiB` WASM peak versus My RIME `16.0 MiB` (`4.0x`). Yune is slower
  to ready (`1000 ms` vs `634 ms`), but faster on first input (`74 ms` vs
  `95 ms`).
- **Browser Jyutping (carried 2026-06-28)**: Yune public demo is byte-backed at
  `160.0 MiB` WASM peak. This is a guard row, not a fair peer comparison,
  because My RIME's Jyutping uses a different Cantonese-only dictionary.

## Current Evidence Bundle

The normalized dashboard source is
[`evidence/current-performance-dashboard-2026-06-29/`](./evidence/current-performance-dashboard-2026-06-29/).

Fresh native source:
[`evidence/m50-track-a-launch-readiness/final-native-benchmark/summary.csv`](./evidence/m50-track-a-launch-readiness/final-native-benchmark/summary.csv).

## Native Track A

![Current native Track A latency ratios](./evidence/current-performance-dashboard-2026-06-29/visuals/current-native-latency-ratios.svg)

| Dimension | Yune median | librime median | Yune / librime | Current read |
| --- | ---: | ---: | ---: | --- |
| startup | `22,958.900 us` | `28,736.500 us` | `0.799x` | Yune faster |
| session | `23,208.800 us` | `24,859.500 us` | `0.934x` | Yune faster |
| `n` | `61.000 us` | `21.200 us` | `2.877x` | pass |
| `ni` | `45.450 us` | `14.400 us` | `3.156x` | blocker |
| `hao` | `25.067 us` | `11.600 us` | `2.161x` | pass |
| `zhongguo` | `46.150 us` | `166.600 us` | `0.277x` | Yune faster |
| 37-char pinyin | `890.689 us` | `289.773 us` | `3.074x` | blocker |
| 59-char pinyin | `1,543.071 us` | `677.731 us` | `2.277x` | pass |
| abbreviation 10-char | `517.720 us` | `1,218.320 us` | `0.425x` | Yune faster |
| abbreviation 8-char | `547.580 us` | `859.790 us` | `0.637x` | Yune faster |

## Native Track B (Product Path, Non-Peer)

The `jyut6ping3_mobile` product path has no fair librime peer in this dashboard.
It is tracked because it is the TypeDuck keyboard target. These figures come from
the heavy in-process benchmark harness, not the lean iOS-proxy
`native_memory_probe`:

| Measurement | 2026-06-28 | 2026-06-29 | Read |
| --- | ---: | ---: | --- |
| Private bytes (key load) | `420.0 MB` | `183.9 MB` | M47 byte-backing collapsed the owned heap |
| Private bytes (session) | `405.8 MB` | `178.8 MB` | steady private bytes down with byte-backed records |
| Median working set (key load) | `440.1 MB` | `255.0 MB` | resident working set down with the owned-heap cut |
| Peak working set high-water | `504.4 MB` | `504.4 MB` | flat; dominated by in-process deploy/compile transient |
| Session create latency | `141,590.4 us` | `92,093.1 us` | faster with byte-backed records |

The lean iOS-dirty proxy for the comments-intact keyboard profile remains in
[`ios-memory-budget.md`](./ios-memory-budget.md): `~22 MB` private. That is the
iOS-budget proxy; the `179.0 MB` row above is the heavy benchmark harness.

## Memory High-Water

![Current memory high-water by lane](./evidence/current-performance-dashboard-2026-06-29/visuals/current-memory-peaks.svg)

| Lane | Yune | Peer | Current read |
| --- | ---: | ---: | --- |
| Native Track A peak working set | `188.4 MB` | librime max peer peak `17.1 MB` | current blocker |
| Browser `luna_pinyin` WASM peak (carried) | `64.0 MiB` | My RIME `16.0 MiB` | fair browser gap is `4.0x` |
| Browser Jyutping WASM peak (carried) | `160.0 MiB` | My RIME `68.0 MiB` | guard only, dictionary-confounded |

## Browser Peer Dashboard

Carried forward from the 2026-06-28 Playwright run.

![Current browser peer latency](./evidence/current-performance-dashboard-2026-06-29/visuals/current-browser-peer-latency.svg)

![Current browser memory and payload](./evidence/current-performance-dashboard-2026-06-29/visuals/current-browser-memory-payload.svg)

| Scenario | Schema | Ready | Input -> candidate | Commit | WASM peak | Unique encoded resources | Validity |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| Yune public demo | `luna_pinyin` | `1000 ms` | `74 ms` | `107 ms` | `64.0 MiB` | `29.5 MiB` | fair |
| My RIME live | `luna_pinyin` | `634 ms` | `95 ms` | `119 ms` | `16.0 MiB` | `8.5 MiB` | fair |
| Yune public demo | Jyutping | `1347 ms` | `103 ms` | `108 ms` | `160.0 MiB` | `72.2 MiB` | guard only |
| My RIME live | Jyutping | `998 ms` | `99 ms` | `114 ms` | `68.0 MiB` | `24.9 MiB` | guard only |

## Yune Browser Input-Latency Suite

Carried forward from the 2026-06-28 run.

![Current Yune browser input latency suite](./evidence/current-performance-dashboard-2026-06-29/visuals/current-yune-browser-input-latency.svg)

| Schema | Input | Exact keydown-to-paint | Max during input | WASM peak |
| --- | --- | ---: | ---: | ---: |
| `luna_pinyin` | `hao` | `40 ms` | `40 ms` | `64.0 MiB` |
| `luna_pinyin` | `ni` | `22 ms` | `22 ms` | `64.0 MiB` |
| `luna_pinyin` | `zhongguo` | `19 ms` | `30 ms` | `64.0 MiB` |
| `luna_pinyin` | `ceshiyixiachangjushuruxingnengzenyang` | `43 ms` | `45 ms` | `64.0 MiB` |
| `luna_pinyin` | `zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong` | `75 ms` | `78 ms` | `64.0 MiB` |
| `luna_pinyin` | `cszysmsrsd` | `26 ms` | `29 ms` | `64.0 MiB` |
| `luna_pinyin` | `zybfshmsru` | `34 ms` | `47 ms` | `64.0 MiB` |
| `jyut6ping3_mobile` | `hai` | `47 ms` | `47 ms` | `160.0 MiB` |
| `jyut6ping3_mobile` | `ngo` | `23 ms` | `24 ms` | `160.0 MiB` |
| `jyut6ping3_mobile` | `caksi` | `89 ms` | `90 ms` | `160.0 MiB` |
| `jyut6ping3_mobile` | `ngogokdak` | `22 ms` | `33 ms` | `160.0 MiB` |
| `jyut6ping3_mobile` | `sihaacoenggeoisyujapgecukdou` | `130 ms` | `136 ms` | `160.0 MiB` |
| `jyut6ping3_mobile` | `taihaajyugwodaahoucoenggegeoizigosingnangwuidimjoeng` | `74 ms` | `74 ms` | `160.0 MiB` |

## Remaining Current Gaps

![Current remaining performance gaps](./evidence/current-performance-dashboard-2026-06-29/visuals/current-root-cause-gaps.svg)

| Rank | Gap | Current value | Next diagnostic target |
| ---: | --- | --- | --- |
| 1 | Native Track A peak memory | `188.4 MB` vs librime max peer peak `17.1 MB` | post-M48 full Luna preset-vocabulary/process residency attribution |
| 2 | Browser `luna_pinyin` memory | `64.0 MiB` vs My RIME `16.0 MiB` | WASM runtime floor and public-demo resource/heap split |
| 3 | Native `ni` latency | `45.450 us` vs `14.400 us` | exact-row scan under charset filtering without a retained acceptance index |
| 4 | Native 37-char pinyin latency | `890.689 us` vs `289.773 us` | remaining preset-vocabulary sentence-model graph rebuild cost |
| 5 | Browser `luna_pinyin` startup | `1000 ms` vs My RIME `634 ms` | startup asset/runtime phases after current public-demo build |

## History

Older milestone closeout detail remains in:

- [`history/2026-06-28-yune-vs-librime-performance-pre-current-dashboard.md`](./history/2026-06-28-yune-vs-librime-performance-pre-current-dashboard.md)
- [`plans/completed/`](../plans/completed/)
- [`ledgers/milestone-history.md`](../ledgers/milestone-history.md)
