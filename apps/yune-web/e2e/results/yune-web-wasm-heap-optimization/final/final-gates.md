# WEB-01 Final Gates

Date: 2026-06-27

Engine base: post-M45 `main`.

Worktree isolation:

- Direct-to-main workflow in `C:\Users\laubonghaudoi\Documents\GitHub\yune`.
- No parallel native or browser benchmark runs were active during WEB-01
  benchmark evidence capture.
- WEB-01 executable diff contains no `crates/` source changes.

Commands:

| Gate | Result |
| --- | --- |
| `npm.cmd --prefix apps/yune-web run typecheck` | Pass |
| `npm.cmd --prefix apps/yune-web run build` | Pass |
| `npm.cmd --prefix apps/yune-web run build:public` | Pass |
| `YUNE_WEB_WASM_HEAP_BENCHMARK=1 YUNE_WEB_BENCHMARK_SAMPLES=3 YUNE_WEB_BENCHMARK_PHASE=final ... --grep "YUNE WEB WASM HEAP"` | Pass |
| `YUNE_WEB_WASM_ATTRIBUTION=1 YUNE_WEB_WASM_ATTRIBUTION_PHASE=final-attribution ... --grep "YUNE WEB WASM ATTRIBUTION"` | Pass |
| `YUNE_WEB_COMPARATOR_BASELINE=1 YUNE_WEB_COMPARATOR_INCLUDE_MY_RIME=1 YUNE_WEB_COMPARATOR_SAMPLES=7 YUNE_WEB_COMPARATOR_PHASE=final ... --grep "YUNE WEB COMPARATOR"` | Pass |
| Focused smoke: `WASM heap metrics populate`, `M42 User Dictionary learns`, `Shift toggles ASCII mode` | Pass |
| Focused smoke: M22/M25 reverse lookup rows | Fail, existing current-runtime blocker independent of 64 MiB candidate |
| Focused smoke: M22 schema switcher row | Fail after Cangjie -> Luna -> Jyutping switch; page reports about `1.9 GiB` WASM and no Jyutping candidates |

Final verdict:

- `engine-owned-measured-no-go` for the Jyutping `893.1 MiB` browser WASM
  high-water.
- Measured no-go for the browser `INITIAL_MEMORY` lever: `64 MiB` grows to the
  same final heap as the post-M45 baseline, and `48 MiB` grows Luna higher to
  `176.0 MiB`.
- No browser heap, payload, native memory, public-demo speed, packaging, or
  product-delivery win is claimed.

Required follow-up:

- A future WASM-memory runtime/engine plan should start from
  `apps/yune-web/src/worker.ts`, `apps/yune-web/src/yune-integration/adapter.ts`,
  allocator/growth markers, and the `final-attribution` `extras` /
  `jyutping-core` rows that both reach `893.1 MiB`.
