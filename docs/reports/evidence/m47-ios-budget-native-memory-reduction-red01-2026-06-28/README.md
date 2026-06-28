# M47 RED-01 Native Memory Reduction Evidence

Date: 2026-06-28

Harness: Windows native `RimeApi` lean probe, `crates/yune-rime-api/tests/native_memory_probe.rs`, using `PROCESS_MEMORY_COUNTERS_EX` for working set, peak working set, and private bytes plus a test-local counting global allocator for live/high-water Rust heap approximation. These are Windows `WorkingSetSize` / `PrivateUsage` measurements, not iOS `phys_footprint`.

## Change

RED-01 adds an explicit schema/profile gate:

```yaml
dictionary_lookup_filter:
  dictionary: jyut6ping3_scolar
  load_lookup_records: false
```

When the gate is false, `install_schema_dictionary_lookup_filter_from_config` skips loading and retaining `jyut6ping3_scolar` lookup records. Default behavior remains `true`, so public/web and TypeDuck-rich-comment paths continue to load the filter unless a keyboard/lean profile opts out.

## Runs

| Folder | Command shape | Purpose |
| --- | --- | --- |
| `before/` | `YUNE_MEM_SCHEMA=jyut6ping3_mobile`, `YUNE_MEM_DEFAULT=jyut6ping3_mobile` | Baseline isolated mobile default after M47 Phase 0. |
| `after/` | Same plus `YUNE_MEM_DISABLE_DICTIONARY_LOOKUP_RECORDS=1` | Keyboard-profile opt-out for `dictionary_lookup_filter.lookup_records`; probe patches the deployed temporary schema after `deploy()` and before `create_session()`. |

Each run writes `phase-memory.csv` / `.json`, `create-session-events.csv` / `.json`, `owner-attribution.csv` / `.json`, and `summary.json`.

## Before/After

| Metric | Before | After | Delta |
| --- | ---: | ---: | ---: |
| Steady working set | 223.9 MB | 169.2 MB | -54.7 MB |
| Steady private bytes | 202.2 MB | 147.7 MB | -54.5 MB |
| Steady allocator live | 155.0 MB | 104.9 MB | -50.1 MB |
| Peak working set | 231.6 MB | 217.3 MB | -14.3 MB |
| Named heap owners | 96.6 MB | 48.2 MB | -48.4 MB |

`dictionary_lookup_filter.lookup_records` moved from `50,695,595 B` / `127,144` records in `before/owner-attribution.csv` to no owner row in `after/owner-attribution.csv`. The after event stream records:

```text
m47:filter:dictionary_lookup_filter@dictionary_lookup_filter:dictionary:jyut6ping3_scolar:skip_dictionary_load:load_lookup_records=false
```

## Behavior

- Candidate correctness: preserved for the keyboard profile; the opt-out test keeps table candidate text and table comments while skipping dictionary-panel enrichment.
- Dictionary comments: default/eager behavior is preserved. Rich `jyut6ping3_scolar` dictionary-panel comments are intentionally disabled only when a profile sets `load_lookup_records: false`.
- Reverse lookup: unchanged. `script_translator@luna_pinyin` / `luna_pinyin_yune_reverse` remains loaded in this branch and is the next recommended owner.
- iOS status: no iOS-ready claim. This remains Windows proxy evidence.

## Verification

- `cargo test -p yune-rime-api select_schema_loads_typeduck_dictionary_lookup_filter -- --nocapture`
- `cargo test -p yune-rime-api select_schema_can_skip_typeduck_dictionary_lookup_records_for_keyboard_profile -- --nocapture`
- `cargo test -p yune-core dictionary_lookup_filter_ -- --nocapture`
- `cargo test -p yune-rime-api --test typeduck_windows_boundary yune_abi_jyut6ping3_ngohaig_comments_match_v112 -- --nocapture`
- `cargo test -p yune-rime-api --test typeduck_windows_boundary yune_abi_compiled_lookup_jyut6ping3_ngohaig_comments_match_v112 -- --nocapture`
- `cargo test -p yune-rime-api --test yune_web web03_public_demo_launch_schemas_byte_back_compiled_assets -- --nocapture`
- `cargo fmt --check`
- `$env:YUNE_MEM_SCHEMA='jyut6ping3_mobile'; $env:YUNE_MEM_DEFAULT='jyut6ping3_mobile'; $env:YUNE_MEM_DISABLE_DICTIONARY_LOOKUP_RECORDS='1'; $env:YUNE_MEM_EVIDENCE_DIR='docs/reports/evidence/m47-ios-budget-native-memory-reduction-red01-2026-06-28/after'; cargo test -p yune-rime-api --test native_memory_probe -- --ignored --exact native_memory_probe_reports_working_set --nocapture`
