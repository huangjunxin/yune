# Yune vs upstream librime performance report

Date: 2026-06-23

Evidence:

- M33 before: [`evidence/m33-2026-06-23/before-yune-vs-librime/`](./evidence/m33-2026-06-23/before-yune-vs-librime/)
- M33 after low-risk slice: [`evidence/m33-2026-06-23/after-low-risk-yune-vs-librime/`](./evidence/m33-2026-06-23/after-low-risk-yune-vs-librime/)
- Native Criterion logs: [`evidence/m33-2026-06-23/frontend-baselines-before.txt`](./evidence/m33-2026-06-23/frontend-baselines-before.txt) and [`evidence/m33-2026-06-23/frontend-baselines-after-low-risk.txt`](./evidence/m33-2026-06-23/frontend-baselines-after-low-risk.txt)

## Public summary

M33 corrects the earlier unfair `luna_pinyin` comparison. Yune now lazy-loads the
`stroke` reverse-lookup dictionary, matching upstream librime's behavior for the
timed no-reverse-lookup rows. The final public comparison can safely show a real
startup/session improvement, but it must separate cold launch from warm cache
hits and must show that per-key lookup and memory footprint still trail librime
by a wide margin.

Final M33 result on the shared upstream `luna_pinyin` C-ABI workload:

- Cold startup/runtime-ready sample: Yune `909,375.4 us`; librime `80,260.8 us`; Yune is `11.3x` slower.
- Warm startup/runtime-ready median after the first cache-building sample: Yune `47,556.3 us`; librime `26,964.8 us`; Yune is `1.8x` slower.
- Session create/select/destroy median: Yune `47,813.7 us`; librime `25,765.9 us`; Yune is `1.9x` slower.
- Startup peak working set: Yune `182,775,808 bytes`; librime `22,519,808 bytes`; Yune peaks at about `8.1x` librime.
- Key processing still trails: `ni` `212.8x`, `hao` `361.3x`, and `zhongguo` `25.4x` slower than librime.

Against the M33 before run, Yune's cold startup sample dropped from
`3,141,449.8 us` to `909,375.4 us` (`-71.1%`), and warm re-selects now run
around `47.6 ms` because the process-global built-translator cache is hot.
Session create/select/destroy dropped from `2,985,364.0 us` to `47,813.7 us`
(`-98.4%`). Per-key Yune rows regressed in this run: `ni` `+8.7%`, `hao`
`+12.9%`, and `zhongguo` `+10.4%`. Those regressions are small relative to the
startup/session work but are not a win claim.

No browser startup, browser typing, WASM, React, or TypeDuck-Web UI result is
claimed from this benchmark. No chart SVG was regenerated for M33; a chart is
safe to publish only if it shows both the startup/session win and the unresolved
per-key gap.

## Methodology

Both engines were measured through the same librime-shaped C API harness:
[`../../scripts/yune-vs-librime-benchmark.cs`](../../scripts/yune-vs-librime-benchmark.cs),
driven by [`../../scripts/benchmark-yune-vs-librime.ps1`](../../scripts/benchmark-yune-vs-librime.ps1).

Command used for both M33 cross-engine runs:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\benchmark-yune-vs-librime.ps1 -OutputRoot <evidence-dir> -Iterations 9 -SessionIterations 9 -KeyIterations 25
```

Native benchmark command:

```powershell
cmd /c "cargo bench -p yune-rime-api --bench frontend_baselines > target\m33-frontend-baselines-*.txt 2>&1"
```

The cross-engine rows use the same upstream `luna_pinyin` schema id, the same
shared/user data roots, and the same `default` module list. The timed key rows
are `ni`, `hao`, and `zhongguo`; none triggers reverse lookup. After M33, Yune
does not load `stroke` during schema select for those rows, so the former
luna-plus-stroke Yune vs luna-only librime startup/session mismatch is gone.

This is a no-deploy comparison. It does not measure dictionary deployment, web
asset loading, browser paint, or TypeDuck `jyut6ping3` profile behavior. The
startup workload runs multiple samples in one process, so sample `0` is the cold
schema build and samples `1..8` are warm cache hits. Memory counters are Windows
process working-set counters from the benchmark host; peak working set is the
footprint comparison, while ready deltas describe marginal growth for a sample.

## Results

### Cross-engine summary

| Workload | Engine | Cold sample | Warm/steady median | Peak working set |
| --- | --- | ---: | ---: | ---: |
| Startup/runtime-ready | Yune before | `3,141,449.8 us` | `2,881,852.7 us` | `261,500,928 bytes` |
| Startup/runtime-ready | Yune after | `909,375.4 us` | `47,556.3 us` | `182,775,808 bytes` |
| Startup/runtime-ready | librime after | `80,260.8 us` | `26,964.8 us` | `22,519,808 bytes` |
| Session create/select/destroy | Yune before | n/a | `2,985,364.0 us` | `261,500,928 bytes` |
| Session create/select/destroy | Yune after | n/a | `47,813.7 us` | `182,775,808 bytes` |
| Session create/select/destroy | librime after | n/a | `25,765.9 us` | `22,519,808 bytes` |

The `909.4 ms` after-run sample is not a random tail; it is the first cold
schema build in a fresh benchmark process. The `47.6 ms` row is the warm
re-select capability after that process-global cache has been populated. The
session row is the cleanest accepted win from the build-once cache.

### Memory footprint

| Metric | Yune before | Yune after | librime after |
| --- | ---: | ---: | ---: |
| Cold startup ready delta | `219,164,672 bytes` | `155,615,232 bytes` | `4,775,936 bytes` |
| Startup peak working set | `261,500,928 bytes` | `182,775,808 bytes` | `22,519,808 bytes` |
| Warm marginal ready delta | n/a | `24,576 bytes` | `847,872 bytes` |

The warm marginal ready delta is useful for cache-hit accounting, but it is not
the memory footprint. Public copy should lead with peak working set: after M33,
Yune still peaks around `8.1x` librime on this workload.

### Key processing

| Input | Yune before | Yune after | librime after | After ratio |
| --- | ---: | ---: | ---: | ---: |
| `ni` | `5,579.8 us` | `6,064.5 us` | `28.5 us` | `212.8x` |
| `hao` | `11,043.8 us` | `12,463.4 us` | `34.5 us` | `361.3x` |
| `zhongguo` | `34,024.0 us` | `37,572.3 us` | `1,479.8 us` | `25.4x` |

These rows remain the main unresolved native typing gap. M33 did not rewrite the
candidate production pipeline or the table/prism lookup model, so it should not
be described as a per-key latency win.

### Native watched rows

The in-repo `frontend_baselines` benchmark confirms the same shape:

| Row | Before median | After median | Change |
| --- | ---: | ---: | ---: |
| `startup_trace_luna_pinyin_select_schema_total` | `261,245 us` | `223,858 us` | `-14.3%` |
| `startup_trace_luna_pinyin_translator_install` | `194,531 us` | `171,438 us` | `-11.9%` |
| `startup_trace_luna_pinyin_spelling_algebra_expand` | `104,343 us` | `107,597 us` | `+3.1%` |
| `startup_trace_luna_pinyin_translator_index_build` | `8,132 us` | `11,283 us` | `+38.8%` |
| `per_key_real_jyut6ping3_mobile_hai_full_abi` | `20,557.833 us` | `20,979.133 us` | `+2.0%` |
| `per_key_real_jyut6ping3_mobile_jigaajiusihaa_full_abi` | `29,935.692 us` | `31,033.692 us` | `+3.7%` |
| `per_key_real_luna_pinyin_ni_full_abi` | `1,429.950 us` | `1,913.350 us` | `+33.8%` |
| `per_key_real_luna_pinyin_zhongguo_full_abi` | `12,064.550 us` | `12,705.675 us` | `+5.3%` |

The native startup trace is useful for owner attribution, but the cross-engine
harness is the public comparison because it isolates the shared C-ABI surface.

## Interpretation

M33 landed two low-risk changes:

- Process-wide sharing of immutable built dictionary translators keyed by schema
  and resolved asset signatures, with deploy/source invalidation coverage.
- Lazy reverse-lookup dictionary loading, so `stroke` is loaded on first reverse
  lookup rather than during `luna_pinyin` schema select.

The lazy spelling-algebra/prism rewrite was not accepted in M33. The checked-in
upstream prism fixture proves the current prism can map spellings such as `ni`
to syllable descriptors, but, as in librime, the candidate text/comment/order
payload belongs to the table side. The storage missing piece is a lazy queryable
table+prism path, not a prism-only lookup. The typing-latency missing piece is
separate: Yune still has an eager translator/engine pipeline that can materialize
and sort/filter far more candidates than the current page displays. Both pieces
need a broader M34 plan and should not block M31.

Memory-mapping was also deferred. The low-risk slice made warm re-select/session
cheap, but cold launch and footprint remain behind librime. Mmap alone is still
not the right next step unless the hot path can walk borrowed table/prism
structures instead of rebuilding heap `BTreeMap<String, Vec<Candidate>>`
indexes, and it should not be described as the per-key fix unless attribution
shows storage lookup is the dominant per-key owner.

## Safe public claim

It is safe to say:

> After M33, Yune's fair upstream `luna_pinyin` comparison is no longer distorted
> by eager `stroke` reverse-lookup loading. The low-risk native cache/lazy-reverse
> slice reduced cold startup from about `3.14 s` to `0.91 s`, and warm re-select
> plus session select now run around `48 ms`.

It is not safe to say:

> Yune is faster than librime, Yune uses less memory than librime, Yune typing is
> faster, or browser typing/startup improved.

The remaining before-M31 recommendation is to use this fair report for public
copy, avoid a one-sided chart, and keep native typing optimization as a future
M34-style milestone: first profile and bound/lazify candidate production, then
rewrite table+prism storage if the evidence still points there.
