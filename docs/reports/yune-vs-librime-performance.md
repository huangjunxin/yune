# Current Yune Performance Dashboard

Date: 2026-06-29

This dashboard shows the current benchmark state only. Milestone closeout
narrative and older benchmark rows have been moved to
[`history/2026-06-28-yune-vs-librime-performance-pre-current-dashboard.md`](./history/2026-06-28-yune-vs-librime-performance-pre-current-dashboard.md).

The **native lane was freshly re-measured on 2026-06-29** after the completed
M47 byte-backing work. The **browser lane is carried forward from the 2026-06-28
Playwright run**; it needs the WASM/Playwright pipeline against a live My RIME
and was not re-run this pass.

## Technical Summary

- **Native fair lane (`luna_pinyin`)**: Yune is faster than same-run upstream
  librime 1.17.0 on startup (`0.776x`), session (`0.780x`), `zhongguo`
  (`0.352x`), the 37-character row (now `0.940x`), the 59-character row
  (`0.718x`), and both abbreviation rows (`0.445x`, `0.624x`). The only
  remaining misses are the microsecond-scale short prefixes `ni` (`3.514x`,
  `50.3 us` vs `14.3 us`) and `hao` (`2.134x`, `25.0 us` vs `11.7 us`). This run
  did not measure the single-character `n` row.
- **Native memory**: Track A peak working set fell to `105.9 MB` (from the prior
  `128.5 MB`) versus librime's `18.8 MB` max observed peer peak. The gap is real
  but narrowed; the peak is a process high-water, while the median private-bytes
  proxy is `54.8 MB` and the session-create row is `48.4 MB`.
- **Native product path (`jyut6ping3_mobile`, non-peer)**: the M47 lookup-record
  and comment byte-backing collapsed the owned heap. In the same heavy
  deploy+load benchmark harness, Track B private bytes dropped from `420.0 MB`
  (2026-06-28) to `181.4 MB` (2026-06-29). This is not the iOS-proxy number; see
  the Native Track B section.
- **Browser fair lane (`luna_pinyin`, carried 2026-06-28)**: Yune public demo
  uses `64.0 MiB` WASM peak versus My RIME `16.0 MiB` (`4.0x`). Yune is slower to
  ready (`1000 ms` vs `634 ms`), but faster on first input (`74 ms` vs `95 ms`).
- **Browser Jyutping (carried 2026-06-28)**: Yune public demo is byte-backed at
  `160.0 MiB` WASM peak. This row is a guard, not a fair peer comparison, because
  My RIME's Jyutping uses a different Cantonese-only dictionary.

## Current Evidence Bundle

The normalized dashboard source is
[`evidence/current-performance-dashboard-2026-06-29/`](./evidence/current-performance-dashboard-2026-06-29/).

Source inputs:

- Native Track A + Track B: fresh same-run evidence captured for this dashboard
  pass with product deploy enabled
  ([`environment.txt`](./evidence/current-performance-dashboard-2026-06-29/native-current-benchmark/environment.txt)
  records `deploy_product_before_benchmark=True`):
  [`evidence/current-performance-dashboard-2026-06-29/native-current-benchmark/summary.csv`](./evidence/current-performance-dashboard-2026-06-29/native-current-benchmark/summary.csv).
- Browser peer comparator and input-latency suite: carried forward from the
  2026-06-28 Playwright run (not re-run this pass).

## Native Track A

![Current native Track A latency ratios](./evidence/current-performance-dashboard-2026-06-29/visuals/current-native-latency-ratios.svg)

| Dimension | Yune median | librime median | Yune / librime | Current read |
| --- | ---: | ---: | ---: | --- |
| startup | `24,427.800 us` | `31,474.400 us` | `0.776x` | Yune faster |
| session | `24,967.500 us` | `31,992.900 us` | `0.780x` | Yune faster |
| `ni` | `50.250 us` | `14.300 us` | `3.514x` | blocker |
| `hao` | `25.033 us` | `11.733 us` | `2.134x` | watch |
| `zhongguo` | `62.350 us` | `177.237 us` | `0.352x` | Yune faster |
| 37-char pinyin | `285.103 us` | `303.241 us` | `0.940x` | Yune faster |
| 59-char pinyin | `498.625 us` | `694.305 us` | `0.718x` | Yune faster |
| abbreviation 10-char | `559.620 us` | `1,258.790 us` | `0.445x` | Yune faster |
| abbreviation 8-char | `545.390 us` | `873.490 us` | `0.624x` | Yune faster |

## Native Track B (product path, non-peer)

The `jyut6ping3_mobile` product path has no fair librime peer (My RIME's Jyutping
uses a different dictionary), so it is not a comparison row. It is the iOS-target
product, so its owned-heap reduction is tracked here. These figures come from the
heavy in-process benchmark harness (deploy + compile of both `jyut6ping3` and
`jyut6ping3_scolar`, then sustained session/key load), **not** the lean iOS-proxy
`native_memory_probe`:

| Measurement | 2026-06-28 | 2026-06-29 | Read |
| --- | ---: | ---: | --- |
| Private bytes (key load) | `420.0 MB` | `181.4 MB` | M47 byte-backing (RED-07/08) collapsed the owned heap |
| Private bytes (session) | `405.8 MB` | `176.3 MB` | steady private bytes down with byte-backed records |
| Median working set (key load) | `440.1 MB` | `253.9 MB` | resident working set down with the owned-heap cut |
| Peak working set high-water | `504.4 MB` | `505.2 MB` | flat; dominated by the in-process deploy/compile transient |
| Session create latency | `141,590.4 us` | `93,669.3 us` | faster with byte-backed records |

> The lean iOS-dirty proxy (one schema, prebuilt assets, measured right after
> `create_session`) for the comments-intact keyboard profile is `~22 MB` private,
> reported in [`ios-memory-budget.md`](./ios-memory-budget.md). That is the
> iOS-budget number; the `181.4 MB` above is the heavy benchmark harness, not the
> iOS proxy.

## Memory High-Water

![Current memory high-water by lane](./evidence/current-performance-dashboard-2026-06-29/visuals/current-memory-peaks.svg)

| Lane | Yune | Peer | Current read |
| --- | ---: | ---: | --- |
| Native Track A peak working set | `105.9 MB` | librime max peer peak `18.8 MB` | narrowed from `128.5 MB`; still heavier |
| Browser `luna_pinyin` WASM peak (carried) | `64.0 MiB` | My RIME `16.0 MiB` | fair browser gap is `4.0x` |
| Browser Jyutping WASM peak (carried) | `160.0 MiB` | My RIME `68.0 MiB` | guard only, dictionary-confounded |

## Browser Peer Dashboard

Carried forward from the 2026-06-28 Playwright run (not re-measured this pass).

![Current browser peer latency](./evidence/current-performance-dashboard-2026-06-29/visuals/current-browser-peer-latency.svg)

![Current browser memory and payload](./evidence/current-performance-dashboard-2026-06-29/visuals/current-browser-memory-payload.svg)

| Scenario | Schema | Ready | Input -> candidate | Commit | WASM peak | Unique encoded resources | Validity |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| Yune public demo | `luna_pinyin` | `1000 ms` | `74 ms` | `107 ms` | `64.0 MiB` | `29.5 MiB` | fair |
| My RIME live | `luna_pinyin` | `634 ms` | `95 ms` | `119 ms` | `16.0 MiB` | `8.5 MiB` | fair |
| Yune public demo | Jyutping | `1347 ms` | `103 ms` | `108 ms` | `160.0 MiB` | `72.2 MiB` | guard only |
| My RIME live | Jyutping | `998 ms` | `99 ms` | `114 ms` | `68.0 MiB` | `24.9 MiB` | guard only |

## Yune Browser Input-Latency Suite

Carried forward from the 2026-06-28 run (not re-measured this pass).

![Current Yune browser input latency suite](./evidence/current-performance-dashboard-2026-06-29/visuals/current-yune-browser-input-latency.svg)

| Schema | Input | Exact keydown-to-paint | Max during input | WASM peak | First candidate |
| --- | --- | ---: | ---: | ---: | --- |
| `luna_pinyin` | `hao` | `40 ms` | `40 ms` | `64.0 MiB` | `好` |
| `luna_pinyin` | `ni` | `22 ms` | `22 ms` | `64.0 MiB` | `你` |
| `luna_pinyin` | `zhongguo` | `19 ms` | `30 ms` | `64.0 MiB` | `中國大陸` |
| `luna_pinyin` | `ceshiyixiachangjushuruxingnengzenyang` | `43 ms` | `45 ms` | `64.0 MiB` | `測是一下長據書如行能怎樣` |
| `luna_pinyin` | `zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong` | `75 ms` | `78 ms` | `64.0 MiB` | `這個因請其是應該之喫差哦長據子書如才能用` |
| `luna_pinyin` | `cszysmsrsd` | `26 ms` | `29 ms` | `64.0 MiB` | placeholder row |
| `luna_pinyin` | `zybfshmsru` | `34 ms` | `47 ms` | `64.0 MiB` | placeholder row |
| `jyut6ping3_mobile` | `hai` | `47 ms` | `47 ms` | `160.0 MiB` | `係` |
| `jyut6ping3_mobile` | `ngo` | `23 ms` | `24 ms` | `160.0 MiB` | `我` |
| `jyut6ping3_mobile` | `caksi` | `89 ms` | `90 ms` | `160.0 MiB` | `測時` |
| `jyut6ping3_mobile` | `ngogokdak` | `22 ms` | `33 ms` | `160.0 MiB` | `我覺得` |
| `jyut6ping3_mobile` | `sihaacoenggeoisyujapgecukdou` | `130 ms` | `136 ms` | `160.0 MiB` | `試下場據輸入嘅速都` |
| `jyut6ping3_mobile` | `taihaajyugwodaahoucoenggegeoizigosingnangwuidimjoeng` | `74 ms` | `74 ms` | `160.0 MiB` | `睇下如果打好場嘅據自個責會點樣` |

## Remaining Current Gaps

![Current remaining performance gaps](./evidence/current-performance-dashboard-2026-06-29/visuals/current-root-cause-gaps.svg)

| Rank | Gap | Current value | Next diagnostic target |
| ---: | --- | --- | --- |
| 1 | Native Track A peak memory | `105.9 MB` vs librime max peer peak `18.8 MB` | allocator/transient/private-byte attribution for the high-water peak |
| 2 | Browser `luna_pinyin` memory | `64.0 MiB` vs My RIME `16.0 MiB` | WASM runtime floor and public-demo resource/heap split |
| 3 | Native `ni` latency | `50.250 us` vs `14.300 us` | short-prefix translator/prefix constant factor |
| 4 | Native `hao` latency | `25.033 us` vs `11.733 us` | same short-prefix owner |
| 5 | Browser `luna_pinyin` startup | `1000 ms` vs My RIME `634 ms` | startup asset/runtime phases after current public-demo build |

## History

Older milestone closeout detail remains in:

- [`history/2026-06-28-yune-vs-librime-performance-pre-current-dashboard.md`](./history/2026-06-28-yune-vs-librime-performance-pre-current-dashboard.md)
- [`plans/completed/`](../plans/completed/)
- [`ledgers/milestone-history.md`](../ledgers/milestone-history.md)
