# M34 Lazy Candidate Pipeline And Queryable Table+Prism Performance Plan

> **Status:** Complete · **Milestone:** M34 (lazy candidate pipeline and queryable table+prism lookup performance) · **Closed:** 2026-06-23 · **Type:** archived execution record
>
> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Close the remaining native `luna_pinyin` performance gap by adopting the two useful parts of librime's data path in idiomatic Rust: bounded/lazy candidate production for typing latency, and compact queryable table+prism storage for memory and cold-start cost. Do not copy librime's global C++ component architecture, plugin ABI, or build model.

**Architecture:** M33 showed that lazy reverse lookup and build-once dictionary translator caching improve startup/session economics but do not fix the classic typing path. The remaining gap has two different owners:

- **Lever A - typing latency:** Yune currently asks translators for `Vec<Candidate>`, materializes and clones all matching candidates for short prefixes, then sorts and filters the whole list before menu paging. Librime's fast path is lazy and chunked; it does not produce thousands of candidates when the frontend needs one page.
- **Lever B - memory and cold start:** Yune still builds owned heap dictionary structures and indexes. Librime mmaps position-independent table/prism data and pages only what is touched. Yune should learn that storage shape after the query semantics are proven.

M34 must profile first, then pursue both levers with separate evidence. Mmap is not the first task and is not a typing-latency fix by itself.
Do not count query-time spelling-algebra expansion as an expected `ni`/`hao` win unless the attribution proves it: current `luna_pinyin` builds spelling algebra during translator construction, and correction/dynamic lookup are not the known owner for those rows.
Also do not truncate a code-ordered prefix scan early and call it bounded: under the current heap map, enumeration may need to remain complete to preserve the global quality page. The first safe Lever A win is complete cheap enumeration plus bounded expensive materialization.

The local `LibreService/my_rime` clone is a useful reference point, but not a behavior oracle and not an implementation template. It is stock librime compiled to WASM, so its smooth typing mostly demonstrates the same engine data-path lesson M34 is already pursuing: page-bounded lazy candidate iteration over compiled table/prism data. Its fast browser startup also depends on delivery mechanics that belong to M31: selected-schema asset fetch, content-addressed cache, PWA/CDN caching, and worker isolation. M34 may record those findings for attribution, but must not absorb web deployment/cache work.

**Tech Stack:** Rust (`yune-core`, `yune-rime-api`), `Translator`/`Engine` candidate pipeline, compiled `.table.bin` and `.prism.bin` parsers, `DartsDoubleArray`, `StaticTableTranslator`, `frontend_baselines`, `benchmark-yune-vs-librime`, upstream `luna_pinyin` fixtures, TypeDuck `jyut6ping3` profile fixtures, and TypeDuck-Web runtime gates only when browser-visible behavior changes.

---

## Closeout - 2026-06-23

M34 closed as a bounded lazy-candidate-pipeline and lookup-abstraction
milestone. It did not implement M31 public-demo delivery/cache/Cloudflare work,
did not change runtime/browser files, did not widen the public `RimeApi`, did
not change `RimeCandidate`, and did not alter TypeDuck-profile ABI isolation.
Opened: 2026-06-23 at user request.

Implemented:

- Fresh fair M33-surface baseline and after-runs under
  `docs/reports/evidence/m34-queryable-table-prism/`.
- Internal `CandidateRequest` / `TranslationResult` and
  `Translator::translate_with_context_and_request(...)`, with eager translation
  retained as the compatibility fallback.
- Safe bounded `StaticTableTranslator` materialization for short first-page
  `luna_pinyin` typing: prefix enumeration remains complete under the current
  code-ordered heap map, while expensive candidate materialization is bounded
  with stable tie preservation.
- Engine lazy candidate windows for safe short `luna_pinyin` input, with
  full-list expansion for out-of-window actions and candidate-list iterator APIs.
- `Snapshot::candidate_list_complete` so `RimeGetContext` can report
  `is_last_page` honestly without forcing full materialization on every
  first-page context read.
- Internal heap-backed `TableLookup` abstraction as the precondition for future
  storage swaps.
- Native `luna_pinyin` `hao` benchmark rows for future comparison.

Measured result:

- Native `per_key_real_luna_pinyin_ni_full_abi`: `1,760.250us` ->
  `1,132.950us` (`-35.6%`).
- Native `per_key_real_luna_pinyin_zhongguo_full_abi`: `12,697.600us` ->
  `12,119.013us` (`-4.6%`).
- TypeDuck full-ABI watch rows stayed within the accepted guard:
  `hai` `+5.7%`, `jigaajiusihaa` `-6.0%`, correction-on
  `jigaajiusihaa` `-5.5%`.
- Fair cross-engine after-run remains far behind librime:
  `hao` `348.1x`, `ni` `198.4x`, `zhongguo` `26.0x` slower; peak working
  set remains about `8.1x` librime.

Deferred:

- Queryable compiled `.table.bin` runtime storage.
- Prism/table candidate lookup integration.
- Storage hot-path swap away from retained `entries_by_code`.
- Mmap/borrowed storage.
- Browser startup/typing claims.

Gate outcomes:

| Gate | Outcome |
| --- | --- |
| `M34-PERF-01` | Complete: fresh native and fair cross-engine M33-surface baselines captured. |
| `M34-PERF-02` | Complete: attribution recorded with fresh full-ABI/engine rows plus a temporary diagnostic artifact for `ni` and `hao`; no diagnostic instrumentation is retained in production code. |
| `M34-PERF-03` | Complete: full-list readers audited before changing retained table state. |
| `M34-PERF-04` | Complete: internal bounded request/result contract added without public ABI change. |
| `M34-PERF-05` | Complete: safe short `luna_pinyin` first-page refresh uses bounded materialization and lazy full-list expansion. |
| `M34-PERF-06` | Complete: heap-backed `TableLookup` abstraction added and tested. |
| `M34-PERF-07` | Closed by no-go: compiled `.table.bin` query storage was not implemented because current compiled readers still materialize owned dictionaries and payload parity work remains. |
| `M34-PERF-08` | Closed by no-go: prism/table candidate integration was not implemented because the prism does not carry candidate payload bytes and needs a table-backed payload query path first. |
| `M34-PERF-09` | Complete: upstream and TypeDuck parity gates stayed byte-identical. |
| `M34-PERF-10` | Complete with caveat: native first-page `luna_pinyin` full-ABI rows improved, but Yune remains far behind librime on fair cross-engine typing rows. |
| `M34-PERF-11` | Closed by no-go: mmap/borrowed storage was not attempted because compact queryable table/prism storage did not land. |
| `M34-PERF-12` | Complete: Rust, focused parity, workspace, benchmark, report, and diff gates run; no runtime/browser gates were required. |
| `M34-PERF-13` | Complete: `my_rime` reference split recorded with delivery/cache work routed to M31. |

The public report is safe to show only with the caveats above: M34 is a narrow
native first-page win and an architecture step toward future storage work, not a
claim that Yune is faster than librime or that browser delivery improved.

## Status

Complete. M34 is the deep follow-up to the M33 finding that native
`luna_pinyin` still trails librime badly on per-key rows, cold start, and
memory. It closed the bounded candidate-pipeline subset and deferred storage
representation work behind explicit evidence gates.

## Scope

In scope:

- Fresh profiling that splits `ni`/`hao` time across lookup, completion range scan, abbreviation-probe allocation, candidate clone/comment formatting, engine global sort versus bounded top-K selection, filter pipeline, rankers, and context update.
- A bounded or lazy translator/engine candidate contract so the hot path can enumerate needed candidates cheaply but materialize only the first page or a bounded surplus instead of cloning/formatting the entire result set.
- A safe migration path for filters, rankers, userdb merge, AI staging, inspector/debug output, and menu paging under bounded candidate production.
- A lookup abstraction that can answer exact, prefix/completion, sentence-segmentation, and correction-related queries without exposing `entries_by_code` as the only storage shape.
- A compiled table query path that preserves candidate text, comment bytes, order, quality, code, and TypeDuck profile metadata.
- A prism integration path that uses spelling/syllable graph data to discover lookup codes while the table path supplies candidate payloads.
- Fresh M33-derived baselines and after-runs for cold startup, warm schema/session reuse, memory, and per-key lookup.
- A local `my_rime`/librime-WASM reference analysis that separates engine data-path behavior from browser delivery/cache behavior before making any public performance claim.
- Byte-identical behavior for upstream `luna_pinyin` fixtures and TypeDuck `jyut6ping3` fixtures.
- Conditional mmap/borrowed-storage work only after the table+prism query abstraction is proven.

Out of scope:

- Widening default `RimeApi`, `RimeCandidate`, or TypeDuck-profile ABI.
- Recreating librime's C++ plugin ABI, global service locator, or module lifecycle.
- M31 Cloudflare deployment, public-demo UI, or OpenCC output-standard breadth.
- M31 web delivery work: active-schema-only schema fetch, IndexedDB/Cache Storage asset reuse, PWA/service-worker setup, Cloudflare cache configuration, CDN URL rewriting, and browser payload pruning.
- M32 AI-native product UX or remote-provider work.
- TypeDuck-Windows TSF shell/input-delivery work.
- Claiming browser startup or browser typing wins unless a browser/WASM path changes and has real browser evidence.
- Changing candidate ranking semantics to make performance easier. Ranking/order changes require oracle-backed behavior evidence.

## Preconditions

- M33 is complete and the public performance/root-cause reports are treated as the baseline.
- The worktree is clean or unrelated active changes are identified before editing.
- If M31 is active in parallel, its engine OpenCC slice is paused or explicitly split off; M31 Cloudflare/devops/UI-only work may proceed in a separate worktree.
- Any public performance claim remains scoped to the measured surface. Native gains are not browser gains unless the browser harness proves them.

## Acceptance Gates

- `M34-PERF-01`: Fresh before-runs reproduce the M33 native comparison surface: cold startup, warm schema/session reuse, peak and ready memory, `ni`, `hao`, `zhongguo`, and at least two TypeDuck `jyut6ping3` rows such as `hai` and `jigaajiusihaa`.
- `M34-PERF-02`: Hot-path attribution splits `ni`/`hao` time across index lookup, completion range scan, abbreviation-probe allocation, candidate clone/comment formatting, engine global sort versus bounded top-K selection, filter pipeline, rankers, and context/menu update before any rewrite. The milestone chooses lever order from this data and does not claim a query-time spelling-algebra win without proof.
- `M34-PERF-03`: Every steady-state reader of `StaticTableTranslator.entries_by_code`, `Translator::translate`, and `Engine::refresh_candidates` is audited and classified before replacement: exact lookup, prefix/range completion, sentence segmentation, dynamic correction, full-list filter/ranker requirement, debug/inspector, schema switching, and tests.
- `M34-PERF-04`: A bounded/lazy candidate contract is introduced without changing the public C ABI. Under the current code-ordered heap map, the contract separates complete cheap enumeration from bounded expensive materialization; early-stop enumeration is allowed only when a later weight/order-aware query path proves byte-identical first pages. Existing eager `translate` behavior remains available as a compatibility wrapper until all callers migrate.
- `M34-PERF-05`: Engine refresh no longer sorts, filters, and stores an unbounded fully materialized result set for ordinary first-page typing when the active filters/rankers can operate on a bounded candidate stream. It uses top-K partial selection (`select_nth_unstable`, a bounded heap, or an equivalent strategy) only with a stable tie-break that reproduces current `sort_by` output for equal-quality candidates. Any full-list fallback is measured and documented.
- `M34-PERF-06`: A table lookup abstraction reproduces the current heap map byte-for-byte on source-backed and compiled-backed fixtures before compact storage is enabled.
- `M34-PERF-07`: The compiled `.table.bin` query path preserves candidate text, comment bytes, quality/order, code, stems/encoder data, correction/tolerance payloads, and TypeDuck lookup records. Unsupported table sections fail with a documented no-go rather than silently falling back to wrong output.
- `M34-PERF-08`: The prism+table integration can use spelling/prism data to produce byte-identical upstream `luna_pinyin` exact, completion, and sentence rows without deriving expected output from Yune itself.
- `M34-PERF-09`: TypeDuck `jyut6ping3` profile behavior remains byte-identical for rich comments, long composition, partial selection, default-confirm recomposition, and userdb learning invariants.
- `M34-PERF-10`: Native per-key lookup improves materially on `luna_pinyin`; the stretch target is low-hundreds-of-microseconds for `ni`/`hao`, with an explicit no-go if profiling proves a remaining full-list owner cannot be safely bounded. TypeDuck rows must not regress by more than 10% unless the user explicitly accepts the tradeoff.
- `M34-PERF-11`: Memory and cold-start claims are measured separately from per-key latency. If mmap/borrowed storage lands, it must include Windows lifetime/file-locking coverage; if it is deferred, the closeout must name the remaining owner.
- `M34-PERF-12`: Full gates pass: Rust fmt/clippy/tests, focused upstream/TypeDuck parity, native benchmark after-runs, public report updates, and `git diff --check`. Runtime/browser gates run only if runtime or browser-visible files change.
- `M34-PERF-13`: `my_rime` reference findings are recorded without overclaiming. The report separates librime's engine data path from `my_rime`'s web delivery/cache mechanics, and any delivery follow-up is routed to M31 rather than implemented in M34.

## File Responsibilities

- `crates/yune-core/src/translator/mod.rs`: owns current `Translator` contract, `StaticTableTranslator`, completion range scans, candidate cloning/comment formatting, and any bounded/lazy translation API.
- `crates/yune-core/src/engine.rs`: owns `refresh_candidates`, full-list sort/filter/ranker behavior, userdb merge, AI staging merge, context candidate storage, and menu paging.
- `crates/yune-core/src/dictionary/compiled_table.rs`: owns compiled table parsing and any new queryable table payload reader.
- `crates/yune-core/src/dictionary/compiled_prism.rs`: owns prism parsing and spelling-map/double-array access.
- `crates/yune-core/src/dictionary/double_array.rs`: owns the Darts double-array helper used by prism lookup.
- `crates/yune-core/src/dictionary/mod.rs`: owns public dictionary data structures and any internal lookup trait exports.
- `crates/yune-rime-api/src/schema_install.rs`: owns schema dictionary translator construction, cache integration, and source/compiled artifact selection.
- `crates/yune-rime-api/benches/frontend_baselines.rs`: owns native benchmark rows and before/after evidence.
- `crates/yune-core/tests/upstream_luna_pinyin_parity.rs`: owns upstream behavior guards.
- `crates/yune-core/tests/cantonese_parity.rs` and `crates/yune-rime-api/tests/typeduck_web.rs`: own TypeDuck profile correctness and browser-runtime contract guards.
- `C:\Users\laubonghaudoi\Documents\GitHub\my_rime\`: optional local reference checkout for analyzing librime-WASM delivery and current-page candidate behavior. Do not copy source code from this repo into Yune.
- `docs/reports/evidence/m34-queryable-table-prism/`: owns M34 evidence files.
- `docs/reports/yune-vs-librime-performance.md`, `docs/reports/yune-vs-librime-root-cause-analysis.md`, `docs/roadmap.md`, and `docs/requirements.md`: own public closeout and status.

---

## Task 0 - Baseline, Hot-Path Attribution, And Reader Audit

**Files:**

- Read: `docs/reports/evidence/m33-2026-06-23/README.md`
- Read: `docs/reports/yune-vs-librime-performance.md`
- Read: `crates/yune-core/src/translator/mod.rs`
- Read: `crates/yune-core/src/engine.rs`
- Read reference only: `C:\Users\laubonghaudoi\Documents\GitHub\my_rime\src\worker.ts`
- Read reference only: `C:\Users\laubonghaudoi\Documents\GitHub\my_rime\wasm\api.cpp`
- Read reference only: `C:\Users\laubonghaudoi\Documents\GitHub\my_rime\scripts\install_schemas.ts`
- Read reference only: `C:\Users\laubonghaudoi\Documents\GitHub\my_rime\vite.config.ts`
- Create: `docs/reports/evidence/m34-queryable-table-prism/baseline.md`
- Create: `docs/reports/evidence/m34-queryable-table-prism/hot-path-attribution.md`
- Create: `docs/reports/evidence/m34-queryable-table-prism/reader-audit.md`
- Create: `docs/reports/evidence/m34-queryable-table-prism/my-rime-reference.md`

- [ ] **Step 0.1: Confirm repo state and current baselines**

Run:

```powershell
git fetch origin --prune
git status --short --branch
git log --oneline -5 --decorate
```

Expected:

- Worktree is clean or unrelated active changes are listed before editing.
- `origin/main` contains the completed M33 closeout.

- [ ] **Step 0.2: Re-run the native comparison surface**

Run the existing M33 benchmark/report commands that produced:

- cold startup/runtime-ready
- warm schema/session reuse
- peak and ready memory
- `ni`, `hao`, `zhongguo`
- TypeDuck `hai` and `jigaajiusihaa` rows

Expected:

- `baseline.md` records command lines, machine context, source commit, and before values.
- If values drift materially from M33, the plan uses the fresh values as the comparison baseline and says so.

- [ ] **Step 0.3: Attribute `ni` and `hao` before rewriting**

Add temporary or gated native instrumentation that splits per-key time across at least:

- expanded lookup spec generation
- exact lookup index access
- completion range scan
- abbreviation-probe tuple/allocation cost
- candidate clone/comment formatting
- candidate list length before engine sort
- engine global sort versus top-K/partial-sort feasibility
- filter pipeline
- userdb merge
- ranker/AI merge
- context/menu update

Expected:

- `hot-path-attribution.md` records median/p95 attribution for `ni` and `hao`.
- The report states which is larger: storage/index lookup cost, full-result materialization, or engine full-list processing.
- If full-result materialization and engine full-list processing dominate, Task 1 and Task 2 must run before compact storage work claims per-key wins.
- The report explicitly states whether query-time spelling-algebra expansion is present for the measured rows; if it is not measured, M34 must not cite it as a per-key win.

- [ ] **Step 0.4: Audit every full-list reader and caller contract**

Run:

```powershell
rg -n "entries_by_code|entries_from_entries_by_code|entries_by_code_from_entries|BTreeMap<String, Vec<Candidate>>|fn translate\(|refresh_candidates|sort_by|apply_with_context|try_rerank|prediction_never_first|sentence_over_completion|prefix_fallback|assign_ordered_candidate_qualities" crates/yune-core/src crates/yune-rime-api/src
```

Expected:

- `reader-audit.md` classifies every reader as exact lookup, prefix/range lookup, sentence segmentation, correction, full-list filter/ranker, debug/inspector, construction-only, or test-only.
- `reader-audit.md` explicitly classifies the known global-behavior owners before implementation: `prediction_never_first`, `sentence_over_completion`, `prefix_fallback`, and `assign_ordered_candidate_qualities`.
- Each active filter/ranker is classified as bounded-stream-safe, needs small surplus, or genuinely full-list-only.
- No replacement starts until every reader has a proposed lookup or candidate-stream method.

- [ ] **Step 0.5: Record the `my_rime` reference split**

Inspect the local `my_rime` checkout and record the engine-versus-delivery split in `my-rime-reference.md`.

Required findings to verify:

- `my_rime` uses stock librime in WASM; it is a reference for architecture and measurement, not a Yune behavior oracle.
- Its worker fetches selected-schema prebuilt assets on demand rather than all schema assets up front.
- Its bridge returns only the current librime menu page, so smooth typing reflects librime's lazy/page-bounded data path.
- Its `LazyCache`, PWA/CDN resources, and prebuilt schema packages are M31 delivery lessons, not M34 implementation tasks.
- Any public Yune-vs-`my_rime` statement separates engine latency, browser delivery, worker/main-thread behavior, and warm-cache effects.

## Task 1 - Bounded/Lazy Candidate Production Contract

**Files:**

- Modify: `crates/yune-core/src/translator/mod.rs`
- Modify only if needed: `crates/yune-core/src/lib.rs`
- Add tests in the owning module
- Create evidence: `docs/reports/evidence/m34-queryable-table-prism/bounded-translation-contract.md`

- [ ] **Step 1.1: Add a bounded internal translator request**

Introduce an internal request shape that lets the engine ask for a bounded first page plus surplus without changing the C ABI. Under the current code-ordered map, the request bounds expensive materialization, not necessarily prefix enumeration. A possible shape:

```rust
struct CandidateRequest {
    page_size: usize,
    surplus: usize,
    include_completions: bool,
    include_debug_full_count: bool,
}
```

The exact shape can differ, but it must let ordinary typing avoid producing all prefix completions for `ni` and `hao`.

- [ ] **Step 1.2: Keep the eager wrapper during migration**

Expected:

- Existing `Translator::translate(&self, input) -> Vec<Candidate>` remains available as a compatibility wrapper or fallback while callers migrate.
- The bounded path is used only by tests and the engine path migrated in Task 2.
- Public `Candidate`, `RimeApi`, and `RimeCandidate` remain unchanged.

- [ ] **Step 1.3: Bound completion production without changing order**

Expected:

- Exact candidates remain first according to current behavior.
- Completion enumeration remains parity-complete for current code-ordered prefix ranges; it may collect lightweight references/keys with quality and emission-order metadata, but it must not clone/format every candidate.
- Expensive materialization (`Candidate` clone, abbreviation probe, comment formatting, display payload construction) happens only after selecting the stable top-K plus documented surplus.
- Early-stop enumeration is allowed only after Task 3-5 produce a weight/order-aware query path that proves the first page is byte-identical to the eager path.
- If any current filter/ranker makes bounded production unsafe, document the specific blocker and use a measured fallback.

## Task 2 - Bound Engine Refresh, Sort, And Filter Work

**Files:**

- Modify: `crates/yune-core/src/engine.rs`
- Modify: `crates/yune-core/src/translator/mod.rs`
- Modify: `crates/yune-rime-api/benches/frontend_baselines.rs`
- Add/modify focused tests for paging, filtering, and ranking
- Create evidence: `docs/reports/evidence/m34-queryable-table-prism/bounded-engine-refresh.md`

- [ ] **Step 2.1: Make normal first-page refresh bounded**

Expected:

- For ordinary typing, `Engine::refresh_candidates` requests bounded materialization instead of collecting unbounded fully materialized translator output.
- The engine may consume a complete lightweight enumeration when that is required for global quality order, but it does not clone, format, store, or filter thousands of unseen candidates when only the first page is needed.
- Sorting uses top-K partial selection (`select_nth_unstable`, a bounded heap, or an equivalent approach) instead of full `sort_by` only when it also reproduces the current stable tie-break for equal-quality candidates; if stable/global ordering requires full sort, the evidence names that blocker.
- Menu paging either fetches more candidates on demand or falls back to the eager path with measured evidence.

- [ ] **Step 2.2: Keep filters/rankers honest**

Expected:

- Each filter/ranker from Task 0 is either bounded-safe, given a documented surplus, or routed to an eager fallback.
- `prediction_never_first`, `sentence_over_completion`, `prefix_fallback`, and `assign_ordered_candidate_qualities` each have an explicit bounded/materialization strategy or measured fallback.
- The inspector/debug view clearly marks when counts are bounded versus full-list counts.
- Userdb merge and AI staging remain deterministic and do not reorder classic candidates incorrectly.

- [ ] **Step 2.3: Measure per-key latency before storage rewrite**

Expected:

- `ni` and `hao` after-runs are recorded immediately after bounded production.
- If the gap remains large, the evidence names the remaining owner before storage work starts.

## Task 3 - Internal Table Lookup Abstraction

**Files:**

- Modify/create: `crates/yune-core/src/dictionary/query_table.rs` or equivalent internal module
- Modify: `crates/yune-core/src/dictionary/mod.rs`
- Modify: `crates/yune-core/src/translator/mod.rs`
- Add tests in the owning module
- Create evidence: `docs/reports/evidence/m34-queryable-table-prism/table-lookup-abstraction.md`

- [ ] **Step 3.1: Add an internal lookup contract**

Introduce a private/internal abstraction shaped around behavior, not storage. A starting shape is:

```rust
trait TableLookup {
    fn exact_candidates(&self, code: &str) -> &[Candidate];
    fn has_code(&self, code: &str) -> bool;
    fn prefix_codes(&self, prefix: &str) -> Box<dyn Iterator<Item = &str> + '_>;
    fn prefix_candidates(&self, prefix: &str) -> Box<dyn Iterator<Item = (&str, &[Candidate])> + '_>;
}
```

The exact signature can differ, but it must support every reader identified in Task 0 without exposing the heap map as the only possible implementation.

- [ ] **Step 3.2: Implement the contract for the current heap map first**

Expected:

- Current behavior is unchanged.
- Tests compare exact/prefix/sentence lookup output before and after the abstraction.
- No compiled storage rewrite is mixed into this step.

- [ ] **Step 3.3: Preserve current ordering and borrowing rules**

Expected:

- Candidate order remains stable.
- Cloning/allocation behavior is measured but not optimized prematurely.
- Public `Candidate` remains unchanged.

## Task 4 - Query Compiled Table Payloads

**Files:**

- Modify: `crates/yune-core/src/dictionary/compiled_table.rs`
- Modify: `crates/yune-core/src/dictionary/mod.rs`
- Add tests under `crates/yune-core/src/dictionary/`
- Create evidence: `docs/reports/evidence/m34-queryable-table-prism/compiled-table-query.md`

- [ ] **Step 4.1: Inspect compiled `.table.bin` sections before choosing storage**

Record whether the current upstream/fork assets contain:

- multi-level phrase indexes
- string-table/marisa sections
- Yune advanced payloads
- lookup records, stems, encoder, corrections, and tolerance payloads

Expected:

- Unsupported sections are not ignored.
- If required candidate payload cannot be read lazily from current compiled assets, stop and document the exact missing parser/storage work.

- [ ] **Step 4.2: Build a queryable table reader**

Expected:

- Exact and prefix candidate queries can be answered without building the final expanded `BTreeMap<String, Vec<Candidate>>` for the steady-state hot path.
- Candidate text/comment/order/quality are byte-identical to the current path for representative upstream and TypeDuck fixtures.
- If source-YAML parsing remains necessary for some schemas, the decision is explicit and benchmarked.

- [ ] **Step 4.3: Keep error paths honest**

Expected:

- Unsupported compiled sections return structured no-go errors.
- No fallback silently changes candidate order, comments, or filtering.

## Task 5 - Integrate Prism Spelling Graph With Table Queries

**Files:**

- Modify: `crates/yune-core/src/dictionary/compiled_prism.rs`
- Modify: `crates/yune-core/src/dictionary/double_array.rs` only if needed
- Modify: `crates/yune-core/src/translator/mod.rs`
- Add/modify: `crates/yune-core/tests/upstream_luna_pinyin_parity.rs`
- Add/modify: `crates/yune-core/tests/cantonese_parity.rs`
- Create evidence: `docs/reports/evidence/m34-queryable-table-prism/prism-table-integration.md`

- [ ] **Step 5.1: Use prism to discover spellings/codes, table to fetch candidates**

Expected:

- The implementation does not expect prism to carry candidate payloads.
- The table path remains the source of candidate text/comment/order.
- Spelling-algebra expansions are produced lazily or compactly enough to avoid rebuilding the full expanded heap map.

- [ ] **Step 5.2: Guard upstream `luna_pinyin` behavior**

Expected:

- Focused tests cover `ni`, `hao`, `zhongguo`, completion/prefix behavior, and sentence composition rows already owned by upstream fixtures.
- Expected output comes from pinned fixtures/oracle captures, not from the new lookup implementation.

- [ ] **Step 5.3: Guard TypeDuck profile behavior**

Expected:

- TypeDuck rich comment bytes remain unchanged.
- `caksijathaacoenggeoizi` partial/default-confirm behavior and userdb learning invariants remain unchanged.
- Dynamic correction and profile-specific ranking controls do not regress.

## Task 6 - Swap The Storage Hot Path Behind Tests

**Files:**

- Modify: `crates/yune-core/src/translator/mod.rs`
- Modify: `crates/yune-rime-api/src/schema_install.rs`
- Modify: `crates/yune-rime-api/benches/frontend_baselines.rs`
- Create evidence: `docs/reports/evidence/m34-queryable-table-prism/storage-hot-path-swap.md`

- [ ] **Step 6.1: Enable the new lookup path privately**

Expected:

- The public ABI is unchanged.
- The old heap-backed implementation can remain as a fallback while parity is proven.
- Tests can force old versus new lookup paths for A/B comparison if practical.

- [ ] **Step 6.2: Re-run parity and benchmark after each owner is migrated**

Expected:

- Any performance claim is tied to a specific migrated owner.
- If a reader still needs the heap map, document why and leave it isolated.

- [ ] **Step 6.3: Decide whether to delete, shrink, or retain `entries_by_code`**

Expected:

- If deletion is not safe, the closeout says which behavior still needs it.
- If retained, it must no longer dominate cold startup or per-key lookup for the measured rows.

## Task 7 - Conditional Mmap/Borrowed Storage Spike

**Files:**

- Modify only after Task 6: compiled table/prism storage modules
- Add tests for Windows file lifetime if mmap lands
- Create evidence: `docs/reports/evidence/m34-queryable-table-prism/mmap-spike.md`

- [ ] **Step 7.1: Measure whether mmap is now the right lever**

Expected:

- Mmap proceeds only if Task 6 shows the query path can borrow/index compact storage.
- If startup/memory remain dominated by other owners, defer mmap with evidence.
- Mmap is never claimed as the fix for per-key latency unless Task 0/2/6 evidence proves the per-key owner actually moved.
- Browser/WASM demand paging is not claimed from mmap: Emscripten-backed files live in linear memory/MEMFS, so browser startup and payload wins belong to M31 delivery/cache work unless browser evidence proves otherwise.
- If the native mmap implementation needs `unsafe`, the worker must not relax the workspace lint globally. Add a narrow crate/module boundary or documented lint exception that preserves the default `unsafe_code = "forbid"` policy everywhere else.

- [ ] **Step 7.2: If mmap lands, cover Windows behavior**

Expected:

- File lifetime and deploy/rebuild behavior are tested on Windows.
- Schema rebuild or package replacement cannot leave locked files in normal use.
- No runtime resource identifier becomes an arbitrary filesystem path.

## Task 8 - Closeout, Reports, And Roadmap

**Files:**

- Modify: `docs/reports/yune-vs-librime-performance.md`
- Modify: `docs/reports/yune-vs-librime-root-cause-analysis.md`
- Modify: `docs/roadmap.md`
- Modify: `docs/requirements.md`
- Move on completion: `docs/plans/m34-plan-queryable-table-prism-lookup-performance.md` to `docs/plans/completed/`
- Create: `docs/reports/evidence/m34-queryable-table-prism/task-8-gates.md`

- [ ] **Step 8.1: Run final gates**

Minimum:

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p yune-core --test upstream_luna_pinyin_parity
cargo test -p yune-core --test cantonese_parity
cargo test -p yune-rime-api --test typeduck_web
cargo test --workspace
cargo bench -p yune-rime-api --bench frontend_baselines
git diff --check
```

Add runtime/browser gates only if runtime/browser files change:

```powershell
npm.cmd --prefix packages/yune-typeduck-runtime test
npm.cmd --prefix packages/yune-typeduck-runtime run build
```

- [ ] **Step 8.2: Update public claims conservatively**

Expected:

- Separate native startup, native per-key, memory, browser startup, and browser typing.
- Separate Lever A claims from Lever B claims.
- Do not generate a public chart unless it shows fair cold/warm/per-key/memory caveats.
- If M34 does not beat librime on per-key rows, say so directly.

- [ ] **Step 8.3: Archive the plan only when gates and docs are complete**

Expected:

- Roadmap and requirements reflect final status.
- Evidence paths are committed.
- The active plan moves to `docs/plans/completed/` only at closeout.

---

## Parallelization Guidance With M31

M34 and M31 can only be partially parallelized.

Safe to parallelize in separate worktrees:

- M31 Cloudflare configuration, local preview/deployment scaffolding, asset-manifest work, provenance text, and public-demo copy that consumes the already completed M33 report.
- M31 UI-only work that does not change the engine/runtime contract.

Do not parallelize:

- M34 with M31 Task 2 engine OpenCC work. Both can touch `crates/yune-core/src/filter/mod.rs`, `crates/yune-rime-api/src/schema_install.rs`, runtime option plumbing, and schema-install/deploy assumptions.
- M34 with any other Yune engine/schema-load representation rewrite.

If both milestones are staffed at once, the recommended split is:

1. Let M34 own engine lookup/schema-load representation and candidate-pipeline laziness.
2. Let M31 proceed only with devops/provenance/deploy scaffolding or choose `opencc_scope = current_simplification_only`.
3. Queue any M31 engine OpenCC breadth after M34 lands and the worktree is synced.

## Execution Handoff

Start with Task 0 and do not jump directly to mmap. The first hard question is where the `ni`/`hao` milliseconds go. If full-result materialization and engine full-list sort/filter dominate, run the bounded/lazy candidate pipeline work first and measure it before compact storage work claims per-key wins. Do not implement early-stop prefix enumeration on the current code-ordered heap map; preserve complete cheap enumeration and bound only expensive materialization unless a later weight/order-aware table path proves byte-identical first pages. If storage/index lookup unexpectedly dominates, document that evidence and prioritize the table+prism query path. In all cases, keep upstream and TypeDuck output byte-identical and keep mmap as a conditional storage optimization, not a standalone latency fix.
