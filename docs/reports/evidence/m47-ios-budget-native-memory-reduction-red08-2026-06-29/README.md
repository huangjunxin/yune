# M47 RED-08 Native Memory Reduction Evidence

Date: 2026-06-29

Harness: Windows native `RimeApi` probe using
`PROCESS_MEMORY_COUNTERS_EX.WorkingSetSize`, `PrivateUsage`,
`PeakWorkingSetSize`, and the test-local counting allocator in
`crates/yune-rime-api/tests/native_memory_probe.rs`. These are Windows proxy
counters, not iOS `phys_footprint`.

## Verdict

RED-08 is a behavior-preserving reduction for the comments-intact TypeDuck
keyboard profile. It keeps rich TypeDuck dictionary comments and normal Jyutping
typing behavior, but disables primary translator lookup-record retention in the
mobile schema and replaces the compact-table `all_codes()` -> `normal_codes`
`HashSet<String>` with storage-backed compact-table code lookup.

RED-08 also preserves reuse of the large primary and grave-prefix reverse
prebuilt prisms by normalizing runtime-only lookup gates without reordering YAML
checksum input. It does not close all startup hygiene: `jyut6ping3_scolar` still
rebuilds its tiny prism at deploy time, and that accounts for the remaining
near-80 MB deploy peak in these Windows proxy runs.

The steady reduction is exactly the primary `compact_table.lookup_records` owner
that Phase 0 refresh measured as shared/overlapping compiled payload:
`14,219,092 B`. The comments-intact keyboard profile moved from **82.8 MB** to
**67.4 MB** steady Windows working set and from **94.5 MB** to **80.1 MB** peak.
Full mobile moved from **92.9 MB** to **78.8 MB** steady and from **103.7 MB** to
**89.9 MB** peak.

This does not close M47. Both product-honest profiles remain above the 48 MB
steady target and 64 MB peak target on the Windows proxy. The remaining measured
blocker is not allocator-retained/free memory: private bytes are **22.5 MB** for
comments-intact keyboard and **28.1 MB** for full mobile, allocator live is
**16.3 MB** and **22.1 MB**, and named heap is only **4.2 MB** and **4.9 MB**.
The remaining resident bulk is clean/file-backed and shared/overlapping compiled
table, prism, rich-comment lookup, and compact index payload.

## Before / After

| Profile | Before steady / peak | After steady / peak | Before private | After private | Before allocator live | After allocator live | Named owners after |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Comments-intact keyboard | 82.8 / 94.5 MB | **67.4 / 80.1 MB** | 33.6 MB | **22.5 MB** | 25.6 MB | **16.3 MB** | `58,873,628 B` |
| Full mobile | 92.9 / 103.7 MB | **78.8 / 89.9 MB** | 41.5 MB | **28.1 MB** | 32.3 MB | **22.1 MB** | `71,905,737 B` |

## Remaining Owners

Comments-intact keyboard after RED-08:

- `dictionary_lookup_filter.lookup_records`: `17,498,253 B`,
  shared/overlapping rich-comment lookup payload.
- `compact_table.storage`: `15,248,382 B`, mmap/file-backed primary table.
- `prism.spelling_map`: `10,965,828 B`, mmap/file-backed primary prism payload.
- `prism.double_array_units`: `8,388,608 B`, mmap/file-backed primary prism
  double array.
- `compact_table.syllabary_codes`: `4,189,674 B`, remaining named heap.

Full mobile keeps the secondary grave-prefix reverse UI path, adding
`compact_table.lookup_records` `5,536,522 B` plus secondary reverse table/prism
storage. That reverse UI path is not required for normal unprefixed Jyutping
keyboard candidate generation.

The next reduction branch should target compiled asset/profile slimming and
compact payload/index format, not allocator strategy. The comparable six Yune
RIME bins are still about `50,674,920 B` versus TypeDuck-iOS `16,646,044 B`.

## Files

- `comments-intact-keyboard-after/` - rich comments retained, Luna reverse UI
  disabled.
- `full-mobile-after/` - no opt-outs.
- `profile-summary.csv/json` - before/after Windows working set, private bytes,
  allocator live/high-water, named owners, mmap estimate, shared payloads, and
  conclusions.
- `owner-category-summary.csv/json` - after-RED-08 owner rows grouped into heap,
  mmap/file-backed, shared/overlap, overlap-estimate, and other.
- `asset-comparison.csv/json` - Yune versus TypeDuck-iOS compiled asset-size
  comparison carried forward from the Phase 0 refresh evidence.

## Commands

```powershell
$env:YUNE_MEM_SCHEMA='jyut6ping3_mobile'; $env:YUNE_MEM_DEFAULT='jyut6ping3_mobile'; $env:YUNE_MEM_DISABLE_LUNA_REVERSE_TRANSLATOR='1'; $env:YUNE_MEM_EVIDENCE_DIR='docs/reports/evidence/m47-ios-budget-native-memory-reduction-red08-2026-06-29/comments-intact-keyboard-after'; cargo test -p yune-rime-api --test native_memory_probe -- --ignored --exact native_memory_probe_reports_working_set --nocapture
$env:YUNE_MEM_SCHEMA='jyut6ping3_mobile'; $env:YUNE_MEM_DEFAULT='jyut6ping3_mobile'; $env:YUNE_MEM_EVIDENCE_DIR='docs/reports/evidence/m47-ios-budget-native-memory-reduction-red08-2026-06-29/full-mobile-after'; cargo test -p yune-rime-api --test native_memory_probe -- --ignored --exact native_memory_probe_reports_working_set --nocapture
```
