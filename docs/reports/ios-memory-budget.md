# iOS-Budget Native Memory Report

Date: 2026-06-28

This report measures Yune's **native single-active-schema memory** against an iOS
keyboard-extension budget (hard **64 MB**, target **48 MB**). It is a different
lens from
[`yune-vs-librime-performance.md`](./yune-vs-librime-performance.md) (which tracks
librime *parity*): here the bar is a **product memory budget**, because the
intended product is a Cantoboard-style iOS keyboard
([Cantoboard](https://github.com/Cantoboard/Cantoboard),
[librime-ios-build](https://github.com/Cantoboard/librime-ios-build)). The
reduction work it motivates is tracked by
[`plans/active/m47-plan-ios-budget-native-memory-reduction.md`](../plans/active/m47-plan-ios-budget-native-memory-reduction.md).

Findings were adversarially verified (2026-06-28 workflow: 4 refute agents +
synthesis); the calibration corrections below are folded in.

## Harness hygiene (read first)

**Every memory number must name its harness.** Three different processes give
three different numbers; only one is the iOS proxy:

| Source | jyut6ping3_mobile | What it is |
| --- | ---: | --- |
| **Lean native probe** | **~298 MB steady / 482 MB peak** | Real `RimeApi`, mmap path, prebuilt assets, one schema per fresh Rust process — **the iOS proxy** |
| .NET in-process benchmark (Track B) | ~436 MB after-ready / 504 MB peak | A .NET process hosting **both** Yune *and* librime DLLs — polluted upper bound, **not** the iOS number |
| Browser WASM | 160 MiB | WASM linear memory, **no mmap**, owned tables — a different deployment |

The earlier "Jyutping native ~504 MB" headline was the dual-DLL harness; the
iOS-relevant steady is **~298 MB**.

## Methodology

Probe: [`crates/yune-rime-api/tests/native_memory_probe.rs`](../../crates/yune-rime-api/tests/native_memory_probe.rs)
(evidence-only, `#[ignore]`). It drives the native `RimeApi`
(`setup`→`initialize`→`deploy`→`create_session`→`select_schema`→`process_key`)
with `prebuilt_data_dir` set to the committed `.bin` bundle, so `deploy()`
**reuses + mmaps** the compiled assets instead of rebuilding — exactly how an iOS
keyboard loads precompiled bundle assets. It reads this process's own
`WorkingSet64`/`PeakWorkingSet64` at each phase; one schema per fresh process
removes cross-schema contamination; the ~5 MB harness baseline is recorded and
subtracted. Verified faithful (C1, high confidence): the native load path mmaps
via `memmap2` (`schema_install.rs`), and the search order returns the deployed
staging copy.

## Measurement caveat (what this proxy does and does not say)

Windows `WorkingSet64` is **total resident physical memory**, including clean,
shared, file-backed (mmap'd) pages. iOS jetsam accounts a keyboard extension
roughly by **`phys_footprint` (dirty + compressed memory)**, which *excludes*
clean mmap'd pages that can be evicted and re-faulted. So the iOS number for the
same engine could be **lower** than this Windows working set — but, as GPT's
review put it, *a ~300 MB native session footprint does not become 48 MB by
platform accounting alone*. M47 Phase 0 now records Windows `PrivateUsage` and
allocator-live evidence for the dirty/private side of that gap; treat every
number here as a Windows proxy that justifies the work, not as an iOS
`phys_footprint`. The on-device figure is a later Phase 2 validation gate.

**Independent reproduction (GPT review, 2026-06-28, separate Windows machine):**
after_deploy `14.9`, after_session `297.5`, after_select/steady `302.3`, peak
`486.0` MB — same shape and magnitude as below (deploy cheap; `create_session`
is the owner; ~300 MB steady / ~486 MB peak).

## Findings

### 1. The cost is at `create_session`, not `deploy` (verified, high confidence)

jyut6ping3_mobile, lean probe, phase by phase:

| Phase | Working set |
| --- | ---: |
| baseline (harness) | 5.2 MB |
| after `setup`+`initialize` | 5.9 MB |
| **after `deploy()`** | **10.8 MB** |
| **after `create_session()`** | **293.4 MB** (+283 MB) |
| after `select_schema` | 298.3 MB |
| after typing | 298.4 MB |
| **steady** | **298.4 MB** |
| **peak** | **482.0 MB** |

`deploy()` (compile/reuse) is nearly free. The +283 MB lands at
`create_session()`, which eagerly loads the workspace **default** schema's
dictionary and materializes heap structures
(`apply_initial_schema_to_session` → `install_schema_translator_chain` →
`CompactTableStore::from_table_bin_byte_source` + `read_yune_table_advanced_payload`
+ `StaticTableTranslator::from_compact_table_store`, which collects
`all_codes()` into a `normal_codes` HashSet).

### 2. It scales with the default dictionary, not a fixed cost (verified)

By patching `default.yaml`'s `schema_list` to isolate the default schema that
`create_session` loads:

| Default/active schema | Steady WS | Peak WS |
| --- | ---: | ---: |
| luna_pinyin | **62.6 MB** | 294 MB |
| cangjie5 | **94.9 MB** | 102 MB |
| jyut6ping3_mobile | **~298 MB** | 482 MB |

The earlier "~constant ~300 MB across schemas" was an artifact of `create_session`
always loading the default `jyut6ping3` before any switch. Only jyut shows a large
steady↔peak gap (a ~184 MB transient that frees back); luna's 294 MB peak frees to
62 MB steady.

### 3. What it is *not* (verified)

- **Not the sentence model.** A/B disabling `enable_sentence`+`enable_completion`
  for jyut moved steady 298→297 MB (≈ 0). `poet`/sentence-model owner rows are 0
  bytes for jyut (`with_upstream_sentence_model` is only used for luna_pinyin).
- **Not OpenCC.** All `.ocd2` simplifier dictionaries total **77 KB**.

### 4. Pre-Phase-0 classification left ~235 MB un-owned

Before M47 Phase 0, the owner profile
([`track-b-yune-product/memory-owner-profile.csv`](./evidence/current-performance-dashboard-2026-06-28/native-current-benchmark/track-b-yune-product/memory-owner-profile.csv)):

| Owner | Bytes | Class |
| --- | ---: | --- |
| `compact_table.lookup_records` (primary) | 31.9 MB | heap_owned_required |
| `compact_table.lookup_records` (secondary) | 8.1 MB | heap_owned_required (dictionary loaded twice) |
| `compact_table.syllabary_codes` | ~4.2 MB | heap_owned_reducible |
| `compact_table.storage` (table) | ~15 MB | **mmap_file_backed** (not private heap) |
| **Classified heap-required** | **~44 MB** | |

Against the lean probe's **298 MB** steady, classified heap (~44 MB) + mmap'd
file pages (~18 MB, which are clean/shared, not private dirty) left **~235 MB
un-owned**. That was the Phase 0 blocker, not the final attribution. The closeout
below adds private bytes, allocator live/high-water, filter owner rows, and
inside-load phase markers.

## Phase 0 attribution closeout (Windows-measured, 2026-06-28)

Evidence:
[`evidence/m47-ios-budget-native-memory-attribution-2026-06-28/`](./evidence/m47-ios-budget-native-memory-attribution-2026-06-28/)
from the extended native probe. This is Windows `WorkingSetSize` /
`PrivateUsage` evidence plus a test-local counting allocator, **not** iOS
`phys_footprint`.

Two runs matter:

| Run | Steady WS | Peak WS | Steady private | Allocator live | Allocator high-water | Named heap owners | Clean mmap estimate |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Default-list baseline (`default.yaml` first schema, then select `jyut6ping3_mobile`) | **301.2 MB** | **485.1 MB** | **284.7 MB** | **191.5 MB** | **415.1 MB** | 152.7 MB after `create_session` | 14.5 MB after `create_session` |
| Isolated mobile default (`YUNE_MEM_DEFAULT=jyut6ping3_mobile`) | **223.9 MB** | **231.5 MB** | **202.4 MB** | **155.0 MB** | **164.8 MB** | **96.6 MB** | **19.0 MB** |

Phase 0 verdict: the steady-state bulk is **not primarily allocator-retained
free memory**. The isolated mobile run ends at `~202 MB` private bytes with
`~155 MB` allocator-live bytes and only `~165 MB` allocator high-water, so an
allocator decay strategy cannot be the first branch. The remaining unclassified
steady mass is live retained heap / process-private memory that needs owner
reduction.

The formerly "un-owned" bucket is now smaller and more concrete:

- Default-list `create_session`: `~285 MB` private, `~220 MB` allocator live,
  `~153 MB` named heap, `~15 MB` clean mmap estimate. Remaining unclassified
  private is roughly `~132 MB`; unclassified Rust-allocator live is roughly
  `~68 MB`.
- Isolated mobile: `~202 MB` private, `~155 MB` allocator live, `~97 MB` named
  heap, `~19 MB` clean mmap estimate. Remaining unclassified private is roughly
  `~106 MB`; unclassified Rust-allocator live is roughly `~58 MB`.

The double/side load is confirmed and owner-named:

| Owner | Bytes | Required for normal keyboard candidate text? | Verdict |
| --- | ---: | --- | --- |
| Primary `jyut6ping3` compact table `lookup_records` | 31.9 MB | No; rich lookup/comment records, not candidate generation | Lazy/skip candidate for keyboard-only path |
| Secondary `luna_pinyin_yune_reverse` compact table `lookup_records` | 13.8 MB | No for unprefixed Jyutping; used by grave-prefix Mandarin reverse lookup UI | Lazy-load on reverse trigger or optional UI pack |
| `dictionary_lookup_filter.lookup_records` (`jyut6ping3_scolar`) | 50.7 MB | No; dictionary-panel/comment enrichment filter | Highest-leverage lazy/optional branch |

Recommended first reduction branch: **eager-materialization removal for
dictionary-panel/reverse/comment payloads**, starting with
`dictionary_lookup_filter.lookup_records`, then compact `lookup_records`, and
then lazy loading for the `luna_pinyin_yune_reverse` reverse UI translator. Keep
candidate-output correctness gates green; this is not an allocator-strategy
branch.

## Verdict (Windows-measured)

Against the 64 MB hard budget: jyut6ping3_mobile steady **~4.6× over**, peak
**~7.5× over**; cangjie5 ~1.5× over; luna_pinyin borderline-under at steady but
~4.6× over at peak. Even a small-schema base is ~60–95 MB. The multilingual
Jyutping keyboard **does not fit** in its current shape, and closing the gap is
**architecture-level work, not tuning**. The portable levers (attribute and remove
the ~235 MB, drop `create_session` eager materialization, fix the double load,
bound the transient, consider a decaying allocator, slim the mobile profile) are
all Windows-implementable and benefit every platform. **No "iOS-ready" claim**
until a later real-Apple-device validation pass.

## Next work

Phase 0 is complete. Start the first reduction branch by making
dictionary-panel/reverse/comment payloads lazy or optional on the keyboard path,
then re-run the M47 probe. Do not use allocator changes as the first branch
unless a later allocator-specific probe shows a steady live-vs-resident gap that
this Phase 0 run did not show.

## Evidence

- Phased + per-schema working set:
  [`evidence/ios-memory-budget-2026-06-28/native-working-set-by-phase.csv`](./evidence/ios-memory-budget-2026-06-28/native-working-set-by-phase.csv)
- Owner profile (Track B):
  [`evidence/current-performance-dashboard-2026-06-28/native-current-benchmark/track-b-yune-product/memory-owner-profile.csv`](./evidence/current-performance-dashboard-2026-06-28/native-current-benchmark/track-b-yune-product/memory-owner-profile.csv)
- M47 Phase 0 attribution:
  [`evidence/m47-ios-budget-native-memory-attribution-2026-06-28/`](./evidence/m47-ios-budget-native-memory-attribution-2026-06-28/)
- Probe: [`crates/yune-rime-api/tests/native_memory_probe.rs`](../../crates/yune-rime-api/tests/native_memory_probe.rs)
