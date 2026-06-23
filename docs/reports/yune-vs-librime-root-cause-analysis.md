# Why Yune is slower than librime - root-cause analysis

Date: 2026-06-23

Companion report: [`yune-vs-librime-performance.md`](./yune-vs-librime-performance.md).
M33 changed the diagnosis. The old headline gap was partly unfair because Yune
loaded `stroke` reverse-lookup assets during `luna_pinyin` schema select while
librime did not. M33 fixed that fairness issue and added a build-once
translator cache. Warm re-select/session rows are now in the same order of
magnitude as librime, but cold launch, peak memory, and per-key lookup remain
far behind.

## Current verdict after M33

It is not "Rust is slow" and it is not that Yune's AI-native direction prevents
classic performance. The remaining gap is a concrete candidate-pipeline and
representation gap:

1. **Resolved in M33: reverse-lookup load asymmetry.** Yune now defers the
   `stroke` reverse dictionary until the first reverse-lookup query, matching
   librime for no-reverse `luna_pinyin` typing.
2. **Resolved for reselect/session in M33: rebuild-per-select.** Yune now shares
   immutable built dictionary translators process-wide using schema and asset
   signatures, so repeated session select no longer reloads and re-expands the
   same schema.
3. **Still open: eager full-result candidate production.** Yune's translator API
   still returns `Vec<Candidate>`, and `Engine::refresh_candidates` sorts and
   filters the whole vector before menu paging. Short ambiguous inputs such as
   `ni` and `hao` can therefore pay for many unseen completion candidates, plus
   per-candidate abbreviation probes and `Candidate` clone/comment formatting
   work that a borrowed or bounded path should avoid.
4. **Still open: eager table-backed spelling algebra.** The live lookup structure
   is still a materialized `BTreeMap<String, Vec<Candidate>>` built from table
   payloads and spelling-algebra expansion. librime's hot path walks mmap-backed
   compiled structures and applies prism spelling data lazily. This is a
   storage/build-time owner unless fresh attribution proves query-time spelling
   algebra is present for the measured `luna_pinyin` rows.
5. **Still open: no mmap-backed runtime lookup.** Yune still reads/parses
   compiled artifacts into owned heap structures. Mmap was deferred because the
   M33 stop gate showed startup/session already improved dramatically, while the
   remaining product-relevant gaps must be split into per-key candidate-pipeline
   work and storage representation work.

## What changed in M33

Before M33, the public comparison showed roughly `96x` startup/session gaps on
the fresh M33 baseline:

| Row | Yune before | librime before | Ratio |
| --- | ---: | ---: | ---: |
| Startup/runtime-ready | `2,881,852.7 us` | `29,788.8 us` | `96.7x` |
| Session create/select/destroy | `2,985,364.0 us` | `30,998.8 us` | `96.3x` |

After lazy reverse lookup and the built-translator cache, warm cache-hit rows
look much better:

| Row | Yune after | librime after | Ratio |
| --- | ---: | ---: | ---: |
| Warm startup/runtime-ready | `47,556.3 us` | `26,964.8 us` | `1.8x` |
| Session create/select/destroy | `47,813.7 us` | `25,765.9 us` | `1.9x` |

This makes the old "Yune startup is about 100x slower" public claim stale and
unsafe. The fair post-M33 claim is narrower: warm re-select/session are close
enough for public demo copy, but a fresh process still pays a cold
`909,375.4 us` schema build versus librime's `80,260.8 us`, and Yune still peaks
at `182,775,808 bytes` versus librime's `22,519,808 bytes`.

## Remaining hot path

Per-key rows did not improve:

| Input | Yune after | librime after | Ratio |
| --- | ---: | ---: | ---: |
| `ni` | `6,064.5 us` | `28.5 us` | `212.8x` |
| `hao` | `12,463.4 us` | `34.5 us` | `361.3x` |
| `zhongguo` | `37,572.3 us` | `1,479.8 us` | `25.4x` |

The current Yune lookup path still depends on eager candidate production and the
table-backed translator map:

- `Translator::translate` returns a `Vec<Candidate>`, so the caller cannot ask
  for only the current page.
- Exact lookup reads `entries_by_code.get(...)`.
- Completion walks `entries_by_code.range(...)`; for short prefixes this can
  scan and clone far more candidates than the menu will display.
- Candidate abbreviation checks and lookup conversion can allocate or clone per
  candidate before the engine has decided which page will be shown.
- Prefix fallback and sentence segmentation re-read `entries_by_code`.
- TypeDuck correction stress can scan `entries_by_code.keys()`.
- `Engine::refresh_candidates` sorts and filters the whole candidate vector
  before menu paging; top-K partial selection or bounded heaps have not yet
  replaced the full global sort.
- Candidate text, comments, quality, ordering, recomposition, and sentence data
  live in table-derived `Candidate` payloads, not in the prism.

M33 added the focused fixture test
`upstream_luna_pinyin_prism_fixture_does_not_contain_candidate_payloads` to lock
the lazy-prism spike result. The upstream prism can map a spelling such as `ni`
to syllable descriptors, but it does not contain candidate text bytes such as
`U+4F60`, `U+597D`, or `U+4E2D U+56FD`. That is expected: librime uses the prism
as an index into the table, not as the candidate payload store. A byte-identical
storage rewrite therefore needs a broader queryable table+prism redesign, not a
prism-only walk. That is necessary for cold-start and memory work, but per-key
latency also needs bounded/lazy candidate production so the engine stops paying
for unseen candidates.

Until the storage path can iterate candidates in proven weight/order-compatible
order, Yune should not truncate the current code-ordered prefix scan early.
Doing so would risk dropping a high-quality completion that appears late in
lexicographic code order. The safe near-term typing win is to keep enumeration
cheap and complete, then bound the expensive tail: candidate cloning,
abbreviation checks, comment formatting, and final page materialization.

## Why librime remains faster per key

librime's classic path is close to the floor for a table IME:

- Deploy once into compact compiled assets.
- Memory-map the table/prism files at runtime.
- Walk trie/prism structures directly.
- Return candidates through a lazy, chunked iterator instead of materializing
  the full result set.
- Load reverse lookup only on first reverse lookup.
- Share schema/dictionary state across sessions.

Yune now has the last two bullets for the measured no-reverse workload, but not
the lazy candidate pipeline or the mmap-backed lookup model. That is why warm
re-select/session are close while cold launch, peak memory, and per-key typing
remain behind. The ratio pattern is consistent with this split: the gap is worst
for short ambiguous inputs (`ni`, `hao`) where eager full-result production is
most expensive, and smaller for more specific input (`zhongguo`).

## Deferred work

The next performance milestone should not start from generic "make startup
faster" work. It should start from hot-path attribution, then split the work into
two levers:

- Can Yune avoid full-result candidate materialization and global sorting for
  ordinary first-page typing while preserving exact candidate order, including
  stable equal-quality tie breaks, filters, userdb behavior, AI staging, paging,
  and inspector output?
- Can Yune build a byte-identical queryable table+prism path that lets spelling
  lookup find candidate text/comment/order without pre-expanded `entries_by_code`
  materialization?
- Can that design preserve TypeDuck `jyut6ping3` profile behavior, sentence
  segmentation, prefix fallback, correction scans, and candidate quality?
- Once that lookup representation exists, mmap can pay off because the hot path
  would actually walk borrowed compiled structures instead of copying them into
  heap maps.

Until that milestone is opened with evidence, M33 should stay closed as the
bounded fairness/cache win.
