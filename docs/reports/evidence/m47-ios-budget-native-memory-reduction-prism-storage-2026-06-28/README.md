# M47 RED-02 Prism Runtime Storage Evidence

Date: 2026-06-28

Harness: Windows native `RimeApi` lean memory probe for isolated `jyut6ping3_mobile`, with `YUNE_MEM_DEFAULT=jyut6ping3_mobile` and `YUNE_MEM_DISABLE_DICTIONARY_LOOKUP_RECORDS=1`. Values are Windows `WorkingSet64` / `PrivateUsage` proxy evidence, not iOS `phys_footprint`.

## Verdict

RED-02 succeeded as a native runtime heap reduction. The parsed prism double-array and spelling-map payloads moved from retained heap structures to byte-backed runtime reads over the existing compiled prism byte source.

Steady memory moved from 169.1 MB WS / 146.6 MB private / 104.9 MB allocator-live to 138.1 MB WS / 101.8 MB private / 66.6 MB allocator-live. Peak working set moved from 217.2 MB to 172.1 MB.

## Owner Movement

| Owner | Before | Current | Verdict |
| --- | ---: | ---: | --- |
| `prism.double_array_units` | 8,896,560 B heap | 8,896,512 B mmap-backed | moved out of parsed heap |
| `prism.spelling_map` | 31,337,240 B heap | 11,955,056 B mmap-backed | moved out of parsed heap; raw descriptor bytes are smaller than heap Vec expansion |
| `prism.corrections_tolerance` | 96 B heap | 96 B heap | unchanged small metadata |
| `compact_table.lookup_records` | 45,689,298 B heap | 45,689,298 B heap | next measured blocker |

The measured heap-owned named owner total dropped by 40,233,800 B. Mmap/file-backed owner rows increased by 20,851,568 B because the prism byte ranges are now counted as file-backed instead of parsed heap.

## Remaining Blocker

The top remaining heap owner is compact lookup-record materialization:

- primary `jyut6ping3_mobile`: 31,920,140 B, 127,144 records
- secondary `luna_pinyin_yune_reverse`: 13,769,158 B, 70,807 records

Recommended next branch: M47-RED-03 compact lookup-record strategy, before reverse/UI lazy loading, unless a product decision removes or gates those records for the keyboard path.
