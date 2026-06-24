# Why Yune is slower than librime - root-cause analysis

Date: 2026-06-23

Companion report: [`yune-vs-librime-performance.md`](./yune-vs-librime-performance.md).

## Current verdict after M34

The remaining gap is not a generic Rust problem. It is a concrete split between
candidate pipeline work and storage representation work.

Resolved or improved:

1. **M33 fairness:** Yune no longer loads `stroke` reverse lookup during the
   no-reverse `luna_pinyin` startup/session comparison.
2. **M33 repeated schema/session cost:** immutable built dictionary translators
   are shared across compatible schema selects.
3. **M34 bounded first-page work:** short `luna_pinyin` typing can keep complete
   prefix enumeration but materialize only a bounded candidate window for the
   safe no-ranker/no-userdb/no-full-list-filter subset.

Still open:

1. **Engine-only lookup remains far behind librime.** M34 improves the
   full-ABI/context surface more than raw engine-only lookup.
2. **The storage model is still heap-expanded.** `entries_by_code` remains a
   `BTreeMap<String, Vec<Candidate>>` for eager, correction, sentence, and
   TypeDuck fallback behavior.
3. **Compiled table/prism query is not yet the runtime hot path.** The prism can
   find spellings, but candidate payloads live in table data.
4. **Mmap is still conditional.** It only helps once the hot path can borrow or
   index compact queryable table/prism storage instead of rebuilding heap maps.

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

## Measured shape

| Surface | Before | M34 after | Interpretation |
| --- | ---: | ---: | --- |
| native `ni` full ABI | `1,760.250 us` | `1,132.950 us` | bounded first-page/context win |
| native `ni` engine-only | `569.700 us` | `575.250 us` | raw lookup not solved |
| cross-engine `hao` | `13,336.800 us` | `12,216.900 us` | improved, still `348.1x` librime |
| cross-engine `ni` | `5,858.800 us` | `5,693.900 us` | improved, still `198.4x` librime |
| cross-engine `zhongguo` | `36,451.100 us` | `35,909.100 us` | modest improvement, still `26.0x` librime |
| peak working set | `182,874,112 bytes` | `182,333,440 bytes` | no footprint win |

TypeDuck full-ABI guard rows stayed within the accepted M34 range:

- `hai`: `18,389.567 us` -> `19,446.467 us` (`+5.7%`)
- `jigaajiusihaa`: `29,937.777 us` -> `28,155.585 us` (`-6.0%`)
- correction-on `jigaajiusihaa`: `29,649.146 us` -> `28,032.915 us` (`-5.5%`)

## Why librime remains faster

librime's classic table path has a compact deployed data model and a lazy
candidate iterator:

- deployed table/prism assets are compact and mmap-friendly;
- prism/spelling lookup is an index into table payloads;
- candidates are exposed through page/iterator-oriented APIs;
- full candidate payload materialization is avoided until needed;
- reverse lookup is lazy;
- schema/dictionary state is shared.

Yune now has lazy reverse lookup, build-once translator sharing, and a narrow
bounded first-page path. It still lacks the compact queryable table/prism
runtime representation and still falls back to eager full-list behavior for
sentence, correction, TypeDuck profile, userdb/ranker, and most filter cases.

## M35 follow-up

M35 is the planned deep engine-performance follow-up for the remaining storage
gap. It should not start with mmap alone. The safe order is:

1. Measure the current compiled table/prism/reverse asset sizes, ready/peak
   working-set deltas, post-spelling-algebra expansion counts, duplicated
   text/comment bytes, and heap structure overhead before setting the final
   memory target. This must include both the upstream `luna_pinyin` benchmark
   surface and the product `jyut6ping3` schema surface.
2. Evolve the M34 `TableLookup` seam away from heap `&[Candidate]` slices into
   a candidate-view contract that can rank and materialize selected rows without
   retaining every dictionary row as a `Candidate`.
3. Build a queryable table payload reader that preserves candidate text,
   comments, code, order, quality, stems/encoder data, correction/tolerance
   payloads, and TypeDuck lookup records.
4. Integrate prism spelling graph lookup with table payload queries after table
   payload parity passes; for spelling-algebra schemas, this is required for
   the memory win because re-expanding aliases into heap storage recreates the
   blow-up.
5. Enable compact storage first for safe upstream `luna_pinyin` rows. A
   compact-active schema must not build or retain heap `entries_by_code`;
   unsupported schemas or TypeDuck-profile behavior stay on heap fallback.
6. Only then consider mmap/borrowed storage, with Windows file lifetime and
   rebuild behavior covered.

M35 should also attribute cross-engine harness overhead before using the
`198x`/`348x` ratios as public typing-latency headlines. Native engine-only and
full-ABI rows are the clearer engine movement signals unless the harness proves
otherwise.

When reporting M35 memory, distinguish dictionary-specific delta from whole
process peak. The order-of-magnitude target applies to the dictionary heap
delta. A correct owned compact-storage result may still land at roughly
`2-3x` librime peak until native mmap/demand paging is proven.

Browser startup and public delivery improvements remain M31 work, not M34
engine-performance evidence.
