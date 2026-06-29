# M47 Keyboard Profile Pin Evidence

Date: 2026-06-29

Harness: `crates/yune-rime-api/tests/native_memory_probe.rs`, Windows
`PROCESS_MEMORY_COUNTERS_EX.WorkingSetSize` / `PrivateUsage` plus the test-local
counting allocator. These are Windows proxy measurements, not iOS
`phys_footprint`.

## Runs

| Folder | Probe config | Steady WS | Steady private | Allocator live | Peak WS | Meaning |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `committed-default-jyut6ping3/` | `YUNE_MEM_SCHEMA=jyut6ping3` | 271.2 MB | 243.1 MB | 185.1 MB | 491.3 MB | Committed default workspace. |
| `full-mobile-jyut6ping3-mobile/` | `YUNE_MEM_SCHEMA=jyut6ping3_mobile`, `YUNE_MEM_DEFAULT=jyut6ping3_mobile` | 195.2 MB | 159.8 MB | 116.7 MB | 202.8 MB | Full mobile profile, no memory opt-outs. |
| `red05-lean-keyboard-proxy/` | `YUNE_MEM_SCHEMA=jyut6ping3_mobile`, `YUNE_MEM_DEFAULT=jyut6ping3_mobile`, RED-01/RED-03/RED-04 opt-outs | 58.1 MB | 23.3 MB | 16.0 MB | 79.6 MB | Fresh pre-RED-06 lean keyboard-profile baseline. |

## Opt-Out Profile

The lean keyboard profile uses:

- `YUNE_MEM_DISABLE_DICTIONARY_LOOKUP_RECORDS=1`
- `YUNE_MEM_DISABLE_COMPACT_LOOKUP_RECORDS=1`
- `YUNE_MEM_DISABLE_LUNA_REVERSE_TRANSLATOR=1`

These patch the temporary deployed schema to set
`dictionary_lookup_filter/load_lookup_records: false`,
`translator/load_lookup_records: false`, `luna_pinyin/load_lookup_records:
false`, and `luna_pinyin/load_translator: false`.

Verdict: the `~58 MB` row is the memory-light keyboard profile, not the full
mobile product profile. It intentionally omits TypeDuck rich dictionary-panel
comment bytes and grave-prefix Mandarin reverse UI while retaining normal
Jyutping typing behavior.
