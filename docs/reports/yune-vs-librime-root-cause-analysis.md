# Yune vs upstream librime root-cause dashboard

Date: 2026-06-26

This report explains M43 native-engine behavior. It does not claim browser,
frontend, product-delivery, packaging, public-demo, or TypeDuck-profile speed
wins.

## Current Verdict

M43 chose Branch A, `memory-owner-reduction`, after Phase 0 named
`poet.entries_by_code` as the largest safe non-overlapping heap-owned reducible
owner in Track A `luna_pinyin`.

The owner changed materially:

| Owner | Phase 0 | Final | Movement |
| --- | ---: | ---: | --- |
| `poet.entries_by_code` | `38,208,541 B` | `18,694,662 B` | `-19,513,879 B` (`-51.072%`) |

![M43 selected owner reduction](./evidence/m43-native-memory-short-key-owner-reduction/visuals/m43-owner-reduction.svg)

The implementation reduces exactly that owner family by packing
sentence-model entries as compact text ranges and code ids backed by pooled
text/code bytes. It does not add an abbreviation span graph, it does not touch
the M40 full-pinyin branch, and it does not route short keys through the
sentence model.

This is a partial structural reduction. Track A peak stayed within the Phase 0
noise band, but it did not reach the M43 whole-process memory-win target and
did not return below the historical M42 `+5%` memory ceiling. M43 therefore
records whole-process memory as the next blocker rather than claiming memory
parity.

## Phase 0 Branch Selection

Phase 0 owner profile for Track A `luna_pinyin`:

| Owner | Class | Retained estimate | Branch effect |
| --- | --- | ---: | --- |
| `poet.entries_by_code` | `heap_owned_reducible` | `38,208,541 B` | Selected branch trigger. |
| `compact_table.storage` | `mmap_file_backed` | `13,013,460 B` | Excluded from heap-owned trigger. |
| `poet.lookup_index` | `heap_owned_guarded` | `2,660,848 B` | Preserved for M40 lookup. |
| `schema.config` | `overlap_estimate` | `1,864 B` | Not counted as reducible heap owner. |
| `translator.entries_by_code` | `shared` | `0 B` | Compact table path has no Track A heap map. |

The short-key owner profile was captured in Phase 0, but it did not become the
selected branch because a larger bounded memory owner was available. Final
`hao`/`ni` data still shows translator production as the main remaining fixed
overhead:

| Row | Raw prism | Raw table | Translator | Context export | ABI allocation |
| --- | ---: | ---: | ---: | ---: | ---: |
| `hao` final | `0.100 us` | `13.700 us` | `34.500 us` | `1.400 us` | `0.133 us` |
| `ni` final | `0.100 us` | `17.200 us` | `53.100 us` | `1.400 us` | `0.200 us` |

## Implementation Shape

Before M43, the upstream sentence model retained a sorted `Vec<ModelEntry>`
where each entry owned its own text and code strings. Phase 0 showed this was
the largest reducible retained owner.

The final shape:

1. Consumes owned model entries during build, so the old strings are not kept
   alive during steady state.
2. Stores phrase text in one text byte pool with per-entry ranges.
3. Stores sorted code strings once in a code byte pool and gives each entry a
   compact `code_id`.
4. Keeps `SentenceLookupIndex` ordering semantics by resolving code ids through
   the code pool during index build and lookup.
5. Keeps abbreviation vocabulary separate from full-pinyin lookup, preserving
   the M42 branch boundary.

Focused tests now cover the accounting and behavior surfaces:

- `upstream_sentence_model_memory_profile_accounts_packed_entries`
- `static_table_memory_owner_rows_cover_m43_owner_set`
- `m43_memory_owner_profile_exports_required_session_rows`
- `m37_metrics_exports_snapshot_json_for_loaded_benchmarks`
- existing upstream sentence and abbreviation sentence tests

## Guardrails Preserved

| Gate | Final evidence |
| --- | --- |
| Startup/session | Startup `0.788x`, session `0.868x` same-run librime. |
| Short/medium rows | `hao` `3.390x`, `ni` `4.096x`, `zhongguo` `0.368x`; short-key branch was not selected. |
| Full-pinyin long rows | `ceshiyixiachangjushuruxingnengzenyang` `0.985x`; `zhegeyinqingqishiyinggaizhichichaochangjuzishurucainengyong` `0.733x`. |
| Abbreviation behavior | Final candidate-output artifact matches upstream for `cszysmsrsd` and `zybfshmsru`; latency remains a blocker. |
| Storage | Track A `selected_storage=rsmarisa_byte_backed`, table/prism `mmap`, selected heap mirrors `0`, `source_fallback=false`. |
| Bounded output/context | Final row evidence keeps first-page candidate export and page-sized context behavior. |
| Track B guard | `182.780 us/op` median, `191.182 us/op` p95; guard-only, no TypeDuck-profile speed claim. |

## Memory Reconciliation

The retained owner estimate moved, but the process peak did not. This is not a
contradiction: Phase 0 owner accounting identifies a Yune-owned retained
structure, while Windows peak working set includes allocator behavior, mapped
pages, shared runtime state, and transient build/session pressure.

The final evidence therefore supports this exact conclusion:

- structural retained bytes improved materially for the selected owner;
- final peak stays inside Phase 0 observed noise;
- whole-process memory remains too high for a win claim;
- another owner pass is required before memory parity can be claimed.

![M43 Track A peak memory gates](./evidence/m43-native-memory-short-key-owner-reduction/visuals/m43-memory-gates.svg)

## Remaining Root Causes

![Post-M43 remaining native blockers](./evidence/m43-native-memory-short-key-owner-reduction/visuals/m43-next-bottlenecks.svg)

| Rank | Root cause | Current evidence | Next move |
| ---: | --- | --- | --- |
| 1 | Whole-process peak memory is not explained by `poet.entries_by_code` alone. | `19.5 MB` owner drop, but final Track A peak still around `127.5 MB`. | Profile allocator/peak owners and the next retained heap families before changing storage again. |
| 2 | Abbreviation graph/search latency. | `cszysmsrsd` `3.479x`; `zybfshmsru` `5.299x`; output matches oracle. | Separate abbreviation-owner plan; keep M40 full-pinyin path untouched. |
| 3 | Short-key translator fixed overhead. | `hao`/`ni` dominated by translator production after raw lookup and ABI export are small. | Separate short-key branch only if a named owner can reduce both rows beyond noise. |
| 4 | Track B profile path remains guard-only. | Final Track B guard passes, but no TypeDuck speed claim is made. | Separate TypeDuck-profile milestone if product rows require it. |

## Evidence

- Phase 0 verdict:
  [`./evidence/m43-native-memory-short-key-owner-reduction/phase-0-baseline/phase-0-verdict.md`](./evidence/m43-native-memory-short-key-owner-reduction/phase-0-baseline/phase-0-verdict.md)
- Final native comparison:
  [`./evidence/m43-native-memory-short-key-owner-reduction/final-native-benchmark/summary-comparison.csv`](./evidence/m43-native-memory-short-key-owner-reduction/final-native-benchmark/summary-comparison.csv)
- Final owner profile:
  [`./evidence/m43-native-memory-short-key-owner-reduction/final-native-benchmark/memory-owner-profile.csv`](./evidence/m43-native-memory-short-key-owner-reduction/final-native-benchmark/memory-owner-profile.csv)
- Final candidate output:
  [`./evidence/m43-native-memory-short-key-owner-reduction/final-native-benchmark/oracle-vs-yune-candidate-output.md`](./evidence/m43-native-memory-short-key-owner-reduction/final-native-benchmark/oracle-vs-yune-candidate-output.md)
- M43 visualizations:
  [`./evidence/m43-native-memory-short-key-owner-reduction/visuals/`](./evidence/m43-native-memory-short-key-owner-reduction/visuals/)
