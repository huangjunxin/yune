# M35 Compact Table+Prism Runtime Storage Performance Plan

> **Status:** Complete. **Milestone:** M35 (compact table+prism runtime storage). **Opened:** 2026-06-24. **Closed:** 2026-06-24. **Type:** engine-performance plan.
>
> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace Yune's heap-expanded dictionary hot path with a byte-identical compact queryable table+prism runtime path, then stop-gate or enable mmap/borrowed storage, so native `luna_pinyin` lookup, startup memory, and cold-start cost move materially closer to upstream librime without changing public ABI or TypeDuck profile behavior.

**Closeout:** M35 landed compact owned storage for safe upstream `luna_pinyin`, preserved TypeDuck heap fallback, and deferred mmap/borrowed storage by measurement no-go. Evidence lives under [`docs/reports/evidence/m35-compact-table-prism-storage/`](../../reports/evidence/m35-compact-table-prism-storage/). Native `luna_pinyin_zhongguo_full_abi` improved `14,759.755us` -> `1,527.055us`; `spelling_algebra_expand` dropped `148,570.200us` / `17,784,832` bytes to `122.200us` / `0` bytes. Fair whole-process peak remains about `182 MB`, so no memory-footprint or "faster than librime" public claim is made.

**Architecture:** M34 proved that bounded first-page materialization helps the full-ABI/context path, but it did not change raw engine lookup or peak memory. M35 attacks the remaining gap by changing the storage shape: evolve the M34 `TableLookup` seam from `&[Candidate]` heap slices into lightweight candidate views, add a compact table payload reader that can answer exact/prefix/all-code queries without prebuilding `Candidate` values, and make prism spelling/canonical-code lookup part of the critical path for spelling-algebra schemas after table payload parity is proven. Heap fallback remains available for unsupported schemas, but a compact-active schema must not build or retain the heap `BTreeMap` as a safety net. Mmap is conditional: mapping bytes while still rebuilding heap maps is not a win.

**Tech Stack:** Rust (`yune-core`, `yune-rime-api`), `StaticTableTranslator`, `Engine`, `CandidateRequest` / `TranslationResult`, `dictionary::query_table`, `compiled_table`, `compiled_prism`, `DartsDoubleArray`, `schema_install`, native `frontend_baselines`, `scripts/benchmark-yune-vs-librime.ps1`, upstream `rime/librime 1.17.0` fixtures, TypeDuck `jyut6ping3` fixtures, and TypeDuck-Web runtime/browser gates only if runtime/browser-visible files change.

---

## Why This Exists

M34 left the important storage problem open:

- native `ni` full ABI improved, but engine-only `ni` stayed flat;
- peak working set remains about `182 MB` versus librime's about `22.6 MB`;
- fair cross-engine per-key rows still show Yune far behind librime;
- `StaticTableTranslator.entries_by_code` remains a `BTreeMap<String, Vec<Candidate>>`;
- the M34 `TableLookup` trait still returns `&[Candidate]`, so it cannot remove heap `Candidate` storage by itself.

M35 is the next deep engine-performance milestone. It deliberately learns from librime's data path - compact indexed table payloads plus lazy/page-oriented lookup - without copying librime's C++ service-locator architecture, plugin ABI, or build system.

## Scope

In scope:

- Fresh post-M34 baselines and reader audits before any storage change.
- A new candidate-view/query abstraction that does not require retained `Vec<Candidate>` values.
- A compact table payload reader for exact, prefix, and all-code queries.
- Byte-identical A/B parity between heap-backed and compact-backed lookup for upstream `luna_pinyin` fixtures.
- Preservation of comment bytes, preedit behavior, quality/order, source classification, code, stems/encoder data, correction/tolerance data, and lookup records.
- Prism/table integration only after table payload parity passes.
- A safe storage-switch path in `StaticTableTranslator` and schema install, with heap fallback for unsupported sections or full-list behaviors, but never both compact and heap retained for the same compact-active schema.
- Measurement of native per-key, startup/session, and peak/ready memory before and after.
- Optional mmap/borrowed storage only after compact query storage is already byte-identical and selected by the runtime hot path.
- Documentation updates for performance claims and next deferrals.

Out of scope:

- Widening default `RimeApi`, `RimeCandidate`, or TypeDuck profile ABI slots.
- Changing candidate order, comments, userdb learning, or profile-specific TypeDuck behavior to make performance easier.
- Treating `my_rime` as a behavior oracle.
- M31 Cloudflare/PWA/public-demo delivery work.
- M32 AI product UX or remote-provider work.
- TypeDuck-Windows TSF shell/input-delivery work.
- Replacing Yune's deterministic engine architecture with librime's C++ component registry.

## Preconditions

- M34 is complete and archived.
- `docs/reports/evidence/m34-queryable-table-prism/` is treated as the baseline evidence bundle.
- Work starts from current `origin/main`.
- If M31 runs in parallel, it must avoid overlapping engine/storage/schema-install edits until M35 lands or is explicitly paused. M31 UI/devops/Cloudflare-only work can proceed in another worktree, but M31 OpenCC engine work should serialize against M35.

## Acceptance Gates

- `M35-PERF-01`: Fresh post-M34 native and fair cross-engine baselines are captured for startup/runtime-ready, session create/select/destroy, peak and ready memory, `ni`, `hao`, `zhongguo`, and TypeDuck watch rows (`hai`, `jigaajiusihaa`, correction-on `jigaajiusihaa`).
- `M35-PERF-02`: The storage seam is changed from `&[Candidate]` heap slices to a candidate-view API that can represent heap and compact candidates without materializing `Candidate` until selected or required by fallback.
- `M35-PERF-03`: Heap-backed behavior through the new candidate-view API is byte-identical to the pre-M35 eager path for exact lookup, prefix completion, bounded first page, out-of-window paging, and candidate-list iterator reads.
- `M35-PERF-04`: Compact table lookup preserves candidate text, comment bytes, raw code, quality/order, source classification, preedit formatting inputs, stems/encoder data, correction/tolerance data, and TypeDuck lookup records, or emits a documented no-go for unsupported sections.
- `M35-PERF-05`: Compact exact, prefix, and all-code queries reproduce heap-backed output for upstream `luna_pinyin` source-backed and compiled-backed fixtures before the compact path is used by default.
- `M35-PERF-06`: Prism/table integration is enabled only after table payload parity passes; for schemas whose spelling algebra currently creates the memory blow-up, prism/canonical-code lookup is a prerequisite for compact runtime enablement, not a cosmetic follow-up.
- `M35-PERF-07`: `StaticTableTranslator` can choose heap or compact storage through an internal storage enum or equivalent private abstraction. Unsupported schemas and behaviors fall back to heap without changing output, but compact-active schemas must not build or retain `entries_by_code` heap storage.
- `M35-PERF-08`: The compact path is enabled first for upstream `luna_pinyin` safe rows, with byte-identical first-page, paging, and full-list API behavior. TypeDuck `jyut6ping3` compact enablement is allowed only if rich comments, lookup records, long composition, partial selection, default-confirm recomposition, and userdb learning invariants stay byte-identical.
- `M35-PERF-09`: Native engine-only and full-ABI rows improve materially for short upstream inputs. Stretch target: bring `ni` / `hao` engine-only into the low-hundreds-of-microseconds range. If the target is not met, the closeout must name the measured remaining owner.
- `M35-PERF-10`: Memory attribution measures compiled asset sizes, source row counts, post-spelling-algebra row counts, duplicated text/comment bytes, and retained heap structure cost before setting the final threshold. The expected result is an order-of-magnitude cut to the dictionary-specific ready/peak delta, not to the whole process peak; the stretch target is fair-harness peak working set in the `40-70 MB` range (roughly `2-3x` librime) or a measured no-go naming the retained owner.
- `M35-PERF-11`: Mmap/borrowed storage is attempted only after compact query storage is hot-path active. If mmap requires unsafe code or a lint exception, it is isolated, documented, reviewed, and justified by measurement; otherwise M35 may stop at compact `Arc<[u8]>` or compact owned arrays.
- `M35-PERF-12`: TypeDuck profile rows do not regress by more than 10% and remain byte-identical. Any TypeDuck compact-path no-go leaves the heap fallback active and documented rather than weakening profile behavior.
- `M35-PERF-13`: Public performance reports distinguish native engine, full-ABI, cross-engine harness, memory, and browser delivery. Cross-engine latency ratios must not be used as the main typing headline unless the harness separates engine work from P/Invoke/context/memory-sampling overhead.
- `M35-PERF-14`: Full gates pass: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, focused upstream and TypeDuck parity tests, `cargo test --workspace`, native benchmarks, fair cross-engine rerun, docs/report updates, and `git diff --check`. Runtime/browser gates run only if runtime/browser-visible files change.

## File Responsibilities

- `crates/yune-core/src/dictionary/query_table.rs`: evolves from heap-only `TableLookup` over `&[Candidate]` into candidate-view lookup traits or small private adapters that support heap and compact storage.
- `crates/yune-core/src/dictionary/compiled_table.rs`: owns compact table payload parsing/querying and any no-go errors for unsupported sections.
- `crates/yune-core/src/dictionary/compiled_prism.rs`: owns prism spelling-map and double-array query helpers.
- `crates/yune-core/src/dictionary/double_array.rs`: owns Darts traversal helpers needed by prism lookup.
- `crates/yune-core/src/translator/mod.rs`: owns `StaticTableTranslator` storage selection, candidate materialization, bounded/eager fallback, comments/preedit/quality/source behavior, and TypeDuck profile guardrails.
- `crates/yune-core/src/engine.rs`: owns lazy window/full-list expansion, paging, filters/rankers/userdb/AI interactions, and context completeness.
- `crates/yune-rime-api/src/schema_install.rs`: owns loading compiled/source assets, translator construction, cache keys, and storage-choice invalidation.
- `crates/yune-rime-api/benches/frontend_baselines.rs`: owns native watched rows.
- `scripts/benchmark-yune-vs-librime.ps1` and `scripts/yune-vs-librime-benchmark.cs`: own fair cross-engine reruns if the evidence format changes.
- `crates/yune-core/tests/upstream_luna_pinyin_parity.rs`: owns upstream behavior fixtures.
- `crates/yune-core/tests/cantonese_parity.rs` and `crates/yune-rime-api/tests/typeduck_web.rs`: own TypeDuck behavior and runtime contract guards.
- `docs/reports/evidence/m35-compact-table-prism-storage/`: owns new evidence.
- `docs/reports/yune-vs-librime-performance.md`, `docs/reports/yune-vs-librime-root-cause-analysis.md`, `docs/roadmap.md`, and `docs/requirements.md`: own status and public claims.

---

## Task 0 - Baseline, Audit, And Stop Conditions

**Files:**

- Read: `docs/reports/yune-vs-librime-performance.md`
- Read: `docs/reports/yune-vs-librime-root-cause-analysis.md`
- Read: `docs/reports/evidence/m34-queryable-table-prism/`
- Read: `crates/yune-core/src/dictionary/query_table.rs`
- Read: `crates/yune-core/src/translator/mod.rs`
- Read: `crates/yune-rime-api/src/schema_install.rs`
- Create: `docs/reports/evidence/m35-compact-table-prism-storage/baseline.md`
- Create: `docs/reports/evidence/m35-compact-table-prism-storage/memory-attribution.md`
- Create: `docs/reports/evidence/m35-compact-table-prism-storage/reader-audit.md`
- Create: `docs/reports/evidence/m35-compact-table-prism-storage/stop-conditions.md`

- [ ] **Step 0.1: Confirm repo state**

Run:

```powershell
git fetch origin --prune
git status --short --branch
git log --oneline -5 --decorate
```

Expected:

- Worktree is clean or unrelated active changes are listed before editing.
- `origin/main` contains the completed M34 closeout and M34 visual/report commit.

- [ ] **Step 0.2: Capture fresh M35 baselines**

Run the native and cross-engine commands used by M34:

```powershell
cargo bench -p yune-rime-api --bench frontend_baselines
powershell -ExecutionPolicy Bypass -File scripts\benchmark-yune-vs-librime.ps1 -OutputRoot docs\reports\evidence\m35-compact-table-prism-storage\baseline-yune-vs-librime -Iterations 9 -SessionIterations 9 -KeyIterations 25
```

Expected:

- `baseline.md` records command lines, commit, machine context, medians, p95, peak working set, and any drift versus M34.
- If numbers drift materially, M35 uses the fresh baseline and says so.

- [ ] **Step 0.3: Attribute dictionary memory before setting the target**

Measure and record:

- compiled `luna_pinyin.table.bin`, `.prism.bin`, and `.reverse.bin` sizes;
- compiled product-schema sizes for `jyut6ping3_mobile` / `jyut6ping3`, `cangjie5`, and the reduced `luna_pinyin` reverse-lookup dependency used by TypeDuck-Web where available;
- Yune ready and peak working-set deltas for `luna_pinyin` schema select;
- Yune ready and peak working-set deltas for the product `jyut6ping3_mobile` / `jyut6ping3` schema select;
- source row count before spelling algebra;
- post-spelling-algebra code count and candidate-reference count;
- duplicate text/comment byte estimates caused by expansion;
- retained `Candidate`, `Vec`, `String`, and `BTreeMap` structure estimates;
- whether reverse lookup or stroke assets are present in the measured path.

Expected:

- `memory-attribution.md` explains the dictionary-memory blow-up before any compact storage work starts.
- The evidence separates upstream `luna_pinyin` benchmark memory from product `jyut6ping3` memory, so M35 cannot optimize only the parity benchmark while leaving the shipped product path heavy.
- The final memory target is derived from this attribution, not from a fixed placeholder threshold.
- If compact storage cannot drop the majority of dictionary-specific heap delta, M35 must close as no-go rather than claiming a small memory win hidden by the process baseline.

- [ ] **Step 0.4: Re-audit readers of heap table state**

Run:

```powershell
rg -n "entries_by_code|TableLookup|exact_candidates|prefix_candidates|all_codes|Vec<Candidate>|CandidateRequest|TranslationResult|refresh_candidates|candidate_list_complete|prediction_never_first|sentence_over_completion|prefix_fallback|assign_ordered_candidate_qualities" crates/yune-core/src crates/yune-rime-api/src
```

Expected:

- `reader-audit.md` lists every caller that currently assumes materialized `Candidate` values.
- Each caller is classified as candidate-view-safe, selected-materialization-only, or heap-fallback-required.
- Any unclassified caller blocks storage replacement.

- [ ] **Step 0.5: Write stop conditions before implementation**

`stop-conditions.md` must state:

- stop if candidate-view parity cannot reproduce heap eager output;
- stop if compiled table payload cannot preserve comments/order/lookup records;
- stop if prism-only lookup is attempted without table payloads;
- stop if compact-active schemas still build or retain the heap `entries_by_code` map;
- stop if a spelling-algebra schema tries to recover parity by materializing expanded heap aliases instead of using prism/canonical-code lookup;
- stop if mmap would require unsafe/lint exceptions before compact query storage has a measured win;
- stop if TypeDuck rich-comment or learning invariants diverge.

## Task 1 - Candidate-View Lookup Contract

**Files:**

- Modify: `crates/yune-core/src/dictionary/query_table.rs`
- Modify: `crates/yune-core/src/translator/mod.rs`
- Test: `crates/yune-core/src/tests/translator.rs`
- Create: `docs/reports/evidence/m35-compact-table-prism-storage/candidate-view-contract.md`

- [ ] **Step 1.1: Add failing heap-parity tests for candidate views**

Add tests that compare old eager translation with the new candidate-view path for:

- exact `luna_pinyin` short input;
- prefix completion;
- equal-quality stable tie order;
- bounded first page plus out-of-window full expansion;
- candidate-list iterator reads.

Expected:

- Tests fail until the view API is wired.
- Expected rows are explicit fixtures or old eager output captured before changing the path, not recomputed from the new compact implementation.

- [ ] **Step 1.2: Replace heap-slice leakage with candidate views**

Change `TableLookup` or add a sibling trait so lookup returns lightweight references with enough data to rank and materialize:

```rust
pub(crate) trait LookupCandidateView {
    fn text(&self) -> &str;
    fn raw_comment(&self) -> &str;
    fn raw_quality(&self) -> f32;
    fn source_hint(&self) -> CandidateSource;
}
```

The exact names may differ, but the contract must avoid returning `&[Candidate]` from compact storage.

Expected:

- Heap lookup implements the view API by borrowing existing candidates.
- The public `Candidate` type and C ABI do not change.
- `candidate-view-contract.md` explains the API and why materialization remains selected-only.

- [ ] **Step 1.3: Keep eager compatibility wrappers**

Preserve a compatibility path that still produces full `Vec<Candidate>` values for full-list filters, rankers, candidate iterators, debug readers, and unsupported schemas.

Expected:

- M34 lazy first-page behavior remains intact.
- Existing tests continue to pass.

## Task 2 - Compact Table Payload Reader

**Files:**

- Modify: `crates/yune-core/src/dictionary/compiled_table.rs`
- Modify or create: `crates/yune-core/src/dictionary/query_table.rs`
- Test: `crates/yune-core/src/tests/facade_tests/compiled_payloads.rs`
- Test: `crates/yune-core/src/tests/dictionary.rs`
- Create: `docs/reports/evidence/m35-compact-table-prism-storage/compact-table-reader.md`

- [ ] **Step 2.1: Add compact-reader fixture tests**

Create tests that load representative `.table.bin` fixtures and assert:

- exact code lookup returns the same text/order/weight/comment inputs as `parse_rime_table_bin_dictionary`;
- prefix lookup returns the same code sequence as the heap map;
- unsupported sections return structured no-go errors;
- advanced Yune payload fields survive parsing.

Expected:

- Tests fail before the compact reader exists.

- [ ] **Step 2.2: Implement compact table indexes**

Implement a compact table structure that stores or borrows:

- code index;
- candidate payload offsets or compact records;
- text/comment/code payloads;
- raw quality/order;
- advanced payloads for stems, encoder, correction/tolerance, lookup records, preset vocabulary.

Expected:

- Compact lookup can answer exact, prefix, and all-code queries without prebuilding `Vec<Candidate>` for every row.
- It may start as compact owned arrays or `Arc<[u8]>`; mmap is not required in this task.

- [ ] **Step 2.3: A/B compare heap and compact table outputs**

Add tests that build both heap and compact stores from the same source/compiled fixture and assert byte-identical materialized candidates after passing through `StaticTableTranslator` formatting.

Expected:

- Differences are classified as bugs or documented no-go cases.

## Task 3 - StaticTableTranslator Storage Switch

**Files:**

- Modify: `crates/yune-core/src/translator/mod.rs`
- Modify: `crates/yune-rime-api/src/schema_install.rs`
- Test: `crates/yune-core/src/tests/translator.rs`
- Test: `crates/yune-rime-api/src/tests/dictionary_data.rs`
- Create: `docs/reports/evidence/m35-compact-table-prism-storage/storage-switch.md`

- [ ] **Step 3.1: Add private storage enum or equivalent adapter**

Introduce an internal storage choice such as:

```rust
enum TableStorage {
    Heap(BTreeMap<String, Vec<Candidate>>),
    Compact(CompactTableStore),
}
```

The exact names may differ, but all call sites must go through the view/query abstraction.

Expected:

- Existing eager behavior can still use heap fallback for schemas that are not compact-active.
- Compact storage is opt-in and profile/schema gated.
- A compact-active schema does not build or retain `entries_by_code`; holding both heap and compact storage is a failed memory gate unless the heap copy is short-lived construction scratch and dropped before ready-state measurement.

- [ ] **Step 3.2: Route safe upstream `luna_pinyin` rows through compact storage only after prism parity**

Enable compact storage first for upstream `luna_pinyin` safe rows where:

- no unsupported table section is present;
- exact/prefix output parity passes;
- prism/canonical-code lookup covers spelling-algebra aliases without materializing expanded heap aliases;
- no full-list behavior requires heap fallback beyond documented expansion.

Expected:

- Focused upstream parity tests pass.
- `storage-switch.md` records what is compact-backed, what still falls back, and proof that compact-active ready-state memory does not include retained heap `entries_by_code`.

- [ ] **Step 3.3: Preserve full-list expansion semantics**

Ensure candidate deletion, paging past the bounded window, candidate-list iterator APIs, debug/inspector reads, and ABI context reads still get complete data when they ask for it.

Expected:

- Existing M34 lazy-window tests still pass.
- New tests cover compact-backed full expansion.

## Task 4 - Prism/Table Integration

**Files:**

- Modify: `crates/yune-core/src/dictionary/compiled_prism.rs`
- Modify: `crates/yune-core/src/dictionary/double_array.rs`
- Modify: `crates/yune-core/src/translator/mod.rs`
- Test: `crates/yune-core/tests/upstream_luna_pinyin_parity.rs`
- Create: `docs/reports/evidence/m35-compact-table-prism-storage/prism-table-integration.md`

- [ ] **Step 4.1: Add prism lookup tests that require table payloads**

Write tests proving prism lookup only supplies spelling/code discovery and table lookup supplies candidate payloads.

Expected:

- A prism-only path cannot pass the tests.
- For spelling-algebra-backed schemas, re-expanding aliases into owned heap storage is not an acceptable substitute for prism/table lookup.

- [ ] **Step 4.2: Integrate prism spelling graph with compact table lookup**

Use prism data to discover canonical lookup codes where it is byte-identical to the current spelling-algebra/schema install path.

Expected:

- Upstream `luna_pinyin` exact, completion, and sentence-adjacent rows remain byte-identical.
- If query-time prism lookup does not beat existing install-time expansion or cannot reproduce byte-identical spelling behavior, document it and keep the old path with heap fallback; do not enable compact storage for that schema.

## Task 5 - TypeDuck Guard And Optional Compact Enablement

**Files:**

- Modify only if needed: `crates/yune-core/src/translator/mod.rs`
- Test: `crates/yune-core/tests/cantonese_parity.rs`
- Test: `crates/yune-rime-api/tests/typeduck_web.rs`
- Create: `docs/reports/evidence/m35-compact-table-prism-storage/typeduck-guard.md`

- [ ] **Step 5.1: Prove heap fallback preserves TypeDuck**

Run TypeDuck parity with compact storage disabled for unsupported TypeDuck cases.

Expected:

- Rich comments, partial selection, long composition, default-confirm recomposition, and userdb learning stay byte-identical.

- [ ] **Step 5.2: Enable TypeDuck compact storage only if all profile payloads survive**

Attempt TypeDuck compact storage only after lookup records, comments, corrections, tolerance, and composition behavior are proven.

Expected:

- If any TypeDuck profile invariant diverges, leave TypeDuck on heap fallback and record the no-go.
- If compact TypeDuck storage lands, TypeDuck watched rows must not regress more than 10%.

## Task 6 - Mmap / Borrowed Storage Gate

**Files:**

- Modify only if accepted: `crates/yune-core/src/dictionary/compiled_table.rs`
- Modify only if accepted: `crates/yune-rime-api/src/schema_install.rs`
- Create: `docs/reports/evidence/m35-compact-table-prism-storage/mmap-gate.md`

- [ ] **Step 6.1: Measure compact owned storage before mmap**

Run the native and cross-engine baselines after compact storage is active.

Expected:

- `mmap-gate.md` records whether compact owned storage already meets the memory/startup target.

- [ ] **Step 6.2: Attempt mmap only if measurement justifies it**

If mmap is still needed:

- document the crate/API choice;
- document Windows file lifetime and rebuild behavior;
- document whether unsafe code is required;
- avoid adding a workspace-wide unsafe exception.

Expected:

- If mmap lands, tests cover file lifetime and rebuild invalidation.
- If mmap is deferred, the evidence names the remaining owner and next step.

## Task 7 - Measurement, Reports, And Closeout

**Files:**

- Modify: `docs/reports/yune-vs-librime-performance.md`
- Modify: `docs/reports/yune-vs-librime-root-cause-analysis.md`
- Modify: `docs/roadmap.md`
- Modify: `docs/requirements.md`
- Move on closeout: `docs/plans/m35-plan-compact-table-prism-runtime-storage.md` to `docs/plans/archive/m35-plan-compact-table-prism-runtime-storage.md`
- Create: `docs/reports/evidence/m35-compact-table-prism-storage/harness-attribution.md`
- Create: `docs/reports/evidence/m35-compact-table-prism-storage/task-7-gates.md`

- [ ] **Step 7.1: Run final native and cross-engine evidence**

Run:

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p yune-core --test upstream_luna_pinyin_parity
cargo test -p yune-core --test cantonese_parity
cargo test -p yune-rime-api --test typeduck_web
cargo test --workspace
cargo bench -p yune-rime-api --bench frontend_baselines
powershell -ExecutionPolicy Bypass -File scripts\benchmark-yune-vs-librime.ps1 -OutputRoot docs\reports\evidence\m35-compact-table-prism-storage\after-yune-vs-librime -Iterations 9 -SessionIterations 9 -KeyIterations 25
git diff --check
```

Expected:

- All commands pass or any non-run gate is explicitly justified.
- Runtime/browser gates are run only if runtime/browser files changed.

- [ ] **Step 7.2: Attribute or fix cross-engine harness overhead**

Before using Yune-vs-librime latency ratios as a public headline, measure or document:

- native engine-only timing;
- native full-ABI timing;
- cross-engine C# harness timing;
- P/Invoke/context marshalling cost if measurable;
- whether per-call memory sampling is inside the measured key loop.

Expected:

- `harness-attribution.md` says which number is safe as the main typing-latency claim.
- If the existing cross-engine harness remains dominated by interop or memory-sampling overhead, public reports must lead with native engine-only/full-ABI rows and treat cross-engine ratios as compatibility-harness evidence, not raw engine speed.

- [ ] **Step 7.3: Update public reports conservatively**

Update the performance and root-cause reports with:

- exact before/after numbers;
- native engine-only versus full-ABI split;
- fair cross-engine split;
- harness-overhead caveats;
- memory split, including upstream `luna_pinyin` and product `jyut6ping3` schema rows;
- process peak versus dictionary-specific delta distinction;
- browser/non-browser caveats.

Expected:

- No "faster than librime" claim unless fair evidence supports it.
- No browser startup/typing claim unless browser evidence exists.

- [ ] **Step 7.4: Archive the plan and update status**

When complete:

- move this plan to `docs/plans/archive/`;
- mark M35 complete or closed-by-no-go in `docs/roadmap.md`;
- mark `M35-PERF-REQ-*` rows complete or closed-by-no-go in `docs/requirements.md`;
- record the exact commit and evidence directory in the final message.

## Execution Handoff

Start from current `origin/main`. Read `AGENTS.md`, `docs/CONVENTIONS.md`, `docs/roadmap.md`, `docs/requirements.md`, this plan, `docs/reports/yune-vs-librime-performance.md`, `docs/reports/yune-vs-librime-root-cause-analysis.md`, and the M34 evidence directory before editing. Keep M31 delivery/cache work out of this milestone. Stage only M35 files and preserve unrelated worktree changes.
