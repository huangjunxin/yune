# M43 Native Memory And Short-Key Owner Reduction Plan

> **Status:** Complete with measured memory blocker - **Milestone:** M43 (native memory and short-key owner
> reduction) - **Created:** 2026-06-26 - **Type:** native-engine plan
>
> **For agentic workers:** REQUIRED SUB-SKILL: Use
> superpowers:subagent-driven-development (recommended) or
> superpowers:executing-plans to implement this plan task-by-task. Steps use
> checkbox (`- [x]`) syntax for tracking.

## Goal

Reduce the largest remaining native-engine performance gap after M42 without
regressing the closed M40/M42 behavior: first prove whole-process memory and
`hao`/`ni` fixed-overhead owners, then implement exactly the measured owner
branch that is safe to change.

M43 is not an abbreviation-latency milestone. The two M42 abbreviation rows
(`cszysmsrsd` and `zybfshmsru`) remain behavior guards and final benchmark
rows, but their `3.469x` and `5.069x` same-run librime latency blockers are not
the implementation target for this milestone. A future M44-style plan can
attack abbreviation graph/search latency after M43 either reduces memory or
records a measured memory/short-key blocker.

## Architecture

M43 is a native-engine-only, owner-first milestone with a hard Phase 0 branch:

1. Measure retained memory owners and short-key fixed overhead on the M42
   baseline.
2. Choose exactly one primary implementation branch:
   - memory-owner reduction if retained heap-owned reducible duplication is the
     largest bounded owner;
   - short-key fixed-overhead reduction if `hao`/`ni` remain dominated by a
     named translator/materialization/export owner and memory has no safe
     bounded owner;
   - reporting/no-go if Phase 0 disproves the suspected owners or every
     plausible fix would violate storage, behavior, or bounded-output guards.
3. Preserve all M42 behavior, M40 long-row wins, selected storage contracts, and
   Track B guard evidence before closeout.

This plan intentionally prefers structural accounting before allocator rewrites.
A Windows allocator-level heap profile may be attached if available, but the
minimum required evidence is a deterministic Yune-owned owner profile that
accounts for the major retained structures by module and purpose.

## Tech Stack

- Rust native engine: `crates/yune-core` and `crates/yune-rime-api`.
- Benchmark harness:
  `crates/yune-rime-api/benches/native_inprocess_benchmark.rs`.
- Existing metric export surface: `crates/yune-core/src/m37_metrics.rs` and the
  native benchmark `m37_metrics.csv` writer.
- Likely memory-owner modules:
  `crates/yune-core/src/dictionary/compiled_table.rs`,
  `crates/yune-core/src/poet/mod.rs`,
  `crates/yune-core/src/poet/index.rs`,
  `crates/yune-core/src/translator/mod.rs`, and
  `crates/yune-rime-api/src/schema_install.rs`.
- Likely short-key modules:
  `crates/yune-core/src/translator/mod.rs`,
  `crates/yune-rime-api/src/context_api.rs`, and
  `crates/yune-rime-api/src/session.rs`.
- Evidence root: `docs/reports/evidence/m43-native-memory-short-key-owner-reduction/`.
- Oracle target: upstream `rime/librime 1.17.0` at
  `33e78140250125871856cdc5b42ddc6a5fcd3cd4`.

## M42 Baseline To Preserve

M43 starts from the published M42 closeout at commit `3c53c69b`.

| Row | M42 Yune median | Same-run librime median | Ratio / status |
| --- | ---: | ---: | --- |
| startup/runtime-ready | `23,856.300us` | `31,421.900us` | `0.759x` |
| session create/select/destroy | `23,776.500us` | `27,766.600us` | `0.856x` |
| `hao` | `38.800us` | `11.333us` | `3.424x` |
| `ni` | `57.150us` | `14.000us` | `4.082x` |
| `zhongguo` | `60.188us` | `166.025us` | `0.363x` |
| `ceshiyixiachangjushuruxingnengzenyang` | `278.438us` | `290.873us` | `0.957x` |
| `zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong` | `474.683us` | `658.592us` | `0.721x` |
| `cszysmsrsd` | `4,127.580us` | `1,189.890us` | behavior pass; `3.469x` latency blocker |
| `zybfshmsru` | `4,257.100us` | `839.860us` | behavior pass; `5.069x` latency blocker |

M42 storage and memory baseline:

- Track A max peak working set: `119,775,232 B`.
- Track A 37-character working set: `113,610,752 B`.
- Track A 59-character working set: `114,339,840 B`.
- Track B guard median: `186.513us/op`; p95: `204.680us/op`.
- Track B guard max peak working set: `504,901,632 B`.
- `selected_storage=rsmarisa_byte_backed`.
- table/prism mapping mode: `mmap`.
- selected table/prism heap mirror bytes: `0`.
- `source_fallback=false`.
- positive runtime `rsmarisa` exact/prefix counters on target rows.

M42 owner evidence:

- `ni`: `56.25us/op` process-key owner, `53.0us/op` translator owner,
  `20` owned candidates/op.
- `hao`: `38.2us/op` process-key owner, `34.7us/op` translator owner,
  `20` owned candidates/op.
- `cszysmsrsd` and `zybfshmsru`: translator and upstream sentence-model owners
  remain abbreviation-latency blockers, not M43 targets.

## Scope Boundaries

In scope:

- Track A native `luna_pinyin` whole-process memory owner attribution.
- Track A native `hao` and `ni` fixed-overhead owner attribution.
- One owner-backed implementation branch: memory reduction or short-key fixed
  overhead reduction.
- Preservation of startup/session, `zhongguo`, both Track A long rows, both M42
  abbreviation behavior rows, Track B guard row, storage status, bounded
  output/context, and upstream-observable behavior.
- Performance/root-cause report updates after final evidence exists.

Out of scope:

- Abbreviation graph/search latency optimization for `cszysmsrsd` and
  `zybfshmsru`.
- Web harness, frontend, browser startup, public demo, product delivery,
  packaging, deployment, or browser memory claims.
- TypeDuck-profile performance work beyond the existing Track B guard row.
- Source-YAML fallback or selected table/prism heap mirrors as a shortcut.
- Broad librime feature parity, `.gram`/octagram, plugin ABI, or AI behavior.
- Moving this plan to completed before all closeout gates have evidence.

## Phase 0: Owner Capture And Branch Selection

- [x] Capture a fresh same-run native benchmark under
  `docs/reports/evidence/m43-native-memory-short-key-owner-reduction/phase-0-baseline/`
  with the M42 row set: startup, session, `hao`, `ni`, `zhongguo`, both Track A
  long rows, `cszysmsrsd`, `zybfshmsru`, and the Track B 50+ guard row.
- [x] Capture a run-to-run noise baseline before choosing a branch. At minimum,
  record repeated Track A peak samples and repeated `hao`/`ni` medians from the
  same benchmark configuration, then publish the observed variance band used by
  M43 success and no-regression decisions.
- [x] Add a deterministic structural owner profile export, for example
  `memory-owner-profile.csv`, that records retained owner estimates by module
  and structure. Each row must classify bytes as `heap_owned_reducible`,
  `heap_owned_guarded`, `mmap_file_backed`, `shared`, or `overlap_estimate`.
  Branch-selection thresholds use only non-overlapping
  `heap_owned_reducible` bytes. The minimum owner set is:
  `compact_table.syllabary_codes`, `compact_table.syllable_ids_by_code`,
  `compact_table.storage`, `translator.entries_by_code`,
  `poet.entries_by_code`, `poet.lookup_index`,
  `poet.abbreviation_vocabulary`, `schema.config`, `schema.processors`,
  `session.userdb`, and `runtime.session_state`.
- [x] Add a reconciliation section to the Phase 0 verdict that compares owner
  estimates against measured Track A working set/peak movement. The verdict
  must name excluded `mmap_file_backed`, `shared`, and `overlap_estimate` bytes
  so duplicated logical strings or mapped storage cannot double-count into a
  branch trigger.
- [x] Add `m37_metrics.csv` fields before trusting new metrics. The benchmark
  must not silently omit any new M43 metric from the CSV bundle.
- [x] Produce a short-key owner profile for `hao` and `ni` that splits at least
  these buckets: raw prism lookup, raw table lookup, translator production,
  candidate clone/materialization, ranking/sorting/filtering, context export,
  ABI string allocation, and free-context work.
- [x] Prove M42 abbreviation candidate output still matches upstream for
  `cszysmsrsd` and `zybfshmsru` before any implementation branch starts.
- [x] Write a Phase 0 verdict file that chooses one branch:
  `memory-owner-reduction`, `short-key-fixed-overhead`, or
  `reporting-no-go`.

Branch selection rules:

- Choose `memory-owner-reduction` if a bounded retained owner or duplicate
  owner family accounts for at least `10 MB` of non-overlapping
  `heap_owned_reducible` bytes, or if the top two related owners together
  account for at least `15 MB` of non-overlapping `heap_owned_reducible` bytes,
  and the proposed fix can preserve mmap/`rsmarisa`, zero selected heap mirrors,
  no source fallback, and candidate output. Mapped storage such as
  `compact_table.storage` may be reported, but it cannot satisfy the trigger
  while classified as `mmap_file_backed`.
- Choose `short-key-fixed-overhead` only if memory profiling does not name a
  safe bounded owner and `hao`/`ni` remain at least `75%` dominated by a named
  translator/materialization/export bucket that can be reduced without touching
  the M40 full-pinyin sentence path or M42 abbreviation path.
- Choose `reporting-no-go` if owner evidence disproves the suspected duplicated
  strings/model-entry hypothesis, if allocator/structural evidence is too weak
  to justify a rewrite, or if all plausible fixes require source fallback,
  selected heap mirrors, unbounded candidate materialization, or behavior drift.

## Branch A: Memory-Owner Reduction

Run only if Phase 0 selects `memory-owner-reduction`.

- [x] Write failing or guard tests for the exact owner being changed before
  implementation. Examples:
  - compact table tests proving canonical code lookup still returns identical
    candidates when syllabary storage changes;
  - poet tests proving `ModelEntry` storage changes preserve sentence candidate
    order and M42 abbreviation output;
  - session/API tests proving storage status and source-fallback counters remain
    unchanged.
- [x] Reduce exactly the selected owner family first. Acceptable shapes include
  interned/string-id storage, borrowed ranges over already-retained compiled
  bytes, compact numeric ids, or lazily built owner state. Do not add a new
  always-on mirror whose retained bytes erase the win.
- [x] If the selected owner is `compact_table.syllabary_codes` plus
  `syllable_ids_by_code`, store each code once and make lookup reuse the same
  representation. Preserve `lookup_canonical_codes`, `rsmarisa` path
  segmentation, and all existing compact table tests.
- [x] If the selected owner is `poet.entries_by_code`, change `ModelEntry`
  storage without changing `SentenceLookupIndex` ordering semantics,
  `compare_model_entry_by_code`, abbreviation vocabulary filtering, or
  candidate text/comment output.
- [x] If the selected owner is schema/session/userdb state, keep runtime
  resource identifiers logical, keep userdb learning behavior unchanged, and
  do not delay a structure unless first use is measured and bounded.
- [x] Re-run the Phase 0 owner profile after the change and prove the selected
  owner moved materially.

Memory branch closeout requires one of these outcomes:

- **Whole-process memory win:** Track A peak working set is at least `10%`
  lower than the M42 peak
  (`<=107,797,708 B`), with no candidate-output regression; or
- **Partial structural reduction, not a whole-process memory win:** a named
  non-overlapping `heap_owned_reducible` owner family drops by at least `15 MB`,
  the drop is corroborated by the post-change owner profile, Track A peak stays
  within the Phase 0 observed noise band rather than merely under the `+5%`
  ceiling, and the final reports explicitly say the whole-process memory target
  did not move enough and needs another owner pass.

If neither threshold is met, M43 may close only as a measured blocker with
explicit documentation that no memory win was achieved.

## Branch B: Short-Key Fixed-Overhead Reduction

Run only if Phase 0 selects `short-key-fixed-overhead`.

- [x] Add focused tests for the exact `hao`/`ni` owner before implementation.
  The tests must assert candidate text/order/comments where behavior could
  change and metric counters where the optimization changes materialization or
  context/export shape.
- [x] Keep `upstream_sentence_model_calls=0` for `hao` and `ni`; do not route
  short keys through the M40 sentence model or the M42 abbreviation path.
- [x] If candidate materialization is the owner, bound owned candidate cloning
  to first-page/context needs while preserving paging behavior when later pages
  are requested.
- [x] If lookup enumeration is the owner, reduce repeated exact/prefix table
  walks without losing dictionary candidates or changing ranking.
- [x] If ABI/context export is the owner, reduce string allocation/free work
  without changing `RimeCandidate`, `RimeContext`, ownership, or free rules.
- [x] Report any new always-on cache or retained data structure with retained
  bytes in `memory-owner-profile.csv`. Branch B must keep Track A peak within
  the Phase 0 observed noise band of the M42 baseline, not merely below the
  `+5%` memory ceiling.
- [x] Re-run the short-key owner profile and final native benchmark after the
  change.

Short-key branch success requires both short rows to improve by at least `15%`
from M42 medians, clear the Phase 0 observed run-to-run noise band, and show a
commensurate drop in the named owner counter:

- `hao <=32.980us`.
- `ni <=48.577us`.

This is a self-relative Yune improvement target, not librime parity. Even at
the threshold, `hao` remains about `2.91x` same-run librime and `ni` remains
about `3.47x` same-run librime. Final reports must publish the residual
same-run librime ratios and must not describe Branch B as closing the short-key
librime gap unless the final same-run ratios actually prove that.

If only one row improves or either row regresses, M43 may close only as a
measured blocker unless the user explicitly accepts a narrower result.

## Branch C: Reporting / No-Go

Run only if Phase 0 selects `reporting-no-go`.

- [x] Not selected. Phase 0 chose Branch A (`memory-owner-reduction`), so the reporting/no-go branch did not run.

## Non-Regression Gates

| Gate | Required closeout evidence |
| --- | --- |
| Startup/session | Final startup and session rows stay within same-run librime and within `5%` of M42 medians unless the final report records a measured accepted blocker. |
| Short rows | `hao`, `ni`, and `zhongguo` stay within `5%` of M42 medians when they are not the selected optimization branch. If Branch B is selected, `hao` and `ni` must meet the Branch B success thresholds, clear the observed noise band, show a named-owner counter drop, report residual same-run librime ratios, and avoid claiming short-key parity unless ratios prove it. |
| Long rows | Both Track A long rows stay within `1.25x` same-run librime and within `5%` of M42 medians. The M40 full-pinyin sentence lookup path must not invoke M42 abbreviation span expansion. |
| Abbreviation behavior | `cszysmsrsd` and `zybfshmsru` final native candidate count, text, comments, order, preedit, commit preview, and first-page metadata still match the M42 upstream oracle artifact. Latency may remain a blocker; do not claim an abbreviation speed win. |
| Storage | Track A storage remains `rsmarisa_byte_backed`; table/prism stay mmap or byte-backed; selected table/prism heap mirror bytes stay `0`; `source_fallback=false`; positive `rsmarisa` counters remain present. |
| Memory | Track A peak must not exceed M42 peak `119,775,232 B` by more than `5%` (`125,763,994 B`) in any branch. Branch A may claim a whole-process memory win only at `<=107,797,708 B`; estimate-backed owner movement without peak movement is a partial structural reduction, not a whole-process win. Branch B must stay within the Phase 0 observed memory noise band and report retained bytes for any new cache. |
| Bounded output/context | First-page candidate export and `RimeGetContext` stay page-bounded; any full-list fallback remains explicit and counted. |
| Track B guard | The 50+ `jyut6ping3_mobile` row remains guard-only and within `10%` of M42 median/p95 unless a measured blocker is accepted. No TypeDuck-profile speed claim is allowed. |
| Native-only claim | No web, frontend, product, packaging, delivery, public-demo, or browser speed claim appears in M43 docs or reports. |

## Closeout Evidence

M43 is complete with final selected-branch evidence.

Required final artifacts:

- `docs/reports/evidence/m43-native-memory-short-key-owner-reduction/final-gates.md`
- final same-run native benchmark bundle containing:
  - `summary-comparison.csv`;
  - `summary.csv`;
  - `samples.csv`;
  - `m37_metrics.csv`;
  - `startup_session_trace.csv`;
  - `product_path_status.csv`;
  - `raw_lookup_microbench.csv`;
  - `memory-owner-profile.csv`;
  - a Phase 0 and final noise-band summary for Track A peak and `hao`/`ni`
    medians;
  - short-key owner profile if Branch B ran;
  - candidate-output comparison for `cszysmsrsd` and `zybfshmsru`.
- Updated reports:
  - `docs/reports/yune-vs-librime-performance.md`
  - `docs/reports/yune-vs-librime-root-cause-analysis.md`
- Updated milestone docs:
  - `docs/roadmap.md`
  - `docs/requirements.md`
  - `docs/decisions.md`
  - `docs/ledgers/milestone-history.md` after closeout only.

Required final commands:

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
git diff --check
```

## Implementation Task List

### Task 1: Phase 0 Owner Evidence

**Files:**

- Modify: `crates/yune-rime-api/benches/native_inprocess_benchmark.rs`
- Modify if new counters are needed: `crates/yune-core/src/m37_metrics.rs`
- Modify if structural owner APIs are needed:
  `crates/yune-core/src/dictionary/compiled_table.rs`,
  `crates/yune-core/src/poet/mod.rs`,
  `crates/yune-core/src/translator/mod.rs`,
  `crates/yune-rime-api/src/schema_install.rs`
- Create evidence under:
  `docs/reports/evidence/m43-native-memory-short-key-owner-reduction/phase-0-baseline/`

- [x] Add structural memory-owner accounting for the required owner set.
- [x] Add CSV export for `memory-owner-profile.csv`.
- [x] Add or update tests proving the new counters export through the benchmark
  surface.
- [x] Run the fresh Phase 0 benchmark and write the branch verdict.

### Task 2: Selected Branch Tests

**Files depend on Phase 0 verdict.**

- Branch A likely tests:
  `crates/yune-core/src/tests/dictionary.rs`,
  `crates/yune-core/src/tests/poet.rs`,
  `crates/yune-core/src/tests/translator.rs`,
  `crates/yune-rime-api/src/tests/session_api.rs`.
- Branch B likely tests:
  `crates/yune-core/src/tests/translator.rs`,
  `crates/yune-rime-api/src/tests/session_api.rs`,
  `crates/yune-rime-api/src/tests/frontend_client.rs`.

- [x] Add failing or guard tests for the selected owner.
- [x] Prove the tests fail or prove the guard captures the current counter
  shape before implementation.

### Task 3: Selected Branch Implementation

**Files depend on Phase 0 verdict.**

- Branch A likely implementation:
  `crates/yune-core/src/dictionary/compiled_table.rs`,
  `crates/yune-core/src/poet/mod.rs`,
  `crates/yune-core/src/poet/index.rs`,
  `crates/yune-core/src/translator/mod.rs`.
- Branch B likely implementation:
  `crates/yune-core/src/translator/mod.rs`,
  `crates/yune-rime-api/src/context_api.rs`,
  `crates/yune-rime-api/src/session.rs`.

- [x] Implement only the selected owner change.
- [x] Keep storage status, source-fallback, bounded output, and candidate-output
  behavior observable in tests.
- [x] Run the focused tests for the touched module.

### Task 4: Final Benchmark And Reports

**Files:**

- Modify: `docs/reports/yune-vs-librime-performance.md`
- Modify: `docs/reports/yune-vs-librime-root-cause-analysis.md`
- Create final evidence under:
  `docs/reports/evidence/m43-native-memory-short-key-owner-reduction/`

- [x] Run the final native benchmark with all required rows.
- [x] Compare against M42 baselines and same-run librime.
- [x] Update report visualizations only after final data exists.
- [x] Record whether M43 is a whole-process memory win, partial structural
  memory reduction, self-relative short-key improvement, or measured blocker.
  Short-key reporting must include residual same-run librime ratios.

### Task 5: Closeout Docs And Gates

**Files:**

- Modify: `docs/roadmap.md`
- Modify: `docs/requirements.md`
- Modify: `docs/decisions.md`
- Modify after completion only: `docs/ledgers/milestone-history.md`
- Move after completion only:
  `docs/plans/completed/m43-plan-native-memory-short-key-owner-reduction.md`

- [x] Update roadmap, requirements, and decisions to match final evidence.
- [x] Run all required final commands.
- [x] Move the plan to completed only after every closeout gate passes.
