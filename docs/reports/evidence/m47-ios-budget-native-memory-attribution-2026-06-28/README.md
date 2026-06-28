# M47 Phase 0 Native Memory Attribution Evidence

Date: 2026-06-28

Harness: Windows native `RimeApi` lean probe, `crates/yune-rime-api/tests/native_memory_probe.rs`, using `PROCESS_MEMORY_COUNTERS_EX` for working set, peak working set, and private bytes plus a test-local counting global allocator for live/high-water Rust heap approximation. These are Windows `WorkingSetSize` / `PrivateUsage` measurements, not iOS `phys_footprint`.

## Runs

| Folder | Command shape | Purpose |
| --- | --- | --- |
| `./` | `YUNE_MEM_SCHEMA=jyut6ping3_mobile`, repo default `default.yaml` | Reproduces the documented current default-list baseline: steady `~301.2 MB`, peak `~485.1 MB`, private `~284.7 MB`. |
| `isolated-mobile-default/` | `YUNE_MEM_SCHEMA=jyut6ping3_mobile`, `YUNE_MEM_DEFAULT=jyut6ping3_mobile` | Isolates `jyut6ping3_mobile` as the schema loaded at `create_session()`: steady `~223.9 MB`, peak `~231.5 MB`, private `~202.4 MB`. |

## Files

Each run writes:

- `phase-memory.csv` / `.json`: lifecycle phase counters.
- `create-session-events.csv` / `.json`: ordered bisection events inside schema load.
- `owner-attribution.csv` / `.json`: named owner snapshots after `create_session()` and at final steady state.
- `summary.json`: metric definitions and owner totals.

## Verdict

Phase 0 refutes "mostly allocator-retained free memory" as the steady-state explanation. In the isolated mobile run, steady private bytes are `~202.4 MB`, allocator live is `~155.0 MB`, and named heap owners are `~96.6 MB`; allocator high-water is only `~164.8 MB`. The remaining unclassified steady mass is therefore live retained heap / process-private memory that needs owner reduction, not a simple allocator decay win.

Highest-leverage first reduction branch: lazy or optional eager materialization for dictionary-panel and reverse/UI data:

- `dictionary_lookup_filter.lookup_records`: `50,695,595 B`, `jyut6ping3_scolar`, dictionary-panel/comment enrichment, not candidate generation.
- `compact_table.lookup_records` primary `jyut6ping3`: `31,920,140 B`, rich lookup/comment records.
- `compact_table.lookup_records` secondary `luna_pinyin_yune_reverse`: `13,769,158 B`, grave-prefix Mandarin reverse lookup UI path, not normal unprefixed Jyutping input.

The normal keyboard candidate source remains the primary `jyut6ping3` translator; the side dictionaries enrich panels/comments or reverse lookup behavior.
