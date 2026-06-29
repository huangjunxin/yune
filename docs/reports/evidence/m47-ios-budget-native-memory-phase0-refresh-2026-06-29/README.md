# M47 Phase 0 Attribution Refresh Evidence

Date: 2026-06-29
Commit measured: `ebff6a3920146b8ed3d5669c6390337c248d802f`

Harness: Windows native `RimeApi` probe using `PROCESS_MEMORY_COUNTERS_EX`
`WorkingSetSize`, `PrivateUsage`, `PeakWorkingSetSize`, and the test-local
counting allocator in `crates/yune-rime-api/tests/native_memory_probe.rs`. These
are Windows proxy counters, not iOS `phys_footprint`.

## Verdict

This refresh does not start a reduction branch. It reruns the Phase 0 style
attribution probes on current `main` after RED-07.

The original ~235 MB un-owned M47 bucket is already closed by the earlier Phase 0
evidence as live retained heap / process-private memory, not allocator-retained
free memory. On current `main`, the remaining comments-intact and full-mobile
profiles no longer have a large un-owned heap bucket:

| Profile | Steady WS | Peak WS | Private | Alloc live / high | Named owners | Heap named | Clean mmap estimate | Shared/overlap | Lookup rows |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Comments-intact keyboard | 82.8 MB | 94.5 MB | 33.6 MB | 25.6 / 59.9 MB | 73,092,720 B | 4,195,278 B | 34,602,818 B | 34,292,637 B | 31,717,345 B |
| Full mobile | 92.9 MB | 103.7 MB | 41.5 MB | 32.3 / 59.9 MB | 86,124,829 B | 4,856,616 B | 40,740,505 B | 40,525,721 B | 37,253,867 B |

`PrivateUsage` is already below 48 MB for both current profiles, while working
set and peak remain over the target. The measured blocker is therefore not a bulk
allocator-retained/free-memory gap. The remaining resident size is dominated by
mapped/shared compiled table, prism, lookup payload, and compact index/code rows.
Windows cannot prove the exact iOS `phys_footprint` charge for clean file-backed
mappings; on-device `phys_footprint`/`vmmap` remains the later platform proof.

## Double Dictionary Load

The full-mobile probe confirms two translator dictionary owners:

- Primary keyboard translator `jyut6ping3`: `compact_table.storage` 15,248,382 B,
  `compact_table.lookup_records` 14,219,092 B, `prism.spelling_map` 10,965,828 B,
  `prism.double_array_units` 8,388,608 B, and `compact_table.syllabary_codes`
  4,189,674 B.
- Secondary/reverse translator `luna_pinyin_yune_reverse`: `compact_table.storage`
  4,640,555 B, `compact_table.lookup_records` 5,536,522 B,
  `prism.spelling_map` 989,228 B, `prism.double_array_units` 507,904 B, and
  `compact_table.syllabary_codes` 661,218 B.

The secondary/reverse translator is required for grave-prefix Mandarin reverse UI
behavior, not for normal unprefixed Jyutping keyboard candidate generation. The
comments-intact keyboard probe disables that translator and keeps rich TypeDuck
comments. The separate `dictionary_lookup_filter.lookup_records` owner from
`jyut6ping3_scolar` is dictionary-panel/comment enrichment, not normal candidate
text generation; on current `main` it is byte-backed/shared-overlapping rather
than retained eager lookup-record heap.

## Files

- `comments-intact-keyboard-current/` - rich comments retained, Luna reverse UI
  disabled.
- `full-mobile-current/` - no opt-outs.
- `profile-summary.csv/json` - profile-level current Phase 0 split.
- `owner-category-summary.csv/json` - owner rows grouped into heap, mmap,
  shared/overlap, overlap-estimate, and other.
- `phase0-marker-summary.csv/json` - selected create-session bisection markers.
- `asset-comparison.csv/json` - current Yune asset-size comparison against the
  TypeDuck-iOS reference values carried forward from RED-07 evidence.

## Commands

```powershell
$env:YUNE_MEM_SCHEMA='jyut6ping3_mobile'; $env:YUNE_MEM_DEFAULT='jyut6ping3_mobile'; $env:YUNE_MEM_DISABLE_LUNA_REVERSE_TRANSLATOR='1'; $env:YUNE_MEM_EVIDENCE_DIR='docs/reports/evidence/m47-ios-budget-native-memory-phase0-refresh-2026-06-29/comments-intact-keyboard-current'; cargo test -p yune-rime-api --test native_memory_probe -- --ignored --exact native_memory_probe_reports_working_set --nocapture
$env:YUNE_MEM_SCHEMA='jyut6ping3_mobile'; $env:YUNE_MEM_DEFAULT='jyut6ping3_mobile'; $env:YUNE_MEM_EVIDENCE_DIR='docs/reports/evidence/m47-ios-budget-native-memory-phase0-refresh-2026-06-29/full-mobile-current'; cargo test -p yune-rime-api --test native_memory_probe -- --ignored --exact native_memory_probe_reports_working_set --nocapture
cargo fmt --check
```
