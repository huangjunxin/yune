# M39 Final Gates

Date: 2026-06-25

Scope: native engine evidence only. This closeout does not make browser,
frontend, product-delivery, packaging, or public demo speed claims.

## Evidence

- Final native benchmark:
  [`phase-4-final-native/`](./phase-4-final-native/)
- Baseline:
  [`phase-0-baseline/`](./phase-0-baseline/)
- Owner attribution:
  [`phase-1-attribution/owner-attribution.md`](./phase-1-attribution/owner-attribution.md)
- Memory attribution:
  [`phase-3-memory/memory-owner-summary.md`](./phase-3-memory/memory-owner-summary.md)
- Completed plan:
  [`../../../plans/completed/m39-plan-long-input-engine-hardening.md`](../../../plans/completed/m39-plan-long-input-engine-hardening.md)

Final benchmark command:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\benchmark-native-rime-inprocess.ps1 -OutputRoot docs\reports\evidence\m39-long-input-engine-hardening\phase-4-final-native -Iterations 9 -SessionIterations 20 -KeyIterations 20 -TrackAInputs "ni,hao,zhongguo,ceshiyixiachangjushuruxingnengzenyang,zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong" -TrackBInputs "hai,ngohaig,jigaajiusihaa,loengjathau,caksijathaacoenggeoizi,neigojangingkeisatjinggoiziwunciucoenggeoizisyujapsinhojijung" -DeployProductBeforeBenchmark
```

## Gate Table

| Gate | Result | Evidence |
| --- | --- | --- |
| `M39-ENGINE-01` same-run native benchmark | Pass | Final run includes startup, session, `ni`, `hao`, `zhongguo`, both Track A long rows, and the Track B long profile row. |
| `M39-ENGINE-02` startup/session no regression | Pass | Startup `0.917x`; session `0.938x` versus same-run librime. |
| `M39-ENGINE-03` short/medium no regression | Pass | `hao` `3.281x`, `ni` `3.863x`, `zhongguo` `0.329x`; all remain inside the existing short-row gates. |
| `M39-ENGINE-04` long-input parity | Pass | Track A 37-character row `1.765x`; Track A 59-character row `1.320x`; Track B 50+ profile row median `188.857 us/op`, p95 `194.910 us/op`, below Phase 0. |
| `M39-ENGINE-05` storage hot path | Pass | Track A remains `rsmarisa_byte_backed`, table/prism are `mmap`, selected heap mirrors are `0`, and `rsmarisa` exact/prefix counters are positive. |
| `M39-ENGINE-06` bounded output/context | Pass | Track A target rows use bounded first-page reads with no full-list fallback. Track B profile fallback is counted and explained as a separate owner. |
| `M39-ENGINE-07` memory attribution/no regression | Pass | Track A max peak moved from `163,598,336` to `123,985,920` bytes; Track B peak moved from `504,557,568` to `504,041,472` bytes. |
| `M39-ENGINE-08` behavior | Pass | Focused behavior gates, workspace tests, clippy, format, and diff checks passed. |
| `M39-ENGINE-09` honest claims | Pass | Reports remain native-engine-only and do not make browser/frontend/product-delivery claims. |

## Final Latency

| Row | Yune median | librime median | Ratio | Gate |
| --- | ---: | ---: | ---: | --- |
| startup/runtime-ready | `25,048.200 us` | `27,314.000 us` | `0.917x` | Pass |
| session create/select/destroy | `25,255.500 us` | `26,938.500 us` | `0.938x` | Pass |
| `hao` | `38.933 us` | `11.867 us` | `3.281x` | Pass |
| `ni` | `56.200 us` | `14.550 us` | `3.863x` | Pass |
| `zhongguo` | `60.588 us` | `183.887 us` | `0.329x` | Pass |
| `ceshiyixiachangjushuruxingnengzenyang` | `514.903 us` | `291.786 us` | `1.765x` | Pass |
| `zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong` | `917.961 us` | `695.653 us` | `1.320x` | Pass |

## Track B Profile Row

| Row | Phase 0 median | Final median | Phase 0 p95 | Final p95 | Result |
| --- | ---: | ---: | ---: | ---: | --- |
| `neigojangingkeisatjinggoiziwunciucoenggeoizisyujapsinhojijung` | `189.207 us/op` | `188.857 us/op` | `202.084 us/op` | `194.910 us/op` | Pass |

Task 1 proved this row does not share the Track A owner. Track A was dominated
by upstream sentence-model scanning; Track B remains a TypeDuck profile path
with no upstream sentence-model calls, no-marisa exact/prefix lookup, and
profile fallback/full-list merge behavior that is required for existing rich
comment and composition semantics.

## Owner Movement

| Row | Phase 1 owner | Final owner shape |
| --- | --- | --- |
| Track A 37-character row | `upstream_sentence_model_ns` at `436,917.530 us/op` | Indexed bounded upstream sentence model at `444.995 us/op`; no full-list fallback. |
| Track A 59-character row | `upstream_sentence_model_ns` at `1,228,565.656 us/op` | Indexed bounded upstream sentence model at `823.125 us/op`; no full-list fallback. |
| Track B 50+ row | Profile-specific no-marisa prefix/fallback path | Preserved profile path: upstream model `0`, bounded request counted, profile fallback counted and explained. |

## Storage And Memory

| Track | Final storage | Heap mirrors | Final memory result |
| --- | --- | --- | --- |
| Track A `luna_pinyin` | `selected_storage=rsmarisa_byte_backed`; table/prism `mmap`; `source_fallback=false`; `rsmarisa_status=ok`; `rsmarisa_mapping_mode=mmap`; `rsmarisa_num_keys=463586` | table `0`, prism `0` | Max peak `123,985,920` bytes, below Phase 0 `163,598,336` bytes. |
| Track B `jyut6ping3_mobile` | `selected_storage=byte_backed`; table/prism `mmap`; `source_fallback=false`; `rsmarisa_status=missing_string_table` | table `0`, prism `0` | Final median working set `441,450,496` bytes and peak `504,041,472` bytes, below Phase 0. |

## Quality Gates

| Command | Result |
| --- | --- |
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo test -p yune-core translator:: -- --nocapture` | Pass |
| `cargo test -p yune-core upstream_luna_pinyin -- --nocapture` | Pass |
| `cargo test -p yune-rime-api --test typeduck_windows_boundary -- --nocapture` | Pass |
| `git diff --check` | Pass |
