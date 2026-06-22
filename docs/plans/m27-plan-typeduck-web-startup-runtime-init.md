# M27 TypeDuck-Web Startup Runtime Init Follow-Up Plan

> **Status:** Parked - **Milestone:** M27 (TypeDuck-Web startup/runtime init) - **Updated:** 2026-06-22 - **Type:** execution plan

**Goal:** Reduce the measured TypeDuck-Web startup owner that M26 identified after browser assets load: schema selection plus `TypeDuckRuntime.init` / Yune runtime initialization under the coarse `runtime:initialized` marker.

**Trigger:** Start this milestone when the project is ready for a startup-focused performance slice. It is intentionally parked so M26 can close with its lower-risk dynamic-correction optimization while preserving compatibility behavior and patch discipline.

## Evidence From M26

M26 startup attribution evidence lives under `third_party/typeduck-web/e2e/results/m26-performance/`:

- `startup-attribution-before.json`: fresh startup `startup:complete.totalMs=10324`; assets load finishes at `120ms`; `schema:select` finishes at `10324ms`.
- `startup-attribution-before.json`: reload startup `startup:complete.totalMs=10435`; assets load finishes at `81ms`; `schema:select` finishes at `10435ms`.
- `startup-attribution-after.json`: preserves the same nested marker shape after the M26 dynamic-correction slice.

M26 native startup evidence also shows real-asset startup remains much larger than warm per-key work:

- `startup_real_jyut6ping3_mobile_runtime_ready` after M26: median `15103350.100us`, p95 `15636872.300us`.
- `startup_real_luna_pinyin_runtime_ready` after M26: median `789428.900us`, p95 `802446.100us`.

## Scope

In scope:

- Attribute `schema:select` / runtime init below the current worker marker, including `TypeDuckRuntime.init`, `yune_typeduck_init`, schema install/deploy selection, dictionary parse/index build, config compile, userdb open/sync, and candidate/translators construction where measurable.
- Reduce repeated work in warm reload/cache-hit startup when evidence proves the reuse is behavior-preserving.
- Preserve browser/WASM source patch discipline: any `third_party/typeduck-web/source/` edit must be regenerated into `third_party/typeduck-web/patches/yune-typeduck-runtime.patch` and checked reverse/forward.

Out of scope:

- Changing `RimeApi`, `RimeCandidate`, or TypeDuck profile ABI layout.
- Replacing the dictionary storage format without a separate design review.
- Treating browser startup numbers as native engine numbers.
- Reopening M24 or M25 dogfood ledgers.

## Candidate Slices

1. Split `schema:select` into native runtime owners: `runtime:setup`, `schema:install`, `dictionary:parse`, `translator:index-build`, `userdb:open`, `session:create`, and `schema:selected`.
2. Cache or reuse parsed schema/deploy data across reloads only when the persisted asset/deploy stamp proves freshness.
3. Avoid initializing non-active schema dictionaries or side lookup resources on the default startup path.
4. Measure whether compiled `.table.bin`/`.prism.bin` consumption or source fallback dominates startup for `jyut6ping3_mobile`.

## Required Gates

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p yune-core --test cantonese_parity`
- `cargo test -p yune-core --test upstream_luna_pinyin_parity`
- `cargo test -p yune-rime-api --test typeduck_web`
- `cargo bench -p yune-rime-api --bench frontend_baselines`
- `npm --prefix packages/yune-typeduck-runtime test`
- `npm --prefix packages/yune-typeduck-runtime run build`
- `npm --prefix third_party/typeduck-web/source run build`
- Focused Playwright startup evidence.
- TypeDuck-Web patch reverse/forward checks if source changed.
