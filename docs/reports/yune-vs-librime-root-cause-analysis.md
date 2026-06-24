# Why Yune is slower than librime - root-cause analysis

Date: 2026-06-24

Companion report: [`yune-vs-librime-performance.md`](./yune-vs-librime-performance.md).

## Current Verdict After M35

The remaining gap is not a generic Rust problem. It is now split between a
landed upstream compact-storage win and remaining TypeDuck/product, harness, and
whole-process-memory owners.

Resolved or improved:

1. **M33 fairness:** Yune no longer loads `stroke` reverse lookup during the
   no-reverse `luna_pinyin` startup/session comparison.
2. **M33 repeated schema/session cost:** immutable built dictionary translators
   are shared across compatible schema selects.
3. **M34 bounded first-page work:** short `luna_pinyin` typing can keep complete
   prefix enumeration but materialize only a bounded candidate window for the
   safe no-ranker/no-userdb/no-full-list-filter subset.
4. **M35 compact upstream storage:** upstream `luna_pinyin` can use compact
   table storage plus prism canonical-code lookup without retaining heap
   `entries_by_code` or expanded spelling-algebra aliases.

Still open:

1. **Engine-only lookup is improved but still far behind librime.** M35 moves
   `hao_engine_only` `1092.879us` -> `750.517us`, `ni_engine_only`
   `891.791us` -> `697.044us`, and `zhongguo_engine_only` `740.966us` ->
   `485.482us`, but not into the low-hundreds target range.
2. **TypeDuck remains heap-backed.** `jyut6ping3` keeps heap fallback because
   rich comments, lookup records, partial selection, default-confirm
   recomposition, long composition, dynamic correction, and userdb learning are
   product invariants.
3. **Whole-process peak remains high.** Upstream dictionary-specific native
   deltas improved, but the fair harness peak remains about `182 MB` versus
   librime's about `22 MB`.
4. **Mmap/borrowed storage is still conditional.** It now has a valid compact
   query substrate, but M35 closes mmap as a no-go until a separate design covers
   byte borrowing, rebuild invalidation, Windows file lifetime, and demand
   paging.

## What changed in M34

M34 added an internal bounded candidate request and lazy engine window:

- `Translator::translate_with_context_and_request(...)` defaults to eager
  compatibility behavior.
- `StaticTableTranslator` uses bounded materialization only for the safe subset.
- Prefix enumeration remains complete under the current code-ordered heap map;
  only candidate clone/comment/preedit materialization is bounded.
- `Engine::refresh_candidates` uses the bounded path for short `luna_pinyin`
  input when filters/rankers/userdb allow it.
- Out-of-window candidate actions and full-list candidate iterators force a
  complete eager refresh.
- `RimeGetContext` receives a `candidate_list_complete` bit so it can report
  `is_last_page` honestly without materializing every candidate for first-page
  reads.
- A private `TableLookup` abstraction now covers exact/prefix/all-code queries
  for the current heap map.

The public C ABI did not change. `RimeApi`, `RimeCandidate`, and the TypeDuck
profile ABI remain isolated.

## What Changed In M35

M35 added the compact runtime storage substrate that M34 deliberately deferred:

- `TableLookup` now returns lightweight candidate views instead of heap
  `&[Candidate]` slices.
- `CompactTableStore` answers exact, prefix, and all-code queries without
  retaining per-row `Candidate` values.
- `RimePrismBinPayload::lookup_canonical_codes(...)` maps typed spellings to
  canonical table codes; table storage still supplies candidate payloads.
- `StaticTableTranslator` uses a private heap-or-compact storage enum.
- `schema_install` preserves parsed prism payloads and enables compact storage
  only for safe upstream `luna_pinyin`.
- TypeDuck `jyut6ping3` remains heap-backed by design.

The public C ABI still did not change. Default `RimeApi`, `RimeCandidate`, and
TypeDuck profile ABI slots are untouched.

## Measured Shape

| Surface | Before | M34 after | Interpretation |
| --- | ---: | ---: | --- |
| native `ni` full ABI | `1,760.250 us` | `1,132.950 us` | bounded first-page/context win |
| native `ni` engine-only | `569.700 us` | `575.250 us` | raw lookup not solved |
| cross-engine `hao` | `13,336.800 us` | `12,216.900 us` | improved, still `348.1x` librime |
| cross-engine `ni` | `5,858.800 us` | `5,693.900 us` | improved, still `198.4x` librime |
| cross-engine `zhongguo` | `36,451.100 us` | `35,909.100 us` | modest improvement, still `26.0x` librime |
| peak working set | `182,874,112 bytes` | `182,333,440 bytes` | no footprint win |

M35 movement:

| Surface | M35 baseline | M35 after | Interpretation |
| --- | ---: | ---: | --- |
| native `hao` engine-only | `1092.879us` | `750.517us` | compact upstream path improves, target not met |
| native `ni` engine-only | `891.791us` | `697.044us` | compact upstream path improves, target not met |
| native `zhongguo` full ABI | `14759.755us` | `1527.055us` | spelling-algebra expansion removed from hot path |
| `spelling_algebra_expand` startup | `148570.200us` / `17784832 bytes` | `122.200us` / `0 bytes` | expanded alias heap removed |
| `translator_install` startup | `233169.800us` / `37556224 bytes` | `55155.800us` / `9822208 bytes` | retained upstream dictionary delta cut |
| fair `hao` | `15906.800us` | `12547.200us` | improved, still `354.4x` librime |
| fair `ni` | `9225.100us` | `5678.500us` | improved, still `197.9x` librime |
| fair `zhongguo` | `45608.600us` | `35848.500us` | improved, still `24.7x` librime |
| fair peak working set | `182910976 bytes` | `182444032 bytes` | whole-process footprint not solved |

TypeDuck full-ABI guard rows stayed heap-backed and within the M35 guard/no-go
expectation:

- `hai`: `18,900.742 us` -> `18,450.767 us` (`-2.4%`)
- `jigaajiusihaa`: `28,836.874 us` -> `26,953.441 us` (`-6.5%`)
- correction-on `jigaajiusihaa`: `24,811.675 us` -> `26,707.480 us` (`+7.6%`)

The companion performance report now embeds M35 visualizations for native
watched-row movement, fair cross-engine gap, and dictionary-local memory versus
whole-process peak. Those charts intentionally keep the remaining librime gap
visible instead of turning the compact-storage win into a broad performance
claim.

## Why Librime Remains Faster

librime's classic table path has a compact deployed data model and a lazy
candidate iterator:

- deployed table/prism assets are compact and mmap-friendly;
- prism/spelling lookup is an index into table payloads;
- candidates are exposed through page/iterator-oriented APIs;
- full candidate payload materialization is avoided until needed;
- reverse lookup is lazy;
- schema/dictionary state is shared.

Yune now has lazy reverse lookup, build-once translator sharing, a narrow
bounded first-page path, and compact upstream `luna_pinyin` table+prism storage.
It still falls back to eager/full-list behavior for TypeDuck profile,
userdb/ranker, correction-heavy paths, and many filter cases, and it still keeps
whole-process memory far above librime.

## Follow-Up After M35

M35 closed the first compact owned-storage slice. The remaining safe order is:

1. Keep compact upstream storage active and broaden it only when byte-identical
   fixture coverage exists.
2. Treat TypeDuck compact storage as a separate profile-specific project, not an
   automatic follow-on, because lookup records and rich comments are required.
3. Measure the remaining per-key owner after compact lookup; do not use fair
   cross-engine ratios as the main typing headline while harness overhead is
   still mixed in.
4. Design borrowed/mmap storage separately, with Windows file lifetime and
   rebuild behavior covered before any unsafe or lint exception is accepted.

Native engine-only and full-ABI rows remain the clearer M35 engine movement
signals. The fair cross-engine rows are compatibility-harness evidence and
should keep the unresolved ratio visible.

When reporting M35 memory, distinguish dictionary-specific delta from whole
process peak. The order-of-magnitude target applies to the dictionary heap
delta. A correct owned compact-storage result may still land at roughly
`2-3x` librime peak until native mmap/demand paging is proven.

Browser startup and public delivery improvements remain M31 work, not M35
engine-performance evidence.
