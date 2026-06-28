# M47 Prism Attribution Correction Evidence

Date: 2026-06-28

Harness: Windows native `RimeApi` lean probe, `crates/yune-rime-api/tests/native_memory_probe.rs`, with `YUNE_MEM_DISABLE_DICTIONARY_LOOKUP_RECORDS=1`. Measurements are Windows `PROCESS_MEMORY_COUNTERS_EX.WorkingSetSize` / `PrivateUsage` plus the test-local allocator wrapper; they are not iOS `phys_footprint`.

## Purpose

RED-01 removed the eager `dictionary_lookup_filter.lookup_records` owner row, but the post-RED-01 run still had allocator live around `104.9 MB` while named heap owners were only `48.2 MB`. This attribution-only correction adds owner rows for parsed `RimePrismBinPayload` heap retained by `StaticTableTranslator`.

No RED-02 reduction is implemented here.

## Files

- `before/`: exact RED-01 after evidence copied from `../m47-ios-budget-native-memory-reduction-red01-2026-06-28/after/`.
- `current/`: regenerated post-RED-01 probe after adding parsed-prism owner rows.
- `owner-row-comparison.csv`: before/current owner table for the relevant repeated translator slots.
- `commands.txt`: commands used for the TDD red check, verification, and probe.

## Process Counters

| Metric | RED-01 before attribution | Current attribution run | Change |
| --- | ---: | ---: | ---: |
| Steady working set | 169.2 MB | 169.1 MB | noise only |
| Steady private bytes | 147.7 MB | 146.6 MB | noise only |
| Steady allocator live | 104.9 MB | 104.9 MB | 0.0 MB |
| Peak working set | 217.3 MB | 217.2 MB | noise only |
| Named heap owner bytes | 48.2 MB | 86.6 MB | +38.4 MB |
| Named total owner bytes | 70.3 MB | 108.7 MB | +38.4 MB |

This is an attribution correction, not a memory reduction. The runtime counters are effectively unchanged; the owner table now names `40,233,896 B` of previously unnamed retained heap.

## Prism Owners

| Owner | Current bytes | Notes |
| --- | ---: | --- |
| `prism.spelling_map` | `31,337,240 B` | Parsed `Vec<Vec<RimePrismSpellingDescriptor>>`; largest newly named owner. |
| `prism.double_array_units` | `8,896,560 B` | Parsed Darts double-array `Vec<u32>` units. |
| `prism.corrections_tolerance` | `96 B` | Empty correction/tolerance vector headers in this profile. |
| `prism.tips_payload` | `0 B` | No non-empty descriptor tips in this profile. |

The main keyboard prism (`jyut6ping3_mobile`) accounts for `28,767,208 B` spelling-map bytes and `8,388,632 B` double-array bytes. The reverse/UI prism (`luna_pinyin_yune_reverse`) accounts for `2,570,032 B` spelling-map bytes and `507,928 B` double-array bytes. `dictionary_lookup_filter.lookup_records` remains absent, confirming RED-01's opt-out stayed active.

## Verdict

The hypothesis is confirmed for most of the post-RED-01 unnamed heap gap: parsed prism payloads account for **~38.4 MB** of the remaining live heap. The largest newly named owner is the primary `jyut6ping3_mobile` parsed prism spelling map, followed by its parsed double-array units.

The next reduction branch should be a prism storage branch: make prism spelling-map descriptors and double-array units mmap/byte-backed or lazily parsed on demand. Do not start reverse-lookup work until this prism branch is evaluated, because the newly named primary prism payload is larger than the reverse/UI prism payload.

## Verification

- `cargo test -p yune-core compact_table_memory_owner_rows_cover_parsed_prism_payload_owner_set -- --nocapture`
- `cargo test -p yune-core prism -- --nocapture`
- `cargo test -p yune-rime-api select_schema_can_skip_typeduck_dictionary_lookup_records_for_keyboard_profile -- --nocapture`
- `$root='docs/reports/evidence/m47-ios-budget-native-memory-prism-attribution-2026-06-28'; $env:YUNE_MEM_SCHEMA='jyut6ping3_mobile'; $env:YUNE_MEM_DEFAULT='jyut6ping3_mobile'; $env:YUNE_MEM_DISABLE_DICTIONARY_LOOKUP_RECORDS='1'; $env:YUNE_MEM_EVIDENCE_DIR="$root/current"; cargo test -p yune-rime-api --test native_memory_probe -- --ignored --exact native_memory_probe_reports_working_set --nocapture`
