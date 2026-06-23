# M33 Native Lookup Performance Evidence

Date: 2026-06-23

This folder contains the M33 before/after evidence for the bounded native
lookup/fairness milestone.

## Commands

Cross-engine before and after:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\benchmark-yune-vs-librime.ps1 -OutputRoot docs\reports\evidence\m33-2026-06-23\before-yune-vs-librime -Iterations 9 -SessionIterations 9 -KeyIterations 25
powershell -ExecutionPolicy Bypass -File scripts\benchmark-yune-vs-librime.ps1 -OutputRoot docs\reports\evidence\m33-2026-06-23\after-low-risk-yune-vs-librime -Iterations 9 -SessionIterations 9 -KeyIterations 25
```

Native before and after:

```powershell
cmd /c "cargo bench -p yune-rime-api --bench frontend_baselines > target\m33-frontend-baselines-before.txt 2>&1"
cmd /c "cargo bench -p yune-rime-api --bench frontend_baselines > target\m33-frontend-baselines-after-low-risk.txt 2>&1"
```

The native logs were copied into this folder as:

- [`frontend-baselines-before.txt`](./frontend-baselines-before.txt)
- [`frontend-baselines-after-low-risk.txt`](./frontend-baselines-after-low-risk.txt)

## Accepted M33 slice

- Build-once sharing for immutable dictionary translators, with cache invalidated
  by schema and source/compiled asset signatures.
- Lazy `stroke` reverse-lookup dictionary loading, so no-reverse `luna_pinyin`
  startup/session rows no longer compare a luna-plus-stroke Yune load against a
  luna-only librime load.

## Deferred levers

- Lazy table+prism spelling-algebra lookup: no-go for M33. The upstream prism
  fixture maps spellings to syllable descriptors but, as in librime, candidate
  text/comment/order payloads live on the table side. A byte-identical storage
  rewrite needs a broader queryable table+prism design, while the typing-latency
  gap also needs bounded/lazy candidate production so short prefixes do not
  materialize unseen completion candidates.
- mmap compiled artifacts: deferred. The low-risk slice made warm re-select and
  session select cheap, but cold startup, peak footprint, and per-key lookup are
  still behind. Mmap should be paired with a table+prism query path, not added
  as a standalone switch, and should not be claimed as the typing-latency fix
  unless attribution proves storage lookup is the per-key owner.

## Headline numbers

| Row | Yune before | Yune after | librime after |
| --- | ---: | ---: | ---: |
| Cold startup/runtime-ready | `3,141,449.8 us` | `909,375.4 us` | `80,260.8 us` |
| Warm startup/runtime-ready | `2,881,852.7 us` | `47,556.3 us` | `26,964.8 us` |
| Session create/select/destroy median | `2,985,364.0 us` | `47,813.7 us` | `25,765.9 us` |
| Startup peak working set | `261,500,928 bytes` | `182,775,808 bytes` | `22,519,808 bytes` |
| Key `ni` median | `5,579.8 us` | `6,064.5 us` | `28.5 us` |
| Key `hao` median | `11,043.8 us` | `12,463.4 us` | `34.5 us` |
| Key `zhongguo` median | `34,024.0 us` | `37,572.3 us` | `1,479.8 us` |

## Interpretation

The startup/session rows are now fair and safe to show with caveats. The cold
startup and footprint rows remain materially behind librime, and the per-key
rows are still not competitive and should not be described as a typing win.
Browser startup and browser typing were not measured in M33.
